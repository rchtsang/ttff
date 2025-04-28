//! registers.rs
//!
//! /*{ peripheral_group['name'] }*/ register definition
#![allow(nonstandard_style)]

use flagset::FlagSet;

use crate::types::*;
use super::*;

/*% for cluster_type in _cluster_types(peripheral_group) -%*/
mod /*{ _cluster_mod(cluster_type) }*/;
pub use /*{ _cluster_mod(cluster_type) }*/::*;
/*% endfor %*/

/// /*{ peripheral_group['name'] }*/ register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum /*{ _reg_type(peripheral_group) }*/ {
    /*% for reg_type in _reg_types(peripheral_group) -%*/
    /// /*{ reg_type.description }*/
    /*% if reg_type.dim -%*/
    /*{ reg_type.name }*/(u8),
    /*%- else -%*/
    /*{ reg_type.name }*/,
    /*%- endif %*/
    /*% endfor -%*/
    /*% for cluster_type in _cluster_types(peripheral_group) -%*/
    /// /*{ cluster_type.description }*/
    /*% if cluster_type.dim -%*/
    /*{ cluster_type.name }*/(u8, /*{ _reg_type(cluster_type) }*/),
    /*%- else -%*/
    /*{ cluster_type.name }*/(/*{ _reg_type(cluster_type) }*/),
    /*%- endif %*/
    /*% endfor %*/
}

impl /*{ _reg_type(peripheral_group) }*/ {

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
        assert!(offset < /*{ peripheral_group['addressBlock']['size'] }*/, "address not in peripheral!");
        match offset {
            /*% for reg_type in _reg_types(peripheral_group) -%*/
            /*% if reg_type.dim -%*/
            /*{ hex(reg_type.offset) }*/..=/*{ hex(reg_type.offset + reg_type.dim_increment * reg_type.dim - 1) }*/ => { Some(/*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(((offset - /*{ reg_type.offset }*/) / 4) as u8)) }
            /*%- else -%*/
            /*{ hex(reg_type.offset) }*/ => { Some(/*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/) }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(peripheral_group) -%*/
            /*% if cluster_type.dim -%*/
            /*% for n in range(cluster_type.dim) -%*/
            /*{ hex(cluster_type.offset + cluster_type.dim_increment * n) }*/..=/*{ hex(cluster_type.offset + cluster_type.dim_increment * (n + 1) - 1) }*/ => { /*{ _reg_type(cluster_type) }*/::lookup_offset(offset - /*{ cluster_type.offset + cluster_type.dim_increment * n }*/).map(|reg| /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(n, reg)) }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ hex(cluster_type.offset) }*/..=/*{ hex(cluster_type.offset + cluster_type.size - 1) }*/ => { /*{ _reg_type(cluster_type) }*/::lookup_offset(offset - /*{ cluster_type.offset }*/).map(|reg| /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(reg)) }
            /*%- endif %*/
            /*% endfor -%*/
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let mut types = vec![
            /*% for reg_type in _reg_types(peripheral_group) -%*/
            /*% if reg_type.dim -%*/
            /*% for i in range(reg_type.dim) -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(/*{ i }*/),
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/,
            /*%- endif %*/
            /*% endfor %*/
        ];
        /*% for cluster_type in _cluster_types(peripheral_group) -%*/
        for reg_type in /*{ _reg_type(cluster_type) }*/::list() {
            types.push(/*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(reg_type));
        }
        /*% endfor %*/
        types
    }
}

impl /*{ _reg_type(peripheral_group) }*/ {
    fn _data(&self) -> &'static RegInfo {
        match self {
            /*% for reg_type in _reg_types(peripheral_group) -%*/
            /*% if reg_type.dim -%*/
            /*% for i in range(reg_type.dim) -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(/*{ i }*/) => { &RegInfo { offset: /*{ reg_type.offset + reg_type.dim_increment * i }*/, perms: /*{ reg_type.perms }*/, reset: /*{ reg_type.reset }*/ } }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/ => { &RegInfo { offset: /*{ reg_type.offset }*/, perms: /*{ reg_type.perms }*/, reset: /*{ reg_type.reset }*/ } }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(peripheral_group) -%*/
            /*% if cluster_type.dim -%*/
            /*% for n in range(cluster_type.dim) -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(/*{ n }*/, reg) => { reg._data(/*{ n }*/) }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(reg) => { reg._data() }
            /*%- endif %*/
            /*% endfor %*/
            
            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/*% for reg_type in _reg_types(peripheral_group) -%*/
/// /*{ reg_type.name }*/
///
/// /*{ reg_type.description }*/
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct /*{ reg_type.name.upper() }*/ {
    /*% for field in _fields(reg_type.register) -%*/
    /// /*{ field.description }*/
    #[bits(/*{ field.width }*/)]
    pub /*{ field.name }*/: /*{ field.type }*/,
    /*% endfor %*/
}

/*% endfor %*/