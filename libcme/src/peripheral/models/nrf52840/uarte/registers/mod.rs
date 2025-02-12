//! registers.rs
//!
//! UARTE register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::RegInfo;
use super::*;
use context::Permission;

pub mod psel;
pub use psel::*;
pub mod rxd;
pub use rxd::*;
pub mod txd;
pub use txd::*;

/// UARTE register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UARTERegType {
    /// Start UART receiver
    TASKS_STARTRX,
    /// Stop UART receiver
    TASKS_STOPRX,
    /// Start UART transmitter
    TASKS_STARTTX,
    /// Stop UART transmitter
    TASKS_STOPTX,
    /// Flush RX FIFO into RX buffer
    TASKS_FLUSHRX,
    /// CTS is activated (set low). Clear To Send.
    EVENTS_CTS,
    /// CTS is deactivated (set high). Not Clear To Send.
    EVENTS_NCTS,
    /// Data received in RXD (but potentially not yet transferred to Data RAM)
    EVENTS_RXDRDY,
    /// Receive buffer is filled up
    EVENTS_ENDRX,
    /// Data sent from TXD
    EVENTS_TXDRDY,
    /// Last TX byte transmitted
    EVENTS_ENDTX,
    /// Error detected
    EVENTS_ERROR,
    /// Receiver timeout
    EVENTS_RXTO,
    /// UART receiver has started
    EVENTS_RXSTARTED,
    /// UART transmitter has started
    EVENTS_TXSTARTED,
    /// Transmitter stopped
    EVENTS_TXSTOPPED,
    /// Shortcuts between local events and tasks
    SHORTS,
    /// Enable or disable interrupt
    INTEN,
    /// Enable interrupt
    INTENSET,
    /// Disable interrupt
    INTENCLR,
    /// Error source This register is read/write one to clear.
    ERRORSRC,
    /// Enable UART
    ENABLE,
    /// Baud rate. Accuracy depends on the HFCLK source selected.
    BAUDRATE,
    /// Configuration of parity and hardware flow control
    CONFIG,
    /// Unspecified
    PSEL(PSELRegType),
    /// RXD EasyDMA channel
    RXD(RXDRegType),
    /// TXD EasyDMA channel
    TXD(TXDRegType),
}

impl UARTERegType {

    pub fn address(&self, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data().offset as u64))
    }

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
            0x000 => { Some(UARTERegType::TASKS_STARTRX) }
            0x004 => { Some(UARTERegType::TASKS_STOPRX) }
            0x008 => { Some(UARTERegType::TASKS_STARTTX) }
            0x00C => { Some(UARTERegType::TASKS_STOPTX) }
            0x02C => { Some(UARTERegType::TASKS_FLUSHRX) }
            0x100 => { Some(UARTERegType::EVENTS_CTS) }
            0x104 => { Some(UARTERegType::EVENTS_NCTS) }
            0x108 => { Some(UARTERegType::EVENTS_RXDRDY) }
            0x110 => { Some(UARTERegType::EVENTS_ENDRX) }
            0x11C => { Some(UARTERegType::EVENTS_TXDRDY) }
            0x120 => { Some(UARTERegType::EVENTS_ENDTX) }
            0x124 => { Some(UARTERegType::EVENTS_ERROR) }
            0x144 => { Some(UARTERegType::EVENTS_RXTO) }
            0x14C => { Some(UARTERegType::EVENTS_RXSTARTED) }
            0x150 => { Some(UARTERegType::EVENTS_TXSTARTED) }
            0x158 => { Some(UARTERegType::EVENTS_TXSTOPPED) }
            0x200 => { Some(UARTERegType::SHORTS) }
            0x300 => { Some(UARTERegType::INTEN) }
            0x304 => { Some(UARTERegType::INTENSET) }
            0x308 => { Some(UARTERegType::INTENCLR) }
            0x480 => { Some(UARTERegType::ERRORSRC) }
            0x500 => { Some(UARTERegType::ENABLE) }
            0x524 => { Some(UARTERegType::BAUDRATE) }
            0x56C => { Some(UARTERegType::CONFIG) }
            0x508..=0x517 => { PSELRegType::lookup_offset(offset - 0x508).map(|reg| UARTERegType::PSEL(reg)) }
            0x534..=0x53f => { RXDRegType::lookup_offset(offset - 0x534).map(|reg| UARTERegType::RXD(reg)) }
            0x544..=0x54f => { TXDRegType::lookup_offset(offset - 0x544).map(|reg| UARTERegType::TXD(reg)) }
            _ => { None }
        }
    }
}

