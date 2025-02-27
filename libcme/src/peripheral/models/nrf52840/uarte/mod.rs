//! uarte.rs
//! 
//! UARTE module
//! UART with EasyDMA 0
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


pub static UARTE0_BASE: u32 = 0x40002000;
pub static UARTE1_BASE: u32 = 0x40028000;

#[derive(Clone)]
pub struct UARTEState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
}

impl fmt::Debug for UARTEState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UARTE @ {:#x}", self.base_address)
    }
}

impl PeripheralState for UARTEState {
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

impl AsRef<[u8]> for UARTEState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for UARTEState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for UARTEState {
    fn default() -> Self {
        let base_address = 0x40002000;
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
    }
}

impl UARTEState {
    pub fn new_with(base_address: u32) -> Self {
        let mut backing = Box::new([0u32; 0x400]);
        for reg_type in UARTERegType::list() {
            let offset = reg_type.offset();
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
        let byte_offset = offset & 0b11;
        let Some(reg_type) = UARTERegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[byte_offset..byte_offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            UARTERegType::CONFIG     => { todo!() }
            UARTERegType::BAUDRATE   => { todo!() }
            UARTERegType::ENABLE     => { todo!() }
            UARTERegType::ERRORSRC   => { todo!() }
            UARTERegType::INTENCLR   => { todo!() }
            UARTERegType::INTENSET   => { todo!() }
            UARTERegType::INTEN      => { todo!() }
            UARTERegType::SHORTS     => { todo!() }
            UARTERegType::EVENTS_TXSTOPPED => { todo!() }
            UARTERegType::EVENTS_TXSTARTED => { todo!() }
            UARTERegType::EVENTS_RXSTARTED => { todo!() }
            UARTERegType::EVENTS_RXTO => { todo!() }
            UARTERegType::EVENTS_ERROR => { todo!() }
            UARTERegType::EVENTS_ENDTX => { todo!() }
            UARTERegType::EVENTS_TXDRDY => { todo!() }
            UARTERegType::EVENTS_ENDRX => { todo!() }
            UARTERegType::EVENTS_RXDRDY => { todo!() }
            UARTERegType::EVENTS_NCTS => { todo!() }
            UARTERegType::EVENTS_CTS => { todo!() }
            UARTERegType::TASKS_FLUSHRX => { todo!() }
            UARTERegType::TASKS_STOPTX => { todo!() }
            UARTERegType::TASKS_STARTTX => { todo!() }
            UARTERegType::TASKS_STOPRX => { todo!() }
            UARTERegType::TASKS_STARTRX => { todo!() }

            UARTERegType::TXD(reg) => { todo!() }
            UARTERegType::RXD(reg) => { todo!() }
            UARTERegType::PSEL(reg) => { todo!() }
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
        let Some(reg_type) = UARTERegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[byte_offset..byte_offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            UARTERegType::CONFIG     => { todo!() }
            UARTERegType::BAUDRATE   => { todo!() }
            UARTERegType::ENABLE     => { todo!() }
            UARTERegType::ERRORSRC   => { todo!() }
            UARTERegType::INTENCLR   => { todo!() }
            UARTERegType::INTENSET   => { todo!() }
            UARTERegType::INTEN      => { todo!() }
            UARTERegType::SHORTS     => { todo!() }
            UARTERegType::EVENTS_TXSTOPPED => { todo!() }
            UARTERegType::EVENTS_TXSTARTED => { todo!() }
            UARTERegType::EVENTS_RXSTARTED => { todo!() }
            UARTERegType::EVENTS_RXTO => { todo!() }
            UARTERegType::EVENTS_ERROR => { todo!() }
            UARTERegType::EVENTS_ENDTX => { todo!() }
            UARTERegType::EVENTS_TXDRDY => { todo!() }
            UARTERegType::EVENTS_ENDRX => { todo!() }
            UARTERegType::EVENTS_RXDRDY => { todo!() }
            UARTERegType::EVENTS_NCTS => { todo!() }
            UARTERegType::EVENTS_CTS => { todo!() }
            UARTERegType::TASKS_FLUSHRX => { todo!() }
            UARTERegType::TASKS_STOPTX => { todo!() }
            UARTERegType::TASKS_STARTTX => { todo!() }
            UARTERegType::TASKS_STOPRX => { todo!() }
            UARTERegType::TASKS_STARTRX => { todo!() }

            UARTERegType::TXD(reg) => { todo!() }
            UARTERegType::RXD(reg) => { todo!() }
            UARTERegType::PSEL(reg) => { todo!() }
        }
    }
}


impl UARTEState {
    // register reference getters

    pub fn get_config(&self) -> &CONFIG {
        let word_offset = UARTERegType::CONFIG.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CONFIG) }
    }

