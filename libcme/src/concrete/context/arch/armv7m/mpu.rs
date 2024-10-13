//! mpu.rs
//! 
//! memory protection unit implementation

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MPUReg {

}

impl MPUReg {
    pub fn read_evt(&self, read_val: u32) -> Result<Option<Event>, Error> {
        todo!()
    }

    pub fn write_evt(&self, write_val: u32) -> Result<Option<Event>, Error> {
        todo!()
    }

    pub fn lookup_offset(offset: usize) -> Option<MPUReg> {
        todo!() // see B3.5.4 for implementation
    }
}