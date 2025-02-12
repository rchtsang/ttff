//! gpiote.rs
//! 
//! GPIOTE module
//! GPIO Tasks and Events
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


static GPIOTE_BASE: u32 = 0x40006000;

#[derive(Clone)]
pub struct GPIOTEState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
}

impl fmt::Debug for GPIOTEState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GPIOTE @ {:#x}", self.base_address)
    }
}

impl PeripheralState for GPIOTEState {
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

impl AsRef<[u8]> for GPIOTEState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for GPIOTEState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for GPIOTEState {
    fn default() -> Self {
        let base_address = 0x40006000;
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
    }
}

impl GPIOTEState {
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
        let Some(reg_type) = GPIOTERegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:?} (treated as memory)");
            let slice = &self.view_as_bytes()[byte_offset..];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            GPIOTERegType::CONFIG(n)  => { todo!() }
            GPIOTERegType::INTENCLR   => { todo!() }
            GPIOTERegType::INTENSET   => { todo!() }
            GPIOTERegType::EVENTS_PORT => { todo!() }
            GPIOTERegType::EVENTS_IN(n) => { todo!() }
            GPIOTERegType::TASKS_CLR(n) => { todo!() }
            GPIOTERegType::TASKS_SET(n) => { todo!() }
            GPIOTERegType::TASKS_OUT(n) => { todo!() }

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
        let Some(reg_type) = GPIOTERegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[byte_offset..];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            GPIOTERegType::CONFIG(n)  => { todo!() }
            GPIOTERegType::INTENCLR   => { todo!() }
            GPIOTERegType::INTENSET   => { todo!() }
            GPIOTERegType::EVENTS_PORT => { todo!() }
            GPIOTERegType::EVENTS_IN(n) => { todo!() }
            GPIOTERegType::TASKS_CLR(n) => { todo!() }
            GPIOTERegType::TASKS_SET(n) => { todo!() }
            GPIOTERegType::TASKS_OUT(n) => { todo!() }

        }
    }
}


impl GPIOTEState {
    // register reference getters

    pub fn get_config(&self, n: u8) -> &CONFIG {
        let word_offset = GPIOTERegType::CONFIG(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CONFIG) }
    }

    pub fn get_intenclr(&self) -> &INTENCLR {
        let word_offset = GPIOTERegType::INTENCLR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENCLR) }
    }

    pub fn get_intenset(&self) -> &INTENSET {
        let word_offset = GPIOTERegType::INTENSET.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENSET) }
    }

    pub fn get_events_port(&self) -> &EVENTS_PORT {
        let word_offset = GPIOTERegType::EVENTS_PORT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_PORT) }
    }

    pub fn get_events_in(&self, n: u8) -> &EVENTS_IN {
        let word_offset = GPIOTERegType::EVENTS_IN(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_IN) }
    }

    pub fn get_tasks_clr(&self, n: u8) -> &TASKS_CLR {
        let word_offset = GPIOTERegType::TASKS_CLR(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_CLR) }
    }

    pub fn get_tasks_set(&self, n: u8) -> &TASKS_SET {
        let word_offset = GPIOTERegType::TASKS_SET(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_SET) }
    }

    pub fn get_tasks_out(&self, n: u8) -> &TASKS_OUT {
        let word_offset = GPIOTERegType::TASKS_OUT(n).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_OUT) }
    }


    pub fn get_config_mut(&mut self, n: u8) -> &mut CONFIG {
        let word_offset = GPIOTERegType::CONFIG(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CONFIG) }
    }
    
    pub fn get_intenclr_mut(&mut self) -> &mut INTENCLR {
        let word_offset = GPIOTERegType::INTENCLR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENCLR) }
    }
    
    pub fn get_intenset_mut(&mut self) -> &mut INTENSET {
        let word_offset = GPIOTERegType::INTENSET.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENSET) }
    }
    
    pub fn get_events_port_mut(&mut self) -> &mut EVENTS_PORT {
        let word_offset = GPIOTERegType::EVENTS_PORT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_PORT) }
    }
    
    pub fn get_events_in_mut(&mut self, n: u8) -> &mut EVENTS_IN {
        let word_offset = GPIOTERegType::EVENTS_IN(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_IN) }
    }
    
    pub fn get_tasks_clr_mut(&mut self, n: u8) -> &mut TASKS_CLR {
        let word_offset = GPIOTERegType::TASKS_CLR(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_CLR) }
    }
    
    pub fn get_tasks_set_mut(&mut self, n: u8) -> &mut TASKS_SET {
        let word_offset = GPIOTERegType::TASKS_SET(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_SET) }
    }
    
    pub fn get_tasks_out_mut(&mut self, n: u8) -> &mut TASKS_OUT {
        let word_offset = GPIOTERegType::TASKS_OUT(n).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_OUT) }
    }
    


}