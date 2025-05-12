//! uart.rs
//! 
//! UART module
//! Universal Asynchronous Receiver/Transmitter
use std::fmt;
use std::collections::VecDeque;

use thiserror::Error;
use bitfield_struct::bitfield;
use crossbeam::channel::{
    Sender,
    Receiver,
    TrySendError,
    TryRecvError,
};

use libcme::prelude::*;
use libcme::peripheral::{ Error, Event };
// use libcme::utils::*;

// use super::*;

mod registers;
pub use registers::*;

pub static UART0_BASE: u32 = 0x40002000;


#[derive(Debug, Error)]
pub enum UartError {
    #[error("attempted to write to read-only reg: {0:?}")]
    WriteViolation(UARTRegType),
    #[error("attempted to read to write-only reg: {0:?}")]
    ReadViolation(UARTRegType),
    #[error("tx channel: {0:?}")]
    TxChannel(TrySendError<u8>),
    #[error("rx channel: {0:?}")]
    RxChannel(TryRecvError),
}

impl From<TrySendError<u8>> for UartError {
    fn from(err: TrySendError<u8>) -> Self {
        Self::TxChannel(err)
    }
}

impl From<TryRecvError> for UartError {
    fn from(err: TryRecvError) -> Self {
        Self::RxChannel(err)
    }
}


#[derive(Clone)]
pub struct UARTState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
    rx_channel: (Sender<u8>, Receiver<u8>),
    tx_channel: (Sender<u8>, Receiver<u8>),
    // rxd_buf: [u8; 6]
}

impl fmt::Debug for UARTState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UART @ {:#x}", self.base_address)
    }
}

impl PeripheralState for UARTState {
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

impl AsRef<[u8]> for UARTState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for UARTState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl UARTState {
    pub fn new_with(
        rx_channel: (Sender<u8>, Receiver<u8>),
        tx_channel: (Sender<u8>, Receiver<u8>),
    ) -> Self {
        let base_address = UART0_BASE;
        let backing = Box::new([0u32; 0x400]);
        // let rxd_buf = [0; 6];
        let state = Self {
            base_address,
            backing,
            rx_channel,
            tx_channel,
            // rxd_buf,
        };
        state.reset()
    }