    pub fn get_baudrate(&self) -> &BAUDRATE {
        let word_offset = UARTERegType::BAUDRATE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const BAUDRATE) }
    }

    pub fn get_enable(&self) -> &ENABLE {
        let word_offset = UARTERegType::ENABLE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ENABLE) }
    }

    pub fn get_errorsrc(&self) -> &ERRORSRC {
        let word_offset = UARTERegType::ERRORSRC.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ERRORSRC) }
    }

    pub fn get_intenclr(&self) -> &INTENCLR {
        let word_offset = UARTERegType::INTENCLR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENCLR) }
    }

    pub fn get_intenset(&self) -> &INTENSET {
        let word_offset = UARTERegType::INTENSET.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENSET) }
    }

    pub fn get_inten(&self) -> &INTEN {
        let word_offset = UARTERegType::INTEN.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTEN) }
    }

    pub fn get_shorts(&self) -> &SHORTS {
        let word_offset = UARTERegType::SHORTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHORTS) }
    }

    pub fn get_events_txstopped(&self) -> &EVENTS_TXSTOPPED {
        let word_offset = UARTERegType::EVENTS_TXSTOPPED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_TXSTOPPED) }
    }

    pub fn get_events_txstarted(&self) -> &EVENTS_TXSTARTED {
        let word_offset = UARTERegType::EVENTS_TXSTARTED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_TXSTARTED) }
    }

    pub fn get_events_rxstarted(&self) -> &EVENTS_RXSTARTED {
        let word_offset = UARTERegType::EVENTS_RXSTARTED.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_RXSTARTED) }
    }

    pub fn get_events_rxto(&self) -> &EVENTS_RXTO {
        let word_offset = UARTERegType::EVENTS_RXTO.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_RXTO) }
    }

    pub fn get_events_error(&self) -> &EVENTS_ERROR {
        let word_offset = UARTERegType::EVENTS_ERROR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_ERROR) }
    }

    pub fn get_events_endtx(&self) -> &EVENTS_ENDTX {
        let word_offset = UARTERegType::EVENTS_ENDTX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_ENDTX) }
    }

    pub fn get_events_txdrdy(&self) -> &EVENTS_TXDRDY {
        let word_offset = UARTERegType::EVENTS_TXDRDY.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_TXDRDY) }
    }

    pub fn get_events_endrx(&self) -> &EVENTS_ENDRX {
        let word_offset = UARTERegType::EVENTS_ENDRX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_ENDRX) }
    }

    pub fn get_events_rxdrdy(&self) -> &EVENTS_RXDRDY {
        let word_offset = UARTERegType::EVENTS_RXDRDY.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_RXDRDY) }
    }

    pub fn get_events_ncts(&self) -> &EVENTS_NCTS {
        let word_offset = UARTERegType::EVENTS_NCTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_NCTS) }
    }

    pub fn get_events_cts(&self) -> &EVENTS_CTS {
        let word_offset = UARTERegType::EVENTS_CTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_CTS) }
    }

    pub fn get_tasks_flushrx(&self) -> &TASKS_FLUSHRX {
        let word_offset = UARTERegType::TASKS_FLUSHRX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_FLUSHRX) }
    }

    pub fn get_tasks_stoptx(&self) -> &TASKS_STOPTX {
        let word_offset = UARTERegType::TASKS_STOPTX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STOPTX) }
    }

    pub fn get_tasks_starttx(&self) -> &TASKS_STARTTX {
        let word_offset = UARTERegType::TASKS_STARTTX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STARTTX) }
    }

    pub fn get_tasks_stoprx(&self) -> &TASKS_STOPRX {
        let word_offset = UARTERegType::TASKS_STOPRX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STOPRX) }
    }

    pub fn get_tasks_startrx(&self) -> &TASKS_STARTRX {
        let word_offset = UARTERegType::TASKS_STARTRX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STARTRX) }
    }


    pub fn get_config_mut(&mut self) -> &mut CONFIG {
        let word_offset = UARTERegType::CONFIG.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CONFIG) }
    }
    
    pub fn get_baudrate_mut(&mut self) -> &mut BAUDRATE {
        let word_offset = UARTERegType::BAUDRATE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut BAUDRATE) }
    }
    
    pub fn get_enable_mut(&mut self) -> &mut ENABLE {
        let word_offset = UARTERegType::ENABLE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ENABLE) }
    }
    
    pub fn get_errorsrc_mut(&mut self) -> &mut ERRORSRC {
        let word_offset = UARTERegType::ERRORSRC.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ERRORSRC) }
    }
    
    pub fn get_intenclr_mut(&mut self) -> &mut INTENCLR {
        let word_offset = UARTERegType::INTENCLR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENCLR) }
    }
    
    pub fn get_intenset_mut(&mut self) -> &mut INTENSET {
        let word_offset = UARTERegType::INTENSET.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENSET) }
    }
    
    pub fn get_inten_mut(&mut self) -> &mut INTEN {
        let word_offset = UARTERegType::INTEN.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTEN) }
    }
    
    pub fn get_shorts_mut(&mut self) -> &mut SHORTS {
        let word_offset = UARTERegType::SHORTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHORTS) }
    }
    
    pub fn get_events_txstopped_mut(&mut self) -> &mut EVENTS_TXSTOPPED {
        let word_offset = UARTERegType::EVENTS_TXSTOPPED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_TXSTOPPED) }
    }
    
    pub fn get_events_txstarted_mut(&mut self) -> &mut EVENTS_TXSTARTED {
        let word_offset = UARTERegType::EVENTS_TXSTARTED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_TXSTARTED) }
    }
    
    pub fn get_events_rxstarted_mut(&mut self) -> &mut EVENTS_RXSTARTED {
        let word_offset = UARTERegType::EVENTS_RXSTARTED.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_RXSTARTED) }
    }
    
    pub fn get_events_rxto_mut(&mut self) -> &mut EVENTS_RXTO {
        let word_offset = UARTERegType::EVENTS_RXTO.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_RXTO) }
    }
    
    pub fn get_events_error_mut(&mut self) -> &mut EVENTS_ERROR {
        let word_offset = UARTERegType::EVENTS_ERROR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_ERROR) }
    }
    
    pub fn get_events_endtx_mut(&mut self) -> &mut EVENTS_ENDTX {
        let word_offset = UARTERegType::EVENTS_ENDTX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_ENDTX) }
    }
    
    pub fn get_events_txdrdy_mut(&mut self) -> &mut EVENTS_TXDRDY {
        let word_offset = UARTERegType::EVENTS_TXDRDY.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_TXDRDY) }
    }
    
    pub fn get_events_endrx_mut(&mut self) -> &mut EVENTS_ENDRX {
        let word_offset = UARTERegType::EVENTS_ENDRX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_ENDRX) }
    }
    
    pub fn get_events_rxdrdy_mut(&mut self) -> &mut EVENTS_RXDRDY {
        let word_offset = UARTERegType::EVENTS_RXDRDY.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_RXDRDY) }
    }
    
    pub fn get_events_ncts_mut(&mut self) -> &mut EVENTS_NCTS {
        let word_offset = UARTERegType::EVENTS_NCTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_NCTS) }
    }
    
    pub fn get_events_cts_mut(&mut self) -> &mut EVENTS_CTS {
        let word_offset = UARTERegType::EVENTS_CTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_CTS) }
    }
    
    pub fn get_tasks_flushrx_mut(&mut self) -> &mut TASKS_FLUSHRX {
        let word_offset = UARTERegType::TASKS_FLUSHRX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_FLUSHRX) }
    }
    
    pub fn get_tasks_stoptx_mut(&mut self) -> &mut TASKS_STOPTX {
        let word_offset = UARTERegType::TASKS_STOPTX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STOPTX) }
    }
    
    pub fn get_tasks_starttx_mut(&mut self) -> &mut TASKS_STARTTX {
        let word_offset = UARTERegType::TASKS_STARTTX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STARTTX) }
    }
    
    pub fn get_tasks_stoprx_mut(&mut self) -> &mut TASKS_STOPRX {
        let word_offset = UARTERegType::TASKS_STOPRX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STOPRX) }
    }
    
    pub fn get_tasks_startrx_mut(&mut self) -> &mut TASKS_STARTRX {
        let word_offset = UARTERegType::TASKS_STARTRX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STARTRX) }
    }
    

    pub fn get_txd_ptr(&self) -> &txd::PTR {
        let word_offset = TXDRegType::PTR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const txd::PTR) }
    }

    pub fn get_txd_maxcnt(&self) -> &txd::MAXCNT {
        let word_offset = TXDRegType::MAXCNT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const txd::MAXCNT) }
    }

    pub fn get_txd_amount(&self) -> &txd::AMOUNT {
        let word_offset = TXDRegType::AMOUNT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const txd::AMOUNT) }
    }

    pub fn get_rxd_ptr(&self) -> &rxd::PTR {
        let word_offset = RXDRegType::PTR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const rxd::PTR) }
    }

    pub fn get_rxd_maxcnt(&self) -> &rxd::MAXCNT {
        let word_offset = RXDRegType::MAXCNT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const rxd::MAXCNT) }
    }

    pub fn get_rxd_amount(&self) -> &rxd::AMOUNT {
        let word_offset = RXDRegType::AMOUNT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const rxd::AMOUNT) }
    }

    pub fn get_psel_rts(&self) -> &psel::RTS {
        let word_offset = PSELRegType::RTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const psel::RTS) }
    }

    pub fn get_psel_txd(&self) -> &psel::TXD {
        let word_offset = PSELRegType::TXD.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const psel::TXD) }
    }

    pub fn get_psel_cts(&self) -> &psel::CTS {
        let word_offset = PSELRegType::CTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const psel::CTS) }
    }

    pub fn get_psel_rxd(&self) -> &psel::RXD {
        let word_offset = PSELRegType::RXD.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const psel::RXD) }
    }


    pub fn get_txd_ptr_mut(&mut self) -> &mut txd::PTR {
        let word_offset = TXDRegType::PTR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut txd::PTR) }
    }

    pub fn get_txd_maxcnt_mut(&mut self) -> &mut txd::MAXCNT {
        let word_offset = TXDRegType::MAXCNT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut txd::MAXCNT) }
    }

    pub fn get_txd_amount_mut(&mut self) -> &mut txd::AMOUNT {
        let word_offset = TXDRegType::AMOUNT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut txd::AMOUNT) }
    }

    pub fn get_rxd_ptr_mut(&mut self) -> &mut rxd::PTR {
        let word_offset = RXDRegType::PTR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut rxd::PTR) }
    }

    pub fn get_rxd_maxcnt_mut(&mut self) -> &mut rxd::MAXCNT {
        let word_offset = RXDRegType::MAXCNT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut rxd::MAXCNT) }
    }

    pub fn get_rxd_amount_mut(&mut self) -> &mut rxd::AMOUNT {
        let word_offset = RXDRegType::AMOUNT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut rxd::AMOUNT) }
    }

    pub fn get_psel_rts_mut(&mut self) -> &mut psel::RTS {
        let word_offset = PSELRegType::RTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut psel::RTS) }
    }

    pub fn get_psel_txd_mut(&mut self) -> &mut psel::TXD {
        let word_offset = PSELRegType::TXD.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut psel::TXD) }
    }

    pub fn get_psel_cts_mut(&mut self) -> &mut psel::CTS {
        let word_offset = PSELRegType::CTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut psel::CTS) }
    }

    pub fn get_psel_rxd_mut(&mut self) -> &mut psel::RXD {
        let word_offset = PSELRegType::RXD.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut psel::RXD) }
    }

}