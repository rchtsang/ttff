//! registers.rs
//!
//! GPIO register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::RegInfo;
use super::*;
use context::Permission;


/// GPIO register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GPIORegType {
    /// Write GPIO port
    OUT,
    /// Set individual bits in GPIO port
    OUTSET,
    /// Clear individual bits in GPIO port
    OUTCLR,
    /// Read GPIO port
    IN,
    /// Direction of GPIO pins
    DIR,
    /// DIR set register
    DIRSET,
    /// DIR clear register
    DIRCLR,
    /// Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers
    LATCH,
    /// Select between default DETECT signal behavior and LDETECT mode
    DETECTMODE,
    /// Description collection: Configuration of GPIO pins
    PIN_CNF(u8),
}

impl GPIORegType {

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
            0x504 => { Some(GPIORegType::OUT) }
            0x508 => { Some(GPIORegType::OUTSET) }
            0x50C => { Some(GPIORegType::OUTCLR) }
            0x510 => { Some(GPIORegType::IN) }
            0x514 => { Some(GPIORegType::DIR) }
            0x518 => { Some(GPIORegType::DIRSET) }
            0x51C => { Some(GPIORegType::DIRCLR) }
            0x520 => { Some(GPIORegType::LATCH) }
            0x524 => { Some(GPIORegType::DETECTMODE) }
            0x700..=0x77f => { Some(GPIORegType::PIN_CNF(((offset - 0x700) / 4) as u8)) }
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let mut types = vec![
            GPIORegType::OUT,
            GPIORegType::OUTSET,
            GPIORegType::OUTCLR,
            GPIORegType::IN,
            GPIORegType::DIR,
            GPIORegType::DIRSET,
            GPIORegType::DIRCLR,
            GPIORegType::LATCH,
            GPIORegType::DETECTMODE,
            GPIORegType::PIN_CNF(0),
            GPIORegType::PIN_CNF(1),
            GPIORegType::PIN_CNF(2),
            GPIORegType::PIN_CNF(3),
            GPIORegType::PIN_CNF(4),
            GPIORegType::PIN_CNF(5),
            GPIORegType::PIN_CNF(6),
            GPIORegType::PIN_CNF(7),
            GPIORegType::PIN_CNF(8),
            GPIORegType::PIN_CNF(9),
            GPIORegType::PIN_CNF(10),
            GPIORegType::PIN_CNF(11),
            GPIORegType::PIN_CNF(12),
            GPIORegType::PIN_CNF(13),
            GPIORegType::PIN_CNF(14),
            GPIORegType::PIN_CNF(15),
            GPIORegType::PIN_CNF(16),
            GPIORegType::PIN_CNF(17),
            GPIORegType::PIN_CNF(18),
            GPIORegType::PIN_CNF(19),
            GPIORegType::PIN_CNF(20),
            GPIORegType::PIN_CNF(21),
            GPIORegType::PIN_CNF(22),
            GPIORegType::PIN_CNF(23),
            GPIORegType::PIN_CNF(24),
            GPIORegType::PIN_CNF(25),
            GPIORegType::PIN_CNF(26),
            GPIORegType::PIN_CNF(27),
            GPIORegType::PIN_CNF(28),
            GPIORegType::PIN_CNF(29),
            GPIORegType::PIN_CNF(30),
            GPIORegType::PIN_CNF(31),
        ];
        types
    }
}

