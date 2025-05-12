//! uicr.rs
//! 
//! UICR module
//! User Information Configuration Registers
use std::fmt;
use std::collections::VecDeque;

use bitfield_struct::bitfield;

use libcme::prelude::*;
use libcme::peripheral::{ Error, Event };
// use libcme::utils::*;

// use super::*;

mod registers;
pub use registers::*;

pub static UICR_BASE: u32 = 0x10001000;



#[derive(Clone)]
pub struct UICRState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
}

impl fmt::Debug for UICRState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UICR @ {:#x}", self.base_address)
    }
}

impl PeripheralState for UICRState {
    fn base_address(&self) -> Address {
        Address::from(self.base_address)
    }

    fn size(&self) -> u64 {
        (self.backing.len() * 4) as u64
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

impl AsRef<[u8]> for UICRState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for UICRState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for UICRState {
    fn default() -> Self {
        let base_address = 0x10001000;
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
    }
}

impl UICRState {
    pub fn new_with(base_address: u32) -> Self {
        let mut backing = Box::new([0u32; 0x400]);
        for reg_type in UICRRegType::list() {
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

    #[instrument(skip_all)]
    pub fn _read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        _events: &mut VecDeque<Event>
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let word_offset = offset / 4;
        // let byte_offset = offset & 0b11;
        let Some(reg_type) = UICRRegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[offset..offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            UICRRegType::NRFFW(_i)
            | UICRRegType::NRFHW(_i)
            | UICRRegType::CUSTOMER(_i) 
            | UICRRegType::PSELRESET(_i) => {
                let src = self.backing[word_offset].to_le_bytes();
                dst.copy_from_slice(&src);
                Ok(())
            }
            UICRRegType::APPROTECT
            | UICRRegType::NFCPINS => {
                let src = self.backing[word_offset].to_le_bytes();
                dst.copy_from_slice(&src);
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
        let Some(reg_type) = UICRRegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[offset..offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            UICRRegType::NRFFW(_i)
            | UICRRegType::NRFHW(_i)
            | UICRRegType::CUSTOMER(_i) => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                self.backing[word_offset] = val;
                Ok(())
            }
            UICRRegType::PSELRESET(_i) => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                // do i need to (val | 0x7FFFFFC0),
                // are the unused bits written?
                self.backing[word_offset] = val;
                Ok(())
            }
            UICRRegType::APPROTECT => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                if val == 0xFF {
                    self.get_approtect_mut().set_pall(0xFF);
                } else {
                    self.get_approtect_mut().set_pall(0);
                }
                Ok(())
            }
            UICRRegType::NFCPINS => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                self.backing[word_offset] = val & 1;
                Ok(())
            }
            
        }
    }
}


impl UICRState {
    // register reference getters

    pub fn get_nrffw(&self, i: u8) -> &NRFFW {
        let word_offset = UICRRegType::NRFFW(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const NRFFW) }
    }
    
    pub fn get_nrfhw(&self, i: u8) -> &NRFHW {
        let word_offset = UICRRegType::NRFHW(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const NRFHW) }
    }
    
    pub fn get_customer(&self, i: u8) -> &CUSTOMER {
        let word_offset = UICRRegType::CUSTOMER(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CUSTOMER) }
    }
    
    pub fn get_pselreset(&self, i: u8) -> &PSELRESET {
        let word_offset = UICRRegType::PSELRESET(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PSELRESET) }
    }
    
    pub fn get_approtect(&self) -> &APPROTECT {
        let word_offset = UICRRegType::APPROTECT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const APPROTECT) }
    }
    
    pub fn get_nfcpins(&self) -> &NFCPINS {
        let word_offset = UICRRegType::NFCPINS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const NFCPINS) }
    }
    
    

    pub fn get_nrffw_mut(&mut self, i: u8) -> &mut NRFFW {
        let word_offset = UICRRegType::NRFFW(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut NRFFW) }
    }
    
    pub fn get_nrfhw_mut(&mut self, i: u8) -> &mut NRFHW {
        let word_offset = UICRRegType::NRFHW(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut NRFHW) }
    }
    
    pub fn get_customer_mut(&mut self, i: u8) -> &mut CUSTOMER {
        let word_offset = UICRRegType::CUSTOMER(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CUSTOMER) }
    }
    
    pub fn get_pselreset_mut(&mut self, i: u8) -> &mut PSELRESET {
        let word_offset = UICRRegType::PSELRESET(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PSELRESET) }
    }
    
    pub fn get_approtect_mut(&mut self) -> &mut APPROTECT {
        let word_offset = UICRRegType::APPROTECT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut APPROTECT) }
    }
    
    pub fn get_nfcpins_mut(&mut self) -> &mut NFCPINS {
        let word_offset = UICRRegType::NFCPINS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut NFCPINS) }
    }
    
    

    

    
}