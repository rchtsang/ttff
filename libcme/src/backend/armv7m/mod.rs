//! cm3.rs
//! 
//! cortex-m3 emulation context
//! 
//! implement the minimum necessary peripherals to emulate a cortex-m3
//! microprocessor.
use std::{
    fmt,
    collections::VecDeque,
    sync::Arc,
};

use thiserror::Error;
use flagset::{FlagSet, flags};

use fugue_ir::{
    disassembly::IRBuilderArena, Translator, VarnodeData
};
use fugue_core::prelude::*;
use fugue_core::eval::fixed_state::FixedState;

use crate::types::*;
use crate::utils::*;
use crate::peripheral::{
    self,
    Peripheral,
};
use crate::backend::Backend as BackendTrait;

mod userop;
mod system;
mod helpers;
mod mmap;
pub use mmap::*;
mod events;
pub use events::*;
mod exception;
pub use exception::*;
mod scs;
pub use scs::*;
mod faults;
pub use faults::*;


/// largest expected instruction 16 bytes in x86, 4 in ARM
const MAX_INSN_SIZE: usize = 4;
/// default proc_sp value is UNKNOWN. pick 0 for simplicity.
const DEFAULT_PROC_SP: u32 = 0;


#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("unpredictable behavior: {0}")]
    UnpredictableBehavior(&'static str),
    #[error("system error: {0}")]
    System(&'static str),
    #[error("invalid userop id: {0}")]
    InvalidUserOp(usize),
    #[error("invalid address: {0:#x}")]
    InvalidAddress(BitVec),
    #[error("invalid system control register: {0:#x?}")]
    InvalidSysCtrlReg(Address),
    #[error("unimplemented system control register: {0:?}")]
    UnimplementedSysCtrlReg(SCRegType),
    #[error("attempted to write to read-only address: {0:#x?}")]
    WriteAccessViolation(Address),
    #[error("attempted to read to write-only address: {0:#x?}")]
    ReadAccessViolation(Address),
    #[error("illegal access alignment @ [{0:#x?}; {1}], expected: {2:?}")]
    AlignmentViolation(Address, usize, Alignment),
}

impl From<Error> for super::Error {
    fn from(err: Error) -> Self {
        super::Error::Arch("armv7m", Arc::new(err.into()))
    }
}

/// armv7m operation mode
/// 
/// see armv7m arch ref manual B1.3.1
/// 
/// privileged execution state is lumped into operation mode
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    /// entered on reset and/or as result of exception return
    Thread,
    /// entered on exception. must be in handler mode to issue exception return.
    /// always privileged execution
    Handler(ExceptionType),
    /// entered if halt on debug event
    Debug,
}

impl Into<EmuThread> for Mode {
    fn into(self) -> EmuThread {
        match self {
            Mode::Thread => { EmuThread::Main },
            Mode::Handler(ref exc_type) => { EmuThread::ISR { num: exc_type.into() } },
            Mode::Debug => { panic!("debug mode is not a valid thread, need to save state") }
        }
    }
}

/// processor execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    Alive,
    WaitingForEvent,
    WaitingForInterrupt,
    Halted,
    Killed,
}

/// the cortex-m3 execution context
/// 
/// a context must contain all state information needed for execution, the evaluator should not require state
#[derive(Clone)]
pub struct Backend {
    id: usize,
    status: Status,
    lang: Language,
    endian: Endian,
    pc: VarnodeData,
    sp: VarnodeData,
    apsr: VarnodeData, // cpsr in ghidra sla

    /// execution mode
    mode: Mode,
    /// event register (B1.5.18)
    event: system::EVENT,
    /// armv7m xPSR is a combination of APSR, IPSR, and EPSR
    /// and is not defined as part of the ghidra sleigh spec.
    /// hence we must handle this manually
    xpsr: system::XPSR,
    /// banked main stack pointer (always used in handler mod)
    main_sp: Option<u32>,
    /// banked process stack pointer (optionally used in thread mode)
    proc_sp: Option<u32>,
    /// special-purpose CONTROL register (B1.4.4)
    control: system::CONTROL,
    primask: system::PRIMASK,
    faultmask: system::FAULTMASK,
    basepri: system::BASEPRI,

    regs: FixedState,
    tmps: FixedState,
    scs: SysCtrlSpace,
    mmap: MemoryMap,

    events: VecDeque<Event>,
}

