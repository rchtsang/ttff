//! registers.rs
//!
//! CLOCK register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::RegInfo;
use super::*;
use context::Permission;


/// CLOCK register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CLOCKRegType {
    /// Start HFXO crystal oscillator
    TASKS_HFCLKSTART,
    /// Stop HFXO crystal oscillator
    TASKS_HFCLKSTOP,
    /// Start LFCLK
    TASKS_LFCLKSTART,
    /// Stop LFCLK
    TASKS_LFCLKSTOP,
    /// Start calibration of LFRC
    TASKS_CAL,
    /// Start calibration timer
    TASKS_CTSTART,
    /// Stop calibration timer
    TASKS_CTSTOP,
    /// HFXO crystal oscillator started
    EVENTS_HFCLKSTARTED,
    /// LFCLK started
    EVENTS_LFCLKSTARTED,
    /// Calibration of LFRC completed
    EVENTS_DONE,
    /// Calibration timer timeout
    EVENTS_CTTO,
    /// Calibration timer has been started and is ready to process new tasks
    EVENTS_CTSTARTED,
    /// Calibration timer has been stopped and is ready to process new tasks
    EVENTS_CTSTOPPED,
    /// Enable interrupt
    INTENSET,
    /// Disable interrupt
    INTENCLR,
    /// Status indicating that HFCLKSTART task has been triggered
    HFCLKRUN,
    /// HFCLK status
    HFCLKSTAT,
    /// Status indicating that LFCLKSTART task has been triggered
    LFCLKRUN,
    /// LFCLK status
    LFCLKSTAT,
    /// Copy of LFCLKSRC register, set when LFCLKSTART task was triggered
    LFCLKSRCCOPY,
    /// Clock source for the LFCLK
    LFCLKSRC,
    /// HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task.
    HFXODEBOUNCE,
    /// Calibration timer interval
    CTIV,
    /// Clocking options for the trace port debug interface
    TRACECONFIG,
    /// LFRC mode configuration
    LFRCMODE,
}

impl CLOCKRegType {

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
            0x000 => { Some(CLOCKRegType::TASKS_HFCLKSTART) }
            0x004 => { Some(CLOCKRegType::TASKS_HFCLKSTOP) }
            0x008 => { Some(CLOCKRegType::TASKS_LFCLKSTART) }
            0x00C => { Some(CLOCKRegType::TASKS_LFCLKSTOP) }
            0x010 => { Some(CLOCKRegType::TASKS_CAL) }
            0x014 => { Some(CLOCKRegType::TASKS_CTSTART) }
            0x018 => { Some(CLOCKRegType::TASKS_CTSTOP) }
            0x100 => { Some(CLOCKRegType::EVENTS_HFCLKSTARTED) }
            0x104 => { Some(CLOCKRegType::EVENTS_LFCLKSTARTED) }
            0x10C => { Some(CLOCKRegType::EVENTS_DONE) }
            0x110 => { Some(CLOCKRegType::EVENTS_CTTO) }
            0x128 => { Some(CLOCKRegType::EVENTS_CTSTARTED) }
            0x12C => { Some(CLOCKRegType::EVENTS_CTSTOPPED) }
            0x304 => { Some(CLOCKRegType::INTENSET) }
            0x308 => { Some(CLOCKRegType::INTENCLR) }
            0x408 => { Some(CLOCKRegType::HFCLKRUN) }
            0x40C => { Some(CLOCKRegType::HFCLKSTAT) }
            0x414 => { Some(CLOCKRegType::LFCLKRUN) }
            0x418 => { Some(CLOCKRegType::LFCLKSTAT) }
            0x41C => { Some(CLOCKRegType::LFCLKSRCCOPY) }
            0x518 => { Some(CLOCKRegType::LFCLKSRC) }
            0x528 => { Some(CLOCKRegType::HFXODEBOUNCE) }
            0x538 => { Some(CLOCKRegType::CTIV) }
            0x55C => { Some(CLOCKRegType::TRACECONFIG) }
            0x5B4 => { Some(CLOCKRegType::LFRCMODE) }
            _ => { None }
        }
    }
}

