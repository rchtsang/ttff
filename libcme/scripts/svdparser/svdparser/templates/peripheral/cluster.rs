//! /*{ _cluster_mod(cluster) }*/.rs
//!
//! /*{ cluster['name'].replace('[%s]', '') }*/ module
//! 

use bitfield_struct::bitfield;

use libcme::types::RegInfo;
use super::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum /*{ _reg_type(cluster) }*/ {
    /*% for reg_type in _reg_types(cluster) -%*/
    /// /*{ reg_type.description }*/
    /*% if reg_type.dim -%*/
    /*{ reg_type.name }*/(u8),
    /*%- else -%*/
    /*{ reg_type.name }*/,
    /*%- endif %*/
    /*% endfor -%*/
    /*% for cluster_type in _cluster_types(cluster) -%*/
    /// /*{ cluster_type.description }*/
    /*% if cluster_type.dim -%*/
    /*{ cluster_type.name }*/(u8, /*{ _reg_type(cluster_type) }*/),
    /*%- else -%*/
    /*{ cluster_type.name }*/(/*{ _reg_type(cluster_type) }*/),
    /*%- endif %*/
    /*% endfor %*/
}

impl /*{ _reg_type(cluster) }*/ {

    /*% if 'dim' in cluster -%*/
    pub fn address(&self, n: u8, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data(n).offset as u64))
    }

    /// returns the register byte offset from the peripheral base address
    pub fn offset(&self, n: u8) -> usize {
        self._data(n).offset
    }

    /// returns access permissions of peripheral register
    pub fn permissions(&self, n: u8) -> u8 {
        self._data(n).perms
    }

    /// returns the peripheral register reset value
    pub fn reset_value(&self, n: u8) -> Option<u32> {
        self._data(n).reset
    }
    /*%- else -%*/
    pub fn address(&self, base: impl Into<u64>) -> Address {
        Address::from(base.into() + (self._data().offset as u64))
    }

    /// returns the register byte offset from the peripheral base address
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    /// returns access permissions of peripheral register
    pub fn permissions(&self) -> u8 {
        self._data().perms
    }

    /// returns the peripheral register reset value
    pub fn reset_value(&self) -> Option<u32> {
        self._data().reset
    }
    /*%- endif %*/

    pub fn lookup_address(base: impl Into<u64>, address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(base.into())
            .expect("address not in peripheral!");
        Self::lookup_offset(offset as usize)
    }

    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(offset < /*{ _cluster_size(cluster) }*/, "address not in peripheral!");
        match offset {
            /*% for reg_type in _reg_types(cluster) -%*/
            /*% if reg_type.dim -%*/
            /*{ hex(reg_type.offset) }*/..=/*{ hex(reg_type.offset + reg_type.dim_increment * reg_type.dim - 1) }*/ => { Some(/*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/(((offset - /*{ hex(reg_type.offset) }*/) / 4) as u8)) }
            /*%- else -%*/
            /*{ hex(reg_type.offset) }*/ => { Some(/*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/) }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(cluster) -%*/
            /*% if cluster_type.dim -%*/
            /*% for n in range(cluster_type.dim) -%*/
            /*{ hex(cluster_type.offset + cluster_type.dim_increment * n) }*/..=/*{ hex(cluster_type.offset + cluster_type.dim_increment * (n + 1) - 1) }*/ => { /*{ _reg_type(cluster_type) }*/::lookup_offset(offset - /*{ cluster_type.offset + cluster_type.dim_increment * n }*/).map(|reg| /*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(n, reg)) }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ hex(cluster_type.offset) }*/..=/*{ hex(cluster_type.offset + cluster_type.size - 1) }*/ => { /*{ _reg_type(cluster_type) }*/::lookup_offset(offset - /*{ hex(cluster_type.offset) }*/).map(|reg| /*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(reg)) }
            /*%- endif %*/
            /*% endfor %*/
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let mut types = vec![
            /*% for reg_type in _reg_types(cluster) -%*/
            /*% if reg_type.dim -%*/
            /*% for i in range(reg_type.dim) -%*/
            /*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/(/*{ i }*/),
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/,
            /*%- endif %*/
            /*% endfor %*/
        ];
        /*% for cluster_type in _cluster_types(cluster) -%*/
        for reg_type in /*{ _reg_type(cluster_type) }*/::list() {
            types.push(/*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(reg_type));
        }
        /*% endfor %*/
        types
    }
}

impl /*{ _reg_type(cluster) }*/ {
    pub(super) fn _data(/*{ "&self, n: u8" if 'dim' in cluster else "&self" }*/) -> &'static RegInfo {
        match self {
            /*% if 'dim' in cluster -%*/
            /*% for cluster_n in range(int(cluster['dim'], 0)) -%*/
            /*% for reg_type in _reg_types(cluster) -%*/
            /*% if reg_type.dim -%*/
            /*% for i in range(reg_type.dim) -%*/
            /*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/(/*{ str(i) }*/) if n == /*{ str(cluster_n) }*/ => { &RegInfo { offset: /*{ hex(cluster_n * int(cluster['dimIncrement'], 0) + reg_type.offset + reg_type.dim_increment * i) }*/, perms: /*{ reg_type.perms }*/, reset: /*{ reg_type.reset }*/ } }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/ if n == /*{ str(cluster_n) }*/ => { &RegInfo { offset: /*{ hex(cluster_n * int(cluster['dimIncrement'], 0) + reg_type.offset) }*/, perms: /*{ reg_type.perms }*/, reset: /*{ reg_type.reset }*/ } }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(cluster) -%*/
            /*% if cluster_type.dim -%*/
            /*% for n in range(cluster_type.dim) -%*/
            /*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(/*{ n }*/, reg) if n == /*{ cluster_n }*/ => { reg._data(/*{ n }*/) }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(reg) if n == /*{ cluster_n }*/ => { reg._data() }
            /*%- endif %*/
            /*% endfor -%*/
            /*% endfor -%*/
            /*%- else -%*/
            /*% for reg_type in _reg_types(cluster) -%*/
            /*% if reg_type.dim -%*/
            /*% for i in range(reg_type.dim) -%*/
            /*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/(/*{ i }*/) => { &RegInfo { offset: /*{ hex(reg_type.offset + reg_type.dim_increment * i) }*/, perms: /*{ reg_type.perms }*/, reset: /*{ reg_type.reset }*/ } }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(cluster) }*/::/*{ reg_type.name }*/ => { &RegInfo { offset: /*{ hex(reg_type.offset) }*/, perms: /*{ reg_type.perms }*/, reset: /*{ reg_type.reset }*/ } }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(cluster) -%*/
            /*% if cluster_type.dim -%*/
            /*% for n in range(cluster_type.dim) -%*/
            /*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(/*{ n }*/, reg) => { reg._data(/*{ n }*/) }
            /*% endfor -%*/
            /*%- else -%*/
            /*{ _reg_type(cluster) }*/::/*{ cluster_type.name }*/(reg) => { reg._data() }
            /*%- endif %*/
            /*% endfor -%*/
            /*%- endif %*/

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/*% for reg_type in _reg_types(cluster) -%*/
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