impl<'irb> fmt::Debug for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context {{ id: {:#x} }}", self.id)
    }
}


impl Backend {

    pub fn new_with(
        builder: &LanguageBuilder,
        scs_config: Option<SysCtrlConfig>,
    ) -> Result<Self, super::Error> {
        let lang = builder.build("ARM:LE:32:Cortex", "default")?;
        let t = lang.translator();
        let scs_config = scs_config.unwrap_or_default();

        Ok(Self {
            id: 0,
            status: Status::Alive,
            pc: t.program_counter().clone(),
            sp: lang.convention().stack_pointer().varnode().clone(),
            endian: if t.is_big_endian() { Endian::Big } else { Endian::Little },
            mode: Mode::Thread,
            event: system::EVENT::default(),
            xpsr: system::XPSR(0),
            main_sp: None,
            proc_sp: None,
            control: system::CONTROL::default(),
            primask: system::PRIMASK::default(),
            faultmask: system::FAULTMASK::default(),
            basepri: system::BASEPRI::default(),
            apsr: t.register_by_name("cpsr").unwrap(),
            regs: FixedState::new(t.register_space_size()),
            tmps: FixedState::new(t.unique_space_size()),
            mmap: MemoryMap::default(),
            scs: SysCtrlSpace::new_from(scs_config),
            events: VecDeque::new(),
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
}

impl BackendTrait for Backend {
    fn lang(&self) -> &Language {
        &self.lang
    }

    fn current_thread(&self) -> EmuThread {
        self.mode.into()
    }

    fn do_isr_preempt(&self) -> Option<EmuThread> {
        // check the current execution priority,
        // then look at the first exception in the queue.
        // if it is higher, perform the context switch and 
        // return the new thread context
        // otherwise do nothing and return None.
        // the queue should be in sorted order, such that the
        // first element is always the highest priority
        todo!()
    }

    fn do_isr_return(&self) -> Option<EmuThread> {
        todo!()
    }

    fn map_mem(&mut self,
        base: &Address,
        size: usize,
    ) -> Result<(), super::Error> {
        assert!((*base + size as u64) < self.scs.range.start || *base >= self.scs.range.end,
            "cannot map memory in system control space");
        self.mmap.map_mem(base, size)
    }

    fn map_mmio(&mut self,
        peripheral: Peripheral,
    ) -> Result<(), super::Error> {
        let base = peripheral.base_address().offset();
        let size = peripheral.size() as u64;
        assert!(0x40000000 <= base && (base + size) < 0x50000000,
            "peripheral must be mapped into external MMIO space");
        self.mmap.map_mmio(peripheral)
    }

    fn read_pc(&self) -> Result<Address, super::Error> {
        let val = self.regs.read_val_with(
            self.pc.offset() as usize,
            self.pc.size(),
            self.endian
        )?;
        val.to_u64()
            .map(Address::from)
            .ok_or_else(| | super::Error::AddressInvalid(val))
    }

    fn write_pc(&mut self, address: &Address) -> Result<(), super::Error> {
        let val = BitVec::from(address.offset())
            .unsigned_cast(self.pc.bits());
        self.regs.write_val_with(
            self.pc.offset() as usize,
            &val,
            self.endian
        )?;
        Ok(())
    }

    fn read_sp(&self) -> Result<Address, super::Error> {
        let val = self.regs.read_val_with(
            self.sp.offset() as usize,
            self.sp.size(),
            self.endian
        )?;
        val.to_u64()
            .map(Address::from)
            .ok_or_else(| | super::Error::AddressInvalid(val))
    }

    fn write_sp(&mut self, address: &Address) -> Result<(), super::Error> {
        let val = BitVec::from(address.offset())
            .unsigned_cast(self.sp.bits());
        self.regs.write_val_with(
            self.sp.offset() as usize,
            &val,
            self.endian
        )?;
        Ok(())
    }

    fn fetch<'irb>(&self, address: &Address, irb: &'irb IRBuilderArena) -> LiftResult<'irb> {
        let mut lifter = self.lang.lifter();
        let bytes = self._mem_view_bytes(address, Some(MAX_INSN_SIZE))?;
        let pcode_result = lifter.lift(irb, address.clone(), bytes);
        if let Err(err) = pcode_result {
            return Err(Arc::new(err.into()));
        }
        let pcode = pcode_result.unwrap();
        let disasm_result = lifter.disassemble(irb, address.clone(), bytes);
        if let Err(err) = disasm_result {
            return Err(Arc::new(err.into()));
        }
        let disasm = disasm_result.unwrap();

        Ok(Arc::new(Insn { disasm, pcode }))
    }

    fn load(&mut self, address: &Address, size: usize) -> Result<BitVec, super::Error> {
        let big_endian = self.lang.translator().is_big_endian();
        let mut dst = vec![0u8; size];
        self.load_bytes(address, &mut dst)?;

        if big_endian {
            Ok(BitVec::from_be_bytes(&dst))
        } else {
            Ok(BitVec::from_le_bytes(&dst))
        }
    }

    fn store(&mut self, address: &Address, val: &BitVec) -> Result<(), super::Error> {
        let size = val.bytes();
        let mut src = vec![0u8; size];
        if self.lang.translator().is_big_endian() {
            val.to_be_bytes(&mut src);
        } else {
            val.to_le_bytes(&mut src);
        }

        self.store_bytes(address, &src)
    }

    fn read(&mut self, vnd: &VarnodeData) -> Result<BitVec, super::Error> {
        let spc = vnd.space();
        if spc.is_constant() {
            Ok(BitVec::from_u64(vnd.offset(), vnd.bits()))
        } else if spc.is_register() {
            Ok(self.regs.read_val_with(vnd.offset() as usize, vnd.size(), self.endian)?)
        } else if spc.is_unique() {
            Ok(self.tmps.read_val_with(vnd.offset() as usize, vnd.size(), self.endian)?)
        } else if spc.is_default() {
            self.load(&Address::from(vnd.offset()), vnd.size())
        } else {
            panic!("read from {spc:?} unsupported")
        }
    }

    fn write(&mut self, vnd: &VarnodeData, val: &BitVec) -> Result<(), super::Error> {
        let spc = vnd.space();
        if spc.is_register() {
            Ok(self.regs.write_val_with(vnd.offset() as usize, val, self.endian)?)
        } else if spc.is_unique() {
            Ok(self.tmps.write_val_with(vnd.offset() as usize, val, self.endian)?)
        } else if spc.is_default() {
            self.store(&Address::from(vnd.offset()), val)
        } else if spc.is_constant() {
            panic!("cannot write to constant varnode!")
        } else {
            panic!("write to {spc:?} unsupported")
        }
    }

    fn load_bytes(&mut self, address: &Address, dst: &mut [u8]) -> Result<(), super::Error> {
        if self._is_scs_region(address, dst.len()) {
            let offset = ((address.offset() as u32) - 0xe000e000u32) as usize;
            self.scs.read_bytes(offset, dst, &mut self.events)
        } else {
            self.mmap.load_bytes(address, dst, &mut self.events)
        }
    }

    fn store_bytes(&mut self, address: &Address, src: &[u8]) -> Result<(), super::Error> {
        if self._is_scs_region(address, src.len()) {
            let offset = ((address.offset() as u32) - 0xe000e000u32) as usize;
            self.scs.write_bytes(offset, src, &mut self.events)
        } else {
            self.mmap.store_bytes(address, src, &mut self.events)
        }
    }

    fn userop(
        &mut self,
        output: Option<&VarnodeData>,
        inputs: &[VarnodeData],
    ) -> Result<Option<fugue_core::ir::Location>, super::Error> {
        let (index, inputs, output) = get_userop_params(output, inputs);
        self._userop(index, inputs, output)
    }
}

impl Backend {
    fn _is_scs_region(&self, address: &Address, size: usize) -> bool {
        (*address + size as u64) < self.scs.range.end
        && *address >= self.scs.range.start
    }

    fn _mem_view_bytes(&self, address: &Address, size: Option<usize>) -> Result<&[u8], super::Error> {
        if self._is_scs_region(address, size.unwrap_or(0)) {
            todo!("view bytes in scs region")
        } else {
            self.mmap.mem_view_bytes(address, size)
        }
    }

    fn _mem_view_bytes_mut(&mut self, address: &Address, size: Option<usize>) -> Result<&mut [u8], super::Error> {
        if self._is_scs_region(address, size.unwrap_or(0)) {
            todo!("view bytes in scs region")
        } else {
            self.mmap.mem_view_bytes_mut(address, size)
        }
    }
}


#[cfg(test)]
mod tests;