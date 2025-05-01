//! scs module
//! 
//! implementing the system control space

/*
 * TODO:
 * - implement endian sensitivity
 * - replace struct/int/byte conversions with unsafe std::mem::transmute for performance
 */
use std::ops::Range;
use std::fmt;
use bitfield_struct::bitfield;
use ahash::AHashMap;

use crate::types::Permission;

use crate::backend;
use super::*;

mod regs;
pub use regs::*;
mod exception;
use exception::ExceptionState;
pub mod nvic;
pub use nvic::*;
pub mod systick;
pub use systick::*;
pub mod mpu;
pub use mpu::*;
pub mod dcb;
pub use dcb::*;

mod helpers;

/// system control space base address
static BASE: u32 = 0xe000e000;

/// config containing reset values for scs registers
#[derive(Debug)]
pub struct SysCtrlConfig {
    map: AHashMap<SCRegType, u32>,
}

impl Default for SysCtrlConfig {
    fn default() -> Self {
        Self {
            map: AHashMap::default(),
        }
    }
}

/// system control space
/// 
/// memory-mapped 4kb address space containing 32-bit registers for
/// configuration, status, and control [0xe000e000, 0xe000efff]
/// 
/// ARM DDI 0403E.e B3.2
#[derive(Clone)]
pub struct SysCtrlSpace {
    pub range: Range<Address>,
    backing: Box<[u32; 0x400]>,
    pub exceptions: ExceptionState,
    pub mpu: MPUState,
}
// TODO: refactor backing into separate registers struct...

impl AsRef<[u32; 0x400]> for SysCtrlSpace {
    fn as_ref(&self) -> &[u32; 0x400] {
        self.backing.as_ref()
    }
}

impl AsRef<[u8]> for SysCtrlSpace {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl fmt::Debug for SysCtrlSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCS")
    }
}

impl SysCtrlSpace {
    pub fn new_from(config: SysCtrlConfig) -> Self {
        let range = Address::from(0xe000e000u64)..Address::from(0xe000f000u64);
        let mut backing = Box::new([0u32; 0x400]);
        let exceptions = ExceptionState::default();
        let mpu = MPUState::default();
        for (scregtype, reset_val) in config.map {
            let offset = scregtype.offset();
            backing[offset] = reset_val;
        }
        Self { range, backing, exceptions, mpu }
    }

    /// direct view into the scs as transmuted bytes
    pub fn view_as_bytes(&self) -> &[u8; 0x1000] {
        unsafe { &*(self.backing.as_ref() as *const [u32; 0x400] as *const [u8; 0x1000]) }
    }

