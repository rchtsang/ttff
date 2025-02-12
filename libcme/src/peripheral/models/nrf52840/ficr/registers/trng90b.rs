//! trng90b.rs.rs
//!
//! TRNG90B module
//! 

use bitfield_struct::bitfield;

use crate::types::RegInfo;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TRNG90BRegType {
    /// Amount of bytes for the required entropy bits
    BYTES,
    /// Repetition counter cutoff
    RCCUTOFF,
    /// Adaptive proportion cutoff
    APCUTOFF,
    /// Amount of bytes for the startup tests
    STARTUP,
    /// Sample count for ring oscillator 1
    ROSC1,
    /// Sample count for ring oscillator 2
    ROSC2,
    /// Sample count for ring oscillator 3
    ROSC3,
    /// Sample count for ring oscillator 4
    ROSC4,
}

impl TRNG90BRegType {

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
        assert!(offset < 0x0020, "address not in peripheral!");
        match offset {
            0x000 => { Some(TRNG90BRegType::BYTES) }
            0x004 => { Some(TRNG90BRegType::RCCUTOFF) }
            0x008 => { Some(TRNG90BRegType::APCUTOFF) }
            0x00C => { Some(TRNG90BRegType::STARTUP) }
            0x010 => { Some(TRNG90BRegType::ROSC1) }
            0x014 => { Some(TRNG90BRegType::ROSC2) }
            0x018 => { Some(TRNG90BRegType::ROSC3) }
            0x01C => { Some(TRNG90BRegType::ROSC4) }
            _ => { None }
        }
    }
}

impl TRNG90BRegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            TRNG90BRegType::BYTES           => { &RegInfo { offset: 0x000, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::RCCUTOFF        => { &RegInfo { offset: 0x004, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::APCUTOFF        => { &RegInfo { offset: 0x008, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::STARTUP         => { &RegInfo { offset: 0x00c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::ROSC1           => { &RegInfo { offset: 0x010, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::ROSC2           => { &RegInfo { offset: 0x014, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::ROSC3           => { &RegInfo { offset: 0x018, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TRNG90BRegType::ROSC4           => { &RegInfo { offset: 0x01c, perms: 0b100, reset: Some(0xFFFFFFFF) } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// BYTES
///
/// Amount of bytes for the required entropy bits
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BYTES {
    /// Amount of bytes for the required entropy bits
    #[bits(32)]
    pub bytes: u32,
}


/// RCCUTOFF
///
/// Repetition counter cutoff
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RCCUTOFF {
    /// Repetition counter cutoff
    #[bits(32)]
    pub rccutoff: u32,
}


/// APCUTOFF
///
/// Adaptive proportion cutoff
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct APCUTOFF {
    /// Adaptive proportion cutoff
    #[bits(32)]
    pub apcutoff: u32,
}


/// STARTUP
///
/// Amount of bytes for the startup tests
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STARTUP {
    /// Amount of bytes for the startup tests
    #[bits(32)]
    pub startup: u32,
}


/// ROSC1
///
/// Sample count for ring oscillator 1
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ROSC1 {
    /// Sample count for ring oscillator 1
    #[bits(32)]
    pub rosc1: u32,
}


/// ROSC2
///
/// Sample count for ring oscillator 2
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ROSC2 {
    /// Sample count for ring oscillator 2
    #[bits(32)]
    pub rosc2: u32,
}


/// ROSC3
///
/// Sample count for ring oscillator 3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ROSC3 {
    /// Sample count for ring oscillator 3
    #[bits(32)]
    pub rosc3: u32,
}


/// ROSC4
///
/// Sample count for ring oscillator 4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ROSC4 {
    /// Sample count for ring oscillator 4
    #[bits(32)]
    pub rosc4: u32,
}


