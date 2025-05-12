//! gpio.rs
//! 
//! GPIO module
//! GPIO Port 1
use std::fmt;
use std::collections::VecDeque;

use thiserror::Error;
use bitfield_struct::bitfield;

use libcme::prelude::*;
use libcme::peripheral::{ Error, Event };

// use super::*;

mod registers;
pub use registers::*;

pub static P0_BASE: u32 = 0x50000000;

#[derive(Debug, Error)]
pub enum GpioError {
    #[error("write to read-only register: {0:?}")]
    WriteViolation(GPIORegType),
}


#[derive(Clone)]
pub struct GPIOState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
}

impl fmt::Debug for GPIOState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GPIO @ {:#x}", self.base_address)
    }
}

impl PeripheralState for GPIOState {
    fn base_address(&self) -> Address {
        Address::from(self.base_address)
    }

    fn size(&self) -> u64 {
        self.backing.len() as u64
    }

    fn read_bytes(&mut self,
        address: &Address,
        dst: &mut [u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let offset = address.offset()
            .checked_sub(self.base_address.into())
            .expect("address not in peripheral!");
        self._read_bytes(offset as usize, dst, events)
    }

    fn write_bytes(&mut self,
        address: &Address,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let offset = address.offset()
            .checked_sub(self.base_address.into())
            .expect("address not in peripheral!");
        self._write_bytes(offset as usize, src, events)
    }
}

impl AsRef<[u8]> for GPIOState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for GPIOState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for GPIOState {
    fn default() -> Self {
        let base_address = 0x50000000;
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
    }
}

impl GPIOState {
    pub fn new_with(base_address: u32) -> Self {
        let mut backing = Box::new([0u32; 0x400]);
        for reg_type in GPIORegType::list() {
            let offset = reg_type.offset() / 4;
            if let Some(reset_value) = reg_type.reset() {
                backing[offset] = reset_value;
            }
        }
        Self { base_address, backing }
    }

    pub fn reset(self) -> Self {
        Self::new_with(self.base_address)
    }

    /// direct view as bytes
    pub fn view_as_bytes(&self) -> &[u8; 0x1000] {
        let bytes: &[u8] = self.as_ref();
        unsafe { &*(bytes as *const [u8] as *const [u8; 0x1000]) }
    }

    /// direct mutable view as bytes
    pub fn view_as_bytes_mut(&mut self) -> &mut [u8; 0x1000] {
        let bytes: &mut [u8] = self.as_mut();
        unsafe { &mut *(bytes as *mut [u8] as *mut [u8; 0x1000]) }
    }

    #[instrument]
    pub fn _read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        events: &mut VecDeque<Event>
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let word_offset = offset / 4;
        // let byte_offset = offset & 0b11;
        let Some(reg_type) = GPIORegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[offset..offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            GPIORegType::OUT
            | GPIORegType::OUTSET
            | GPIORegType::OUTCLR
            | GPIORegType::IN => {
                let out_offset = GPIORegType::OUT.offset();
                let slice = &self.view_as_bytes()[out_offset..out_offset + 4];
                dst.copy_from_slice(slice);
                Ok(())
            }
            GPIORegType::DIR
            | GPIORegType::DIRSET
            | GPIORegType::DIRCLR => {
                let dir_offset = GPIORegType::DIR.offset();
                let slice = &self.view_as_bytes()[dir_offset..dir_offset + 4];
                dst.copy_from_slice(slice);
                Ok(())
            }
            GPIORegType::LATCH
            | GPIORegType::DETECTMODE => {
                let slice = self.backing[word_offset].to_le_bytes();
                dst.copy_from_slice(&slice);
                Ok(())
            }
            GPIORegType::PIN_CNF(_i) => {
                let slice = self.backing[word_offset].to_le_bytes();
                dst.copy_from_slice(&slice);
                Ok(())
            }
            
            
        }
    }