impl CLOCKRegType {
    fn _data(&self) -> &'static RegInfo {
        match self {
            CLOCKRegType::TASKS_HFCLKSTART => { &RegInfo { offset: 0x000, perms: 0b010, reset: None } }
            CLOCKRegType::TASKS_HFCLKSTOP => { &RegInfo { offset: 0x004, perms: 0b010, reset: None } }
            CLOCKRegType::TASKS_LFCLKSTART => { &RegInfo { offset: 0x008, perms: 0b010, reset: None } }
            CLOCKRegType::TASKS_LFCLKSTOP => { &RegInfo { offset: 0x00c, perms: 0b010, reset: None } }
            CLOCKRegType::TASKS_CAL       => { &RegInfo { offset: 0x010, perms: 0b010, reset: None } }
            CLOCKRegType::TASKS_CTSTART   => { &RegInfo { offset: 0x014, perms: 0b010, reset: None } }
            CLOCKRegType::TASKS_CTSTOP    => { &RegInfo { offset: 0x018, perms: 0b010, reset: None } }
            CLOCKRegType::EVENTS_HFCLKSTARTED => { &RegInfo { offset: 0x100, perms: 0b110, reset: None } }
            CLOCKRegType::EVENTS_LFCLKSTARTED => { &RegInfo { offset: 0x104, perms: 0b110, reset: None } }
            CLOCKRegType::EVENTS_DONE     => { &RegInfo { offset: 0x10c, perms: 0b110, reset: None } }
            CLOCKRegType::EVENTS_CTTO     => { &RegInfo { offset: 0x110, perms: 0b110, reset: None } }
            CLOCKRegType::EVENTS_CTSTARTED => { &RegInfo { offset: 0x128, perms: 0b110, reset: None } }
            CLOCKRegType::EVENTS_CTSTOPPED => { &RegInfo { offset: 0x12c, perms: 0b110, reset: None } }
            CLOCKRegType::INTENSET        => { &RegInfo { offset: 0x304, perms: 0b110, reset: None } }
            CLOCKRegType::INTENCLR        => { &RegInfo { offset: 0x308, perms: 0b110, reset: None } }
            CLOCKRegType::HFCLKRUN        => { &RegInfo { offset: 0x408, perms: 0b100, reset: None } }
            CLOCKRegType::HFCLKSTAT       => { &RegInfo { offset: 0x40c, perms: 0b100, reset: None } }
            CLOCKRegType::LFCLKRUN        => { &RegInfo { offset: 0x414, perms: 0b100, reset: None } }
            CLOCKRegType::LFCLKSTAT       => { &RegInfo { offset: 0x418, perms: 0b100, reset: None } }
            CLOCKRegType::LFCLKSRCCOPY    => { &RegInfo { offset: 0x41c, perms: 0b100, reset: None } }
            CLOCKRegType::LFCLKSRC        => { &RegInfo { offset: 0x518, perms: 0b110, reset: None } }
            CLOCKRegType::HFXODEBOUNCE    => { &RegInfo { offset: 0x528, perms: 0b110, reset: Some(0x00000010) } }
            CLOCKRegType::CTIV            => { &RegInfo { offset: 0x538, perms: 0b110, reset: None } }
            CLOCKRegType::TRACECONFIG     => { &RegInfo { offset: 0x55c, perms: 0b110, reset: Some(0x00000000) } }
            CLOCKRegType::LFRCMODE        => { &RegInfo { offset: 0x5b4, perms: 0b110, reset: Some(0x00000000) } }

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}


/// TASKS_HFCLKSTART
///
/// Start HFXO crystal oscillator
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_HFCLKSTART {
    /// Start HFXO crystal oscillator
    #[bits(1)]
    pub tasks_hfclkstart: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_HFCLKSTOP
///
/// Stop HFXO crystal oscillator
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_HFCLKSTOP {
    /// Stop HFXO crystal oscillator
    #[bits(1)]
    pub tasks_hfclkstop: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_LFCLKSTART
///
/// Start LFCLK
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_LFCLKSTART {
    /// Start LFCLK
    #[bits(1)]
    pub tasks_lfclkstart: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_LFCLKSTOP
///
/// Stop LFCLK
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_LFCLKSTOP {
    /// Stop LFCLK
    #[bits(1)]
    pub tasks_lfclkstop: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_CAL
///
/// Start calibration of LFRC
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_CAL {
    /// Start calibration of LFRC
    #[bits(1)]
    pub tasks_cal: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_CTSTART
///
/// Start calibration timer
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_CTSTART {
    /// Start calibration timer
    #[bits(1)]
    pub tasks_ctstart: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// TASKS_CTSTOP
///
/// Stop calibration timer
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TASKS_CTSTOP {
    /// Stop calibration timer
    #[bits(1)]
    pub tasks_ctstop: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_HFCLKSTARTED
///
/// HFXO crystal oscillator started
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_HFCLKSTARTED {
    /// HFXO crystal oscillator started
    #[bits(1)]
    pub events_hfclkstarted: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_LFCLKSTARTED
///
/// LFCLK started
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_LFCLKSTARTED {
    /// LFCLK started
    #[bits(1)]
    pub events_lfclkstarted: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_DONE
///
/// Calibration of LFRC completed
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_DONE {
    /// Calibration of LFRC completed
    #[bits(1)]
    pub events_done: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_CTTO
///
/// Calibration timer timeout
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_CTTO {
    /// Calibration timer timeout
    #[bits(1)]
    pub events_ctto: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_CTSTARTED
///
/// Calibration timer has been started and is ready to process new tasks
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_CTSTARTED {
    /// Calibration timer has been started and is ready to process new tasks
    #[bits(1)]
    pub events_ctstarted: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// EVENTS_CTSTOPPED
///
/// Calibration timer has been stopped and is ready to process new tasks
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EVENTS_CTSTOPPED {
    /// Calibration timer has been stopped and is ready to process new tasks
    #[bits(1)]
    pub events_ctstopped: bool,
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
    /// Write '1' to enable interrupt for event HFCLKSTARTED
    #[bits(1)]
    pub hfclkstarted: bool,
    /// Write '1' to enable interrupt for event LFCLKSTARTED
    #[bits(1)]
    pub lfclkstarted: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Write '1' to enable interrupt for event DONE
    #[bits(1)]
    pub done: bool,
    /// Write '1' to enable interrupt for event CTTO
    #[bits(1)]
    pub ctto: bool,
    /// 
    #[bits(5)]
    pub __: u32,
    /// Write '1' to enable interrupt for event CTSTARTED
    #[bits(1)]
    pub ctstarted: bool,
    /// Write '1' to enable interrupt for event CTSTOPPED
    #[bits(1)]
    pub ctstopped: bool,
    /// 
    #[bits(20)]
    pub __: u32,
}

/// INTENCLR
///
/// Disable interrupt
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct INTENCLR {
    /// Write '1' to disable interrupt for event HFCLKSTARTED
    #[bits(1)]
    pub hfclkstarted: bool,
    /// Write '1' to disable interrupt for event LFCLKSTARTED
    #[bits(1)]
    pub lfclkstarted: bool,
    /// 
    #[bits(1)]
    pub __: u32,
    /// Write '1' to disable interrupt for event DONE
    #[bits(1)]
    pub done: bool,
    /// Write '1' to disable interrupt for event CTTO
    #[bits(1)]
    pub ctto: bool,
    /// 
    #[bits(5)]
    pub __: u32,
    /// Write '1' to disable interrupt for event CTSTARTED
    #[bits(1)]
    pub ctstarted: bool,
    /// Write '1' to disable interrupt for event CTSTOPPED
    #[bits(1)]
    pub ctstopped: bool,
    /// 
    #[bits(20)]
    pub __: u32,
}

/// HFCLKRUN
///
/// Status indicating that HFCLKSTART task has been triggered
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HFCLKRUN {
    /// HFCLKSTART task triggered or not
    #[bits(1)]
    pub status: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// HFCLKSTAT
///
/// HFCLK status
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HFCLKSTAT {
    /// Source of HFCLK
    #[bits(1)]
    pub src: bool,
    /// 
    #[bits(15)]
    pub __: u32,
    /// HFCLK state
    #[bits(1)]
    pub state: bool,
    /// 
    #[bits(15)]
    pub __: u32,
}

/// LFCLKRUN
///
/// Status indicating that LFCLKSTART task has been triggered
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LFCLKRUN {
    /// LFCLKSTART task triggered or not
    #[bits(1)]
    pub status: bool,
    /// 
    #[bits(31)]
    pub __: u32,
}

/// LFCLKSTAT
///
/// LFCLK status
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LFCLKSTAT {
    /// Source of LFCLK
    #[bits(2)]
    pub src: u8,
    /// 
    #[bits(14)]
    pub __: u32,
    /// LFCLK state
    #[bits(1)]
    pub state: bool,
    /// 
    #[bits(15)]
    pub __: u32,
}

/// LFCLKSRCCOPY
///
/// Copy of LFCLKSRC register, set when LFCLKSTART task was triggered
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LFCLKSRCCOPY {
    /// Clock source
    #[bits(2)]
    pub src: u8,
    /// 
    #[bits(30)]
    pub __: u32,
}

/// LFCLKSRC
///
/// Clock source for the LFCLK
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LFCLKSRC {
    /// Clock source
    #[bits(2)]
    pub src: u8,
    /// 
    #[bits(14)]
    pub __: u32,
    /// Enable or disable bypass of LFCLK crystal oscillator with external clock source
    #[bits(1)]
    pub bypass: bool,
    /// Enable or disable external source for LFCLK
    #[bits(1)]
    pub external: bool,
    /// 
    #[bits(14)]
    pub __: u32,
}

/// HFXODEBOUNCE
///
/// HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HFXODEBOUNCE {
    /// HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us.
    #[bits(8)]
    pub hfxodebounce: u8,
    /// 
    #[bits(24)]
    pub __: u32,
}

/// CTIV
///
/// Calibration timer interval
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CTIV {
    /// Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds.
    #[bits(7)]
    pub ctiv: u8,
    /// 
    #[bits(25)]
    pub __: u32,
}

/// TRACECONFIG
///
/// Clocking options for the trace port debug interface
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TRACECONFIG {
    /// Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two.
    #[bits(2)]
    pub traceportspeed: u8,
    /// 
    #[bits(14)]
    pub __: u32,
    /// Pin multiplexing of trace signals. See pin assignment chapter for more details.
    #[bits(2)]
    pub tracemux: u8,
    /// 
    #[bits(14)]
    pub __: u32,
}

/// LFRCMODE
///
/// LFRC mode configuration
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct LFRCMODE {
    /// Set LFRC mode
    #[bits(1)]
    pub mode: bool,
    /// 
    #[bits(15)]
    pub __: u32,
    /// Active LFRC mode. This field is read only.
    #[bits(1)]
    pub status: bool,
    /// 
    #[bits(15)]
    pub __: u32,
}

