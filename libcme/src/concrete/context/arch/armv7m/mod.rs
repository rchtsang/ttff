//! cm3.rs
//! 
//! cortex-m3 emulation context
//! 
//! implement the minimum necessary peripherals to emulate a cortex-m3
//! microprocessor.
use std::sync::Arc;
use std::ops::Range;

use thiserror::Error;
use nohash::IntMap;
use iset::IntervalMap;
use parking_lot::RwLock;
use flagset::{FlagSet, flags};

use fugue_ir::{
    disassembly::{IRBuilderArena, Opcode}, Translator, VarnodeData
};
use fugue_core::prelude::*;
use fugue_core::ir::Location;
use fugue_core::eval::fixed_state::FixedState;

use crate::concrete::{
    types::*,
    context,
    context::{CtxRequest, CtxResponse},
};
use crate::peripheral::Peripheral;

pub use crate::concrete::context::Context as ContextTrait;
pub type TranslationCache<'irb> = IntMap<u64, LiftResult<'irb>>;

mod userop;
mod exception;
pub use exception::*;
mod scs;
pub use scs::*;
mod systick;
pub use systick::*;
mod nvic;
pub use nvic::*;
mod mpu;
pub use mpu::*;
mod faults;
pub use faults::*;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("invalid userop id: {0}")]
    InvalidUserOp(usize),
}

impl Into<context::Error> for Error {
    fn into(self) -> context::Error {
        context::Error::from(super::Error::from(self))
    }
}

/// armv7m architecture event
/// 
/// these events can be triggered on writes to system control
/// and must be dealt with immediately with an update to the
/// context state if necessary
#[derive(Debug, Clone)]
pub enum Event {
    // ICSR and SHCSR
    ExceptionSetActive(ExceptionType, bool),
    ExceptionSetPending(ExceptionType, bool),
    ExceptionEnabled(ExceptionType, bool),

    // VTOR
    VectorTableOffsetWrite(u32),

    // AIRCR
    ExternSysResetRequest,   // (SYSRESETREQ) external system reset request
    LocalSysResetRequest,   // (VECTRESET) local system reset
    ExceptionClrAllActive,  // (VECTCLRACTIVE) clear all active state info for fixed and configurable exceptions, clear ipsr to 0
    VectorKeyWrite,         // (VECTKEY) 0x05fa written to vector key register

    // SCR keeps state that influences execution
    SetTransitionWakupEvent(bool),  // transitions from inactive to pending are/aren't wakeup events
    SetDeepSleep(bool),             // selected sleep state is/isn't deep sleep
    SetSleepOnExit(bool),           // enter/don't enter sleep state

    // CCR keeps state that influences executon
    ThreadModeExceptionsEnabled(bool),      // (NONBASETHRDENA) allow/disallow enter/return to thread mode with active exceptions (except priority boosting)
    STIRUnprivilegedAccessAllowed(bool),    // (USERSETMPEND) allow/disallow unprivileged access to the STIR
    UnalignedAccessTrapEnabled(bool),       // (UNALIGN_TRP) enable/disable trapping on unaligned word/halfword accesses
    DivideByZeroTrapEnabled(bool),          // (DIV_0_TRP) enable/disable trapping on divide by 0
    PreciseDataAccessFaultIgnored(bool),    // (BFHFNMIGN) set lockup/ignored for precise data access faults at priorities -1 or -2
    Stack8ByteAligned(bool),                // (STKALIGN) guarantee 4-byte/8-byte stack alignment w/ SP adjustment
    DataCacheEnabled(bool),                 // (DC) enable/disable data and unified caches
    InsnCacheEnabled(bool),                 // (IC) enable/disable instruction caches
    BranchPredictionEnabled(bool),          // (BP) enable/disable program flow prediction

    // SHPR sets system handler priorities, needed by exception/interrupt handling system
    SetSystemHandlerPriority { id: u8, priority: u8}, // (PRI_x in SHPR1, SHPR2, or SHPR3) set system handler x's priority level
    
