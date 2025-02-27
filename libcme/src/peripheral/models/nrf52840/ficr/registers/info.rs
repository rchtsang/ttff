//! info.rs.rs
//!
//! INFO module
//! 

use bitfield_struct::bitfield;

use crate::types::RegInfo;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum INFORegType {
    /// Part code
    PART,
    /// Build code (hardware version and production configuration)
    VARIANT,
    /// Package option
    PACKAGE,
    /// RAM variant
    RAM,
    /// Flash variant
    FLASH,
}

impl INFORegType {

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
        assert!(offset < 0x0014, "address not in peripheral!");
        match offset {
            0x000 => { Some(INFORegType::PART) }
            0x004 => { Some(INFORegType::VARIANT) }
            0x008 => { Some(INFORegType::PACKAGE) }
            0x00C => { Some(INFORegType::RAM) }
            0x010 => { Some(INFORegType::FLASH) }
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let mut types = vec![
            INFORegType::PART,
            INFORegType::VARIANT,
            INFORegType::PACKAGE,
            INFORegType::RAM,
            INFORegType::FLASH,
        ];
        types
    }
}

impl INFORegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            INFORegType::PART            => { &RegInfo { offset: 0x000, perms: 0b100, reset: Some(0x00052840) } }
            INFORegType::VARIANT         => { &RegInfo { offset: 0x004, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            INFORegType::PACKAGE         => { &RegInfo { offset: 0x008, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            INFORegType::RAM             => { &RegInfo { offset: 0x00c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            INFORegType::FLASH           => { &RegInfo { offset: 0x010, perms: 0b100, reset: Some(0xFFFFFFFF) } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// PART
///
/// Part code
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PART {
    /// Part code
    #[bits(32)]
    pub part: u32,
}


/// VARIANT
///
/// Build code (hardware version and production configuration)
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct VARIANT {
    /// Build code (hardware version and production configuration). Encoded as ASCII.
    #[bits(32)]
    pub variant: u32,
}


/// PACKAGE
///
/// Package option
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PACKAGE {
    /// Package option
    #[bits(32)]
    pub package: u32,
}


/// RAM
///
/// RAM variant
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RAM {
    /// RAM variant
    #[bits(32)]
    pub ram: u32,
}


/// FLASH
///
/// Flash variant
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FLASH {
    /// Flash variant
    #[bits(32)]
    pub flash: u32,
}


