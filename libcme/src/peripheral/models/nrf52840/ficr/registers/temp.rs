//! temp.rs.rs
//!
//! TEMP module
//! 

use bitfield_struct::bitfield;

use crate::types::RegInfo;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TEMPRegType {
    /// Slope definition A0
    A0,
    /// Slope definition A1
    A1,
    /// Slope definition A2
    A2,
    /// Slope definition A3
    A3,
    /// Slope definition A4
    A4,
    /// Slope definition A5
    A5,
    /// Y-intercept B0
    B0,
    /// Y-intercept B1
    B1,
    /// Y-intercept B2
    B2,
    /// Y-intercept B3
    B3,
    /// Y-intercept B4
    B4,
    /// Y-intercept B5
    B5,
    /// Segment end T0
    T0,
    /// Segment end T1
    T1,
    /// Segment end T2
    T2,
    /// Segment end T3
    T3,
    /// Segment end T4
    T4,
}

impl TEMPRegType {

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
        assert!(offset < 0x0044, "address not in peripheral!");
        match offset {
            0x000 => { Some(TEMPRegType::A0) }
            0x004 => { Some(TEMPRegType::A1) }
            0x008 => { Some(TEMPRegType::A2) }
            0x00C => { Some(TEMPRegType::A3) }
            0x010 => { Some(TEMPRegType::A4) }
            0x014 => { Some(TEMPRegType::A5) }
            0x018 => { Some(TEMPRegType::B0) }
            0x01C => { Some(TEMPRegType::B1) }
            0x020 => { Some(TEMPRegType::B2) }
            0x024 => { Some(TEMPRegType::B3) }
            0x028 => { Some(TEMPRegType::B4) }
            0x02C => { Some(TEMPRegType::B5) }
            0x030 => { Some(TEMPRegType::T0) }
            0x034 => { Some(TEMPRegType::T1) }
            0x038 => { Some(TEMPRegType::T2) }
            0x03C => { Some(TEMPRegType::T3) }
            0x040 => { Some(TEMPRegType::T4) }
            _ => { None }
        }
    }
}

impl TEMPRegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            TEMPRegType::A0              => { &RegInfo { offset: 0x000, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::A1              => { &RegInfo { offset: 0x004, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::A2              => { &RegInfo { offset: 0x008, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::A3              => { &RegInfo { offset: 0x00c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::A4              => { &RegInfo { offset: 0x010, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::A5              => { &RegInfo { offset: 0x014, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::B0              => { &RegInfo { offset: 0x018, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::B1              => { &RegInfo { offset: 0x01c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::B2              => { &RegInfo { offset: 0x020, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::B3              => { &RegInfo { offset: 0x024, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::B4              => { &RegInfo { offset: 0x028, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::B5              => { &RegInfo { offset: 0x02c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::T0              => { &RegInfo { offset: 0x030, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::T1              => { &RegInfo { offset: 0x034, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::T2              => { &RegInfo { offset: 0x038, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::T3              => { &RegInfo { offset: 0x03c, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            TEMPRegType::T4              => { &RegInfo { offset: 0x040, perms: 0b100, reset: Some(0xFFFFFFFF) } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// A0
///
/// Slope definition A0
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct A0 {
    /// A (slope definition) register.
    #[bits(12)]
    pub a: u16,
    /// 
    #[bits(20)]
    pub __: u32,
}


/// A1
///
/// Slope definition A1
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct A1 {
    /// A (slope definition) register.
    #[bits(12)]
    pub a: u16,
    /// 
    #[bits(20)]
    pub __: u32,
}


/// A2
///
/// Slope definition A2
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct A2 {
    /// A (slope definition) register.
    #[bits(12)]
    pub a: u16,
    /// 
    #[bits(20)]
    pub __: u32,
}


/// A3
///
/// Slope definition A3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct A3 {
    /// A (slope definition) register.
    #[bits(12)]
    pub a: u16,
    /// 
    #[bits(20)]
    pub __: u32,
}


/// A4
///
/// Slope definition A4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct A4 {
    /// A (slope definition) register.
    #[bits(12)]
    pub a: u16,
    /// 
    #[bits(20)]
    pub __: u32,
}


/// A5
///
/// Slope definition A5
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct A5 {
    /// A (slope definition) register.
    #[bits(12)]
    pub a: u16,
    /// 
    #[bits(20)]
    pub __: u32,
}


/// B0
///
/// Y-intercept B0
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct B0 {
    /// B (y-intercept)
    #[bits(14)]
    pub b: u16,
    /// 
    #[bits(18)]
    pub __: u32,
}


/// B1
///
/// Y-intercept B1
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct B1 {
    /// B (y-intercept)
    #[bits(14)]
    pub b: u16,
    /// 
    #[bits(18)]
    pub __: u32,
}


/// B2
///
/// Y-intercept B2
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct B2 {
    /// B (y-intercept)
    #[bits(14)]
    pub b: u16,
    /// 
    #[bits(18)]
    pub __: u32,
}


/// B3
///
/// Y-intercept B3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct B3 {
    /// B (y-intercept)
    #[bits(14)]
    pub b: u16,
    /// 
    #[bits(18)]
    pub __: u32,
}


/// B4
///
/// Y-intercept B4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct B4 {
    /// B (y-intercept)
    #[bits(14)]
    pub b: u16,
    /// 
    #[bits(18)]
    pub __: u32,
}


/// B5
///
/// Y-intercept B5
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct B5 {
    /// B (y-intercept)
    #[bits(14)]
    pub b: u16,
    /// 
    #[bits(18)]
    pub __: u32,
}


/// T0
///
/// Segment end T0
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T0 {
    /// T (segment end) register
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
}


/// T1
///
/// Segment end T1
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T1 {
    /// T (segment end) register
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
}


/// T2
///
/// Segment end T2
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T2 {
    /// T (segment end) register
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
}


/// T3
///
/// Segment end T3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T3 {
    /// T (segment end) register
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
}


/// T4
///
/// Segment end T4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T4 {
    /// T (segment end) register
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
}


