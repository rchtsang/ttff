//! registers.rs
//!
//! /*% peripheral_name %*/ register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::*;
use super::*;

/*! cluster_mods --->
pub mod %cluster_name%;
pub use %cluster_name%::*;
!*/

/// /*% peripheral_name %*/ register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum /*% reg_type_name %*/ {
    /*! reg_type_variants --->
    /// %reg_type_variant_desc%
    %reg_type_variant%,
    !*/
}

impl /*% reg_type_name %*/ {

    pub fn address(&self, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data().offset as u64))
    }

    pub fn offset(&self) -> usize {
        self._data().offset
    }

    pub fn perms(&self) -> FlagSet<Permission> {
        unsafe { FlagSet::<Permission>::new_unchecked(self._data().perms) }
    }

    pub fn reset(&self) -> Option<u32> {
        self._data().reset
    }

    pub fn lookup_address(base: impl Into<u64>, address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(base.into())
            .expect("address not in peripheral!");
        Self::lookup_offset(offset as usize)
    }

    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(offset < /*% peripheral_size %*/, "address not in peripheral!");
        match offset {
            /*! reg_type_offset_match_arms --->
            %match_ptrn% => { Some(%reg_type_name%::%reg_type_variant%) }
            !*/
            /*! cluster_type_offset_match_arms --->
            %match_ptrn% => { %cluster_type_offset% }
            !*/
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let mut types = vec![
            /*! reg_type_enumeration --->
            %reg_type_name%::%reg_type_variant%,
            !*/
        ];
        /*! cluster_type_enumeration --->
        for reg_type in %cluster_reg_type%::list() {
            types.push(%mapped_reg_type%);
        }
        !*/
        types
    }
}

impl /*% reg_type_name %*/ {
    fn _data(&self) -> &'static RegInfo {
        match self {
            /*! reg_type_info_match_arms --->
            %reg_type_name%::%reg_type_variant:<15% => { &RegInfo { offset: %byte_offset:#05x%, perms: 0b%perms:03b%, reset: %reset_value% } }
            !*/
            /*! cluster_type_info_match_arms --->
            %reg_type_name%::%cluster_type_variant:<15% => { %cluster_info% }
            !*/

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/*! register_struct_defs --->

#register.rs#
!*/

