//! temp.rs
//!
//! TEMP module
//! 

use bitfield_struct::bitfield;

use libcme::types::RegInfo;
use super::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TEMPRegType {
    /// Slope definition A0.
    A0,
    /// Slope definition A1.
    A1,
    /// Slope definition A2.
    A2,
    /// Slope definition A3.
    A3,
    /// Slope definition A4.
    A4,
    /// Slope definition A5.
    A5,
    /// y-intercept B0.
    B0,
    /// y-intercept B1.
    B1,
    /// y-intercept B2.
    B2,
    /// y-intercept B3.
    B3,
    /// y-intercept B4.
    B4,
    /// y-intercept B5.
    B5,
    /// Segment end T0.
    T0,
    /// Segment end T1.
    T1,
    /// Segment end T2.
    T2,
    /// Segment end T3.
    T3,
    /// Segment end T4.
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
        assert!(offset < 68, "address not in peripheral!");
        match offset {
            0x0 => { Some(TEMPRegType::A0) }
            0x4 => { Some(TEMPRegType::A1) }
            0x8 => { Some(TEMPRegType::A2) }
            0xc => { Some(TEMPRegType::A3) }
            0x10 => { Some(TEMPRegType::A4) }
            0x14 => { Some(TEMPRegType::A5) }
            0x18 => { Some(TEMPRegType::B0) }
            0x1c => { Some(TEMPRegType::B1) }
            0x20 => { Some(TEMPRegType::B2) }
            0x24 => { Some(TEMPRegType::B3) }
            0x28 => { Some(TEMPRegType::B4) }
            0x2c => { Some(TEMPRegType::B5) }
            0x30 => { Some(TEMPRegType::T0) }
            0x34 => { Some(TEMPRegType::T1) }
            0x38 => { Some(TEMPRegType::T2) }
            0x3c => { Some(TEMPRegType::T3) }
            0x40 => { Some(TEMPRegType::T4) }
            
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let types = vec![
            TEMPRegType::A0,
            TEMPRegType::A1,
            TEMPRegType::A2,
            TEMPRegType::A3,
            TEMPRegType::A4,
            TEMPRegType::A5,
            TEMPRegType::B0,
            TEMPRegType::B1,
            TEMPRegType::B2,
            TEMPRegType::B3,
            TEMPRegType::B4,
            TEMPRegType::B5,
            TEMPRegType::T0,
            TEMPRegType::T1,
            TEMPRegType::T2,
            TEMPRegType::T3,
            TEMPRegType::T4,
            
        ];
        
        types
    }
}

impl TEMPRegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            TEMPRegType::A0 => { &RegInfo { offset: 0x0, perms: 0b100, reset: Some(0x00000320) } }
            TEMPRegType::A1 => { &RegInfo { offset: 0x4, perms: 0b100, reset: Some(0x00000343) } }
            TEMPRegType::A2 => { &RegInfo { offset: 0x8, perms: 0b100, reset: Some(0x0000035D) } }
            TEMPRegType::A3 => { &RegInfo { offset: 0xc, perms: 0b100, reset: Some(0x00000400) } }
            TEMPRegType::A4 => { &RegInfo { offset: 0x10, perms: 0b100, reset: Some(0x00000452) } }
            TEMPRegType::A5 => { &RegInfo { offset: 0x14, perms: 0b100, reset: Some(0x0000037B) } }
            TEMPRegType::B0 => { &RegInfo { offset: 0x18, perms: 0b100, reset: Some(0x00003FCC) } }
            TEMPRegType::B1 => { &RegInfo { offset: 0x1c, perms: 0b100, reset: Some(0x00003F98) } }
            TEMPRegType::B2 => { &RegInfo { offset: 0x20, perms: 0b100, reset: Some(0x00003F98) } }
            TEMPRegType::B3 => { &RegInfo { offset: 0x24, perms: 0b100, reset: Some(0x00000012) } }
            TEMPRegType::B4 => { &RegInfo { offset: 0x28, perms: 0b100, reset: Some(0x0000004D) } }
            TEMPRegType::B5 => { &RegInfo { offset: 0x2c, perms: 0b100, reset: Some(0x00003E10) } }
            TEMPRegType::T0 => { &RegInfo { offset: 0x30, perms: 0b100, reset: Some(0x000000E2) } }
            TEMPRegType::T1 => { &RegInfo { offset: 0x34, perms: 0b100, reset: Some(0x00000000) } }
            TEMPRegType::T2 => { &RegInfo { offset: 0x38, perms: 0b100, reset: Some(0x00000014) } }
            TEMPRegType::T3 => { &RegInfo { offset: 0x3c, perms: 0b100, reset: Some(0x00000019) } }
            TEMPRegType::T4 => { &RegInfo { offset: 0x40, perms: 0b100, reset: Some(0x00000050) } }
            

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/// A0
///
/// Slope definition A0.
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
/// Slope definition A1.
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
/// Slope definition A2.
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
/// Slope definition A3.
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
/// Slope definition A4.
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
/// Slope definition A5.
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
/// y-intercept B0.
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
/// y-intercept B1.
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
/// y-intercept B2.
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
/// y-intercept B3.
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
/// y-intercept B4.
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
/// y-intercept B5.
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
/// Segment end T0.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T0 {
    /// T (segment end)register.
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// T1
///
/// Segment end T1.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T1 {
    /// T (segment end)register.
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// T2
///
/// Segment end T2.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T2 {
    /// T (segment end)register.
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// T3
///
/// Segment end T3.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T3 {
    /// T (segment end)register.
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

/// T4
///
/// Segment end T4.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct T4 {
    /// T (segment end)register.
    #[bits(8)]
    pub t: u8,
    /// 
    #[bits(24)]
    pub __: u32,
    
}

