//! registers.rs
//!
//! GPIOTE register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::RegInfo;
use super::*;
use context::Permission;


/// GPIOTE register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GPIOTERegType {
    /// Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is configured in CONFIG[n].POLARITY.
    TASKS_OUT(u8),
    /// Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it high.
    TASKS_SET(u8),
    /// Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it low.
    TASKS_CLR(u8),
    /// Description collection: Event generated from pin specified in CONFIG[n].PSEL
    EVENTS_IN(u8),
    /// Event generated from multiple input GPIO pins with SENSE mechanism enabled
    EVENTS_PORT,
    /// Enable interrupt
    INTENSET,
    /// Disable interrupt
    INTENCLR,
    /// Description collection: Configuration for OUT[n], SET[n], and CLR[n] tasks and IN[n] event
    CONFIG(u8),
}

impl GPIOTERegType {

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
            0x000..=0x01f => { Some(GPIOTERegType::TASKS_OUT(((offset - 0x000) / 4) as u8)) }
            0x030..=0x04f => { Some(GPIOTERegType::TASKS_SET(((offset - 0x030) / 4) as u8)) }
            0x060..=0x07f => { Some(GPIOTERegType::TASKS_CLR(((offset - 0x060) / 4) as u8)) }
            0x100..=0x11f => { Some(GPIOTERegType::EVENTS_IN(((offset - 0x100) / 4) as u8)) }
            0x17C => { Some(GPIOTERegType::EVENTS_PORT) }
            0x304 => { Some(GPIOTERegType::INTENSET) }
            0x308 => { Some(GPIOTERegType::INTENCLR) }
            0x510..=0x52f => { Some(GPIOTERegType::CONFIG(((offset - 0x510) / 4) as u8)) }
            _ => { None }
        }
    }
}

