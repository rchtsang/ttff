//! validation.rs
//! 
//! utilities for validating things

use fugue_ir::Address;
use crate::types::Alignment;

/// helper function to check for an expected alignment
pub fn check_alignment(
    address: u32,
    size: usize,
    expected: Alignment,
) -> Result<(), (Address, usize, Alignment)> {
    match expected {
        Alignment::Byte if (
            (size == 1)
        ) => { Ok(()) }
        Alignment::Half if (
            (address & 1 == 0) && (size == 2)
        ) => { Ok(()) }
        Alignment::Word if (
            (address & 0b11 == 0) && (size == 4)
        ) => { Ok(()) }
        Alignment::Even if (
            ((address & 0b11 == 2) && (size == 2))
            || ((address & 1 == 0) && (size & 1 == 0))
        ) => { Ok(()) }
        Alignment::Any if (
            ((address & 1 == 1) && (size == 1))
            || ((address & 0b11 == 2) && (size == 2))
            || ((address & 0b11 == 0) && (size & 1 == 0))
        ) => { Ok(()) }
        _ => {
            Err((address.into(), size, expected))
        }
    }
}