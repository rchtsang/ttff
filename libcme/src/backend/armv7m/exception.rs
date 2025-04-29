//! exception.rs
//! 
//! exception structs and types

use std::cmp::Ordering;

use crate::utils::bytes_as_u32_le;

use super::*;

/// exception struct
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Exception {
    pub num: u32,
    pub typ: ExceptionType,
    pub priority: i16,
    pub enabled: bool,
    // offset into vector table
    pub offset: usize,
    pub entry: Option<Address>,
    pub state: FlagSet<ExceptionState>
}

impl Exception {
    pub fn new_with(typ: ExceptionType, priority: i16, entry: Option<&[u8]>) -> Self {
        let num = (&typ).into();
        let offset = (num * 4) as usize;
        let state = ExceptionState::Inactive.into();
        let priority = match typ {
            ExceptionType::Reset        => { -3 }
            ExceptionType::NMI          => { -2 }
            ExceptionType::HardFault    => { -1 }
            _ => { priority }
        };
        let enabled = false;
        let entry = entry.map(|slice| {
            assert_eq!(slice.len(), 4, "entry must be word-aligned");
            Address::from(bytes_as_u32_le(slice))
        });

        Self { num, typ, priority, enabled, offset, entry, state }
    }
}

impl Default for Exception {
    fn default() -> Self {
        Self {
            num: 0,
            typ: ExceptionType::Reserved(0),
            priority: 256,
            enabled: false,
            offset: 0,
            entry: None,
            state: FlagSet::default(),
        }
    }
}

/// exception type
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExceptionType {
    Reset,
    NMI,
    HardFault,
    MemFault,
    BusFault,
    UsageFault,
    DebugMonitor,
    SVCall,
    PendSV,
    SysTick,
    ExternalInterrupt(u32),
    Reserved(u32),
}

impl From<u32> for ExceptionType {
    fn from(value: u32) -> Self {
        match value {
            0  => { panic!("no system handler 0!") }
            1  => { ExceptionType::Reset }
            2  => { ExceptionType::NMI }
            3  => { ExceptionType::HardFault }
            4  => { ExceptionType::MemFault }
            5  => { ExceptionType::BusFault }
            6  => { ExceptionType::UsageFault }
            11 => { ExceptionType::SVCall }
            12 => { ExceptionType::DebugMonitor }
            14 => { ExceptionType::PendSV }
            15 => { ExceptionType::SysTick }
            (7..=10) | 13 => { ExceptionType::Reserved(value) }
            n => { ExceptionType::ExternalInterrupt(n) }
        }
    }
}

impl From<&ExceptionType> for u32 {
    fn from(value: &ExceptionType) -> u32 {
        match value {
            ExceptionType::Reset => { 1 }
            ExceptionType::NMI => { 2 }
            ExceptionType::HardFault => { 3 }
            ExceptionType::MemFault => { 4 }
            ExceptionType::BusFault => { 5 }
            ExceptionType::UsageFault => { 6 }
            ExceptionType::SVCall => { 11 }
            ExceptionType::DebugMonitor => { 12 }
            ExceptionType::PendSV => { 14 }
            ExceptionType::SysTick => { 15 }
            ExceptionType::ExternalInterrupt(n) => { *n }
            ExceptionType::Reserved(n) => { *n }
        }
    }
}

flags! {
    /// exception state
    #[derive(Hash)]
    pub enum ExceptionState: u8 {
        Inactive = 0x80,
        Active   = 0x01,
        Pending  = 0x02,
        // Active and Pending, only asynchronous exceptions
    }
}

/// implementing priority values
/// https://developer.arm.com/documentation/ka001378/latest/
#[repr(transparent)]
pub struct Priority;

impl Priority {
    pub fn compare(v1: i16, v2: i16, prigroup: u8) -> Ordering {
        if v1 < 0 || v2 < 0 {
            return v1.cmp(&v2);
        }
        let v1 = v1 as u8;
        let v2 = v2 as u8;
        match prigroup {
            0..=7 => {
                let g1 = v1 >> (prigroup + 1);
                let s1 = v1 & (0xFF >> (7 - prigroup));
                let g2 = v2 >> (prigroup + 1);
                let s2 = v2 & (0xFF >> (7 - prigroup));
                (g1, s1).cmp(&(g2, s2))
            }
            _ => { unreachable!("invalid prigroup value: {prigroup}") }
        }
        // comment above and uncomment below to ignore prigroup effect
        // assert!(prigroup < 8, "invalid prigroup value: {prigroup}");
        // (v1).cmp(&v2)
    }
}

/// a vector table interface
pub struct VectorTable;

impl VectorTable {
    pub fn get_entry<'a>(vt: &'a [u32], typ: ExceptionType) -> Option<u32> {
        let excp_num = u32::from(&typ) as usize;
        vt.get(excp_num)
            .map(|val| *val)
    }
}