    /// direct mutable view into the scs as transmuted bytes
    pub fn view_as_bytes_mut(&mut self) -> &mut [u8; 0x1000] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32; 0x400] as *mut [u8; 0x1000]) }
    }

    #[instrument]
    pub fn read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), backend::Error> {
        let address = BASE + offset as u32;
        let word_offset = offset / 4;
        let byte_offset = offset & 0b11;
        let Some(reg_type) = SCRegType::lookup_offset(offset) else {
            // if register isn't implemented as a struct yet, just treat it as
            // memory and issue a warning, returning the error that
            // must be ignored at a higher level.
            let err = backend::Error::from(Error::InvalidSysCtrlReg(address.into()));
            warn!("{err:?} (treated as memory)");
            let slice = unsafe {
                &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
            };
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            SCRegType::ICSR
            | SCRegType::VTOR
            | SCRegType::AIRCR
            | SCRegType::SCR
            | SCRegType::CCR
            | SCRegType::SHCSR
            | SCRegType::HFSR
            | SCRegType::DFSR
            | SCRegType::MMFAR
            | SCRegType::BFAR
            | SCRegType::AFSR
            | SCRegType::ICTR => {
                check_alignment(address, dst.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
                let slice = unsafe {
                    &*(&self.backing[word_offset] as *const u32 as *const[u8; 4])
                };
                dst.copy_from_slice(slice);
            }
            // SCRegType::CPACR => todo!(),
            // SCRegType::FPCCR => todo!(),
            // SCRegType::FPCAR => todo!(),
            // SCRegType::FPDSCR => todo!(),
            // SCRegType::MVFR0 => todo!(),
            // SCRegType::MVFR1 => todo!(),
            // SCRegType::MVFR2 => todo!(),
            // SCRegType::MCR => todo!(),
            // SCRegType::ACTLR => todo!(),
            SCRegType::STIR => {
                // write-only register
                let address: Address = (BASE + offset as u32).into();
                let err = Error::WriteAccessViolation(address);
                return Err(backend::Error::from(err).into());
            }
            SCRegType::SysTick(_streg_type) => {
                let mut stregs = self.systick_regs_mut();
                return stregs.read_bytes(offset, dst, events);
            }
            SCRegType::NVIC(_nvicreg_type) => {
                let mut nvicregs = self.nvic_regs_mut();
                return nvicregs.read_bytes(offset, dst, events);
            }
            SCRegType::MPU(_mpureg_type) => {
                let mut mpuregs = self.mpu_regs_mut();
                return mpuregs.read_bytes(offset, dst, events);
            }
            SCRegType::SHPR1(_)
            | SCRegType::SHPR2(_)
            | SCRegType::SHPR3(_)
            | SCRegType::CFSR
            | _ => {
                check_alignment(address, dst.len(), Alignment::Any)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
                let slice = unsafe {
                    &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
                };
                dst.copy_from_slice(&slice[byte_offset..]);
            }
        }
        Ok(())
    }

    #[instrument]
    pub fn write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), backend::Error> {
        let address = BASE + offset as u32;
        let word_offset = offset / 4;
        let byte_offset = offset & 0b11;
        let Some(reg_type) = SCRegType::lookup_offset(offset) else {
            let err = backend::Error::from(Error::InvalidSysCtrlReg(address.into()));
            warn!("{err:?} (treated as memory)");
            let slice = unsafe {
                &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
            };
            (&mut slice[byte_offset..]).copy_from_slice(src);
            return Err(err.into());
        };
        let write_val = src.iter()
            .enumerate().take(4)
            .fold(0u32, |val, (i, &byte)| {
                val | ((byte as u32) << i)
            });
        match reg_type {
            SCRegType::ICSR => {
                check_alignment(address, src.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;

                let icsr = self.get_icsr_mut();

                let masked_write_val = write_val & 0x9e000000;
                let new_icsr = ICSR::from_bits(masked_write_val);
                let new_pendstclr = new_icsr.pendstclr();
                let new_pendstset = new_icsr.pendstset();
                let new_pendsvclr = new_icsr.pendsvclr();
                let new_pendsvset = new_icsr.pendsvset();
                let new_nmipendset = new_icsr.nmipendset();

                if new_pendstclr & new_pendstset {
                    let err_str = "PENDSTSET and PENDSTCLR both set to 1";
                    let err = Error::UnpredictableBehavior(err_str);
                    warn!("{err:?}");
                    return Err(backend::Error::from(err).into());
                }
                if new_pendsvclr & new_pendsvset {
                    let err_str = "PENDSVSET and PENDSVCLR both set to 1";
                    let err = Error::UnpredictableBehavior(err_str);
                    warn!("{err:?}");
                    return Err(backend::Error::from(err).into());
                }

                if new_pendstclr { // TODO: does this self clear?
                    let excp = ExceptionType::SysTick;
                    let evt = Event::ExceptionSetPending(excp, false);
                    events.push_back(evt);
                }
                if new_pendstset & (new_pendstset ^ icsr.pendstset()) {
                    let excp = ExceptionType::SysTick;
                    let evt = Event::ExceptionSetPending(excp, true);
                    events.push_back(evt);
                }
                if new_pendsvclr { // TODO: does this self clear?
                    let excp = ExceptionType::PendSV;
                    let evt = Event::ExceptionSetPending(excp, false);
                    events.push_back(evt);
                }
                if new_pendsvset & (new_pendsvset ^ icsr.pendsvset()) {
                    let excp = ExceptionType::PendSV;
                    let evt = Event::ExceptionSetPending(excp, true);
                    events.push_back(evt);
                }
                if new_nmipendset & (new_nmipendset ^ icsr.nmipendset()) {
                    let excp = ExceptionType::NMI;
                    let evt = Event::ExceptionSetActive(excp, true);
                    events.push_back(evt);
                }
            }
            SCRegType::VTOR => {
                // let vtor = self.get_vtor_mut();
                // there appears to be a bug int the bitfield macro that makes
                // the 0 field private
                // vtor.0 = write_val & 0xffffff80;
                let vtor_ref = &mut self.backing[word_offset];
                *vtor_ref = write_val & 0xffffff80;
                let evt = Event::VectorTableOffsetWrite(*vtor_ref);
                events.push_back(evt);
            }
            SCRegType::AIRCR => {
                let masked_write_val = write_val & 0xffff0707;
                let new_aircr = AIRCR::from(masked_write_val);
                let new_vectreset = new_aircr.vectreset();
                let new_vectclractive = new_aircr.vectclractive();
                let new_sysresetreq = new_aircr.sysresetreq();
                let new_prigroup = new_aircr.prigroup();
                let new_vectkey = new_aircr.vectkey_stat();

                let dbg_state = self.debug_regs().get_debug_state();

                let aircr = self.get_aircr_mut();

                if new_vectreset {
                    if !dbg_state {
                        let err_str = "Write to VECTRESET while not halted in Debug state";
                        let err = Error::UnpredictableBehavior(err_str);
                        warn!("{err:?}");
                        return Err(backend::Error::from(err).into());
                    }
                    events.push_back(Event::LocalSysResetRequest);
                }
                if new_vectclractive {
                    if !dbg_state {
                        let err_str = "Write to VECTCLRACTIVE while not halted in Debug state";
                        let err = Error::UnpredictableBehavior(err_str);
                        warn!("{err:?}");
                        return Err(backend::Error::from(err).into());
                    }
                    events.push_back(Event::ExceptionClrAllActive);
                }
                if new_sysresetreq ^ aircr.sysresetreq() {
                    if new_sysresetreq {
                        // make local system reset request
                        events.push_back(Event::LocalSysResetRequest);
                    } else {
                        // clear local system reset request
                        // assuming it hasn't happened yet.
                        // don't know if this is actually correct behavior.
                        // arch doesn't specify.
                        let maybe_idx = events.iter()
                            .enumerate()
                            .find(|&(_, evt)| {
                                *evt == Event::LocalSysResetRequest
                            }).map(|(i, _)| i);
                        if let Some(idx) = maybe_idx {
                            let removed = events.remove(idx).unwrap();
                            assert_eq!(removed, Event::LocalSysResetRequest,
                                "removed the wrong event!");
                        }
                    }
                    aircr.set_sysresetreq(new_sysresetreq);
                }
                if new_prigroup != aircr.prigroup() {
                    events.push_back(Event::SetPriorityGrouping(new_prigroup));
                    aircr.set_prigroup(new_prigroup);
                }
                if new_vectkey == 0x05FA {
                    events.push_back(Event::VectorKeyWrite);
                }
            }
            SCRegType::SCR => {
                let masked_write_val = write_val & 0b10110;
                let new_scr = SCR::from_bits(masked_write_val);
                let new_sleeponexit = new_scr.sleeponexit();
                let new_sleepdeep = new_scr.sleepdeep();
                let new_sevonpend = new_scr.sevonpend();

                let scr = self.get_scr_mut();

                if new_sleeponexit ^ scr.sleeponexit() {
                    let evt = Event::SetSleepOnExit(new_sleeponexit);
                    events.push_back(evt);
                    scr.set_sleeponexit(new_sleeponexit);
                }
                if new_sleepdeep ^ scr.sleepdeep() {
                    let evt = Event::SetDeepSleep(new_sleepdeep);
                    events.push_back(evt);
                    scr.set_sleepdeep(new_sleepdeep);
                }
                if new_sevonpend ^ scr.sevonpend() {
                    let evt = Event::SetTransitionWakupEvent(new_sevonpend);
                    events.push_back(evt);
                    scr.set_sevonpend(new_sevonpend);
                }
            }
            SCRegType::CCR => {
                let masked_write_val = write_val & 0x0007031b;
                let new_ccr = CCR::from_bits(masked_write_val);
                let new_nonbasethrdena = new_ccr.nonbasethrdena();
                let new_usersetmpend = new_ccr.usersetmpend();
                let new_unalign_trp = new_ccr.unalign_trp();
                let new_div_0_trp = new_ccr.div_0_trp();
                let new_bfhfnmign = new_ccr.bfhfnmign();
                let new_stkalign = new_ccr.stkalign();
                let new_dc = new_ccr.dc();
                let new_ic = new_ccr.ic();
                let new_bp = new_ccr.bp();

                let current_val = self.backing[SCRegType::CCR.offset() / 4];
                let changed = CCR::from_bits(masked_write_val ^ current_val);

                let ccr = self.get_ccr_mut();

                if changed.nonbasethrdena() {
                    let evt = Event::ThreadModeExceptionsEnabled(new_nonbasethrdena);
                    events.push_back(evt);
                    ccr.set_nonbasethrdena(new_nonbasethrdena);
                }
                if changed.usersetmpend() {
                    let evt = Event::STIRUnprivilegedAccessAllowed(new_usersetmpend);
                    events.push_back(evt);
                    ccr.set_usersetmpend(new_usersetmpend);
                }
                if changed.unalign_trp() {
                    let evt = Event::UnalignedAccessTrapEnabled(new_unalign_trp);
                    events.push_back(evt);
                    ccr.set_unalign_trp(new_unalign_trp);
                }
                if changed.div_0_trp() {
                    let evt = Event::DivideByZeroTrapEnabled(new_div_0_trp);
                    events.push_back(evt);
                    ccr.set_div_0_trp(new_div_0_trp);
                }
                if changed.bfhfnmign() {
                    let evt = Event::PreciseDataAccessFaultIgnored(new_bfhfnmign);
                    events.push_back(evt);
                    ccr.set_bfhfnmign(new_bfhfnmign);
                }
                if changed.stkalign() {
                    let evt = Event::PreciseDataAccessFaultIgnored(new_stkalign);
                    events.push_back(evt);
                    // TODO: have some configuration that makes this RO/RW
                    ccr.set_stkalign(new_stkalign);
                }
                if changed.dc() {
                    let evt = Event::DataCacheEnabled(new_dc);
                    events.push_back(evt);
                    // TODO: have config that makes this RAZ/WI
                    ccr.set_dc(new_dc);
                }
                if changed.ic() {
                    let evt = Event::InsnCacheEnabled(new_ic);
                    events.push_back(evt);
                    // TODO: have config that makes this RAZ/WI
                    ccr.set_ic(new_ic);
                }
                if changed.bp() {
                    let evt = Event::BranchPredictionEnabled(new_bp);
                    events.push_back(evt);
                    // TODO: have config that makes this RAO/WI or RAZ/WI.
                    ccr.set_bp(new_bp);
                }
            }
            SCRegType::SHPR1(id)
            | SCRegType::SHPR2(id)
            | SCRegType::SHPR3(id) => {
                check_alignment(address, src.len(), Alignment::Any)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
                let slice = unsafe {
                    &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
                };
                let slice = &mut slice[byte_offset..];
                for (i, val) in src.iter().enumerate() {
                    if slice[i] == *val {
                        continue;
                    }

                    let typ = ExceptionType::from((id as usize + i) as u32);
                    let evt = Event::ExceptionSetPriority(typ, *val);
                    events.push_back(evt);
                    slice[i] = *val;
                }
            }
            SCRegType::SHCSR => {
                check_alignment(address, src.len(), Alignment::Word)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;

                let masked_write_val = write_val & 0x0007ff8f;
                let new_shcsr = SHCSR::from_bits(masked_write_val);
                let new_memfaultact = new_shcsr.memfaultact();
                let new_busfaultact = new_shcsr.busfaultact();
                let new_usgfaultact = new_shcsr.usgfaultact();
                let new_svcallact = new_shcsr.svcallact();
                let new_monitoract = new_shcsr.monitoract();
                let new_pendsvact = new_shcsr.pendsvact();
                let new_systickact = new_shcsr.systickact();
                let new_usgfaultpended = new_shcsr.usgfaultpended();
                let new_memfaultpended = new_shcsr.memfaultpended();
                let new_busfaultpended = new_shcsr.busfaultpended();
                let new_svcallpended = new_shcsr.svcallpended();
                let new_memfaultena = new_shcsr.memfaultena();
                let new_busfaultena = new_shcsr.busfaultena();
                let new_usgfaultena = new_shcsr.usgfaultena();

                let current_val = self.backing[SCRegType::SHCSR.offset() / 4];
                let changed = SHCSR::from_bits(masked_write_val ^ current_val);

                // let shcsr = self.get_shcsr_mut();

                if changed.memfaultact() {
                    let excp = ExceptionType::MemFault;
                    let evt = Event::ExceptionSetActive(excp, new_memfaultact);
                    events.push_back(evt);
                }
                if changed.busfaultact() {
                    let excp = ExceptionType::BusFault;
                    let evt = Event::ExceptionSetActive(excp, new_busfaultact);
                    events.push_back(evt);
                }
                if changed.usgfaultact() {
                    let excp = ExceptionType::UsageFault;
                    let evt = Event::ExceptionSetActive(excp, new_usgfaultact);
                    events.push_back(evt);
                }
                if changed.svcallact() {
                    let excp = ExceptionType::SVCall;
                    let evt = Event::ExceptionSetActive(excp, new_svcallact);
                    events.push_back(evt);
                }
                if changed.monitoract() {
                    let excp = ExceptionType::DebugMonitor;
                    let evt = Event::ExceptionSetActive(excp, new_monitoract);
                    events.push_back(evt);
                }
                if changed.pendsvact() {
                    let excp = ExceptionType::PendSV;
                    let evt = Event::ExceptionSetActive(excp, new_pendsvact);
                    events.push_back(evt);
                }
                if changed.systickact() {
                    let excp = ExceptionType::SysTick;
                    let evt = Event::ExceptionSetActive(excp, new_systickact);
                    events.push_back(evt);
                }
                if changed.usgfaultpended() {
                    let excp = ExceptionType::UsageFault;
                    let evt = Event::ExceptionSetPending(excp, new_usgfaultpended);
                    events.push_back(evt);
                }
                if changed.memfaultpended() {
                    let excp = ExceptionType::MemFault;
                    let evt = Event::ExceptionSetPending(excp, new_memfaultpended);
                    events.push_back(evt);
                }
                if changed.busfaultpended() {
                    let excp = ExceptionType::BusFault;
                    let evt = Event::ExceptionSetPending(excp, new_busfaultpended);
                    events.push_back(evt);
                }
                if changed.svcallpended() {
                    let excp = ExceptionType::SVCall;
                    let evt = Event::ExceptionSetPending(excp, new_svcallpended);
                    events.push_back(evt);
                }
                if changed.memfaultena() {
                    let excp = ExceptionType::MemFault;
                    let evt = Event::ExceptionEnabled(excp, new_memfaultena);
                    events.push_back(evt);
                }
                if changed.busfaultena() {
                    let excp = ExceptionType::BusFault;
                    let evt = Event::ExceptionEnabled(excp, new_busfaultena);
                    events.push_back(evt);
                }
                if changed.usgfaultena() {
                    let excp = ExceptionType::UsageFault;
                    let evt = Event::ExceptionEnabled(excp, new_usgfaultena);
                    events.push_back(evt);
                }
                // note that we do not update SHCSR here, but in handling generated events
                // since there are multiple ways to change interrupt state in software.
            }
            SCRegType::CFSR => todo!(),
            SCRegType::HFSR => todo!(),
            SCRegType::DFSR => todo!(),
            SCRegType::MMFAR => todo!(),
            SCRegType::BFAR => todo!(),
            SCRegType::AFSR => todo!(),
            // SCRegType::CPACR => todo!(),
            // SCRegType::FPCCR => todo!(),
            // SCRegType::FPCAR => todo!(),
            // SCRegType::FPDSCR => todo!(),
            // SCRegType::MVFR0 => todo!(),
            // SCRegType::MVFR1 => todo!(),
            // SCRegType::MVFR2 => todo!(),
            // SCRegType::MCR => todo!(),
            SCRegType::ICTR => todo!(),
            // SCRegType::ACTLR => todo!(),
            SCRegType::STIR => todo!(),
            SCRegType::SysTick(_streg_type) => {
                let mut stregs = self.systick_regs_mut();
                return stregs.write_bytes(offset, src, events);
            }
            SCRegType::NVIC(_nvicreg_type) => {
                let mut nvicregs = self.nvic_regs_mut();
                return nvicregs.write_bytes(offset, src, events);
            }
            SCRegType::MPU(_mpureg_type) => {
                let mut mpuregs = self.mpu_regs_mut();
                return mpuregs.write_bytes(offset, src, events);
            }
            _ => {
                check_alignment(address, src.len(), Alignment::Any)
                    .map_err(|(address, size, expected)| {
                        Error::AlignmentViolation(address, size, expected)
                    })?;
                let slice = unsafe {
                    &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
                };
                (&mut slice[byte_offset..]).copy_from_slice(src);
            }
        }
        Ok(())
    }

    /// get wrapper for interacting with systick registers
    pub fn systick_regs_mut(&mut self) -> SysTickRegsMut {
        let slice = &mut self.backing[..0x40];
        Self::_systick_regs_mut(slice)
    }

    fn _systick_regs_mut<'a>(backing: &'a mut [u32]) -> SysTickRegsMut<'a> {
        assert!(backing.len() >= 0x40, "backing not long enough");
        let backing = unsafe {
            &mut *(backing as *mut [u32] as *mut [u32; 0x40])
        };
        SysTickRegsMut::new(backing)
    }

    /// get wrapper for reading systick registers
    pub fn systick_regs(&self) -> SysTickRegs {
        let slice = &self.backing[..0x40];
        Self::_systick_regs(slice)
    }

    fn _systick_regs<'a>(backing: &'a [u32]) -> SysTickRegs<'a> {
        assert!(backing.len() >= 0x40, "backing not long enough");
        let backing = unsafe {
            &*(backing as *const [u32] as *const [u32; 0x40])
        };
        SysTickRegs::new(backing)
    }

    /// get wrapper for interacting with nvic registers
    pub fn nvic_regs_mut(&mut self) -> NVICRegsMut {
        let slice = &mut self.backing[..0x340];
        Self::_nvic_regs_mut(slice)
    }

    fn _nvic_regs_mut<'a>(backing: &'a mut [u32]) -> NVICRegsMut<'a> {
        assert!(backing.len() >= 0x340, "backing not long enough");
        let backing = unsafe {
            &mut *(backing as *mut [u32] as *mut [u32; 0x340])
        };
        NVICRegsMut::new(backing)
    }

    /// get wrapper for reading nvic registers
    pub fn nvic_regs(&self) -> NVICRegs {
        let slice = &self.backing[..0x340];
        Self::_nvic_regs(slice)
    }

    fn _nvic_regs<'a>(backing: &'a [u32]) -> NVICRegs<'a> {
        assert!(backing.len() >= 0x340, "backing not long enough");
        let backing = unsafe {
            &*(backing as *const [u32] as *const [u32; 0x340])
        };
        NVICRegs::new(backing)
    }

    /// get wrapper for interacting with mpu registers
    pub fn mpu_regs_mut(&mut self) -> MPURegsMut {
        let slice = &mut self.backing[..0xdec];
        Self::_mpu_regs_mut(slice)
    }

    fn _mpu_regs_mut<'a>(backing: &'a mut [u32]) -> MPURegsMut<'a> {
        assert!(backing.len() >= 0xdec, "backing not long enough");
        let backing = unsafe {
            &mut *(backing as *mut [u32] as *mut [u32; 0xdec])
        };
        MPURegsMut::new(backing)
    }

    /// get wrapper for reading mpu registers
    pub fn mpu_regs(&self) -> MPURegs {
        let slice = &self.backing[..0xdec];
        Self::_mpu_regs(slice)
    }

    fn _mpu_regs<'a>(backing: &'a [u32]) -> MPURegs<'a> {
        assert!(backing.len() >= 0xdec, "backing not long enough");
        let backing = unsafe {
            &*(backing as *const [u32] as *const [u32; 0xdec])
        };
        MPURegs::new(backing)
    }

    /// get wrapper for interacting with dbg registers
    pub fn debug_regs(&mut self) -> DebugRegs {
        let slice = &mut self.backing[..0x3c0];
        assert!(slice.len() >= 0x3c0, "backing not long enough");
        let backing = unsafe {
            &mut *(slice as *mut [u32] as *mut [u32; 0x3c0])
        };
        DebugRegs::new(backing)
    }
}