    // CFSR, MMFSR, BFSR, UFSR, HFSR
    FaultStatusClr(Fault),
}

/// armv7m operation mode
/// 
/// see armv7m arch ref manual B1.3.1
/// 
/// privileged execution state is lumped into operation mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// entered on reset and/or as result of exception return
    Thread { privileged: bool, main_sp: bool },
    /// entered on exception. must be in handler mode to issue exception return.
    /// always privileged execution
    Handler,
    /// entered if halt on debug event
    Debug,
}

#[derive(Clone, Copy, Debug)]
enum MapIx {
    Mem(usize),
    Mmio(usize),
    Scs,
}

/// the cortex-m3 execution context
/// 
/// a context must contain all state information needed for execution, the evaluator should not require state
#[derive(Clone)]
pub struct Context<'irb> {
    lang: Language,
    endian: Endian,
    pc: VarnodeData,
    sp: VarnodeData,
    apsr: VarnodeData, // cpsr in ghidra sla

    /// execution mode
    mode: Mode,
    /// armv7m xPSR is a combination of APSR, IPSR, and EPSR
    /// and is not defined as part of the ghidra sleigh spec.
    /// hence we must handle this manually
    xpsr: u32,
    /// banked main stack pointer (always used in handler mod)
    main_sp: Option<u32>,
    /// banked process stack pointer (optionally used in thread mode)
    proc_sp: Option<u32>,

    regs: FixedState,
    tmps: FixedState,
    mmap: IntervalMap<Address, MapIx>,
    scs: SysCtrlSpace,
    mem: Vec<FixedState>,
    mmio: Vec<Peripheral>,
    irb: &'irb IRBuilderArena,
    cache: Arc<RwLock<TranslationCache<'irb>>>,
}


impl<'irb> Context<'irb> {

    pub fn new_with(
        builder: &LanguageBuilder,
        irb: &'irb IRBuilderArena,
        scs_config: Option<SysCtrlConfig>,
    ) -> Result<Self, context::Error> {
        let lang = builder.build("ARM:LE:32:Cortex", "default")?;
        let t = lang.translator();
        let scs_config = scs_config.unwrap_or_default();
        let mut mmap = IntervalMap::default();
        mmap.insert(Address::from(0xe000e000u64)..Address::from(0xe000f000u64), MapIx::Scs);

        Ok(Self {
            pc: t.program_counter().clone(),
            sp: lang.convention().stack_pointer().varnode().clone(),
            endian: if t.is_big_endian() { Endian::Big } else { Endian::Little },
            mode: Mode::Thread { privileged: false, main_sp: true },
            xpsr: 0u32,
            main_sp: None,
            proc_sp: None,
            apsr: t.register_by_name("cpsr").unwrap(),
            regs: FixedState::new(t.register_space_size()),
            tmps: FixedState::new(t.unique_space_size()),
            mmap,
            scs: SysCtrlSpace::new_from(scs_config),
            mem: vec![],
            mmio: vec![],
            cache: Arc::new(RwLock::new(TranslationCache::default())),
            irb,
            lang,
        })
    }

    pub fn translator(&self) -> &Translator {
        &self.lang.translator()
    }

    pub fn pc(&self) -> &VarnodeData {
        &self.pc
    }

    pub fn sp(&self) -> &VarnodeData {
        &self.sp
    }

    pub fn apsr(&self) -> &VarnodeData {
        &self.apsr
    }

    pub fn map_mem(&mut self,
        base: impl Into<Address>,
        size: usize,
    ) -> Result<(), context::Error> {
        let base = base.into();
        // mapped memory must be word-aligned
        assert_eq!(base.offset() & 0b11, 0, "base {base:#x?} is not word-aligned!");
        assert_eq!(size & 0b11, 0, "size {size:#x} is not word-aligned!");

        // check for collision with existing mapped regions
        let range = base..(base + size as u64);
        if let Some(colliding) = self.mmap.intervals(range.clone()).next() {
            return Err(context::Error::MapConflict(range, colliding));
        }

        // create memory and add to map
        let mem = FixedState::new(size);
        let idx = MapIx::Mem(self.mem.len());
        self.mem.push(mem);
        self.mmap.insert(range, idx);

        Ok(())
    }

