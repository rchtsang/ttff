//! scs module
//! 
//! implementing the system control space

/*
 * TODO:
 * - implement endian sensitivity
 * - replace struct/int/byte conversions with unsafe std::mem::transmute for performance
 */
use std::fmt;
use bitfield_struct::bitfield;
use ahash::AHashMap;

use super::*;
use context::Permission;

use crate::utils::*;

mod regs;
pub use regs::*;
pub mod nvic;
pub use nvic::*;
pub mod systick;
pub use systick::*;
pub mod mpu;
pub use mpu::*;
pub mod dcb;
pub use dcb::*;

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
    backing: Box<[u32; 0x400]>,
    pub nvic: NVICState,
    pub mpu: MPUState,
}

impl fmt::Debug for SysCtrlSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCS")
    }
}

impl SysCtrlSpace {
    pub fn new_from(config: SysCtrlConfig) -> Self {
        let mut backing = Box::new([0u32; 0x400]);
        let nvic = NVICState::default();
        let mpu = MPUState::default();
        for (scregtype, reset_val) in config.map {
            let offset = scregtype.offset();
            backing[offset] = reset_val;
        }
        Self { backing, nvic, mpu }
    }

    /// direct view into the scs as transmuted bytes
    pub fn view_as_bytes(&self) -> &[u8; 0x1000] {
        unsafe { &*(self.backing.as_ref() as *const [u32; 0x400] as *const [u8; 0x1000]) }
    }

