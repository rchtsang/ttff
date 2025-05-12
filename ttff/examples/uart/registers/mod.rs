//! registers.rs
//!
//! UART register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use libcme::types::*;
use super::*;



/// UART register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UARTRegType {
    /// Start UART receiver
    TASKS_STARTRX,
    /// Stop UART receiver
    TASKS_STOPRX,
    /// Start UART transmitter
    TASKS_STARTTX,
    /// Stop UART transmitter
    TASKS_STOPTX,
    /// Suspend UART
    TASKS_SUSPEND,
    /// CTS is activated (set low). Clear To Send.
    EVENTS_CTS,
    /// CTS is deactivated (set high). Not Clear To Send.
    EVENTS_NCTS,
    /// Data received in RXD
    EVENTS_RXDRDY,
    /// Data sent from TXD
    EVENTS_TXDRDY,
    /// Error detected
    EVENTS_ERROR,
    /// Receiver timeout
    EVENTS_RXTO,
    /// Shortcuts between local events and tasks
    SHORTS,
    /// Enable interrupt
    INTENSET,
    /// Disable interrupt
    INTENCLR,
    /// Error source
    ERRORSRC,
    /// Enable UART
    ENABLE,
    /// Pin select for RTS
    PSELRTS,
    /// Pin select for TXD
    PSELTXD,
    /// Pin select for CTS
    PSELCTS,
    /// Pin select for RXD
    PSELRXD,
    /// RXD register
    RXD,
    /// TXD register
    TXD,
    /// Baud rate
    BAUDRATE,
    /// Configuration of parity and hardware flow control
    CONFIG,
    
}

impl UARTRegType {

    pub fn address(&self, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data().offset as u64))
    }

    // offset in bytes
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    pub fn perms(&self) -> FlagSet<Permission> {
        unsafe { FlagSet::<Permission>::new_unchecked(self._data().perms) }
    }

    pub fn reset(&self) -> Option<u32> {
        self._data().reset
    }

    pub fn lookup_address(base: impl Into<u64>, address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(base.into())
            .expect("address not in peripheral!");
        Self::lookup_offset(offset as usize)
    }

    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(offset < 0x1000, "address not in peripheral!");
        match offset {
            0x0 => { Some(UARTRegType::TASKS_STARTRX) }
            0x4 => { Some(UARTRegType::TASKS_STOPRX) }
            0x8 => { Some(UARTRegType::TASKS_STARTTX) }
            0xc => { Some(UARTRegType::TASKS_STOPTX) }
            0x1c => { Some(UARTRegType::TASKS_SUSPEND) }
            0x100 => { Some(UARTRegType::EVENTS_CTS) }
            0x104 => { Some(UARTRegType::EVENTS_NCTS) }
            0x108 => { Some(UARTRegType::EVENTS_RXDRDY) }
            0x11c => { Some(UARTRegType::EVENTS_TXDRDY) }
            0x124 => { Some(UARTRegType::EVENTS_ERROR) }
            0x144 => { Some(UARTRegType::EVENTS_RXTO) }
            0x200 => { Some(UARTRegType::SHORTS) }
            0x304 => { Some(UARTRegType::INTENSET) }
            0x308 => { Some(UARTRegType::INTENCLR) }
            0x480 => { Some(UARTRegType::ERRORSRC) }
            0x500 => { Some(UARTRegType::ENABLE) }
            0x508 => { Some(UARTRegType::PSELRTS) }
            0x50c => { Some(UARTRegType::PSELTXD) }
            0x510 => { Some(UARTRegType::PSELCTS) }
            0x514 => { Some(UARTRegType::PSELRXD) }
            0x518 => { Some(UARTRegType::RXD) }
            0x51c => { Some(UARTRegType::TXD) }
            0x524 => { Some(UARTRegType::BAUDRATE) }
            0x56c => { Some(UARTRegType::CONFIG) }
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let types = vec![
            UARTRegType::TASKS_STARTRX,
            UARTRegType::TASKS_STOPRX,
            UARTRegType::TASKS_STARTTX,
            UARTRegType::TASKS_STOPTX,
            UARTRegType::TASKS_SUSPEND,
            UARTRegType::EVENTS_CTS,
            UARTRegType::EVENTS_NCTS,
            UARTRegType::EVENTS_RXDRDY,
            UARTRegType::EVENTS_TXDRDY,
            UARTRegType::EVENTS_ERROR,
            UARTRegType::EVENTS_RXTO,
            UARTRegType::SHORTS,
            UARTRegType::INTENSET,
            UARTRegType::INTENCLR,
            UARTRegType::ERRORSRC,
            UARTRegType::ENABLE,
            UARTRegType::PSELRTS,
            UARTRegType::PSELTXD,
            UARTRegType::PSELCTS,
            UARTRegType::PSELRXD,
            UARTRegType::RXD,
            UARTRegType::TXD,
            UARTRegType::BAUDRATE,
            UARTRegType::CONFIG,
        ];
        types
    }
}