    pub fn reset(mut self) -> Self {
        self.backing = Box::new([0u32; 0x400]);
        // self.rxd_buf = [0; 6];
        for reg_type in UARTRegType::list() {
            let offset = reg_type.offset();
            if let Some(reset_value) = reg_type.reset() {
                self.backing[offset] = reset_value;
            }
        }
        self
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
        let Some(reg_type) = UARTRegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[offset..offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            UARTRegType::TASKS_STARTRX
            | UARTRegType::TASKS_STOPRX
            | UARTRegType::TASKS_STARTTX
            | UARTRegType::TASKS_STOPTX
            | UARTRegType::TASKS_SUSPEND
            | UARTRegType::TXD => {
                let err = UartError::ReadViolation(reg_type);
                Err(peripheral::Error::State(err.into()))
            }
            UARTRegType::EVENTS_CTS
            | UARTRegType::EVENTS_NCTS
            | UARTRegType::EVENTS_RXDRDY
            | UARTRegType::EVENTS_TXDRDY
            | UARTRegType::EVENTS_ERROR
            | UARTRegType::EVENTS_RXTO 
            | UARTRegType::SHORTS
            | UARTRegType::INTENSET
            | UARTRegType::INTENCLR
            | UARTRegType::ERRORSRC
            | UARTRegType::ENABLE
            | UARTRegType::PSELRTS
            | UARTRegType::PSELTXD
            | UARTRegType::PSELCTS
            | UARTRegType::PSELRXD
            | UARTRegType::BAUDRATE
            | UARTRegType::CONFIG => {
                let val = self.backing[word_offset].to_le_bytes();
                dst.copy_from_slice(&val);
                Ok(())
            }
            UARTRegType::RXD => {
                if self.get_enable().enable() == 4
                    && self.get_tasks_startrx().tasks_startrx()
                    && !self.get_tasks_suspend().tasks_suspend()
                    && !self.get_events_rxdrdy().events_rxdrdy()
                {
                    let val = self.rx_channel.1.try_recv()
                        .map_err(|e| {
                            let e = UartError::from(e);
                            peripheral::Error::State(e.into())
                        })?;
                    debug!("read byte {val:#x} from UART RXD");
                    dst[0] = val;
                    self.get_events_rxdrdy_mut().set_events_rxdrdy(true);
                }
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
        let Some(reg_type) = UARTRegType::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[offset..offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            UARTRegType::TASKS_STARTRX => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                // note that for tasks, val probably has to be exactly `1`
                let val = (u32::from_le_bytes(val) != 0) as u32;
                self.backing[word_offset] = val;
                let tasks_stoprx = UARTRegType::TASKS_STOPRX.offset();
                // treat stoprx as the inversion of startrx and vice versa
                // it's write-only so this is only for internal representation
                self.backing[tasks_stoprx] = (val != 1) as u32;
                Ok(())
            }
            UARTRegType::TASKS_STOPRX => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = (u32::from_le_bytes(val) != 0) as u32;
                self.backing[word_offset] = (val != 0) as u32;
                let tasks_startrx = UARTRegType::TASKS_STARTRX.offset();
                self.backing[tasks_startrx] = (val != 1) as u32;
                Ok(())
            }
            UARTRegType::TASKS_STARTTX => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                // note that for tasks, val probably has to be exactly `1`
                let val = (u32::from_le_bytes(val) != 0) as u32;
                self.backing[word_offset] = val;
                let tasks_stoptx = UARTRegType::TASKS_STOPTX.offset();
                // treat stoprx as the inversion of startrx and vice versa
                // it's write-only so this is only for internal representation
                self.backing[tasks_stoptx] = (val != 1) as u32;
                Ok(())
            }
            UARTRegType::TASKS_STOPTX => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = (u32::from_le_bytes(val) != 0) as u32;
                self.backing[word_offset] = (val != 0) as u32;
                let tasks_starttx = UARTRegType::TASKS_STARTTX.offset();
                self.backing[tasks_starttx] = (val != 1) as u32;
                Ok(())
            }
            UARTRegType::TASKS_SUSPEND => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                // i think tasks are all single bit values,
                // need to clarify in docs
                self.backing[word_offset] = (val != 0) as u32;
                Ok(())
            }
            UARTRegType::EVENTS_CTS
            | UARTRegType::EVENTS_NCTS
            | UARTRegType::EVENTS_RXDRDY
            | UARTRegType::EVENTS_TXDRDY
            | UARTRegType::EVENTS_ERROR
            | UARTRegType::EVENTS_RXTO => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                // i think events are all single bit values,
                // need to clarify in docs
                self.backing[word_offset] = (val != 0) as u32;
                Ok(())
            }
            UARTRegType::SHORTS => {
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                let new_shorts = SHORTS::from_bits(val);
                *self.get_shorts_mut() = new_shorts;
                Ok(())
            }
            UARTRegType::INTENSET => {
                // need to set both intenset and intenclr registers
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                // let old_intenset = self.backing[word_offset];
                self.backing[word_offset] |= val;
                let intenclr_offset = UARTRegType::INTENCLR.offset();
                self.backing[intenclr_offset] |= val;

                // interrupt enable probably not be necessary here
                // if old_intenset & 0x00020287 == 0 && val & 0x00020287 != 0 {
                //     let evt = peripheral::Event::EnableInterrupt {
                //         int_num: 2,
                //     };
                //     events.push_back(evt);
                // }
                Ok(())
            }
            UARTRegType::INTENCLR => {
                // need to clear both intenset and intentclr registers
                let mut val = [0u8; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                // let old_intenclr = self.backing[word_offset];
                self.backing[word_offset] &= val ^ 0xFFFFFFFF;
                let intenset_offset = UARTRegType::INTENSET.offset();
                self.backing[intenset_offset] &= val ^ 0xFFFFFFFF;

                // interrupt disable probably not be necessary here
                // if old_intenclr & 0x00020287 != 0
                //     && self.backing[word_offset] & 0x00020287 == 0
                // {
                //     let evt = peripheral::Event::DisableInterrupt {
                //         int_num: 2,
                //     };
                //     events.push_back(evt);
                // }
                Ok(())
            }
            UARTRegType::ERRORSRC => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) & 0xF;
                self.backing[word_offset] = val;
                Ok(())
            }
            UARTRegType::ENABLE => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) & 0xF;
                // do i need a peripheral enable event?
                self.get_enable_mut().set_enable(val as u8);
                Ok(())
            }
            UARTRegType::PSELRTS
            | UARTRegType::PSELTXD
            | UARTRegType::PSELCTS
            | UARTRegType::PSELRXD => {
                let slice = &mut self
                    .view_as_bytes_mut()[offset..offset + src.len()];
                slice.copy_from_slice(src);
                Ok(())
            }
            UARTRegType::RXD => {
                let err = UartError::WriteViolation(reg_type);
                Err(peripheral::Error::State(err.into()))
            }
            UARTRegType::TXD => {
                // starttx task must be set
                // and txdrdy event must be cleared before transmission
                if self.get_enable().enable() == 4
                    && self.get_tasks_starttx().tasks_starttx()
                    && !self.get_tasks_suspend().tasks_suspend()
                    && !self.get_events_txdrdy().events_txdrdy()
                {
                    self.tx_channel.0.try_send(src[0])
                        .map_err(|e| {
                            let e = UartError::from(e);
                            peripheral::Error::State(e.into())
                        })?;
                    debug!("write byte {:#x} to UART RXD", src[0]);
                    self.get_events_txdrdy_mut().set_events_txdrdy(true);
                }
                Ok(())
            }
            UARTRegType::BAUDRATE => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val);
                match val {
                    0x0004F000    /* 1200 baud */    
                    | 0x0009D000  /* 2400 baud */  
                    | 0x0013B000  /* 4800 baud */  
                    | 0x00275000  /* 9600 baud */  
                    | 0x003B0000  /* 14400 baud */  
                    | 0x004EA000  /* 19200 baud */  
                    | 0x0075F000  /* 28800 baud */  
                    | 0x009D5000  /* 38400 baud */  
                    | 0x00EBF000  /* 57600 baud */  
                    | 0x013A9000  /* 76800 baud */  
                    | 0x01D7E000  /* 115200 baud */  
                    | 0x03AFB000  /* 230400 baud */  
                    | 0x04000000  /* 250000 baud */  
                    | 0x075F7000  /* 460800 baud */  
                    | 0x0EBED000  /* 921600 baud */  
                    | 0x10000000  /* 1M baud */ => {
                        self.backing[word_offset] = val;
                    }
                    _ => {
                        // for now assume invalid values are ignored
                    }
                }
                Ok(())
            }
            UARTRegType::CONFIG => {
                let mut val = [0; 4];
                val.copy_from_slice(src);
                let val = u32::from_le_bytes(val) & 0xF;
                self.backing[word_offset] = val;
                Ok(())
            }
            
        }
    }
}