impl UARTERegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            UARTERegType::TASKS_STARTRX   => { &RegInfo { offset: 0x000, perms: 0b010, reset: None } }
            UARTERegType::TASKS_STOPRX    => { &RegInfo { offset: 0x004, perms: 0b010, reset: None } }
            UARTERegType::TASKS_STARTTX   => { &RegInfo { offset: 0x008, perms: 0b010, reset: None } }
            UARTERegType::TASKS_STOPTX    => { &RegInfo { offset: 0x00c, perms: 0b010, reset: None } }
            UARTERegType::TASKS_FLUSHRX   => { &RegInfo { offset: 0x02c, perms: 0b010, reset: None } }
            UARTERegType::EVENTS_CTS      => { &RegInfo { offset: 0x100, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_NCTS     => { &RegInfo { offset: 0x104, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_RXDRDY   => { &RegInfo { offset: 0x108, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_ENDRX    => { &RegInfo { offset: 0x110, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_TXDRDY   => { &RegInfo { offset: 0x11c, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_ENDTX    => { &RegInfo { offset: 0x120, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_ERROR    => { &RegInfo { offset: 0x124, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_RXTO     => { &RegInfo { offset: 0x144, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_RXSTARTED => { &RegInfo { offset: 0x14c, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_TXSTARTED => { &RegInfo { offset: 0x150, perms: 0b110, reset: None } }
            UARTERegType::EVENTS_TXSTOPPED => { &RegInfo { offset: 0x158, perms: 0b110, reset: None } }
            UARTERegType::SHORTS          => { &RegInfo { offset: 0x200, perms: 0b110, reset: None } }
            UARTERegType::INTEN           => { &RegInfo { offset: 0x300, perms: 0b110, reset: None } }
            UARTERegType::INTENSET        => { &RegInfo { offset: 0x304, perms: 0b110, reset: None } }
            UARTERegType::INTENCLR        => { &RegInfo { offset: 0x308, perms: 0b110, reset: None } }
            UARTERegType::ERRORSRC        => { &RegInfo { offset: 0x480, perms: 0b110, reset: None } }
            UARTERegType::ENABLE          => { &RegInfo { offset: 0x500, perms: 0b110, reset: None } }
            UARTERegType::BAUDRATE        => { &RegInfo { offset: 0x524, perms: 0b110, reset: Some(0x04000000) } }
            UARTERegType::CONFIG          => { &RegInfo { offset: 0x56c, perms: 0b110, reset: None } }
            UARTERegType::PSEL(reg)       => { reg._data() }
            UARTERegType::RXD(reg)        => { reg._data() }
            UARTERegType::TXD(reg)        => { reg._data() }

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

/// TASKS_FLUSHRX
///
/// Flush RX FIFO into RX buffer
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_FLUSHRX {
    /// Flush RX FIFO into RX buffer
    #[bits(1)]
    pub tasks_flushrx: bool,
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
/// Data received in RXD (but potentially not yet transferred to Data RAM)
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_RXDRDY {
    /// Data received in RXD (but potentially not yet transferred to Data RAM)
    #[bits(1)]
    pub events_rxdrdy: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_ENDRX
///
/// Receive buffer is filled up
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_ENDRX {
    /// Receive buffer is filled up
    #[bits(1)]
    pub events_endrx: bool,
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

/// EVENTS_ENDTX
///
/// Last TX byte transmitted
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_ENDTX {
    /// Last TX byte transmitted
    #[bits(1)]
    pub events_endtx: bool,
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

/// EVENTS_RXSTARTED
///
/// UART receiver has started
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_RXSTARTED {
    /// UART receiver has started
    #[bits(1)]
    pub events_rxstarted: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_TXSTARTED
///
/// UART transmitter has started
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_TXSTARTED {
    /// UART transmitter has started
    #[bits(1)]
    pub events_txstarted: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_TXSTOPPED
///
/// Transmitter stopped
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_TXSTOPPED {
    /// Transmitter stopped
    #[bits(1)]
    pub events_txstopped: bool,
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
    #[bits(5)]
    pub __: u32,
    /// Shortcut between event ENDRX and task STARTRX
    #[bits(1)]
    pub endrx_startrx: bool,
    /// Shortcut between event ENDRX and task STOPRX
    #[bits(1)]
    pub endrx_stoprx: bool,
    /// 
    #[bits(25)]
    pub __: u32,
}

/// INTEN
///
/// Enable or disable interrupt
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INTEN {
    /// Enable or disable interrupt for event CTS
    #[bits(1)]
    pub cts: bool,
    /// Enable or disable interrupt for event NCTS
    #[bits(1)]
    pub ncts: bool,
    /// Enable or disable interrupt for event RXDRDY
    #[bits(1)]
    pub rxdrdy: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Enable or disable interrupt for event ENDRX
    #[bits(1)]
    pub endrx: bool,
    /// 
    #[bits(2)]
    pub __: u32,
    /// Enable or disable interrupt for event TXDRDY
    #[bits(1)]
    pub txdrdy: bool,
    /// Enable or disable interrupt for event ENDTX
    #[bits(1)]
    pub endtx: bool,
    /// Enable or disable interrupt for event ERROR
    #[bits(1)]
    pub error: bool,
    /// 
    #[bits(7)]
    pub __: u32,
    /// Enable or disable interrupt for event RXTO
    #[bits(1)]
    pub rxto: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Enable or disable interrupt for event RXSTARTED
    #[bits(1)]
    pub rxstarted: bool,
    /// Enable or disable interrupt for event TXSTARTED
    #[bits(1)]
    pub txstarted: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Enable or disable interrupt for event TXSTOPPED
    #[bits(1)]
    pub txstopped: bool,
    /// 
    #[bits(9)]
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
    #[bits(1)]
    pub __: u32,
    /// Write '1' to enable interrupt for event ENDRX
    #[bits(1)]
    pub endrx: bool,
    /// 
    #[bits(2)]
    pub __: u32,
    /// Write '1' to enable interrupt for event TXDRDY
    #[bits(1)]
    pub txdrdy: bool,
    /// Write '1' to enable interrupt for event ENDTX
    #[bits(1)]
    pub endtx: bool,
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
    #[bits(1)]
    pub __: u32,
    /// Write '1' to enable interrupt for event RXSTARTED
    #[bits(1)]
    pub rxstarted: bool,
    /// Write '1' to enable interrupt for event TXSTARTED
    #[bits(1)]
    pub txstarted: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Write '1' to enable interrupt for event TXSTOPPED
    #[bits(1)]
    pub txstopped: bool,
    /// 
    #[bits(9)]
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
    #[bits(1)]
    pub __: u32,
    /// Write '1' to disable interrupt for event ENDRX
    #[bits(1)]
    pub endrx: bool,
    /// 
    #[bits(2)]
    pub __: u32,
    /// Write '1' to disable interrupt for event TXDRDY
    #[bits(1)]
    pub txdrdy: bool,
    /// Write '1' to disable interrupt for event ENDTX
    #[bits(1)]
    pub endtx: bool,
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
    #[bits(1)]
    pub __: u32,
    /// Write '1' to disable interrupt for event RXSTARTED
    #[bits(1)]
    pub rxstarted: bool,
    /// Write '1' to disable interrupt for event TXSTARTED
    #[bits(1)]
    pub txstarted: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Write '1' to disable interrupt for event TXSTOPPED
    #[bits(1)]
    pub txstopped: bool,
    /// 
    #[bits(9)]
    pub __: u32,
}

/// ERRORSRC
///
/// Error source This register is read/write one to clear.
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
    /// Enable or disable UARTE
    #[bits(4)]
    pub enable: u8,
    /// 
    #[bits(28)]
    pub __: u32,
}

/// BAUDRATE
///
/// Baud rate. Accuracy depends on the HFCLK source selected.
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
    /// Stop bits
    #[bits(1)]
    pub stop: bool,
    /// 
    #[bits(27)]
    pub __: u32,
}

