//! txd.rs.rs
//!
//! TXD module
//! 

use bitfield_struct::bitfield;

use crate::types::RegInfo;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TXDRegType {
    /// Data pointer
    PTR,
    /// Maximum number of bytes in transmit buffer
    MAXCNT,
    /// Number of bytes transferred in the last transaction
    AMOUNT,
}

impl TXDRegType {

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
        assert!(offset < 0x000c, "address not in peripheral!");
        match offset {
            0x000 => { Some(TXDRegType::PTR) }
            0x004 => { Some(TXDRegType::MAXCNT) }
            0x008 => { Some(TXDRegType::AMOUNT) }
            _ => { None }
        }
    }
}

impl TXDRegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            TXDRegType::PTR             => { &RegInfo { offset: 0x000, perms: 0b110, reset: None } }
            TXDRegType::MAXCNT          => { &RegInfo { offset: 0x004, perms: 0b110, reset: None } }
            TXDRegType::AMOUNT          => { &RegInfo { offset: 0x008, perms: 0b100, reset: None } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// PTR
///
/// Data pointer
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PTR {
    /// Data pointer
    #[bits(32)]
    pub ptr: u32,
}


/// MAXCNT
///
/// Maximum number of bytes in transmit buffer
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MAXCNT {
    /// Maximum number of bytes in transmit buffer
    #[bits(16)]
    pub maxcnt: u16,
    /// 
    #[bits(16)]
    pub __: u32,
}


/// AMOUNT
///
/// Number of bytes transferred in the last transaction
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AMOUNT {
    /// Number of bytes transferred in the last transaction
    #[bits(16)]
    pub amount: u16,
    /// 
    #[bits(16)]
    pub __: u32,
}


