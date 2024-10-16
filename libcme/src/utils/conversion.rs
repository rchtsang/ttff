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
    std::mem::transmute::<&[u8; 4], &u32>(src.try_into().unwrap())
}