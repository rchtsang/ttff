//! registers.rs
//!
//! FICR register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::RegInfo;
use super::*;
use context::Permission;

pub mod info;
pub use info::*;
pub mod temp;
pub use temp::*;
pub mod nfc;
pub use nfc::*;
pub mod trng90b;
pub use trng90b::*;

/// FICR register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FICRRegType {
    /// Code memory page size
    CODEPAGESIZE,
    /// Code memory size
    CODESIZE,
    /// Description collection: Device identifier
    DEVICEID(u8),
    /// Description collection: Encryption root, word n
    ER(u8),
    /// Description collection: Identity Root, word n
    IR(u8),
    /// Device address type
    DEVICEADDRTYPE,
    /// Description collection: Device address n
    DEVICEADDR(u8),
    /// Description collection: Production test signature n
    PRODTEST(u8),
    /// Device info
    INFO(INFORegType),
    /// Registers storing factory TEMP module linearization coefficients
    TEMP(TEMPRegType),
    /// Unspecified
    NFC(NFCRegType),
    /// NIST800-90B RNG calibration data
    TRNG90B(TRNG90BRegType),
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
            0x010 => { Some(FICRRegType::CODEPAGESIZE) }
            0x014 => { Some(FICRRegType::CODESIZE) }
            0x060..=0x067 => { Some(FICRRegType::DEVICEID(((offset - 0x060) / 4) as u8)) }
            0x080..=0x08f => { Some(FICRRegType::ER(((offset - 0x080) / 4) as u8)) }
            0x090..=0x09f => { Some(FICRRegType::IR(((offset - 0x090) / 4) as u8)) }
            0x0A0 => { Some(FICRRegType::DEVICEADDRTYPE) }
            0x0a4..=0x0ab => { Some(FICRRegType::DEVICEADDR(((offset - 0x0a4) / 4) as u8)) }
            0x350..=0x35b => { Some(FICRRegType::PRODTEST(((offset - 0x350) / 4) as u8)) }
            0x100..=0x113 => { INFORegType::lookup_offset(offset - 0x100).map(|reg| FICRRegType::INFO(reg)) }
            0x404..=0x447 => { TEMPRegType::lookup_offset(offset - 0x404).map(|reg| FICRRegType::TEMP(reg)) }
            0x450..=0x45f => { NFCRegType::lookup_offset(offset - 0x450).map(|reg| FICRRegType::NFC(reg)) }
            0xc00..=0xc1f => { TRNG90BRegType::lookup_offset(offset - 0xc00).map(|reg| FICRRegType::TRNG90B(reg)) }
            _ => { None }
        }
    }
}

impl FICRRegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            FICRRegType::CODEPAGESIZE    => { &RegInfo { offset: 0x010, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::CODESIZE        => { &RegInfo { offset: 0x014, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEID(0)     => { &RegInfo { offset: 0x060, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEID(1)     => { &RegInfo { offset: 0x064, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(0)           => { &RegInfo { offset: 0x080, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(1)           => { &RegInfo { offset: 0x084, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(2)           => { &RegInfo { offset: 0x088, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::ER(3)           => { &RegInfo { offset: 0x08c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(0)           => { &RegInfo { offset: 0x090, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(1)           => { &RegInfo { offset: 0x094, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(2)           => { &RegInfo { offset: 0x098, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::IR(3)           => { &RegInfo { offset: 0x09c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEADDRTYPE  => { &RegInfo { offset: 0x0a0, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEADDR(0)   => { &RegInfo { offset: 0x0a4, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::DEVICEADDR(1)   => { &RegInfo { offset: 0x0a8, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::PRODTEST(0)     => { &RegInfo { offset: 0x350, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::PRODTEST(1)     => { &RegInfo { offset: 0x354, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::PRODTEST(2)     => { &RegInfo { offset: 0x358, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            FICRRegType::INFO(reg)       => { reg._data() }
            FICRRegType::TEMP(reg)       => { reg._data() }
            FICRRegType::NFC(reg)        => { reg._data() }
            FICRRegType::TRNG90B(reg)    => { reg._data() }

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
/// Description collection: Encryption root, word n
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ER {
    /// Encryption root, word n
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

/// PRODTEST
///
/// Description collection: Production test signature n
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PRODTEST {
    /// Production test signature n
    #[bits(32)]
    pub prodtest: u32,
}

