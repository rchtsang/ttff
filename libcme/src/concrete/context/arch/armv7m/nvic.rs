//! nvic.rs
//! 
//! implementation of the nested vector interrupt controller for armv7m

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NVICReg {

}

impl NVICReg {
    pub fn read_evt(&self, read_val: u32) -> Result<Vec<Event>, Error> {
        todo!()
    }

    pub fn write_evt(&self, write_val: u32) -> Result<Vec<Event>, Error> {
        todo!()
    }

    pub fn lookup_offset(offset: usize) -> Option<NVICReg> {
        todo!() // see B3.4.3 for implementation
    }
}