impl GPIORegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            GPIORegType::OUT             => { &RegInfo { offset: 0x504, perms: 0b110, reset: None } }
            GPIORegType::OUTSET          => { &RegInfo { offset: 0x508, perms: 0b110, reset: None } }
            GPIORegType::OUTCLR          => { &RegInfo { offset: 0x50c, perms: 0b110, reset: None } }
            GPIORegType::IN              => { &RegInfo { offset: 0x510, perms: 0b100, reset: None } }
            GPIORegType::DIR             => { &RegInfo { offset: 0x514, perms: 0b110, reset: None } }
            GPIORegType::DIRSET          => { &RegInfo { offset: 0x518, perms: 0b110, reset: None } }
            GPIORegType::DIRCLR          => { &RegInfo { offset: 0x51c, perms: 0b110, reset: None } }
            GPIORegType::LATCH           => { &RegInfo { offset: 0x520, perms: 0b110, reset: None } }
            GPIORegType::DETECTMODE      => { &RegInfo { offset: 0x524, perms: 0b110, reset: None } }
            GPIORegType::PIN_CNF(0)      => { &RegInfo { offset: 0x700, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(1)      => { &RegInfo { offset: 0x704, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(2)      => { &RegInfo { offset: 0x708, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(3)      => { &RegInfo { offset: 0x70c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(4)      => { &RegInfo { offset: 0x710, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(5)      => { &RegInfo { offset: 0x714, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(6)      => { &RegInfo { offset: 0x718, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(7)      => { &RegInfo { offset: 0x71c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(8)      => { &RegInfo { offset: 0x720, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(9)      => { &RegInfo { offset: 0x724, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(10)     => { &RegInfo { offset: 0x728, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(11)     => { &RegInfo { offset: 0x72c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(12)     => { &RegInfo { offset: 0x730, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(13)     => { &RegInfo { offset: 0x734, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(14)     => { &RegInfo { offset: 0x738, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(15)     => { &RegInfo { offset: 0x73c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(16)     => { &RegInfo { offset: 0x740, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(17)     => { &RegInfo { offset: 0x744, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(18)     => { &RegInfo { offset: 0x748, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(19)     => { &RegInfo { offset: 0x74c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(20)     => { &RegInfo { offset: 0x750, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(21)     => { &RegInfo { offset: 0x754, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(22)     => { &RegInfo { offset: 0x758, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(23)     => { &RegInfo { offset: 0x75c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(24)     => { &RegInfo { offset: 0x760, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(25)     => { &RegInfo { offset: 0x764, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(26)     => { &RegInfo { offset: 0x768, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(27)     => { &RegInfo { offset: 0x76c, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(28)     => { &RegInfo { offset: 0x770, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(29)     => { &RegInfo { offset: 0x774, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(30)     => { &RegInfo { offset: 0x778, perms: 0b110, reset: Some(0x00000002) } }
            GPIORegType::PIN_CNF(31)     => { &RegInfo { offset: 0x77c, perms: 0b110, reset: Some(0x00000002) } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// OUT
///
/// Write GPIO port
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct OUT {
    /// Pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// OUTSET
///
/// Set individual bits in GPIO port
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct OUTSET {
    /// Pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// OUTCLR
///
/// Clear individual bits in GPIO port
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct OUTCLR {
    /// Pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// IN
///
/// Read GPIO port
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct IN {
    /// Pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// DIR
///
/// Direction of GPIO pins
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DIR {
    /// Pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// DIRSET
///
/// DIR set register
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DIRSET {
    /// Set as output pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Set as output pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Set as output pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Set as output pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Set as output pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Set as output pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Set as output pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Set as output pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Set as output pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Set as output pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Set as output pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Set as output pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Set as output pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Set as output pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Set as output pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Set as output pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Set as output pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Set as output pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Set as output pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Set as output pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Set as output pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Set as output pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Set as output pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Set as output pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Set as output pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Set as output pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Set as output pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Set as output pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Set as output pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Set as output pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Set as output pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Set as output pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// DIRCLR
///
/// DIR clear register
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DIRCLR {
    /// Set as input pin 0
    #[bits(1)]
    pub pin0: bool,
    /// Set as input pin 1
    #[bits(1)]
    pub pin1: bool,
    /// Set as input pin 2
    #[bits(1)]
    pub pin2: bool,
    /// Set as input pin 3
    #[bits(1)]
    pub pin3: bool,
    /// Set as input pin 4
    #[bits(1)]
    pub pin4: bool,
    /// Set as input pin 5
    #[bits(1)]
    pub pin5: bool,
    /// Set as input pin 6
    #[bits(1)]
    pub pin6: bool,
    /// Set as input pin 7
    #[bits(1)]
    pub pin7: bool,
    /// Set as input pin 8
    #[bits(1)]
    pub pin8: bool,
    /// Set as input pin 9
    #[bits(1)]
    pub pin9: bool,
    /// Set as input pin 10
    #[bits(1)]
    pub pin10: bool,
    /// Set as input pin 11
    #[bits(1)]
    pub pin11: bool,
    /// Set as input pin 12
    #[bits(1)]
    pub pin12: bool,
    /// Set as input pin 13
    #[bits(1)]
    pub pin13: bool,
    /// Set as input pin 14
    #[bits(1)]
    pub pin14: bool,
    /// Set as input pin 15
    #[bits(1)]
    pub pin15: bool,
    /// Set as input pin 16
    #[bits(1)]
    pub pin16: bool,
    /// Set as input pin 17
    #[bits(1)]
    pub pin17: bool,
    /// Set as input pin 18
    #[bits(1)]
    pub pin18: bool,
    /// Set as input pin 19
    #[bits(1)]
    pub pin19: bool,
    /// Set as input pin 20
    #[bits(1)]
    pub pin20: bool,
    /// Set as input pin 21
    #[bits(1)]
    pub pin21: bool,
    /// Set as input pin 22
    #[bits(1)]
    pub pin22: bool,
    /// Set as input pin 23
    #[bits(1)]
    pub pin23: bool,
    /// Set as input pin 24
    #[bits(1)]
    pub pin24: bool,
    /// Set as input pin 25
    #[bits(1)]
    pub pin25: bool,
    /// Set as input pin 26
    #[bits(1)]
    pub pin26: bool,
    /// Set as input pin 27
    #[bits(1)]
    pub pin27: bool,
    /// Set as input pin 28
    #[bits(1)]
    pub pin28: bool,
    /// Set as input pin 29
    #[bits(1)]
    pub pin29: bool,
    /// Set as input pin 30
    #[bits(1)]
    pub pin30: bool,
    /// Set as input pin 31
    #[bits(1)]
    pub pin31: bool,
}

/// LATCH
///
/// Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LATCH {
    /// Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin0: bool,
    /// Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin1: bool,
    /// Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin2: bool,
    /// Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin3: bool,
    /// Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin4: bool,
    /// Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin5: bool,
    /// Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin6: bool,
    /// Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin7: bool,
    /// Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin8: bool,
    /// Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin9: bool,
    /// Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin10: bool,
    /// Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin11: bool,
    /// Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin12: bool,
    /// Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin13: bool,
    /// Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin14: bool,
    /// Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin15: bool,
    /// Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin16: bool,
    /// Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin17: bool,
    /// Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin18: bool,
    /// Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin19: bool,
    /// Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin20: bool,
    /// Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin21: bool,
    /// Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin22: bool,
    /// Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin23: bool,
    /// Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin24: bool,
    /// Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin25: bool,
    /// Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin26: bool,
    /// Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin27: bool,
    /// Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin28: bool,
    /// Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin29: bool,
    /// Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin30: bool,
    /// Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear.
    #[bits(1)]
    pub pin31: bool,
}

/// DETECTMODE
///
/// Select between default DETECT signal behavior and LDETECT mode
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DETECTMODE {
    /// Select between default DETECT signal behavior and LDETECT mode
    #[bits(1)]
    pub detectmode: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// PIN_CNF
///
/// Description collection: Configuration of GPIO pins
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PIN_CNF {
    /// Pin direction. Same physical register as DIR register
    #[bits(1)]
    pub dir: bool,
    /// Connect or disconnect input buffer
    #[bits(1)]
    pub input: bool,
    /// Pull configuration
    #[bits(2)]
    pub pull: u8,
    /// 
    #[bits(4)]
    pub __: u32,
    /// Drive configuration
    #[bits(3)]
    pub drive: u8,
    /// 
    #[bits(5)]
    pub __: u32,
    /// Pin sensing mechanism
    #[bits(2)]
    pub sense: u8,
    /// 
    #[bits(14)]
    pub __: u32,
}

