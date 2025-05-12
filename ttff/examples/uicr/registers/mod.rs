//! registers.rs
//!
//! UICR register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use libcme::types::*;
use super::*;



/// UICR register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UICRRegType {
    /// Description collection: Reserved for Nordic firmware design
    NRFFW(u8),
    /// Description collection: Reserved for Nordic hardware design
    NRFHW(u8),
    /// Description collection: Reserved for customer
    CUSTOMER(u8),
    /// Description collection: Mapping of the nRESET function (see POWER chapter for details)
    PSELRESET(u8),
    /// Access port protection
    APPROTECT,
    /// Setting of pins dedicated to NFC functionality: NFC antenna or GPIO
    NFCPINS,
    
}

impl UICRRegType {

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
            0x14..=0x4f => { Some(UICRRegType::NRFFW(((offset - 20) / 4) as u8)) }
            0x50..=0x7f => { Some(UICRRegType::NRFHW(((offset - 80) / 4) as u8)) }
            0x80..=0xff => { Some(UICRRegType::CUSTOMER(((offset - 128) / 4) as u8)) }
            0x200..=0x207 => { Some(UICRRegType::PSELRESET(((offset - 512) / 4) as u8)) }
            0x208 => { Some(UICRRegType::APPROTECT) }
            0x20c => { Some(UICRRegType::NFCPINS) }
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let types = vec![
            UICRRegType::NRFFW(0),
            UICRRegType::NRFFW(1),
            UICRRegType::NRFFW(2),
            UICRRegType::NRFFW(3),
            UICRRegType::NRFFW(4),
            UICRRegType::NRFFW(5),
            UICRRegType::NRFFW(6),
            UICRRegType::NRFFW(7),
            UICRRegType::NRFFW(8),
            UICRRegType::NRFFW(9),
            UICRRegType::NRFFW(10),
            UICRRegType::NRFFW(11),
            UICRRegType::NRFFW(12),
            UICRRegType::NRFFW(13),
            UICRRegType::NRFFW(14),
            
            UICRRegType::NRFHW(0),
            UICRRegType::NRFHW(1),
            UICRRegType::NRFHW(2),
            UICRRegType::NRFHW(3),
            UICRRegType::NRFHW(4),
            UICRRegType::NRFHW(5),
            UICRRegType::NRFHW(6),
            UICRRegType::NRFHW(7),
            UICRRegType::NRFHW(8),
            UICRRegType::NRFHW(9),
            UICRRegType::NRFHW(10),
            UICRRegType::NRFHW(11),
            
            UICRRegType::CUSTOMER(0),
            UICRRegType::CUSTOMER(1),
            UICRRegType::CUSTOMER(2),
            UICRRegType::CUSTOMER(3),
            UICRRegType::CUSTOMER(4),
            UICRRegType::CUSTOMER(5),
            UICRRegType::CUSTOMER(6),
            UICRRegType::CUSTOMER(7),
            UICRRegType::CUSTOMER(8),
            UICRRegType::CUSTOMER(9),
            UICRRegType::CUSTOMER(10),
            UICRRegType::CUSTOMER(11),
            UICRRegType::CUSTOMER(12),
            UICRRegType::CUSTOMER(13),
            UICRRegType::CUSTOMER(14),
            UICRRegType::CUSTOMER(15),
            UICRRegType::CUSTOMER(16),
            UICRRegType::CUSTOMER(17),
            UICRRegType::CUSTOMER(18),
            UICRRegType::CUSTOMER(19),
            UICRRegType::CUSTOMER(20),
            UICRRegType::CUSTOMER(21),
            UICRRegType::CUSTOMER(22),
            UICRRegType::CUSTOMER(23),
            UICRRegType::CUSTOMER(24),
            UICRRegType::CUSTOMER(25),
            UICRRegType::CUSTOMER(26),
            UICRRegType::CUSTOMER(27),
            UICRRegType::CUSTOMER(28),
            UICRRegType::CUSTOMER(29),
            UICRRegType::CUSTOMER(30),
            UICRRegType::CUSTOMER(31),
            
            UICRRegType::PSELRESET(0),
            UICRRegType::PSELRESET(1),
            
            UICRRegType::APPROTECT,
            UICRRegType::NFCPINS,
        ];
        types
    }
}