    pub fn map_mmio(&mut self,
        peripheral: Peripheral,
    ) -> Result<(), context::Error> {
        // peripheral base must be word-aligned
        assert_eq!(peripheral.range.start.offset() & 0b11, 0,
            "peripheral is not word-aligned!");

        // check for collision with existing mapped regions
        let range = peripheral.range.clone();
        if let Some(colliding) = self.mmap.intervals(range.clone()).next() {
            return Err(context::Error::MapConflict(range, colliding));
        }

        // add peripheral to map
        let idx = MapIx::Mmio(self.mmio.len());
        self.mmio.push(peripheral);
        self.mmap.insert(range, idx);

        Ok(())
    }
}

impl<'irb> context::Context<'irb> for Context<'irb> {
    fn lang(&self) -> &Language {
        &self.lang
    }

    fn request<'ctx>(&'ctx mut self, req: CtxRequest) -> CtxResponse<'irb> {
        match req {
            CtxRequest::Fetch { address } => {
                CtxResponse::Fetch { result: self._fetch(address) }
            }
            CtxRequest::Read { vnd } => {
                CtxResponse::Read { result: self._read_vnd(vnd) }
            }
            CtxRequest::Write { vnd, val } => {
                CtxResponse::Write { result: self._write_vnd(vnd, val) }
            }
            CtxRequest::Load { address, size } => {
                CtxResponse::Load { result: self._map_read_val(address, size) }
            }
            CtxRequest::Store { address, val } => {
                CtxResponse::Store { result: self._map_write_val(address, val) }
            }
            CtxRequest::LoadBytes { address, dst } => {
                CtxResponse::LoadBytes { result: self._map_read_bytes(address, dst) }
            }
            CtxRequest::StoreBytes { address, bytes} => {
                CtxResponse::StoreBytes { result: self._map_write_bytes(address, bytes) }
            }
            CtxRequest::ReadPc => {
                CtxResponse::ReadPc { result: self._get_pc() }
            }
            CtxRequest::WritePc { address } => {
                CtxResponse::WritePc { result: self._set_pc(address) }
            }
            CtxRequest::ReadSp => {
                CtxResponse::ReadSp { result: self._get_sp() }
            }
            CtxRequest::WriteSp { address } => {
                CtxResponse::WriteSp { result: self._set_sp(address) }
            }
            CtxRequest::CallOther { output, inputs } => {
                assert!(inputs[0].space().is_constant(), "input0 of userop must be constant id per pcode spec");
                let index = inputs[0].offset() as usize;
                CtxResponse::CallOther { result: self._userop(index, &inputs[1..], output) }
            }
        }
    }
}

// private implementations
impl<'irb> Context<'irb> {

    fn _get_pc(&self) -> Result<Address, context::Error> {
        let val = self.regs.read_val_with(
            self.pc.offset() as usize,
            self.pc.size(),
            self.endian
        )?;
        val.to_u64()
            .map(Address::from)
            .ok_or_else(| | context::Error::AddressInvalid(val))
    }

    fn _set_pc(&mut self, address: Address) -> Result<(), context::Error> {
        let val = BitVec::from(address.offset())
            .unsigned_cast(self.pc.bits());
        self.regs.write_val_with(
            self.pc.offset() as usize,
            &val,
            self.endian
        )?;
        Ok(())
    }

    fn _get_sp(&self) -> Result<Address, context::Error> {
        let val = self.regs.read_val_with(
            self.sp.offset() as usize,
            self.sp.size(),
            self.endian
        )?;
        val.to_u64()
            .map(Address::from)
            .ok_or_else(| | context::Error::AddressInvalid(val))
    }

    fn _set_sp(&mut self, address: Address) -> Result<(), context::Error> {
        let val = BitVec::from(address.offset())
            .unsigned_cast(self.sp.bits());
        self.regs.write_val_with(
            self.sp.offset() as usize,
            &val,
            self.endian
        )?;
        Ok(())
    }

