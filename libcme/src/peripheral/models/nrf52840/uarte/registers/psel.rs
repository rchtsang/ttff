//! psel.rs.rs
//!
//! PSEL module
//! 

use bitfield_struct::bitfield;

use crate::types::RegInfo;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PSELRegType {
    /// Pin select for RTS signal
    RTS,
    /// Pin select for TXD signal
    TXD,
    /// Pin select for CTS signal
    CTS,
    /// Pin select for RXD signal
    RXD,
}

impl PSELRegType {

    pub fn address(&self, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data().offset as u64))
    }

    /// returns the register byte offset from the peripheral base address
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    /// returns access permissions of peripheral register
    pub fn permissions(&self) -> u8 {
        self._data().perms
    }

    /// returns the peripheral register reset value
    pub fn reset_value(&self) -> Option<u32> {
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
        assert!(offset < 0x0010, "address not in peripheral!");
        match offset {
            0x000 => { Some(PSELRegType::RTS) }
            0x004 => { Some(PSELRegType::TXD) }
            0x008 => { Some(PSELRegType::CTS) }
            0x00C => { Some(PSELRegType::RXD) }
            _ => { None }
        }
    }
}

impl PSELRegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            PSELRegType::RTS             => { &RegInfo { offset: 0x000, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            PSELRegType::TXD             => { &RegInfo { offset: 0x004, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            PSELRegType::CTS             => { &RegInfo { offset: 0x008, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            PSELRegType::RXD             => { &RegInfo { offset: 0x00c, perms: 0b110, reset: Some(0xFFFFFFFF) } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// RTS
///
/// Pin select for RTS signal
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RTS {
    /// Pin number
    #[bits(5)]
    pub pin: u8,
    /// Port number
    #[bits(1)]
    pub port: bool,
    /// 
    #[bits(25)]
    pub __: u32,
    /// Connection
    #[bits(1)]
    pub connect: bool,
}


/// TXD
///
/// Pin select for TXD signal
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TXD {
    /// Pin number
    #[bits(5)]
    pub pin: u8,
    /// Port number
    #[bits(1)]
    pub port: bool,
    /// 
    #[bits(25)]
    pub __: u32,
    /// Connection
    #[bits(1)]
    pub connect: bool,
}


/// CTS
///
/// Pin select for CTS signal
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CTS {
    /// Pin number
    #[bits(5)]
    pub pin: u8,
    /// Port number
    #[bits(1)]
    pub port: bool,
    /// 
    #[bits(25)]
    pub __: u32,
    /// Connection
    #[bits(1)]
    pub connect: bool,
}


/// RXD
///
/// Pin select for RXD signal
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RXD {
    /// Pin number
    #[bits(5)]
    pub pin: u8,
    /// Port number
    #[bits(1)]
    pub port: bool,
    /// 
    #[bits(25)]
    pub __: u32,
    /// Connection
    #[bits(1)]
    pub connect: bool,
}