    #[instrument]
    pub fn _write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let word_offset = offset / 4;
        // let byte_offset = offset & 0b11;
        let Some(reg_type) = GPIORegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[offset..offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            GPIORegType::OUT
            | GPIORegType::DIR
            | GPIORegType::LATCH => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                self.backing[word_offset] = val;
                Ok(())
            }
            GPIORegType::OUTSET => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                let out_offset = GPIORegType::OUT.offset() / 4;
                self.backing[out_offset] |= val;
                Ok(())
            }
            GPIORegType::OUTCLR => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) ^ 0xFFFFFFFF;
                let out_offset = GPIORegType::OUT.offset() / 4;
                self.backing[out_offset] &= val;
                Ok(())
            }
            GPIORegType::IN => {
                let err = GpioError::WriteViolation(reg_type);
                Err(peripheral::Error::State(err.into()))
            }
            GPIORegType::DIRSET => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                let out_offset = GPIORegType::DIR.offset() / 4;
                self.backing[out_offset] |= val;
                Ok(())
            }
            GPIORegType::DIRCLR => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) ^ 0xFFFFFFFF;
                let out_offset = GPIORegType::DIR.offset() / 4;
                self.backing[out_offset] &= val;
                Ok(())
            }
            GPIORegType::DETECTMODE => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) & 1;
                self.backing[word_offset] = val;
                Ok(())
            }
            GPIORegType::PIN_CNF(_i) => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) & 0x0003070F;
                self.backing[word_offset] = val;
                Ok(())
            }
        }
    }
}


impl GPIOState {
    // register reference getters

    pub fn get_out(&self) -> &OUT {
        let word_offset = GPIORegType::OUT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const OUT) }
    }
    
    pub fn get_outset(&self) -> &OUTSET {
        let word_offset = GPIORegType::OUTSET.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const OUTSET) }
    }
    
    pub fn get_outclr(&self) -> &OUTCLR {
        let word_offset = GPIORegType::OUTCLR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const OUTCLR) }
    }
    
    pub fn get_in(&self) -> &IN {
        let word_offset = GPIORegType::IN.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const IN) }
    }
    
    pub fn get_dir(&self) -> &DIR {
        let word_offset = GPIORegType::DIR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DIR) }
    }
    
    pub fn get_dirset(&self) -> &DIRSET {
        let word_offset = GPIORegType::DIRSET.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DIRSET) }
    }
    
    pub fn get_dirclr(&self) -> &DIRCLR {
        let word_offset = GPIORegType::DIRCLR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DIRCLR) }
    }
    
    pub fn get_latch(&self) -> &LATCH {
        let word_offset = GPIORegType::LATCH.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const LATCH) }
    }
    
    pub fn get_detectmode(&self) -> &DETECTMODE {
        let word_offset = GPIORegType::DETECTMODE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DETECTMODE) }
    }
    
    pub fn get_pin_cnf(&self, i: u8) -> &PIN_CNF {
        let word_offset = GPIORegType::PIN_CNF(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PIN_CNF) }
    }
    
    

    pub fn get_out_mut(&mut self) -> &mut OUT {
        let word_offset = GPIORegType::OUT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut OUT) }
    }
    
    pub fn get_outset_mut(&mut self) -> &mut OUTSET {
        let word_offset = GPIORegType::OUTSET.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut OUTSET) }
    }
    
    pub fn get_outclr_mut(&mut self) -> &mut OUTCLR {
        let word_offset = GPIORegType::OUTCLR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut OUTCLR) }
    }
    
    pub fn get_in_mut(&mut self) -> &mut IN {
        let word_offset = GPIORegType::IN.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut IN) }
    }
    
    pub fn get_dir_mut(&mut self) -> &mut DIR {
        let word_offset = GPIORegType::DIR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DIR) }
    }
    
    pub fn get_dirset_mut(&mut self) -> &mut DIRSET {
        let word_offset = GPIORegType::DIRSET.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DIRSET) }
    }
    
    pub fn get_dirclr_mut(&mut self) -> &mut DIRCLR {
        let word_offset = GPIORegType::DIRCLR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DIRCLR) }
    }
    
    pub fn get_latch_mut(&mut self) -> &mut LATCH {
        let word_offset = GPIORegType::LATCH.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut LATCH) }
    }
    
    pub fn get_detectmode_mut(&mut self) -> &mut DETECTMODE {
        let word_offset = GPIORegType::DETECTMODE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DETECTMODE) }
    }
    
    pub fn get_pin_cnf_mut(&mut self, i: u8) -> &mut PIN_CNF {
        let word_offset = GPIORegType::PIN_CNF(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PIN_CNF) }
    }
    
    

    

    
}