    /// direct mutable view into the scs as transmuted bytes
    pub fn view_as_bytes_mut(&mut self) -> &mut [u8; 0x1000] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32; 0x400] as *mut [u8; 0x1000]) }
    }

    /// get byte offset as a register reference (if applicable)
    pub fn get_reg_ref(&self, offset: usize) -> Result<SCRegRef, Error> {
        let reg_type = SCRegType::lookup_offset(offset);
        let Some(reg_type) = reg_type else {
            let address = Address::from(BASE + offset as u32);
            return Err(Error::InvalidSysCtrlReg(address));
        };
        unsafe { reg_type.to_reg_ref(&self.backing[offset / 4]) }
    }

    /// get byte offset as a mutable register reference (if applicable)
    pub fn get_reg_mut(&mut self, offset: usize) -> Result<SCRegMut, Error> {
        let reg_type = SCRegType::lookup_offset(offset);
        let Some(reg_type) = reg_type else {
            let address = Address::from(BASE + offset as u32);
            return Err(Error::InvalidSysCtrlReg(address))
        };
        unsafe { reg_type.to_reg_mut(&mut self.backing[offset / 4]) }
    }

    #[instrument]
    pub fn read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), context::Error> {
        let address = BASE + offset as u32;
        let word_offset = offset / 4;
        let byte_offset = offset & 0b11;
        if let Err(err) = self.get_reg_ref(offset) {
            // if register isn't implemented as a struct yet, just treat it as
            // memory and issue a warning, returning the error that
            // must be ignored at a higher level.
            warn!("{err:?}");
            let slice = unsafe {
                &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
            };
            dst.copy_from_slice(slice);
            return Err(err.into());
        }
        let reg_type = SCRegType::lookup_offset(offset)
            .ok_or_else( | | {
                ArchError::from(Error::InvalidSysCtrlReg(address.into()))
            })?;
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
                check_alignment(address, dst.len(), Alignment::Word)?;
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
                return Err(ArchError::from(err).into());
            }
            SCRegType::SysTick(_streg_type) => {
                let mut stregs = self.systick_regs();
                return stregs.read_bytes(offset, dst, events);
            }
            SCRegType::NVIC(_nvicreg_type) => {
                let mut nvicregs = self.nvic_regs();
                return nvicregs.read_bytes(offset, dst, events);
            }
            SCRegType::MPU(_mpureg_type) => {
                let mut mpuregs = self.mpu_regs();
                return mpuregs.read_bytes(offset, dst, events);
            }
            SCRegType::SHPR1(_)
            | SCRegType::SHPR2(_)
            | SCRegType::SHPR3(_)
            | SCRegType::CFSR
            | _ => {
                check_alignment(address, dst.len(), Alignment::Any)?;
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
    ) -> Result<(), context::Error> {
        let address = BASE + offset as u32;
        let word_offset = offset / 4;
        let byte_offset = offset & 0b11;
        if let Err(err) = self.get_reg_mut(offset) {
            warn!("{err:?}");
            let slice = unsafe {
                &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
            };
            (&mut slice[byte_offset..]).copy_from_slice(src);
            return Err(err.into());
        }
        let write_val = src.iter()
            .enumerate().take(4)
            .fold(0u32, |val, (i, &byte)| {
                val | ((byte as u32) << i)
            });
        let reg_type = SCRegType::lookup_offset(offset)
            .ok_or_else( | | {
                ArchError::from(Error::InvalidSysCtrlReg(address.into()))
            })?;
        match reg_type {
            SCRegType::ICSR => {
                check_alignment(address, src.len(), Alignment::Word)?;

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
                    return Err(ArchError::from(err).into());
                }
                if new_pendsvclr & new_pendsvset {
                    let err_str = "PENDSVSET and PENDSVCLR both set to 1";
                    let err = Error::UnpredictableBehavior(err_str);
                    warn!("{err:?}");
                    return Err(ArchError::from(err).into());
                }

                if new_pendstclr { // TODO: does this self clear?
                    let excp = ExceptionType::SysTick;
                    let evt = Event::ExceptionSetPending(excp, false);
                    events.push_back(evt);
                    icsr.set_pendstset(false);
                }
                if new_pendstset & (new_pendstset ^ icsr.pendstset()) {
                    let excp = ExceptionType::SysTick;
                    let evt = Event::ExceptionSetPending(excp, true);
                    events.push_back(evt);
                    icsr.set_pendstset(true);
                }
                if new_pendsvclr { // TODO: does this self clear?
                    let excp = ExceptionType::PendSV;
                    let evt = Event::ExceptionSetPending(excp, false);
                    events.push_back(evt);
                    icsr.set_pendsvset(false);
                }
                if new_pendsvset & (new_pendsvset ^ icsr.pendsvset()) {
                    let excp = ExceptionType::PendSV;
                    let evt = Event::ExceptionSetPending(excp, true);
                    events.push_back(evt);
                    icsr.set_pendsvset(true);
                }
                if new_nmipendset & (new_nmipendset ^ icsr.nmipendset()) {
                    let excp = ExceptionType::NMI;
                    let evt = Event::ExceptionSetActive(excp, true);
                    events.push_back(evt);
                    icsr.set_nmipendset(true);
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
                        return Err(ArchError::from(err).into());
                    }
                    events.push_back(Event::LocalSysResetRequest);
                }
                if new_vectclractive {
                    if !dbg_state {
                        let err_str = "Write to VECTCLRACTIVE while not halted in Debug state";
                        let err = Error::UnpredictableBehavior(err_str);
                        warn!("{err:?}");
                        return Err(ArchError::from(err).into());
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
            SCRegType::SCR => todo!(),
            SCRegType::CCR => todo!(),
            SCRegType::SHPR1(_) => todo!(),
            SCRegType::SHPR2(_) => todo!(),
            SCRegType::SHPR3(_) => todo!(),
            SCRegType::SHCSR => todo!(),
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
                let mut stregs = self.systick_regs();
                return stregs.write_bytes(offset, src, events);
            }
            SCRegType::NVIC(_nvicreg_type) => {
                let mut nvicregs = self.nvic_regs();
                return nvicregs.write_bytes(offset, src, events);
            }
            SCRegType::MPU(_mpureg_type) => {
                let mut mpuregs = self.mpu_regs();
                return mpuregs.write_bytes(offset, src, events);
            }
            _ => {
                check_alignment(address, src.len(), Alignment::Any)?;
                let slice = unsafe {
                    &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
                };
                (&mut slice[byte_offset..]).copy_from_slice(src);
            }
        }
        Ok(())
    }

    /// get wrapper for interacting with systick registers
    pub fn systick_regs(&mut self) -> SysTickRegs {
        let slice = &mut self.backing[..0x40];
        assert_eq!(slice.len(), 0x40);
        let backing = unsafe {
            &mut *(slice as *mut [u32] as *mut [u32; 0x40])
        };
        SysTickRegs::new(backing)
    }

    /// get wrapper for interacting with nvic registers
    pub fn nvic_regs(&mut self) -> NVICRegs {
        let slice = &mut self.backing[..0x340];
        assert_eq!(slice.len(), 0x340);
        let backing = unsafe {
            &mut *(slice as *mut [u32] as *mut [u32; 0x340])
        };
        NVICRegs::new(backing)
    }

    /// get wrapper for interacting with mpu registers
    pub fn mpu_regs(&mut self) -> MPURegs {
        let slice = &mut self.backing[..0xdec];
        assert_eq!(slice.len(), 0xdec);
        let backing = unsafe {
            &mut *(slice as *mut [u32] as *mut [u32; 0xdec])
        };
        MPURegs::new(backing)
    }

    /// get wrapper for interacting with dbg registers
    pub fn debug_regs(&mut self) -> DebugRegs {
        let slice = &mut self.backing[..0x3c0];
        assert_eq!(slice.len(), 0x3c0);
        let backing = unsafe {
            &mut *(slice as *mut [u32] as *mut [u32; 0x3c0])
        };
        DebugRegs::new(backing)

    }
}

