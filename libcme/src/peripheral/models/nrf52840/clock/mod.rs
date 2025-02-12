//! clock.rs
//! 
//! CLOCK module
//! Clock control
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


static CLOCK_BASE: u32 = 0x40000000;

#[derive(Clone)]
pub struct CLOCKState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
}

impl fmt::Debug for CLOCKState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CLOCK @ {:#x}", self.base_address)
    }
}

impl PeripheralState for CLOCKState {
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

impl AsRef<[u8]> for CLOCKState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for CLOCKState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for CLOCKState {
    fn default() -> Self {
        let base_address = 0x40000000;
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
    }
}

impl CLOCKState {
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
        let Some(reg_type) = CLOCKRegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:?} (treated as memory)");
            let slice = &self.view_as_bytes()[byte_offset..];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            CLOCKRegType::LFRCMODE   => { todo!() }
            CLOCKRegType::TRACECONFIG => { todo!() }
            CLOCKRegType::CTIV       => { todo!() }
            CLOCKRegType::HFXODEBOUNCE => { todo!() }
            CLOCKRegType::LFCLKSRC   => { todo!() }
            CLOCKRegType::LFCLKSRCCOPY => { todo!() }
            CLOCKRegType::LFCLKSTAT  => { todo!() }
            CLOCKRegType::LFCLKRUN   => { todo!() }
            CLOCKRegType::HFCLKSTAT  => { todo!() }
            CLOCKRegType::HFCLKRUN   => { todo!() }
            CLOCKRegType::INTENCLR   => { todo!() }
            CLOCKRegType::INTENSET   => { todo!() }
            CLOCKRegType::EVENTS_CTSTOPPED => { todo!() }
            CLOCKRegType::EVENTS_CTSTARTED => { todo!() }
            CLOCKRegType::EVENTS_CTTO => { todo!() }
            CLOCKRegType::EVENTS_DONE => { todo!() }
            CLOCKRegType::EVENTS_LFCLKSTARTED => { todo!() }
            CLOCKRegType::EVENTS_HFCLKSTARTED => { todo!() }
            CLOCKRegType::TASKS_CTSTOP => { todo!() }
            CLOCKRegType::TASKS_CTSTART => { todo!() }
            CLOCKRegType::TASKS_CAL  => { todo!() }
            CLOCKRegType::TASKS_LFCLKSTOP => { todo!() }
            CLOCKRegType::TASKS_LFCLKSTART => { todo!() }
            CLOCKRegType::TASKS_HFCLKSTOP => { todo!() }
            CLOCKRegType::TASKS_HFCLKSTART => { todo!() }

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
        let Some(reg_type) = CLOCKRegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[byte_offset..];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            CLOCKRegType::LFRCMODE   => { todo!() }
            CLOCKRegType::TRACECONFIG => { todo!() }
            CLOCKRegType::CTIV       => { todo!() }
            CLOCKRegType::HFXODEBOUNCE => { todo!() }
            CLOCKRegType::LFCLKSRC   => { todo!() }
            CLOCKRegType::LFCLKSRCCOPY => { todo!() }
            CLOCKRegType::LFCLKSTAT  => { todo!() }
            CLOCKRegType::LFCLKRUN   => { todo!() }
            CLOCKRegType::HFCLKSTAT  => { todo!() }
            CLOCKRegType::HFCLKRUN   => { todo!() }
            CLOCKRegType::INTENCLR   => { todo!() }
            CLOCKRegType::INTENSET   => { todo!() }
            CLOCKRegType::EVENTS_CTSTOPPED => { todo!() }
            CLOCKRegType::EVENTS_CTSTARTED => { todo!() }
            CLOCKRegType::EVENTS_CTTO => { todo!() }
            CLOCKRegType::EVENTS_DONE => { todo!() }
            CLOCKRegType::EVENTS_LFCLKSTARTED => { todo!() }
            CLOCKRegType::EVENTS_HFCLKSTARTED => { todo!() }
            CLOCKRegType::TASKS_CTSTOP => { todo!() }
            CLOCKRegType::TASKS_CTSTART => { todo!() }
            CLOCKRegType::TASKS_CAL  => { todo!() }
            CLOCKRegType::TASKS_LFCLKSTOP => { todo!() }
            CLOCKRegType::TASKS_LFCLKSTART => { todo!() }
            CLOCKRegType::TASKS_HFCLKSTOP => { todo!() }
            CLOCKRegType::TASKS_HFCLKSTART => { todo!() }

        }
    }
}


impl CLOCKState {
    // register reference getters