impl Default for SysCtrlSpace {
    fn default() -> Self {
        Self {
            range: Address::from(0xe000e000u64)..Address::from(0xe000f000u64),
            backing: Box::new([0u32; 0x400]),
            exceptions: ExceptionState::default(),
            mpu: MPUState::default(),
        }
    }
}

impl SysCtrlSpace {

    // register reference getters

    pub fn get_icsr(&self) -> &ICSR {
        let byte_offset = SCRegType::ICSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICSR) }
    }

    pub fn get_vtor(&self) -> &VTOR {
        let byte_offset = SCRegType::VTOR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const VTOR) }
    }

    pub fn get_aircr(&self) -> &AIRCR {
        let byte_offset = SCRegType::AIRCR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const AIRCR) }
    }

    pub fn get_scr(&self) -> &SCR {
        let byte_offset = SCRegType::SCR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SCR) }
    }

    pub fn get_ccr(&self) -> &CCR {
        let byte_offset = SCRegType::CCR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CCR) }
    }

    pub fn get_shpr1(&self) -> &SHPR1 {
        let byte_offset = SCRegType::SHPR1(0).offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR1) }
    }

    pub fn get_shpr2(&self) -> &SHPR2 {
        let byte_offset = SCRegType::SHPR2(0).offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR2) }
    }

    pub fn get_shpr3(&self) -> &SHPR3 {
        let byte_offset = SCRegType::SHPR3(0).offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR3) }
    }

    pub fn get_shcsr(&self) -> &SHCSR {
        let byte_offset = SCRegType::SHCSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHCSR) }
    }

    pub fn get_cfsr(&self) -> &CFSR {
        let byte_offset = SCRegType::CFSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CFSR) }
    }

    pub fn get_hfsr(&self) -> &HFSR {
        let byte_offset = SCRegType::HFSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const HFSR) }
    }

    pub fn get_dfsr(&self) -> &DFSR {
        let byte_offset = SCRegType::DFSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DFSR) }
    }

    pub fn get_mmfar(&self) -> &MMFAR {
        let byte_offset = SCRegType::MMFAR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const MMFAR) }
    }

    pub fn get_bfar(&self) -> &BFAR {
        let byte_offset = SCRegType::BFAR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const BFAR) }
    }

    // pub fn get_afsr(&self) -> &AFSR {
    //     let byte_offset = SCRegType::AFSR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const AFSR) }
    // }

    pub fn get_cpacr(&self) -> &CPACR {
        let byte_offset = SCRegType::CPACR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CPACR) }
    }

    // pub fn get_fpccr(&self) -> &FPCCR {
    //     let byte_offset = SCRegType::FPCCR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const FPCCR) }
    // }

    // pub fn get_fpcar(&self) -> &FPCAR {
    //     let byte_offset = SCRegType::FPCAR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const FPCAR) }
    // }

    // pub fn get_fpdscr(&self) -> &FPDSCR {
    //     let byte_offset = SCRegType::FPDSCR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const FPDSCR) }
    // }

    // pub fn get_mvfr0(&self) -> &MVFR0 {
    //     let byte_offset = SCRegType::MVFR0.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MVFR0) }
    // }

    // pub fn get_mvfr1(&self) -> &MVFR1 {
    //     let byte_offset = SCRegType::MVFR1.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MVFR1) }
    // }

    // pub fn get_mvfr2(&self) -> &MVFR2 {
    //     let byte_offset = SCRegType::MVFR2.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MVFR2) }
    // }

    // pub fn get_mcr(&self) -> &MCR {
    //     let byte_offset = SCRegType::MCR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MCR) }
    // }

    pub fn get_ictr(&self) -> &ICTR {
        let byte_offset = SCRegType::ICTR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICTR) }
    }

    // pub fn get_actlr(&self) -> &ACTLR {
    //     let byte_offset = SCRegType::ACTLR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const ACTLR) }
    // }

    pub fn get_stir(&self) -> &STIR {
        let byte_offset = SCRegType::STIR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const STIR) }
    }

    pub fn get_icsr_mut(&mut self) -> &mut ICSR {
        let byte_offset = SCRegType::ICSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICSR) }
    }

    pub fn get_vtor_mut(&mut self) -> &mut VTOR {
        let byte_offset = SCRegType::VTOR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut VTOR) }
    }

    pub fn get_aircr_mut(&mut self) -> &mut AIRCR {
        let byte_offset = SCRegType::AIRCR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut AIRCR) }
    }

    pub fn get_scr_mut(&mut self) -> &mut SCR {
        let byte_offset = SCRegType::SCR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SCR) }
    }

    pub fn get_ccr_mut(&mut self) -> &mut CCR {
        let byte_offset = SCRegType::CCR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CCR) }
    }

    pub fn get_shpr1_mut(&mut self) -> &mut SHPR1 {
        let byte_offset = SCRegType::SHPR1(0).offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR1) }
    }

    pub fn get_shpr2_mut(&mut self) -> &mut SHPR2 {
        let byte_offset = SCRegType::SHPR2(0).offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR2) }
    }

    pub fn get_shpr3_mut(&mut self) -> &mut SHPR3 {
        let byte_offset = SCRegType::SHPR3(0).offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR3) }
    }

    pub fn get_shcsr_mut(&mut self) -> &mut SHCSR {
        let byte_offset = SCRegType::SHCSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHCSR) }
    }

    pub fn get_cfsr_mut(&mut self) -> &mut CFSR {
        let byte_offset = SCRegType::CFSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CFSR) }
    }

    pub fn get_hfsr_mut(&mut self) -> &mut HFSR {
        let byte_offset = SCRegType::HFSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut HFSR) }
    }

    pub fn get_dfsr_mut(&mut self) -> &mut DFSR {
        let byte_offset = SCRegType::DFSR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DFSR) }
    }

    pub fn get_mmfar_mut(&mut self) -> &mut MMFAR {
        let byte_offset = SCRegType::MMFAR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MMFAR) }
    }

    pub fn get_bfar_mut(&mut self) -> &mut BFAR {
        let byte_offset = SCRegType::BFAR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut BFAR) }
    }

    // pub fn get_afsr_mut(&mut self) -> &mut AFSR {
    //     let byte_offset = SCRegType::AFSR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut AFSR) }
    // }

    pub fn get_cpacr_mut(&mut self) -> &mut CPACR {
        let byte_offset = SCRegType::CPACR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CPACR) }
    }

    // pub fn get_fpccr_mut(&mut self) -> &mut FPCCR {
    //     let byte_offset = SCRegType::FPCCR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut FPCCR) }
    // }

    // pub fn get_fpcar_mut(&mut self) -> &mut FPCAR {
    //     let byte_offset = SCRegType::FPCAR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut FPCAR) }
    // }

    // pub fn get_fpdscr_mut(&mut self) -> &mut FPDSCR {
    //     let byte_offset = SCRegType::FPDSCR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut FPDSCR) }
    // }

    // pub fn get_mvfr0_mut(&mut self) -> &mut MVFR0 {
    //     let byte_offset = SCRegType::MVFR0.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MVFR0) }
    // }

    // pub fn get_mvfr1_mut(&mut self) -> &mut MVFR1 {
    //     let byte_offset = SCRegType::MVFR1.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MVFR1) }
    // }

    // pub fn get_mvfr2_mut(&mut self) -> &mut MVFR2 {
    //     let byte_offset = SCRegType::MVFR2.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MVFR2) }
    // }

    // pub fn get_mcr_mut(&mut self) -> &mut MCR {
    //     let byte_offset = SCRegType::MCR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MCR) }
    // }

    pub fn get_ictr_mut(&mut self) -> &mut ICTR {
        let byte_offset = SCRegType::ICTR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICTR) }
    }

    // pub fn get_actlr_mut(&mut self) -> &mut ACTLR {
    //     let byte_offset = SCRegType::ACTLR.offset();
    //     let word_offset = byte_offset / 4;
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ACTLR) }
    // }

    pub fn get_stir_mut(&mut self) -> &mut STIR {
        let byte_offset = SCRegType::STIR.offset();
        let word_offset = byte_offset / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut STIR) }
    }

}