//! systick.rs
//! 
//! implementation of system timer for armv7m

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SysTickReg {
    CSR,     // systick control and status register
    RVR,     // systick reload value register
    CVR,     // systick current value register
    CALIB,   // systick calibration value register
}

impl SysTickReg {
    pub fn read_evt(&self, read_val: u32) -> Result<Option<Event>, Error> {
        todo!()
    }

    pub fn write_evt(&self, write_val: u32) -> Result<Option<Event>, Error> {
        todo!()
    }

    pub fn lookup_offset(offset: usize) -> Option<SysTickReg> {
        match offset {
            0x010 => { Some(SysTickReg::CSR) }
            0x014 => { Some(SysTickReg::RVR) }
            0x018 => { Some(SysTickReg::CVR) }
            0x01C => { Some(SysTickReg::CALIB) }
            _ => { None }
        }
    }
}