    pub fn get_lfrcmode(&self) -> &LFRCMODE {
        let word_offset = CLOCKRegType::LFRCMODE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const LFRCMODE) }
    }

    pub fn get_traceconfig(&self) -> &TRACECONFIG {
        let word_offset = CLOCKRegType::TRACECONFIG.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TRACECONFIG) }
    }

    pub fn get_ctiv(&self) -> &CTIV {
        let word_offset = CLOCKRegType::CTIV.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CTIV) }
    }

    pub fn get_hfxodebounce(&self) -> &HFXODEBOUNCE {
        let word_offset = CLOCKRegType::HFXODEBOUNCE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const HFXODEBOUNCE) }
    }

    pub fn get_lfclksrc(&self) -> &LFCLKSRC {
        let word_offset = CLOCKRegType::LFCLKSRC.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const LFCLKSRC) }
    }

    pub fn get_lfclksrccopy(&self) -> &LFCLKSRCCOPY {
        let word_offset = CLOCKRegType::LFCLKSRCCOPY.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const LFCLKSRCCOPY) }
    }

    pub fn get_lfclkstat(&self) -> &LFCLKSTAT {
        let word_offset = CLOCKRegType::LFCLKSTAT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const LFCLKSTAT) }
    }

    pub fn get_lfclkrun(&self) -> &LFCLKRUN {
        let word_offset = CLOCKRegType::LFCLKRUN.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const LFCLKRUN) }
    }

    pub fn get_hfclkstat(&self) -> &HFCLKSTAT {
        let word_offset = CLOCKRegType::HFCLKSTAT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const HFCLKSTAT) }
    }

    pub fn get_hfclkrun(&self) -> &HFCLKRUN {
        let word_offset = CLOCKRegType::HFCLKRUN.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const HFCLKRUN) }
    }

    pub fn get_intenclr(&self) -> &INTENCLR {
        let word_offset = CLOCKRegType::INTENCLR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENCLR) }
    }

    pub fn get_intenset(&self) -> &INTENSET {
        let word_offset = CLOCKRegType::INTENSET.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENSET) }
    }

    pub fn get_events_ctstopped(&self) -> &EVENTS_CTSTOPPED {
        let word_offset = CLOCKRegType::EVENTS_CTSTOPPED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_CTSTOPPED) }
    }

    pub fn get_events_ctstarted(&self) -> &EVENTS_CTSTARTED {
        let word_offset = CLOCKRegType::EVENTS_CTSTARTED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_CTSTARTED) }
    }

    pub fn get_events_ctto(&self) -> &EVENTS_CTTO {
        let word_offset = CLOCKRegType::EVENTS_CTTO.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_CTTO) }
    }

    pub fn get_events_done(&self) -> &EVENTS_DONE {
        let word_offset = CLOCKRegType::EVENTS_DONE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_DONE) }
    }

    pub fn get_events_lfclkstarted(&self) -> &EVENTS_LFCLKSTARTED {
        let word_offset = CLOCKRegType::EVENTS_LFCLKSTARTED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_LFCLKSTARTED) }
    }

    pub fn get_events_hfclkstarted(&self) -> &EVENTS_HFCLKSTARTED {
        let word_offset = CLOCKRegType::EVENTS_HFCLKSTARTED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_HFCLKSTARTED) }
    }

    pub fn get_tasks_ctstop(&self) -> &TASKS_CTSTOP {
        let word_offset = CLOCKRegType::TASKS_CTSTOP.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_CTSTOP) }
    }

    pub fn get_tasks_ctstart(&self) -> &TASKS_CTSTART {
        let word_offset = CLOCKRegType::TASKS_CTSTART.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_CTSTART) }
    }

    pub fn get_tasks_cal(&self) -> &TASKS_CAL {
        let word_offset = CLOCKRegType::TASKS_CAL.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_CAL) }
    }

    pub fn get_tasks_lfclkstop(&self) -> &TASKS_LFCLKSTOP {
        let word_offset = CLOCKRegType::TASKS_LFCLKSTOP.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_LFCLKSTOP) }
    }

    pub fn get_tasks_lfclkstart(&self) -> &TASKS_LFCLKSTART {
        let word_offset = CLOCKRegType::TASKS_LFCLKSTART.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_LFCLKSTART) }
    }

    pub fn get_tasks_hfclkstop(&self) -> &TASKS_HFCLKSTOP {
        let word_offset = CLOCKRegType::TASKS_HFCLKSTOP.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_HFCLKSTOP) }
    }

    pub fn get_tasks_hfclkstart(&self) -> &TASKS_HFCLKSTART {
        let word_offset = CLOCKRegType::TASKS_HFCLKSTART.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_HFCLKSTART) }
    }


    pub fn get_lfrcmode_mut(&mut self) -> &mut LFRCMODE {
        let word_offset = CLOCKRegType::LFRCMODE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut LFRCMODE) }
    }
    
    pub fn get_traceconfig_mut(&mut self) -> &mut TRACECONFIG {
        let word_offset = CLOCKRegType::TRACECONFIG.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TRACECONFIG) }
    }
    
    pub fn get_ctiv_mut(&mut self) -> &mut CTIV {
        let word_offset = CLOCKRegType::CTIV.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CTIV) }
    }
    
    pub fn get_hfxodebounce_mut(&mut self) -> &mut HFXODEBOUNCE {
        let word_offset = CLOCKRegType::HFXODEBOUNCE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut HFXODEBOUNCE) }
    }
    
    pub fn get_lfclksrc_mut(&mut self) -> &mut LFCLKSRC {
        let word_offset = CLOCKRegType::LFCLKSRC.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut LFCLKSRC) }
    }
    
    pub fn get_lfclksrccopy_mut(&mut self) -> &mut LFCLKSRCCOPY {
        let word_offset = CLOCKRegType::LFCLKSRCCOPY.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut LFCLKSRCCOPY) }
    }
    
    pub fn get_lfclkstat_mut(&mut self) -> &mut LFCLKSTAT {
        let word_offset = CLOCKRegType::LFCLKSTAT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut LFCLKSTAT) }
    }
    
    pub fn get_lfclkrun_mut(&mut self) -> &mut LFCLKRUN {
        let word_offset = CLOCKRegType::LFCLKRUN.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut LFCLKRUN) }
    }
    
    pub fn get_hfclkstat_mut(&mut self) -> &mut HFCLKSTAT {
        let word_offset = CLOCKRegType::HFCLKSTAT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut HFCLKSTAT) }
    }
    
    pub fn get_hfclkrun_mut(&mut self) -> &mut HFCLKRUN {
        let word_offset = CLOCKRegType::HFCLKRUN.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut HFCLKRUN) }
    }
    
    pub fn get_intenclr_mut(&mut self) -> &mut INTENCLR {
        let word_offset = CLOCKRegType::INTENCLR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENCLR) }
    }
    
    pub fn get_intenset_mut(&mut self) -> &mut INTENSET {
        let word_offset = CLOCKRegType::INTENSET.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENSET) }
    }
    
    pub fn get_events_ctstopped_mut(&mut self) -> &mut EVENTS_CTSTOPPED {
        let word_offset = CLOCKRegType::EVENTS_CTSTOPPED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_CTSTOPPED) }
    }
    
    pub fn get_events_ctstarted_mut(&mut self) -> &mut EVENTS_CTSTARTED {
        let word_offset = CLOCKRegType::EVENTS_CTSTARTED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_CTSTARTED) }
    }
    
    pub fn get_events_ctto_mut(&mut self) -> &mut EVENTS_CTTO {
        let word_offset = CLOCKRegType::EVENTS_CTTO.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_CTTO) }
    }
    
    pub fn get_events_done_mut(&mut self) -> &mut EVENTS_DONE {
        let word_offset = CLOCKRegType::EVENTS_DONE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_DONE) }
    }
    
    pub fn get_events_lfclkstarted_mut(&mut self) -> &mut EVENTS_LFCLKSTARTED {
        let word_offset = CLOCKRegType::EVENTS_LFCLKSTARTED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_LFCLKSTARTED) }
    }
    
    pub fn get_events_hfclkstarted_mut(&mut self) -> &mut EVENTS_HFCLKSTARTED {
        let word_offset = CLOCKRegType::EVENTS_HFCLKSTARTED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_HFCLKSTARTED) }
    }
    
    pub fn get_tasks_ctstop_mut(&mut self) -> &mut TASKS_CTSTOP {
        let word_offset = CLOCKRegType::TASKS_CTSTOP.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_CTSTOP) }
    }
    
    pub fn get_tasks_ctstart_mut(&mut self) -> &mut TASKS_CTSTART {
        let word_offset = CLOCKRegType::TASKS_CTSTART.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_CTSTART) }
    }
    
    pub fn get_tasks_cal_mut(&mut self) -> &mut TASKS_CAL {
        let word_offset = CLOCKRegType::TASKS_CAL.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_CAL) }
    }
    
    pub fn get_tasks_lfclkstop_mut(&mut self) -> &mut TASKS_LFCLKSTOP {
        let word_offset = CLOCKRegType::TASKS_LFCLKSTOP.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_LFCLKSTOP) }
    }
    
    pub fn get_tasks_lfclkstart_mut(&mut self) -> &mut TASKS_LFCLKSTART {
        let word_offset = CLOCKRegType::TASKS_LFCLKSTART.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_LFCLKSTART) }
    }
    
    pub fn get_tasks_hfclkstop_mut(&mut self) -> &mut TASKS_HFCLKSTOP {
        let word_offset = CLOCKRegType::TASKS_HFCLKSTOP.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_HFCLKSTOP) }
    }
    
    pub fn get_tasks_hfclkstart_mut(&mut self) -> &mut TASKS_HFCLKSTART {
        let word_offset = CLOCKRegType::TASKS_HFCLKSTART.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_HFCLKSTART) }
    }
    


}