impl GPIOTERegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            GPIOTERegType::TASKS_OUT(0)    => { &RegInfo { offset: 0x000, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(1)    => { &RegInfo { offset: 0x004, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(2)    => { &RegInfo { offset: 0x008, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(3)    => { &RegInfo { offset: 0x00c, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(4)    => { &RegInfo { offset: 0x010, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(5)    => { &RegInfo { offset: 0x014, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(6)    => { &RegInfo { offset: 0x018, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_OUT(7)    => { &RegInfo { offset: 0x01c, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(0)    => { &RegInfo { offset: 0x030, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(1)    => { &RegInfo { offset: 0x034, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(2)    => { &RegInfo { offset: 0x038, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(3)    => { &RegInfo { offset: 0x03c, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(4)    => { &RegInfo { offset: 0x040, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(5)    => { &RegInfo { offset: 0x044, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(6)    => { &RegInfo { offset: 0x048, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_SET(7)    => { &RegInfo { offset: 0x04c, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(0)    => { &RegInfo { offset: 0x060, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(1)    => { &RegInfo { offset: 0x064, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(2)    => { &RegInfo { offset: 0x068, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(3)    => { &RegInfo { offset: 0x06c, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(4)    => { &RegInfo { offset: 0x070, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(5)    => { &RegInfo { offset: 0x074, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(6)    => { &RegInfo { offset: 0x078, perms: 0b010, reset: None } }
            GPIOTERegType::TASKS_CLR(7)    => { &RegInfo { offset: 0x07c, perms: 0b010, reset: None } }
            GPIOTERegType::EVENTS_IN(0)    => { &RegInfo { offset: 0x100, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(1)    => { &RegInfo { offset: 0x104, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(2)    => { &RegInfo { offset: 0x108, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(3)    => { &RegInfo { offset: 0x10c, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(4)    => { &RegInfo { offset: 0x110, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(5)    => { &RegInfo { offset: 0x114, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(6)    => { &RegInfo { offset: 0x118, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_IN(7)    => { &RegInfo { offset: 0x11c, perms: 0b110, reset: None } }
            GPIOTERegType::EVENTS_PORT     => { &RegInfo { offset: 0x17c, perms: 0b110, reset: None } }
            GPIOTERegType::INTENSET        => { &RegInfo { offset: 0x304, perms: 0b110, reset: None } }
            GPIOTERegType::INTENCLR        => { &RegInfo { offset: 0x308, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(0)       => { &RegInfo { offset: 0x510, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(1)       => { &RegInfo { offset: 0x514, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(2)       => { &RegInfo { offset: 0x518, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(3)       => { &RegInfo { offset: 0x51c, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(4)       => { &RegInfo { offset: 0x520, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(5)       => { &RegInfo { offset: 0x524, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(6)       => { &RegInfo { offset: 0x528, perms: 0b110, reset: None } }
            GPIOTERegType::CONFIG(7)       => { &RegInfo { offset: 0x52c, perms: 0b110, reset: None } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// TASKS_OUT
///
/// Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is configured in CONFIG[n].POLARITY.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_OUT {
    /// Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is configured in CONFIG[n].POLARITY.
    #[bits(1)]
    pub tasks_out: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_SET
///
/// Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it high.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_SET {
    /// Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it high.
    #[bits(1)]
    pub tasks_set: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_CLR
///
/// Description collection: Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it low.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_CLR {
    /// Task for writing to pin specified in CONFIG[n].PSEL. Action on pin is to set it low.
    #[bits(1)]
    pub tasks_clr: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_IN
///
/// Description collection: Event generated from pin specified in CONFIG[n].PSEL
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_IN {
    /// Event generated from pin specified in CONFIG[n].PSEL
    #[bits(1)]
    pub events_in: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_PORT
///
/// Event generated from multiple input GPIO pins with SENSE mechanism enabled
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_PORT {
    /// Event generated from multiple input GPIO pins with SENSE mechanism enabled
    #[bits(1)]
    pub events_port: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// INTENSET
///
/// Enable interrupt
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INTENSET {
    /// Write '1' to enable interrupt for event IN[0]
    #[bits(1)]
    pub in0: bool,
    /// Write '1' to enable interrupt for event IN[1]
    #[bits(1)]
    pub in1: bool,
    /// Write '1' to enable interrupt for event IN[2]
    #[bits(1)]
    pub in2: bool,
    /// Write '1' to enable interrupt for event IN[3]
    #[bits(1)]
    pub in3: bool,
    /// Write '1' to enable interrupt for event IN[4]
    #[bits(1)]
    pub in4: bool,
    /// Write '1' to enable interrupt for event IN[5]
    #[bits(1)]
    pub in5: bool,
    /// Write '1' to enable interrupt for event IN[6]
    #[bits(1)]
    pub in6: bool,
    /// Write '1' to enable interrupt for event IN[7]
    #[bits(1)]
    pub in7: bool,
    /// 
    #[bits(23)]
    pub __: u32,
    /// Write '1' to enable interrupt for event PORT
    #[bits(1)]
    pub port: bool,
}

/// INTENCLR
///
/// Disable interrupt
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INTENCLR {
    /// Write '1' to disable interrupt for event IN[0]
    #[bits(1)]
    pub in0: bool,
    /// Write '1' to disable interrupt for event IN[1]
    #[bits(1)]
    pub in1: bool,
    /// Write '1' to disable interrupt for event IN[2]
    #[bits(1)]
    pub in2: bool,
    /// Write '1' to disable interrupt for event IN[3]
    #[bits(1)]
    pub in3: bool,
    /// Write '1' to disable interrupt for event IN[4]
    #[bits(1)]
    pub in4: bool,
    /// Write '1' to disable interrupt for event IN[5]
    #[bits(1)]
    pub in5: bool,
    /// Write '1' to disable interrupt for event IN[6]
    #[bits(1)]
    pub in6: bool,
    /// Write '1' to disable interrupt for event IN[7]
    #[bits(1)]
    pub in7: bool,
    /// 
    #[bits(23)]
    pub __: u32,
    /// Write '1' to disable interrupt for event PORT
    #[bits(1)]
    pub port: bool,
}

/// CONFIG
///
/// Description collection: Configuration for OUT[n], SET[n], and CLR[n] tasks and IN[n] event
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CONFIG {
    /// Mode
    #[bits(2)]
    pub mode: u8,
    /// 
    #[bits(6)]
    pub __: u32,
    /// GPIO number associated with SET[n], CLR[n], and OUT[n] tasks and IN[n] event
    #[bits(5)]
    pub psel: u8,
    /// Port number
    #[bits(1)]
    pub port: bool,
    /// 
    #[bits(2)]
    pub __: u32,
    /// When In task mode: Operation to be performed on output when OUT[n] task is triggered. When In event mode: Operation on input that shall trigger IN[n] event.
    #[bits(2)]
    pub polarity: u8,
    /// 
    #[bits(2)]
    pub __: u32,
    /// When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect.
    #[bits(1)]
    pub outinit: bool,
    /// 
    #[bits(11)]
    pub __: u32,
}

