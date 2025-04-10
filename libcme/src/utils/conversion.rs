//! conversion.rs
//! 
//! byte to int conversion functions
use fugue_ir::Address;
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