impl UARTState {
    // register reference getters

    pub fn get_tasks_startrx(&self) -> &TASKS_STARTRX {
        let word_offset = UARTRegType::TASKS_STARTRX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STARTRX) }
    }
    
    pub fn get_tasks_stoprx(&self) -> &TASKS_STOPRX {
        let word_offset = UARTRegType::TASKS_STOPRX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STOPRX) }
    }
    
    pub fn get_tasks_starttx(&self) -> &TASKS_STARTTX {
        let word_offset = UARTRegType::TASKS_STARTTX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STARTTX) }
    }
    
    pub fn get_tasks_stoptx(&self) -> &TASKS_STOPTX {
        let word_offset = UARTRegType::TASKS_STOPTX.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_STOPTX) }
    }
    
    pub fn get_tasks_suspend(&self) -> &TASKS_SUSPEND {
        let word_offset = UARTRegType::TASKS_SUSPEND.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TASKS_SUSPEND) }
    }
    
    pub fn get_events_cts(&self) -> &EVENTS_CTS {
        let word_offset = UARTRegType::EVENTS_CTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_CTS) }
    }
    
    pub fn get_events_ncts(&self) -> &EVENTS_NCTS {
        let word_offset = UARTRegType::EVENTS_NCTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_NCTS) }
    }
    
    pub fn get_events_rxdrdy(&self) -> &EVENTS_RXDRDY {
        let word_offset = UARTRegType::EVENTS_RXDRDY.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_RXDRDY) }
    }
    
    pub fn get_events_txdrdy(&self) -> &EVENTS_TXDRDY {
        let word_offset = UARTRegType::EVENTS_TXDRDY.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_TXDRDY) }
    }
    
    pub fn get_events_error(&self) -> &EVENTS_ERROR {
        let word_offset = UARTRegType::EVENTS_ERROR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_ERROR) }
    }
    
    pub fn get_events_rxto(&self) -> &EVENTS_RXTO {
        let word_offset = UARTRegType::EVENTS_RXTO.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const EVENTS_RXTO) }
    }
    
    pub fn get_shorts(&self) -> &SHORTS {
        let word_offset = UARTRegType::SHORTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const SHORTS) }
    }
    
    pub fn get_intenset(&self) -> &INTENSET {
        let word_offset = UARTRegType::INTENSET.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENSET) }
    }
    
    pub fn get_intenclr(&self) -> &INTENCLR {
        let word_offset = UARTRegType::INTENCLR.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const INTENCLR) }
    }
    
    pub fn get_errorsrc(&self) -> &ERRORSRC {
        let word_offset = UARTRegType::ERRORSRC.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ERRORSRC) }
    }
    
    pub fn get_enable(&self) -> &ENABLE {
        let word_offset = UARTRegType::ENABLE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ENABLE) }
    }
    
    pub fn get_pselrts(&self) -> &PSELRTS {
        let word_offset = UARTRegType::PSELRTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PSELRTS) }
    }
    
    pub fn get_pseltxd(&self) -> &PSELTXD {
        let word_offset = UARTRegType::PSELTXD.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PSELTXD) }
    }
    
    pub fn get_pselcts(&self) -> &PSELCTS {
        let word_offset = UARTRegType::PSELCTS.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PSELCTS) }
    }
    
    pub fn get_pselrxd(&self) -> &PSELRXD {
        let word_offset = UARTRegType::PSELRXD.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const PSELRXD) }
    }
    
    pub fn get_rxd(&self) -> &RXD {
        let word_offset = UARTRegType::RXD.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const RXD) }
    }
    
    pub fn get_txd(&self) -> &TXD {
        let word_offset = UARTRegType::TXD.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const TXD) }
    }
    
    pub fn get_baudrate(&self) -> &BAUDRATE {
        let word_offset = UARTRegType::BAUDRATE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const BAUDRATE) }
    }
    
    pub fn get_config(&self) -> &CONFIG {
        let word_offset = UARTRegType::CONFIG.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CONFIG) }
    }
    
    

    pub fn get_tasks_startrx_mut(&mut self) -> &mut TASKS_STARTRX {
        let word_offset = UARTRegType::TASKS_STARTRX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STARTRX) }
    }
    
    pub fn get_tasks_stoprx_mut(&mut self) -> &mut TASKS_STOPRX {
        let word_offset = UARTRegType::TASKS_STOPRX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STOPRX) }
    }
    
    pub fn get_tasks_starttx_mut(&mut self) -> &mut TASKS_STARTTX {
        let word_offset = UARTRegType::TASKS_STARTTX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STARTTX) }
    }
    
    pub fn get_tasks_stoptx_mut(&mut self) -> &mut TASKS_STOPTX {
        let word_offset = UARTRegType::TASKS_STOPTX.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_STOPTX) }
    }
    
    pub fn get_tasks_suspend_mut(&mut self) -> &mut TASKS_SUSPEND {
        let word_offset = UARTRegType::TASKS_SUSPEND.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TASKS_SUSPEND) }
    }
    
    pub fn get_events_cts_mut(&mut self) -> &mut EVENTS_CTS {
        let word_offset = UARTRegType::EVENTS_CTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_CTS) }
    }
    
    pub fn get_events_ncts_mut(&mut self) -> &mut EVENTS_NCTS {
        let word_offset = UARTRegType::EVENTS_NCTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_NCTS) }
    }
    
    pub fn get_events_rxdrdy_mut(&mut self) -> &mut EVENTS_RXDRDY {
        let word_offset = UARTRegType::EVENTS_RXDRDY.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_RXDRDY) }
    }
    
    pub fn get_events_txdrdy_mut(&mut self) -> &mut EVENTS_TXDRDY {
        let word_offset = UARTRegType::EVENTS_TXDRDY.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_TXDRDY) }
    }
    
    pub fn get_events_error_mut(&mut self) -> &mut EVENTS_ERROR {
        let word_offset = UARTRegType::EVENTS_ERROR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_ERROR) }
    }
    
    pub fn get_events_rxto_mut(&mut self) -> &mut EVENTS_RXTO {
        let word_offset = UARTRegType::EVENTS_RXTO.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut EVENTS_RXTO) }
    }
    
    pub fn get_shorts_mut(&mut self) -> &mut SHORTS {
        let word_offset = UARTRegType::SHORTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut SHORTS) }
    }
    
    pub fn get_intenset_mut(&mut self) -> &mut INTENSET {
        let word_offset = UARTRegType::INTENSET.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENSET) }
    }
    
    pub fn get_intenclr_mut(&mut self) -> &mut INTENCLR {
        let word_offset = UARTRegType::INTENCLR.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut INTENCLR) }
    }
    
    pub fn get_errorsrc_mut(&mut self) -> &mut ERRORSRC {
        let word_offset = UARTRegType::ERRORSRC.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ERRORSRC) }
    }
    
    pub fn get_enable_mut(&mut self) -> &mut ENABLE {
        let word_offset = UARTRegType::ENABLE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ENABLE) }
    }
    
    pub fn get_pselrts_mut(&mut self) -> &mut PSELRTS {
        let word_offset = UARTRegType::PSELRTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PSELRTS) }
    }
    
    pub fn get_pseltxd_mut(&mut self) -> &mut PSELTXD {
        let word_offset = UARTRegType::PSELTXD.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PSELTXD) }
    }
    
    pub fn get_pselcts_mut(&mut self) -> &mut PSELCTS {
        let word_offset = UARTRegType::PSELCTS.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PSELCTS) }
    }
    
    pub fn get_pselrxd_mut(&mut self) -> &mut PSELRXD {
        let word_offset = UARTRegType::PSELRXD.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut PSELRXD) }
    }
    
    pub fn get_rxd_mut(&mut self) -> &mut RXD {
        let word_offset = UARTRegType::RXD.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut RXD) }
    }
    
    pub fn get_txd_mut(&mut self) -> &mut TXD {
        let word_offset = UARTRegType::TXD.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut TXD) }
    }
    
    pub fn get_baudrate_mut(&mut self) -> &mut BAUDRATE {
        let word_offset = UARTRegType::BAUDRATE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut BAUDRATE) }
    }
    
    pub fn get_config_mut(&mut self) -> &mut CONFIG {
        let word_offset = UARTRegType::CONFIG.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CONFIG) }
    }
    
    

    

    
}