//! registers.rs
//!
//! UICR register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::RegInfo;
use super::*;
use context::Permission;


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
    /// Processor debug control
    DEBUGCTRL,
    /// Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD.
    REGOUT0,
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
            0x014..=0x047 => { Some(UICRRegType::NRFFW(((offset - 0x014) / 4) as u8)) }
            0x050..=0x07f => { Some(UICRRegType::NRFHW(((offset - 0x050) / 4) as u8)) }
            0x080..=0x0ff => { Some(UICRRegType::CUSTOMER(((offset - 0x080) / 4) as u8)) }
            0x200..=0x207 => { Some(UICRRegType::PSELRESET(((offset - 0x200) / 4) as u8)) }
            0x208 => { Some(UICRRegType::APPROTECT) }
            0x20C => { Some(UICRRegType::NFCPINS) }
            0x210 => { Some(UICRRegType::DEBUGCTRL) }
            0x304 => { Some(UICRRegType::REGOUT0) }
            _ => { None }
        }
    }
}

impl UICRRegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            UICRRegType::NRFFW(0)        => { &RegInfo { offset: 0x014, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(1)        => { &RegInfo { offset: 0x018, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(2)        => { &RegInfo { offset: 0x01c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(3)        => { &RegInfo { offset: 0x020, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(4)        => { &RegInfo { offset: 0x024, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(5)        => { &RegInfo { offset: 0x028, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(6)        => { &RegInfo { offset: 0x02c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(7)        => { &RegInfo { offset: 0x030, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(8)        => { &RegInfo { offset: 0x034, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(9)        => { &RegInfo { offset: 0x038, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(10)       => { &RegInfo { offset: 0x03c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(11)       => { &RegInfo { offset: 0x040, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFFW(12)       => { &RegInfo { offset: 0x044, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(0)        => { &RegInfo { offset: 0x050, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(1)        => { &RegInfo { offset: 0x054, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(2)        => { &RegInfo { offset: 0x058, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(3)        => { &RegInfo { offset: 0x05c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(4)        => { &RegInfo { offset: 0x060, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(5)        => { &RegInfo { offset: 0x064, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(6)        => { &RegInfo { offset: 0x068, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(7)        => { &RegInfo { offset: 0x06c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(8)        => { &RegInfo { offset: 0x070, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(9)        => { &RegInfo { offset: 0x074, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(10)       => { &RegInfo { offset: 0x078, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NRFHW(11)       => { &RegInfo { offset: 0x07c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(0)     => { &RegInfo { offset: 0x080, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(1)     => { &RegInfo { offset: 0x084, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(2)     => { &RegInfo { offset: 0x088, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(3)     => { &RegInfo { offset: 0x08c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(4)     => { &RegInfo { offset: 0x090, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(5)     => { &RegInfo { offset: 0x094, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(6)     => { &RegInfo { offset: 0x098, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(7)     => { &RegInfo { offset: 0x09c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(8)     => { &RegInfo { offset: 0x0a0, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(9)     => { &RegInfo { offset: 0x0a4, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(10)    => { &RegInfo { offset: 0x0a8, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(11)    => { &RegInfo { offset: 0x0ac, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(12)    => { &RegInfo { offset: 0x0b0, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(13)    => { &RegInfo { offset: 0x0b4, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(14)    => { &RegInfo { offset: 0x0b8, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(15)    => { &RegInfo { offset: 0x0bc, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(16)    => { &RegInfo { offset: 0x0c0, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(17)    => { &RegInfo { offset: 0x0c4, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(18)    => { &RegInfo { offset: 0x0c8, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(19)    => { &RegInfo { offset: 0x0cc, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(20)    => { &RegInfo { offset: 0x0d0, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(21)    => { &RegInfo { offset: 0x0d4, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(22)    => { &RegInfo { offset: 0x0d8, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(23)    => { &RegInfo { offset: 0x0dc, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(24)    => { &RegInfo { offset: 0x0e0, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(25)    => { &RegInfo { offset: 0x0e4, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(26)    => { &RegInfo { offset: 0x0e8, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(27)    => { &RegInfo { offset: 0x0ec, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(28)    => { &RegInfo { offset: 0x0f0, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(29)    => { &RegInfo { offset: 0x0f4, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(30)    => { &RegInfo { offset: 0x0f8, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::CUSTOMER(31)    => { &RegInfo { offset: 0x0fc, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::PSELRESET(0)    => { &RegInfo { offset: 0x200, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::PSELRESET(1)    => { &RegInfo { offset: 0x204, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::APPROTECT       => { &RegInfo { offset: 0x208, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::NFCPINS         => { &RegInfo { offset: 0x20c, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::DEBUGCTRL       => { &RegInfo { offset: 0x210, perms: 0b110, reset: Some(0xFFFFFFFF) } }
            UICRRegType::REGOUT0         => { &RegInfo { offset: 0x304, perms: 0b110, reset: Some(0xFFFFFFFF) } }

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
    /// GPIO pin number onto which nRESET is exposed
    #[bits(5)]
    pub pin: u8,
    /// Port number onto which nRESET is exposed
    #[bits(1)]
    pub port: bool,
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

/// DEBUGCTRL
///
/// Processor debug control
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DEBUGCTRL {
    /// Configure CPU non-intrusive debug features
    #[bits(8)]
    pub cpuniden: u8,
    /// Configure CPU flash patch and breakpoint (FPB) unit behavior
    #[bits(8)]
    pub cpufpben: u8,
    /// 
    #[bits(16)]
    pub __: u32,
}

/// REGOUT0
///
/// Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - V_VDDH-VDD.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct REGOUT0 {
    /// Output voltage from REG0 regulator stage.
    #[bits(3)]
    pub vout: u8,
    /// 
    #[bits(29)]
    pub __: u32,
}

