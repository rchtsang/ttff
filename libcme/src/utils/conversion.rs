//! conversion.rs
//! 
//! byte to int conversion functions

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