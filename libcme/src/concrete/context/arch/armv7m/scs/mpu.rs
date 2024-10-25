//! mpu.rs
//! 
//! memory protection unit implementation

use derive_more::{From, TryFrom, TryInto};
use bitfield_struct::bitfield;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MPURegType {

}

#[derive(Debug, Clone)]
struct MPURegData {
    pub offset: usize,
    pub perms: u8,
    pub reset: Option<u32>,
}

impl MPURegType {
    pub fn lookup_offset(offset: usize) -> Option<MPURegType> {
        todo!()
    }

    /// returns the byte offset into the system control space of
    /// the mpu register type
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    /// returns access permissions of systick register type
    pub fn permissions(&self) -> u8 {
        self._data().perms
    }

    /// returns mpu register reset value
    pub fn reset_value(&self) -> Option<u32> {
        self._data().reset
    }

    fn _data(&self) -> &'static MPURegData {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum MPUReg {
    // todo
}

#[derive(Debug, Clone, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum MPURegRef<'a> {
    // todo
}

#[derive(Debug, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum MPURegMut<'a> {
    // todo
}










impl MPURegType {
    pub(super) unsafe fn to_reg_ref<'a>(&self, int_ref: &'a u32) -> MPURegRef {
        todo!()
    }

    pub(super) unsafe fn to_reg_mut<'a>(&self, int_ref: &'a mut u32) -> MPURegMut {
        todo!()
    }
}