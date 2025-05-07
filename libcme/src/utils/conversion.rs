//! conversion.rs
//! 
//! byte to int conversion functions
use std::num::ParseIntError;

use fugue_core::ir::Location;
use fugue_ir::{Address, VarnodeData};
use fugue_bv::BitVec;

pub fn bytes_as_u32_le(src: &[u8]) -> u32 {
    u32::from_le_bytes(src.try_into().unwrap())
}

pub fn bytes_as_u32_be(src: &[u8]) -> u32 {
    u32::from_be_bytes(src.try_into().unwrap())
}

pub unsafe fn cast_as_u32_ref(src: &[u8]) -> &u32 {
    assert_eq!(src.len(), 4, "slice must be 4 bytes to cast as u32");
    &*(src as *const [u8] as *const [u8; 4] as *const u32)
}

pub fn str_to_uint(str: &str) -> Result<u64, ParseIntError> {
    let _is_negative = str.strip_prefix("-").is_some();
    match &str[..2] {
        "0x" => { u64::from_str_radix(&str[2..], 16) }
        "0o" => { u64::from_str_radix(&str[2..], 8) }
        "0b" => { u64::from_str_radix(&str[2..], 2) }
        _ => { u64::from_str_radix(str, 10) }
    }
}

/// helper to convert BitVec to Address
#[inline(always)]
pub fn bv2addr(bv: BitVec) -> Option<Address> {
    bv.to_u64()
        .map(Address::from)
}

/// helper function to convert boolean to bitvector
#[inline(always)]
pub fn bool2bv(val: bool) -> BitVec {
    BitVec::from(if val { 1u8 } else { 0u8 })
}


/// helper function to get absolute location
pub(crate) fn _absolute_loc(base: Address, vnd: VarnodeData, position: u32) -> Location {
    if !vnd.space().is_constant() {
        return Location { address: vnd.offset().into(), position: 0u32 };
    }

    let offset = vnd.offset() as i64;
    let position = if offset.is_negative() {
        position.checked_sub(offset.abs() as u32)
            .expect("negative offset from position in valid range")
    } else {
        position.checked_add(offset as u32)
            .expect("positive offset from position in valid range")
    };

    Location { address: base.into(), position }
}

/// helper to split userop parameters
pub fn get_userop_params<'a>(
    output: Option<&'a VarnodeData>,
    inputs: &'a [VarnodeData],
) -> (usize, &'a [VarnodeData], Option<&'a VarnodeData>) {
    assert!(inputs[0].space().is_constant(), "input0 of userop must be constant id per pcode spec");
    let index = inputs[0].offset() as usize;
    (index, &inputs[1..], output)
}