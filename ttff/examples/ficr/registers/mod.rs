//! registers.rs
//!
//! FICR register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use libcme::types::*;
use super::*;

pub mod info;
pub use info::*;
pub mod temp;
pub use temp::*;
pub mod nfc;
pub use nfc::*;


/// FICR register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FICRRegType {
    /// Code memory page size
    CODEPAGESIZE,
    /// Code memory size
    CODESIZE,
    /// Description collection: Device identifier
    DEVICEID(u8),
    /// Description collection: Encryption Root, word n
    ER(u8),
    /// Description collection: Identity Root, word n
    IR(u8),
    /// Device address type
    DEVICEADDRTYPE,
    /// Description collection: Device address n
    DEVICEADDR(u8),
    /// Device info
    INFO(INFORegType),
    /// Registers storing factory TEMP module linearization coefficients
    TEMP(TEMPRegType),
    /// Unspecified
    NFC(NFCRegType),
    
}

impl FICRRegType {

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
            0x10 => { Some(FICRRegType::CODEPAGESIZE) }
            0x14 => { Some(FICRRegType::CODESIZE) }
            0x60..=0x67 => { Some(FICRRegType::DEVICEID(((offset - 96) / 4) as u8)) }
            0x80..=0x8f => { Some(FICRRegType::ER(((offset - 128) / 4) as u8)) }
            0x90..=0x9f => { Some(FICRRegType::IR(((offset - 144) / 4) as u8)) }
            0xa0 => { Some(FICRRegType::DEVICEADDRTYPE) }
            0xa4..=0xab => { Some(FICRRegType::DEVICEADDR(((offset - 164) / 4) as u8)) }
            0x100..=0x113 => { INFORegType::lookup_offset(offset - 256).map(|reg| FICRRegType::INFO(reg)) }
            0x404..=0x447 => { TEMPRegType::lookup_offset(offset - 1028).map(|reg| FICRRegType::TEMP(reg)) }
            0x450..=0x45f => { NFCRegType::lookup_offset(offset - 1104).map(|reg| FICRRegType::NFC(reg)) }
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let mut types = vec![
            FICRRegType::CODEPAGESIZE,
            FICRRegType::CODESIZE,
            FICRRegType::DEVICEID(0),
            FICRRegType::DEVICEID(1),
            
            FICRRegType::ER(0),
            FICRRegType::ER(1),
            FICRRegType::ER(2),
            FICRRegType::ER(3),
            
            FICRRegType::IR(0),
            FICRRegType::IR(1),
            FICRRegType::IR(2),
            FICRRegType::IR(3),
            
            FICRRegType::DEVICEADDRTYPE,
            FICRRegType::DEVICEADDR(0),
            FICRRegType::DEVICEADDR(1),
            
            
        ];
        for reg_type in INFORegType::list() {
            types.push(FICRRegType::INFO(reg_type));
        }
        for reg_type in TEMPRegType::list() {
            types.push(FICRRegType::TEMP(reg_type));
        }
        for reg_type in NFCRegType::list() {
            types.push(FICRRegType::NFC(reg_type));
        }
        
        types
    }
}

impl FICRRegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            FICRRegType::CODEPAGESIZE => { &RegInfo { offset: 16, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::CODESIZE => { &RegInfo { offset: 20, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEID(0) => { &RegInfo { offset: 96, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEID(1) => { &RegInfo { offset: 100, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            
            FICRRegType::ER(0) => { &RegInfo { offset: 128, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(1) => { &RegInfo { offset: 132, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(2) => { &RegInfo { offset: 136, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(3) => { &RegInfo { offset: 140, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            
            FICRRegType::IR(0) => { &RegInfo { offset: 144, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(1) => { &RegInfo { offset: 148, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(2) => { &RegInfo { offset: 152, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(3) => { &RegInfo { offset: 156, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            
            FICRRegType::DEVICEADDRTYPE => { &RegInfo { offset: 160, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEADDR(0) => { &RegInfo { offset: 164, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEADDR(1) => { &RegInfo { offset: 168, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            
            FICRRegType::INFO(reg) => { reg._data() }
            FICRRegType::TEMP(reg) => { reg._data() }
            FICRRegType::NFC(reg) => { reg._data() }
            
            
            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/// CODEPAGESIZE
///
/// Code memory page size
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CODEPAGESIZE {
    /// Code memory page size
    #[bits(32)]
    pub codepagesize: u32,
    
}

/// CODESIZE
///
/// Code memory size
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CODESIZE {
    /// Code memory size in number of pages
    #[bits(32)]
    pub codesize: u32,
    
}

/// DEVICEID
///
/// Description collection: Device identifier
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DEVICEID {
    /// 64 bit unique device identifier
    #[bits(32)]
    pub deviceid: u32,
    
}

/// ER
///
/// Description collection: Encryption Root, word n
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ER {
    /// Encryption Root, word n
    #[bits(32)]
    pub er: u32,
    
}

/// IR
///
/// Description collection: Identity Root, word n
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct IR {
    /// Identity Root, word n
    #[bits(32)]
    pub ir: u32,
    
}

/// DEVICEADDRTYPE
///
/// Device address type
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DEVICEADDRTYPE {
    /// Device address type
    #[bits(1)]
    pub deviceaddrtype: bool,
    /// 
    #[bits(31)]
    pub __: u32,
    
}

/// DEVICEADDR
///
/// Description collection: Device address n
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DEVICEADDR {
    /// 48 bit device address
    #[bits(32)]
    pub deviceaddr: u32,
    
}

