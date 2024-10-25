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

/// system control space base address
static BASE: u32 = 0xe000e000;

/// config containing reset values for scs registers
#[derive(Debug)]
pub struct SysCtrlConfig {
    // todo
}

impl Default for SysCtrlConfig {
    fn default() -> Self {
        todo!("need to implement default scs values")
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
}

impl fmt::Debug for SysCtrlSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCS")
    }
}

impl SysCtrlSpace {
    pub fn new_from(config: SysCtrlConfig) -> Self {
        todo!("implement scs constructor")
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
        let word_offset = offset / 4;
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
                let address = Address::from(BASE + offset as u32);
                ArchError::from(Error::InvalidSysCtrlReg(address))
            })?;
        match reg_type {
            SCRegType::ICSR => {

            }
            SCRegType::VTOR => todo!(),
            SCRegType::AIRCR => todo!(),
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
            SCRegType::CPACR => todo!(),
            SCRegType::FPCCR => todo!(),
            SCRegType::FPCAR => todo!(),
            SCRegType::FPDSCR => todo!(),
            SCRegType::MVFR0 => todo!(),
            SCRegType::MVFR1 => todo!(),
            SCRegType::MVFR2 => todo!(),
            SCRegType::MCR => todo!(),
            SCRegType::ICTR => todo!(),
            SCRegType::ACTLR => todo!(),
            SCRegType::STIR => todo!(),
            SCRegType::SysTick(streg_type) => todo!(),
            // SCRegType::NVIC(nvicreg) => todo!(),
            // SCRegType::MPU(mpureg) => todo!(),
            _ => {
                let slice = unsafe {
                    &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
                };
                dst.copy_from_slice(slice);
            }
        }
        Ok(())
    }

    pub fn write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), context::Error> {
        assert_eq!(src.len(), 4, "val must be word-aligned (for now)");
        let word_offset = offset / 4;
        let maybe_reg_mut = self.get_reg_mut(offset);
        if let Err(err) = maybe_reg_mut {
            warn!("{err:?}");
            let slice = unsafe {
                &mut *(&mut self.backing[word_offset] as *mut u32 as *mut [u8; 4])
            };
            slice.copy_from_slice(src);
            return Ok(());
        }
        let write_val = src.iter()
            .enumerate().take(4)
            .fold(0u32, |val, (i, &byte)| {
                val | ((byte as u32) << i)
            });
        let reg_mut = maybe_reg_mut.unwrap();
        // match reg_mut {
            
        // }
        Ok(())
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

    /// get wrapper for interacting with systick registers
    pub fn systick_regs(&mut self) -> SysTickRegs {
        let slice = &mut self.backing[..0x40];
        assert_eq!(slice.len(), 0x40);
        let backing = unsafe {
            &mut *(slice as *mut [u32] as *mut [u32; 0x40])
        };
        SysTickRegs::new(backing)
    }
}

impl Default for SysCtrlSpace {
    fn default() -> Self {
        Self {
            backing: Box::new([0u32; 0x400]),
        }
    }
}