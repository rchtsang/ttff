//! /*% cluster_filename %*/.rs
//!
//! /*% cluster_name %*/ module
//! 

use bitfield_struct::bitfield;

use crate::types::RegInfo;
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum /*% cluster_type_name %*/ {
    /*! cluster_type_variants --->
    /// %cluster_type_variant_desc%
    %cluster_type_variant%,
    !*/
}

impl /*% cluster_type_name %*/ {

    pub fn address(/*% data_params %*/, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data(/*% data_call %*/).offset as u64))
    }

    /// returns the register byte offset from the peripheral base address
    pub fn offset(/*% data_params %*/) -> usize {
        self._data(/*% data_call %*/).offset
    }

    /// returns access permissions of peripheral register
    pub fn permissions(/*% data_params %*/) -> u8 {
        self._data(/*% data_call %*/).perms
    }

    /// returns the peripheral register reset value
    pub fn reset_value(/*% data_params %*/) -> Option<u32> {
        self._data(/*% data_call %*/).reset
    }

    pub fn lookup_address(base: impl Into<u64>, address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(base.into())
            .expect("address not in peripheral!");
        Self::lookup_offset(offset as usize)
    }

    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(offset < /*% cluster_size %*/, "address not in peripheral!");
        match offset {
            /*! reg_type_offset_match_arms --->
            %match_ptrn% => { Some(%cluster_type_name%::%cluster_type_variant%) }
            !*/
            _ => { None }
        }
    }
}

impl /*% cluster_type_name %*/ {
    pub(super) fn _data(/*% data_params %*/) -> &'static RegInfo {
        match self {
            /*! reg_type_info_match_arms --->
            %cluster_type_name%::%cluster_type_variant:<15% => { &RegInfo { offset: %byte_offset:#05x%, perms: 0b%perms:03b%, reset: %reset_value% } }
            !*/
            /*! cluster_array_info_match_arms --->
            %cluster_type_name%::%cluster_type_variant:<15% if %match_cond% => { &RegInfo { offset: %byte_offset:#05x%, perms: 0b%perms:03b%, reset: %reset_value% } }
            !*/

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/*! register_struct_defs --->

#register.rs#

!*/