    fn _lift_block(&mut self,
        address: impl Into<Address>,
        // irb: &'irb IRBuilderArena,
    ) {
        let mut lifter = self.lang.lifter();
        let base = address.into();
        let mut offset = 0usize;
        // largest expected instruction 16 bytes
        const MAX_INSN_SIZE: usize = 16;
        
        let mut branch = false;
        while !branch {
            let address = base + offset as u64;

            let read_result = self._mem_view_bytes(address, MAX_INSN_SIZE);
            if let Err(err) = read_result {
                // read failed
                self.cache.write().insert(address.offset(), Err(err));
                break;
            }
            let bytes = read_result.unwrap();
            let lift_result = Self::_lift(self.irb, address, bytes, &mut lifter);
            if lift_result.is_err() {
                self.cache.write().insert(address.offset(), lift_result);
                break;
            }
            let insn = lift_result.unwrap();
            let pcode = &insn.pcode;
            
            offset += pcode.len();

            match pcode.operations.last().unwrap().opcode {
                Opcode::Branch
                | Opcode::CBranch
                | Opcode::IBranch
                | Opcode::Call
                | Opcode::ICall
                | Opcode::Return
                | Opcode::CallOther => {
                    // usually we can tell if the last opcode is branching
                    branch = true;
                },
                _ => {
                    // otherwise we need to check if the pc gets written to
                    // this may never happen in pcode semantics but idk for sure.
                    // we leave it commented out for now b/c it probably doesn't matter and better performance
                    // if it turns out it's possible we will uncomment and kill this comment
                    // branch = pcode.operations.iter().any(|pcodedata| {
                    //     if let Some(vnd) = pcodedata.output {
                    //         vnd == self.pc
                    //     } else {
                    //         false
                    //     }
                    // });
                },
            }

            self.cache.write().insert(address.offset(), Ok(insn));
        }

        // maybe return something here at some point?
    }

    fn _lift(
        irb: &'irb IRBuilderArena,
        address: impl Into<Address>,
        bytes: &[u8],
        lifter: &mut Lifter,
    ) -> LiftResult<'irb> {
        let address = address.into();
        let pcode_result = lifter.lift(irb, address.clone(), bytes);
        if let Err(err) = pcode_result {
            return Err(err.into());
        }
        let pcode = pcode_result.unwrap();
        let disasm_result = lifter.disassemble(irb, address.clone(), bytes);
        if let Err(err) = disasm_result {
            return Err(err.into());
        }
        let disasm = disasm_result.unwrap();

