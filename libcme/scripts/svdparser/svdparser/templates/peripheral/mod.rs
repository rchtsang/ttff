//! /*% peripheral_filename %*/.rs
//! 
//! /*% peripheral_name %*/ module
//! /*% peripheral_description %*/
use std::fmt;
use std::collections::VecDeque;

use bitfield_struct::bitfield;

use crate::prelude::*;
use crate::peripheral::{ Error, Event };
use crate::concrete::context;
use crate::utils::*;

use super::*;

mod registers;
pub use registers::*;

/*! modules --->
mod %module_name%;
pub use %module_name%::*; 
!*/

/*! base_addresses --->
pub static %peripheral_name%_BASE: u32 = %peripheral_base_address%;
!*/

#[derive(Clone)]
pub struct /*% peripheral_name %*/State {
    pub base_address: u32,
    backing: Box<[u32; /*% backing_size %*/]>,
}

impl fmt::Debug for /*% peripheral_name %*/State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/*% peripheral_name %*/ @ {:#x}", self.base_address)
    }
}

impl PeripheralState for /*% peripheral_name %*/State {
    fn base_address(&self) -> Address {
        Address::from(self.base_address)
    }

    fn size(&self) -> u64 {
        self.backing.len() as u64
    }

    fn read_bytes(&mut self,
        address: &Address,
        dst: &mut [u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let offset = address.offset()
            .checked_sub(self.base_address.into())
            .expect("address not in peripheral!");
        self._read_bytes(offset as usize, dst, events)
    }

    fn write_bytes(&mut self,
        address: &Address,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let offset = address.offset()
            .checked_sub(self.base_address.into())
            .expect("address not in peripheral!");
        self._write_bytes(offset as usize, src, events)
    }
}

impl AsRef<[u8]> for /*% peripheral_name %*/State {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for /*% peripheral_name %*/State {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for /*% peripheral_name %*/State {
    fn default() -> Self {
        let base_address = /*% default_base_address %*/;
        let backing = Box::new([0u32; /*% backing_size %*/]);
        Self { base_address, backing }
    }
}

impl /*% peripheral_name %*/State {
    pub fn new_with(base_address: u32) -> Self {
        let backing = Box::new([0u32; /*% backing_size %*/]);
        Self { base_address, backing }
    }

    /// direct view as bytes
    pub fn view_as_bytes(&self) -> &[u8; /*% byte_size %*/] {
        let bytes: &[u8] = self.as_ref();
        unsafe { &*(bytes as *const [u8] as *const [u8; /*% byte_size %*/]) }
    }

    /// direct mutable view as bytes
    pub fn view_as_bytes_mut(&mut self) -> &mut [u8; /*% byte_size %*/] {
        let bytes: &mut [u8] = self.as_mut();
        unsafe { &mut *(bytes as *mut [u8] as *mut [u8; /*% byte_size %*/]) }
    }

    #[instrument]
    pub fn _read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        events: &mut VecDeque<Event>
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let word_offset = offset / 4;
        let byte_offset = offset & 0b11;
        let Some(reg_type) = /*% reg_type %*/::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[byte_offset..byte_offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            /*! reg_type_variants_match_arms --->
            %reg_type%::%reg_type_variant:<10% => { todo!() }
            !*/

            /*! cluster_type_variants_match_arms --->
            %reg_type%::%cluster_type_variant%(reg) => { todo!() }
            !*/
        }
    }

    #[instrument]
    pub fn _write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let word_offset = offset / 4;
        let byte_offset = offset & 0b11;
        let Some(reg_type) = /*% reg_type %*/::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[byte_offset..byte_offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            /*! reg_type_variants_match_arms --->
            %reg_type%::%reg_type_variant:<10% => { todo!() }
            !*/

            /*! cluster_type_variants_match_arms --->
            %reg_type%::%cluster_type_variant%(reg) => { todo!() }
            !*/
        }
    }
}


impl /*% peripheral_name %*/State {
    // register reference getters

    /*! register_ref_getters --->
    pub fn get_%reg_type_variant_lower%(%ref_params%) -> &%reg_type_variant_struct% {
        let word_offset = %reg_type%::%reg_type_variant%.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const %reg_type_variant_struct%) }
    }

    !*/

    /*! register_mut_getters --->
    pub fn get_%reg_type_variant_lower%_mut(%mut_params%) -> &mut %reg_type_variant_struct% {
        let word_offset = %reg_type%::%reg_type_variant%.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut %reg_type_variant_struct%) }
    }
    
    !*/

    /*! cluster_reg_ref_getters --->
    pub fn get_%cluster_name_lower%_%cluster_reg_type_variant_lower%(%ref_params%) -> &%cluster_mod%::%cluster_reg_type_variant_struct% {
        let word_offset = %cluster_reg_type%::%cluster_reg_type_variant%.offset(%offset_call_params%) / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const %cluster_mod%::%cluster_reg_type_variant_struct%) }
    }

    !*/

    /*! cluster_reg_mut_getters --->
    pub fn get_%cluster_name_lower%_%cluster_reg_type_variant_lower%_mut(%mut_params%) -> &mut %cluster_mod%::%cluster_reg_type_variant_struct% {
        let word_offset = %cluster_reg_type%::%cluster_reg_type_variant%.offset(%offset_call_params%) / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut %cluster_mod%::%cluster_reg_type_variant_struct%) }
    }

    !*/
}