//! exception.rs
//! 
//! exception structs and types

use super::*;

/// exception struct
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Exception {
    num: u32,
    typ: ExceptionType,
    priority: u32,
    // vector entry point defined in vector table
    entry: Address,
    state: FlagSet<ExceptionState>
}

/// exception type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExceptionType {
    Reset,
    NMI,
    HardFault,
    MemManage,
    BusFault,
    UsageFault,
    DebugMonitor,
    SVCall,
    PendSV,
    SysTick,
    ExternalInterrupt(u32),
    Reserved,
}

impl From<u32> for ExceptionType {
    fn from(value: u32) -> Self {
        match value {
            1  => { ExceptionType::Reset }
            2  => { ExceptionType::NMI }
            3  => { ExceptionType::HardFault }
            4  => { ExceptionType::MemManage }
            5  => { ExceptionType::BusFault }
            6  => { ExceptionType::UsageFault }
            7..=10 => { ExceptionType::Reserved }
            11 => { ExceptionType::SVCall }
            12 => { ExceptionType::DebugMonitor }
            13 => { ExceptionType::Reserved }
            14 => { ExceptionType::PendSV }
            15 => { ExceptionType::SysTick }
            n => { ExceptionType::ExternalInterrupt(n) }
        }
    }
}

impl Into<u32> for &ExceptionType {
    fn into(self) -> u32 {
        match self {
            ExceptionType::Reset => { 1 }
            ExceptionType::NMI => { 2 }
            ExceptionType::HardFault => { 3 }
            ExceptionType::MemManage => { 4 }
            ExceptionType::BusFault => { 5 }
            ExceptionType::UsageFault => { 6 }
            ExceptionType::SVCall => { 11 }
            ExceptionType::DebugMonitor => { 12 }
            ExceptionType::PendSV => { 14 }
            ExceptionType::SysTick => { 15 }
            ExceptionType::ExternalInterrupt(n) => { *n }
            _ => {
                panic!("Reserved does not map to a single index!")
            }
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

