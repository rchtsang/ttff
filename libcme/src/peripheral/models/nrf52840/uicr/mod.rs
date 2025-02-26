//! uicr.rs
//! 
//! UICR module
//! User information configuration registers
use std::fmt;
use std::collections::VecDeque;

use bitfield_struct::bitfield;

use crate::prelude::*;
use crate::peripheral::{ Error, Event };
use crate::concrete::context;
use crate::utils::*;

use super::*;

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
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
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
        let byte_offset = offset & 0b11;
        let Some(reg_type) = UICRRegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[byte_offset..byte_offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            UICRRegType::REGOUT0    => { todo!() }
            UICRRegType::DEBUGCTRL  => { todo!() }
            UICRRegType::NFCPINS    => { todo!() }
            UICRRegType::APPROTECT  => { todo!() }
            UICRRegType::PSELRESET(n) => { todo!() }
            UICRRegType::CUSTOMER(n) => { todo!() }
            UICRRegType::NRFHW(n)   => { todo!() }
            UICRRegType::NRFFW(n)   => { todo!() }

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
        let byte_offset = offset & 0b11;
        let Some(reg_type) = UICRRegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[byte_offset..byte_offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            UICRRegType::REGOUT0    => { todo!() }
            UICRRegType::DEBUGCTRL  => { todo!() }
            UICRRegType::NFCPINS    => { todo!() }
            UICRRegType::APPROTECT  => { todo!() }
            UICRRegType::PSELRESET(n) => { todo!() }
            UICRRegType::CUSTOMER(n) => { todo!() }
            UICRRegType::NRFHW(n)   => { todo!() }
            UICRRegType::NRFFW(n)   => { todo!() }

        }
    }
}


impl UICRState {
    // register reference getters

    pub fn get_regout0(&self) -> &REGOUT0 {
        let word_offset = UICRRegType::REGOUT0.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const REGOUT0) }
    }

    pub fn get_debugctrl(&self) -> &DEBUGCTRL {
        let word_offset = UICRRegType::DEBUGCTRL.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DEBUGCTRL) }
    }

    pub fn get_nfcpins(&self) -> &NFCPINS {
        let word_offset = UICRRegType::NFCPINS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const NFCPINS) }
    }

    pub fn get_approtect(&self) -> &APPROTECT {
        let word_offset = UICRRegType::APPROTECT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const APPROTECT) }
    }

    pub fn get_pselreset(&self, n: u8) -> &PSELRESET {
        let word_offset = UICRRegType::PSELRESET(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PSELRESET) }
    }

    pub fn get_customer(&self, n: u8) -> &CUSTOMER {
        let word_offset = UICRRegType::CUSTOMER(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CUSTOMER) }
    }

    pub fn get_nrfhw(&self, n: u8) -> &NRFHW {
        let word_offset = UICRRegType::NRFHW(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const NRFHW) }
    }

    pub fn get_nrffw(&self, n: u8) -> &NRFFW {
        let word_offset = UICRRegType::NRFFW(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const NRFFW) }
    }


    pub fn get_regout0_mut(&mut self) -> &mut REGOUT0 {
        let word_offset = UICRRegType::REGOUT0.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut REGOUT0) }
    }
    
    pub fn get_debugctrl_mut(&mut self) -> &mut DEBUGCTRL {
        let word_offset = UICRRegType::DEBUGCTRL.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DEBUGCTRL) }
    }
    
    pub fn get_nfcpins_mut(&mut self) -> &mut NFCPINS {
        let word_offset = UICRRegType::NFCPINS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut NFCPINS) }
    }
    
    pub fn get_approtect_mut(&mut self) -> &mut APPROTECT {
        let word_offset = UICRRegType::APPROTECT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut APPROTECT) }
    }
    
    pub fn get_pselreset_mut(&mut self, n: u8) -> &mut PSELRESET {
        let word_offset = UICRRegType::PSELRESET(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PSELRESET) }
    }
    
    pub fn get_customer_mut(&mut self, n: u8) -> &mut CUSTOMER {
        let word_offset = UICRRegType::CUSTOMER(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CUSTOMER) }
    }
    
    pub fn get_nrfhw_mut(&mut self, n: u8) -> &mut NRFHW {
        let word_offset = UICRRegType::NRFHW(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut NRFHW) }
    }
    
    pub fn get_nrffw_mut(&mut self, n: u8) -> &mut NRFFW {
        let word_offset = UICRRegType::NRFFW(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut NRFFW) }
    }
    


}