        Ok(Arc::new(Insn { disasm, pcode }))
    }

    fn _fetch(&mut self, address: impl Into<Address>) -> LiftResult<'irb> {
        let address = address.into();

        if !self.cache.read().contains_key(&address.offset()) {
            self._lift_block(address);
        }

        self.cache.read()
            .get(&address.offset())
            .ok_or(context::Error::AddressNotLifted(address.clone()))?
            .clone()
    }

    fn _get_mapped_region(&self, address: impl Into<Address>) -> Result<(Range<Address>, MapIx), context::Error> {
        let address: Address = address.into();
        let mut overlaps = self.mmap.overlap(address.clone());
        let (range, val) = overlaps.next()
            .ok_or(context::Error::Unmapped(address.clone()))?;
        if let Some((other_range, _)) = overlaps.next() {
            return Err(context::Error::MapConflict(range, other_range));
        }
        Ok((range, val.clone()))
    }

    fn _mem_view_bytes(&self, address: impl AsRef<Address>, size: usize) -> Result<&[u8], context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes(offset, size)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(_idx) => {
                panic!("mmio peripherals can't implement view_bytes due to their send/receive data model")
            }
            MapIx::Scs => {
                // viewing bytes in SCS will not trigger any arch events
                let state = self.scs.as_ref();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes(offset, size)
                    .map_err(context::Error::from)
            }
        }
    }

    fn _mem_view_bytes_mut(&mut self, address: impl AsRef<Address>, size: usize) -> Result<&mut [u8], context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get_mut(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes_mut(offset, size)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(_idx) => {
                panic!("mmio peripherals can't implement view_bytes_mut due to their send/receive data model")
            }
            MapIx::Scs => {
                panic!("scs can't implement view_bytes_mut without potentially violating event triggers")
            }
        }
    }

    fn _map_read_bytes(&mut self, address: impl AsRef<Address>, dst: &mut [u8]) -> Result<(), context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.read_bytes(offset, dst)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(idx) => {
                let peripheral = self.mmio.get_mut(idx).unwrap();
                peripheral.read_bytes(address, dst)
                    .map_err(context::Error::from)
            }
            MapIx::Scs => {
                let offset = (*address - range.start).offset() as usize;
                let maybe_evts = self.scs.read_bytes(offset, dst)
                    .map_err(context::Error::from)?;
                for evt in maybe_evts {
                    todo!("deal with generated events");
                }
                Ok(())
            }
        }
    }

    fn _map_write_bytes(&mut self, address: impl AsRef<Address>, src: &[u8]) -> Result<(), context::Error> {
        let address = address.as_ref();
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get_mut(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.write_bytes(offset, src)
                    .map_err(context::Error::from)
            }
            MapIx::Mmio(idx) => {
                let peripheral = self.mmio.get_mut(idx).unwrap();
                peripheral.write_bytes(address, src)
                    .map_err(context::Error::from)
            }
            MapIx::Scs => {
                let offset = (*address - range.start).offset() as usize;
                let maybe_evts = self.scs.write_bytes(offset, src)
                    .map_err(context::Error::from)?;
                for evt in maybe_evts {
                    todo!("deal with generated events");
                }
                Ok(())
            }
        }
    }

    fn _map_read_val(&mut self, address: impl AsRef<Address>, size: usize) -> Result<BitVec, context::Error> {
        let big_endian = self.lang.translator().is_big_endian();
        let mut dst = vec![0u8; size];
        self._map_read_bytes(address, &mut dst)?;

        if big_endian {
            Ok(BitVec::from_be_bytes(&dst))
        } else {
            Ok(BitVec::from_le_bytes(&dst))
        }
    }

    fn _map_write_val(&mut self, address: impl AsRef<Address>, val: &BitVec) -> Result<(), context::Error> {
        let size = val.bytes();
        let mut src = vec![0u8; size];
        if self.lang.translator().is_big_endian() {
            val.to_be_bytes(&mut src);
        } else {
            val.to_le_bytes(&mut src);
        }

        self._map_write_bytes(address, &src)
    }

    fn _read_vnd(&mut self, vnd: &VarnodeData) -> Result<BitVec, context::Error> {
        let spc = vnd.space();
        if spc.is_constant() {
            Ok(BitVec::from_u64(vnd.offset(), vnd.bits()))
        } else if spc.is_register() {
            Ok(self.regs.read_val_with(vnd.offset() as usize, vnd.size(), self.endian)?)
        } else if spc.is_unique() {
            Ok(self.tmps.read_val_with(vnd.offset() as usize, vnd.size(), self.endian)?)
        } else if spc.is_default() {
            self._map_read_val(Address::from(vnd.offset()), vnd.size())
        } else {
            panic!("read from {spc:?} unsupported")
        }
    }

    fn _write_vnd(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), context::Error> {
        let spc = vnd.space();
        if spc.is_register() {
            Ok(self.regs.write_val_with(vnd.offset() as usize, val, self.endian)?)
        } else if spc.is_unique() {
            Ok(self.tmps.write_val_with(vnd.offset() as usize, val, self.endian)?)
        } else if spc.is_default() {
            self._map_write_val(Address::from(vnd.offset()), val)
        } else if spc.is_constant() {
            panic!("cannot write to constant varnode!")
        } else {
            panic!("read from {spc:?} unsupported")
        }
    }
}



#[cfg(test)]
mod tests;