impl UARTRegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            UARTRegType::TASKS_STARTRX => { &RegInfo { offset: 0, perms: 0b010, reset: None } }
            UARTRegType::TASKS_STOPRX => { &RegInfo { offset: 4, perms: 0b010, reset: None } }
            UARTRegType::TASKS_STARTTX => { &RegInfo { offset: 8, perms: 0b010, reset: None } }
            UARTRegType::TASKS_STOPTX => { &RegInfo { offset: 12, perms: 0b010, reset: None } }
            UARTRegType::TASKS_SUSPEND => { &RegInfo { offset: 28, perms: 0b010, reset: None } }
            UARTRegType::EVENTS_CTS => { &RegInfo { offset: 256, perms: 0b110, reset: None } }
            UARTRegType::EVENTS_NCTS => { &RegInfo { offset: 260, perms: 0b110, reset: None } }
            UARTRegType::EVENTS_RXDRDY => { &RegInfo { offset: 264, perms: 0b110, reset: None } }
            UARTRegType::EVENTS_TXDRDY => { &RegInfo { offset: 284, perms: 0b110, reset: None } }
            UARTRegType::EVENTS_ERROR => { &RegInfo { offset: 292, perms: 0b110, reset: None } }
            UARTRegType::EVENTS_RXTO => { &RegInfo { offset: 324, perms: 0b110, reset: None } }
            UARTRegType::SHORTS => { &RegInfo { offset: 512, perms: 0b110, reset: None } }
            UARTRegType::INTENSET => { &RegInfo { offset: 772, perms: 0b110, reset: None } }
            UARTRegType::INTENCLR => { &RegInfo { offset: 776, perms: 0b110, reset: None } }
            UARTRegType::ERRORSRC => { &RegInfo { offset: 1152, perms: 0b110, reset: None } }
            UARTRegType::ENABLE => { &RegInfo { offset: 1280, perms: 0b110, reset: None } }
            UARTRegType::PSELRTS => { &RegInfo { offset: 1288, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UARTRegType::PSELTXD => { &RegInfo { offset: 1292, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UARTRegType::PSELCTS => { &RegInfo { offset: 1296, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UARTRegType::PSELRXD => { &RegInfo { offset: 1300, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UARTRegType::RXD => { &RegInfo { offset: 1304, perms: 0b100, reset: None } }
            UARTRegType::TXD => { &RegInfo { offset: 1308, perms: 0b010, reset: None } }
            UARTRegType::BAUDRATE => { &RegInfo { offset: 1316, perms: 0b110, reset: Some(0x04000000) } }
            UARTRegType::CONFIG => { &RegInfo { offset: 1388, perms: 0b110, reset: None } }
            
            
            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/// TASKS_STARTRX
///
/// Start UART receiver
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_STARTRX {
    /// Start UART receiver
    #[bits(1)]
    pub tasks_startrx: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// TASKS_STOPRX
///
/// Stop UART receiver
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_STOPRX {
    /// Stop UART receiver
    #[bits(1)]
    pub tasks_stoprx: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// TASKS_STARTTX
///
/// Start UART transmitter
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_STARTTX {
    /// Start UART transmitter
    #[bits(1)]
    pub tasks_starttx: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// TASKS_STOPTX
///
/// Stop UART transmitter
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_STOPTX {
    /// Stop UART transmitter
    #[bits(1)]
    pub tasks_stoptx: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// TASKS_SUSPEND
///
/// Suspend UART
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_SUSPEND {
    /// Suspend UART
    #[bits(1)]
    pub tasks_suspend: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// EVENTS_CTS
///
/// CTS is activated (set low). Clear To Send.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_CTS {
    /// CTS is activated (set low). Clear To Send.
    #[bits(1)]
    pub events_cts: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// EVENTS_NCTS
///
/// CTS is deactivated (set high). Not Clear To Send.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_NCTS {
    /// CTS is deactivated (set high). Not Clear To Send.
    #[bits(1)]
    pub events_ncts: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// EVENTS_RXDRDY
///
/// Data received in RXD
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_RXDRDY {
    /// Data received in RXD
    #[bits(1)]
    pub events_rxdrdy: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// EVENTS_TXDRDY
///
/// Data sent from TXD
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_TXDRDY {
    /// Data sent from TXD
    #[bits(1)]
    pub events_txdrdy: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// EVENTS_ERROR
///
/// Error detected
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_ERROR {
    /// Error detected
    #[bits(1)]
    pub events_error: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// EVENTS_RXTO
///
/// Receiver timeout
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_RXTO {
    /// Receiver timeout
    #[bits(1)]
    pub events_rxto: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// SHORTS
///
/// Shortcuts between local events and tasks
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SHORTS {
    /// 
    #[bits(3)]
    pub __: u32,
    /// Shortcut between event CTS and task STARTRX
    #[bits(1)]
    pub cts_startrx: bool,
    /// Shortcut between event NCTS and task STOPRX
    #[bits(1)]
    pub ncts_stoprx: bool,
    /// 
    #[bits(27)]
    pub __: u32,
    
}

/// INTENSET
///
/// Enable interrupt
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INTENSET {
    /// Write '1' to enable interrupt for event CTS
    #[bits(1)]
    pub cts: bool,
    /// Write '1' to enable interrupt for event NCTS
    #[bits(1)]
    pub ncts: bool,
    /// Write '1' to enable interrupt for event RXDRDY
    #[bits(1)]
    pub rxdrdy: bool,
    /// 
    #[bits(4)]
    pub __: u32,
    /// Write '1' to enable interrupt for event TXDRDY
    #[bits(1)]
    pub txdrdy: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Write '1' to enable interrupt for event ERROR
    #[bits(1)]
    pub error: bool,
    /// 
    #[bits(7)]
    pub __: u32,
    /// Write '1' to enable interrupt for event RXTO
    #[bits(1)]
    pub rxto: bool,
    /// 
    #[bits(14)]
    pub __: u32,
    
}

/// INTENCLR
///
/// Disable interrupt
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INTENCLR {
    /// Write '1' to disable interrupt for event CTS
    #[bits(1)]
    pub cts: bool,
    /// Write '1' to disable interrupt for event NCTS
    #[bits(1)]
    pub ncts: bool,
    /// Write '1' to disable interrupt for event RXDRDY
    #[bits(1)]
    pub rxdrdy: bool,
    /// 
    #[bits(4)]
    pub __: u32,
    /// Write '1' to disable interrupt for event TXDRDY
    #[bits(1)]
    pub txdrdy: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Write '1' to disable interrupt for event ERROR
    #[bits(1)]
    pub error: bool,
    /// 
    #[bits(7)]
    pub __: u32,
    /// Write '1' to disable interrupt for event RXTO
    #[bits(1)]
    pub rxto: bool,
    /// 
    #[bits(14)]
    pub __: u32,
    
}

/// ERRORSRC
///
/// Error source
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ERRORSRC {
    /// Overrun error
    #[bits(1)]
    pub overrun: bool,
    /// Parity error
    #[bits(1)]
    pub parity: bool,
    /// Framing error occurred
    #[bits(1)]
    pub framing: bool,
    /// Break condition
    #[bits(1)]
    pub r#break: bool,
    /// 
    #[bits(28)]
    pub __: u32,
    
}

/// ENABLE
///
/// Enable UART
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ENABLE {
    /// Enable or disable UART
    #[bits(4)]
    pub enable: u8,
    /// 
    #[bits(28)]
    pub __: u32,
    
}

/// PSELRTS
///
/// Pin select for RTS
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PSELRTS {
    /// Pin number configuration for UART RTS signal
    #[bits(32)]
    pub pselrts: u32,
    
}

/// PSELTXD
///
/// Pin select for TXD
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PSELTXD {
    /// Pin number configuration for UART TXD signal
    #[bits(32)]
    pub pseltxd: u32,
    
}

/// PSELCTS
///
/// Pin select for CTS
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PSELCTS {
    /// Pin number configuration for UART CTS signal
    #[bits(32)]
    pub pselcts: u32,
    
}

/// PSELRXD
///
/// Pin select for RXD
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PSELRXD {
    /// Pin number configuration for UART RXD signal
    #[bits(32)]
    pub pselrxd: u32,
    
}

/// RXD
///
/// RXD register
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RXD {
    /// RX data received in previous transfers, double buffered
    #[bits(8)]
    pub rxd: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// TXD
///
/// TXD register
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TXD {
    /// TX data to be transferred
    #[bits(8)]
    pub txd: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// BAUDRATE
///
/// Baud rate
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BAUDRATE {
    /// Baud rate
    #[bits(32)]
    pub baudrate: u32,
    
}

/// CONFIG
///
/// Configuration of parity and hardware flow control
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CONFIG {
    /// Hardware flow control
    #[bits(1)]
    pub hwfc: bool,
    /// Parity
    #[bits(3)]
    pub parity: u8,
    /// 
    #[bits(28)]
    pub __: u32,
    
}