impl UICRRegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            UICRRegType::NRFFW(0) => { &RegInfo { offset: 20, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(1) => { &RegInfo { offset: 24, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(2) => { &RegInfo { offset: 28, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(3) => { &RegInfo { offset: 32, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(4) => { &RegInfo { offset: 36, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(5) => { &RegInfo { offset: 40, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(6) => { &RegInfo { offset: 44, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(7) => { &RegInfo { offset: 48, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(8) => { &RegInfo { offset: 52, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(9) => { &RegInfo { offset: 56, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(10) => { &RegInfo { offset: 60, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(11) => { &RegInfo { offset: 64, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(12) => { &RegInfo { offset: 68, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(13) => { &RegInfo { offset: 72, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(14) => { &RegInfo { offset: 76, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            
            UICRRegType::NRFHW(0) => { &RegInfo { offset: 80, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(1) => { &RegInfo { offset: 84, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(2) => { &RegInfo { offset: 88, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(3) => { &RegInfo { offset: 92, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(4) => { &RegInfo { offset: 96, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(5) => { &RegInfo { offset: 100, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(6) => { &RegInfo { offset: 104, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(7) => { &RegInfo { offset: 108, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(8) => { &RegInfo { offset: 112, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(9) => { &RegInfo { offset: 116, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(10) => { &RegInfo { offset: 120, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(11) => { &RegInfo { offset: 124, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            
            UICRRegType::CUSTOMER(0) => { &RegInfo { offset: 128, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(1) => { &RegInfo { offset: 132, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(2) => { &RegInfo { offset: 136, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(3) => { &RegInfo { offset: 140, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(4) => { &RegInfo { offset: 144, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(5) => { &RegInfo { offset: 148, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(6) => { &RegInfo { offset: 152, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(7) => { &RegInfo { offset: 156, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(8) => { &RegInfo { offset: 160, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(9) => { &RegInfo { offset: 164, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(10) => { &RegInfo { offset: 168, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(11) => { &RegInfo { offset: 172, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(12) => { &RegInfo { offset: 176, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(13) => { &RegInfo { offset: 180, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(14) => { &RegInfo { offset: 184, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(15) => { &RegInfo { offset: 188, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(16) => { &RegInfo { offset: 192, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(17) => { &RegInfo { offset: 196, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(18) => { &RegInfo { offset: 200, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(19) => { &RegInfo { offset: 204, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(20) => { &RegInfo { offset: 208, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(21) => { &RegInfo { offset: 212, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(22) => { &RegInfo { offset: 216, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(23) => { &RegInfo { offset: 220, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(24) => { &RegInfo { offset: 224, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(25) => { &RegInfo { offset: 228, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(26) => { &RegInfo { offset: 232, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(27) => { &RegInfo { offset: 236, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(28) => { &RegInfo { offset: 240, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(29) => { &RegInfo { offset: 244, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(30) => { &RegInfo { offset: 248, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(31) => { &RegInfo { offset: 252, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            
            UICRRegType::PSELRESET(0) => { &RegInfo { offset: 512, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::PSELRESET(1) => { &RegInfo { offset: 516, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            
            UICRRegType::APPROTECT => { &RegInfo { offset: 520, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NFCPINS => { &RegInfo { offset: 524, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            
            
            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/// NRFFW
///
/// Description collection: Reserved for Nordic firmware design
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct NRFFW {
    /// Reserved for Nordic firmware design
    #[bits(32)]
    pub nrffw: u32,
    
}

/// NRFHW
///
/// Description collection: Reserved for Nordic hardware design
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct NRFHW {
    /// Reserved for Nordic hardware design
    #[bits(32)]
    pub nrfhw: u32,
    
}

/// CUSTOMER
///
/// Description collection: Reserved for customer
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CUSTOMER {
    /// Reserved for customer
    #[bits(32)]
    pub customer: u32,
    
}

/// PSELRESET
///
/// Description collection: Mapping of the nRESET function (see POWER chapter for details)
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PSELRESET {
    /// GPIO number P0.n onto which Reset is exposed
    #[bits(6)]
    pub pin: u8,
    /// 
    #[bits(25)]
    pub __: u32,
    /// Connection
    #[bits(1)]
    pub connect: bool,
    
}

/// APPROTECT
///
/// Access port protection
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct APPROTECT {
    /// Enable or disable access port protection.
    #[bits(8)]
    pub pall: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// NFCPINS
///
/// Setting of pins dedicated to NFC functionality: NFC antenna or GPIO
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct NFCPINS {
    /// Setting of pins dedicated to NFC functionality
    #[bits(1)]
    pub protect: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