impl Default for SysCtrlSpace {
    fn default() -> Self {
        Self {
            backing: Box::new([0u32; 0x400]),
            nvic: NVICState::default(),
            mpu: MPUState::default(),
        }
    }
}

impl SysCtrlSpace {

    // register reference getters

    pub fn get_icsr(&self) -> &ICSR {
        let word_offset = SCRegType::ICSR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICSR) }
    }

    pub fn get_vtor(&self) -> &VTOR {
        let word_offset = SCRegType::VTOR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const VTOR) }
    }

    pub fn get_aircr(&self) -> &AIRCR {
        let word_offset = SCRegType::AIRCR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const AIRCR) }
    }

    pub fn get_scr(&self) -> &SCR {
        let word_offset = SCRegType::SCR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SCR) }
    }

    pub fn get_ccr(&self) -> &CCR {
        let word_offset = SCRegType::CCR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CCR) }
    }

    pub fn get_shpr1(&self) -> &SHPR1 {
        let word_offset = SCRegType::SHPR1(0).offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR1) }
    }

    pub fn get_shpr2(&self) -> &SHPR2 {
        let word_offset = SCRegType::SHPR2(0).offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR2) }
    }

    pub fn get_shpr3(&self) -> &SHPR3 {
        let word_offset = SCRegType::SHPR3(0).offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHPR3) }
    }

    pub fn get_shcsr(&self) -> &SHCSR {
        let word_offset = SCRegType::SHCSR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHCSR) }
    }

    pub fn get_cfsr(&self) -> &CFSR {
        let word_offset = SCRegType::CFSR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CFSR) }
    }

    pub fn get_hfsr(&self) -> &HFSR {
        let word_offset = SCRegType::HFSR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const HFSR) }
    }

    pub fn get_dfsr(&self) -> &DFSR {
        let word_offset = SCRegType::DFSR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DFSR) }
    }

    pub fn get_mmfar(&self) -> &MMFAR {
        let word_offset = SCRegType::MMFAR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const MMFAR) }
    }

    pub fn get_bfar(&self) -> &BFAR {
        let word_offset = SCRegType::BFAR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const BFAR) }
    }

    // pub fn get_afsr(&self) -> &AFSR {
    //     let word_offset = SCRegType::AFSR.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const AFSR) }
    // }

    pub fn get_cpacr(&self) -> &CPACR {
        let word_offset = SCRegType::CPACR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CPACR) }
    }

    // pub fn get_fpccr(&self) -> &FPCCR {
    //     let word_offset = SCRegType::FPCCR.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const FPCCR) }
    // }

    // pub fn get_fpcar(&self) -> &FPCAR {
    //     let word_offset = SCRegType::FPCAR.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const FPCAR) }
    // }

    // pub fn get_fpdscr(&self) -> &FPDSCR {
    //     let word_offset = SCRegType::FPDSCR.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const FPDSCR) }
    // }

    // pub fn get_mvfr0(&self) -> &MVFR0 {
    //     let word_offset = SCRegType::MVFR0.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MVFR0) }
    // }

    // pub fn get_mvfr1(&self) -> &MVFR1 {
    //     let word_offset = SCRegType::MVFR1.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MVFR1) }
    // }

    // pub fn get_mvfr2(&self) -> &MVFR2 {
    //     let word_offset = SCRegType::MVFR2.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MVFR2) }
    // }

    // pub fn get_mcr(&self) -> &MCR {
    //     let word_offset = SCRegType::MCR.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const MCR) }
    // }

    pub fn get_ictr(&self) -> &ICTR {
        let word_offset = SCRegType::ICTR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ICTR) }
    }

    // pub fn get_actlr(&self) -> &ACTLR {
    //     let word_offset = SCRegType::ACTLR.offset();
    //     unsafe { &*(&self.backing[word_offset] as *const u32 as *const ACTLR) }
    // }

    pub fn get_stir(&self) -> &STIR {
        let word_offset = SCRegType::STIR.offset();
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const STIR) }
    }

    pub fn get_icsr_mut(&mut self) -> &mut ICSR {
        let word_offset = SCRegType::ICSR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICSR) }
    }

    pub fn get_vtor_mut(&mut self) -> &mut VTOR {
        let word_offset = SCRegType::VTOR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut VTOR) }
    }

    pub fn get_aircr_mut(&mut self) -> &mut AIRCR {
        let word_offset = SCRegType::AIRCR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut AIRCR) }
    }

    pub fn get_scr_mut(&mut self) -> &mut SCR {
        let word_offset = SCRegType::SCR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SCR) }
    }

    pub fn get_ccr_mut(&mut self) -> &mut CCR {
        let word_offset = SCRegType::CCR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CCR) }
    }

    pub fn get_shpr1_mut(&mut self) -> &mut SHPR1 {
        let word_offset = SCRegType::SHPR1(0).offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR1) }
    }

    pub fn get_shpr2_mut(&mut self) -> &mut SHPR2 {
        let word_offset = SCRegType::SHPR2(0).offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR2) }
    }

    pub fn get_shpr3_mut(&mut self) -> &mut SHPR3 {
        let word_offset = SCRegType::SHPR3(0).offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHPR3) }
    }

    pub fn get_shcsr_mut(&mut self) -> &mut SHCSR {
        let word_offset = SCRegType::SHCSR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHCSR) }
    }

    pub fn get_cfsr_mut(&mut self) -> &mut CFSR {
        let word_offset = SCRegType::CFSR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CFSR) }
    }

    pub fn get_hfsr_mut(&mut self) -> &mut HFSR {
        let word_offset = SCRegType::HFSR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut HFSR) }
    }

    pub fn get_dfsr_mut(&mut self) -> &mut DFSR {
        let word_offset = SCRegType::DFSR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DFSR) }
    }

    pub fn get_mmfar_mut(&mut self) -> &mut MMFAR {
        let word_offset = SCRegType::MMFAR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MMFAR) }
    }

    pub fn get_bfar_mut(&mut self) -> &mut BFAR {
        let word_offset = SCRegType::BFAR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut BFAR) }
    }

    // pub fn get_afsr_mut(&mut self) -> &mut AFSR {
    //     let word_offset = SCRegType::AFSR.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut AFSR) }
    // }

    pub fn get_cpacr_mut(&mut self) -> &mut CPACR {
        let word_offset = SCRegType::CPACR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CPACR) }
    }

    // pub fn get_fpccr_mut(&mut self) -> &mut FPCCR {
    //     let word_offset = SCRegType::FPCCR.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut FPCCR) }
    // }

    // pub fn get_fpcar_mut(&mut self) -> &mut FPCAR {
    //     let word_offset = SCRegType::FPCAR.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut FPCAR) }
    // }

    // pub fn get_fpdscr_mut(&mut self) -> &mut FPDSCR {
    //     let word_offset = SCRegType::FPDSCR.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut FPDSCR) }
    // }

    // pub fn get_mvfr0_mut(&mut self) -> &mut MVFR0 {
    //     let word_offset = SCRegType::MVFR0.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MVFR0) }
    // }

    // pub fn get_mvfr1_mut(&mut self) -> &mut MVFR1 {
    //     let word_offset = SCRegType::MVFR1.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MVFR1) }
    // }

    // pub fn get_mvfr2_mut(&mut self) -> &mut MVFR2 {
    //     let word_offset = SCRegType::MVFR2.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MVFR2) }
    // }

    // pub fn get_mcr_mut(&mut self) -> &mut MCR {
    //     let word_offset = SCRegType::MCR.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut MCR) }
    // }

    pub fn get_ictr_mut(&mut self) -> &mut ICTR {
        let word_offset = SCRegType::ICTR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ICTR) }
    }

    // pub fn get_actlr_mut(&mut self) -> &mut ACTLR {
    //     let word_offset = SCRegType::ACTLR.offset();
    //     unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ACTLR) }
    // }

    pub fn get_stir_mut(&mut self) -> &mut STIR {
        let word_offset = SCRegType::STIR.offset();
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut STIR) }
    }

}