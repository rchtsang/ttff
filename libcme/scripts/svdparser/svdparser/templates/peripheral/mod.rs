//! /*{ peripheral_group['name'].lower() }*/.rs
//! 
//! /*{ peripheral_group['name'] }*/ module
//! /*{ peripheral_group['description'] }*/
use std::fmt;
use std::collections::VecDeque;

use bitfield_struct::bitfield;

use crate::prelude::*;
use crate::peripheral::{ Error, Event };
use crate::utils::*;

use super::*;

mod registers;
pub use registers::*;

/*% for group in peripheral_group['#derives'].values() -%*/
pub static /*{ group['name'] }*/_BASE: u32 = /*{ group['baseAddress'] }*/
/*% endfor %*/


#[derive(Clone)]
pub struct /*{ peripheral_group['name'] }*/State {
    pub base_address: u32,
    backing: Box<[u32; /*{ hex(_backing_size(peripheral_group)) }*/]>,
}

impl fmt::Debug for /*{ peripheral_group['name'] }*/State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "/*{ peripheral_group['name'] }*/ @ {:#x}", self.base_address)
    }
}

impl PeripheralState for /*{ peripheral_group['name'] }*/State {
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

impl AsRef<[u8]> for /*{ peripheral_group['name'] }*/State {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for /*{ peripheral_group['name'] }*/State {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for /*{ peripheral_group['name'] }*/State {
    fn default() -> Self {
        let base_address = /*{ peripheral_group['baseAddress'] }*/;
        let backing = Box::new([0u32; /*{ hex(_backing_size(peripheral_group)) }*/]);
        Self { base_address, backing }
    }
}

impl /*{ peripheral_group['name'] }*/State {
    pub fn new_with(base_address: u32) -> Self {
        let mut backing = Box::new([0u32; /*{ hex(_backing_size(peripheral_group)) }*/]);
        for reg_type in /*{ _reg_type(peripheral_group) }*/::list() {
            let offset = reg_type.offset();
            if let Some(reset_value) = reg_type.reset() {
                backing[offset] = reset_value;
            }
        }
        Self { base_address, backing }
    }

    pub fn reset(self) -> Self {
        Self::new_with(self.base_address)
    }

    /// direct view as bytes
    pub fn view_as_bytes(&self) -> &[u8; /*{ hex(_byte_size(peripheral_group)) }*/] {
        let bytes: &[u8] = self.as_ref();
        unsafe { &*(bytes as *const [u8] as *const [u8; /*{ hex(_byte_size(peripheral_group)) }*/]) }
    }

    /// direct mutable view as bytes
    pub fn view_as_bytes_mut(&mut self) -> &mut [u8; /*{ hex(_byte_size(peripheral_group)) }*/] {
        let bytes: &mut [u8] = self.as_mut();
        unsafe { &mut *(bytes as *mut [u8] as *mut [u8; /*{ hex(_byte_size(peripheral_group)) }*/]) }
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
        let Some(reg_type) = /*{ _reg_type(peripheral_group) }*/::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[byte_offset..byte_offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        match reg_type {
            /*% for reg_type in _reg_types(peripheral_group) -%*/
            /*% if reg_type.dim -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(i) => { todo!() }
            /*% else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/ => { todo!() }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(peripheral_group) -%*/
            /*% if cluster_type.dim -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(n, reg) => { todo!() }
            /*% else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(reg) => { todo!() }
            /*%- endif %*/
            /*% endfor %*/
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
        let Some(reg_type) = /*{ _reg_type(peripheral_group) }*/::lookup_offset(offset) else {
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &mut self.view_as_bytes_mut()[byte_offset..byte_offset + src.len()];
            slice.copy_from_slice(src);
            return Err(err.into());
        };
        match reg_type {
            /*% for reg_type in _reg_types(peripheral_group) -%*/
            /*% if reg_type.dim -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(i) => { todo!() }
            /*% else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/ => { todo!() }
            /*%- endif %*/
            /*% endfor -%*/
            /*% for cluster_type in _cluster_types(peripheral_group) -%*/
            /*% if cluster_type.dim -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(n, reg) => { todo!() }
            /*% else -%*/
            /*{ _reg_type(peripheral_group) }*/::/*{ cluster_type.name }*/(reg) => { todo!() }
            /*%- endif %*/
            /*% endfor %*/
        }
    }
}


impl /*{ peripheral_group['name'] }*/State {
    // register reference getters

    /*% for reg_type in _reg_types(peripheral_group) -%*/
    /*% if reg_type.dim -%*/
    pub fn get_/*{ reg_type.name.lower() }*/(&self, i: u8) -> &/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const /*{ reg_type.struct }*/) }
    }
    /*% else -%*/
    pub fn get_/*{ reg_type.name.lower() }*/(&self) -> &/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const /*{ reg_type.struct }*/) }
    }
    /*% endif %*/
    /*% endfor %*/

    /*% for reg_type in _reg_types(peripheral_group) -%*/
    /*% if reg_type.dim -%*/
    pub fn get_/*{ reg_type.name.lower() }*/_mut(&mut self, i: u8) -> &mut /*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut /*{ reg_type.struct }*/) }
    }
    /*% else -%*/
    pub fn get_/*{ reg_type.name.lower() }*/_mut(&mut self) -> &mut /*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(peripheral_group) }*/::/*{ reg_type.name }*/.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut /*{ reg_type.struct }*/) }
    }
    /*% endif %*/
    /*% endfor %*/

    /*% for cluster_type in _cluster_types(peripheral_group) -%*/
    /*% for reg_type in _reg_types(cluster_type) -%*/
    /*% if cluster_type.dim and reg_type.dim -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/(&self, n: u8, i: u8) -> &/*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/(i).offset(n) / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*% elif cluster_type.dim and not reg_type.dim -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/(&self, n: u8) -> &/*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/.offset(n) / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*% elif reg_type.dim -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/(&self, i: u8) -> &/*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*% else -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/(&self) -> &/*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*%- endif %*/
    /*% endfor -%*/
    /*% endfor %*/

    /*% for cluster_type in _cluster_types(peripheral_group) -%*/
    /*% for reg_type in _reg_types(cluster_type) -%*/
    /*% if cluster_type.dim and reg_type.dim -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/_mut(&mut self, n: u8, i: u8) -> &mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/(i).offset(n) / 4;
        unsafe { &mut *(&self.backing[word_offset] as *mut u32 as *mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*% elif cluster_type.dim and not reg_type.dim -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/_mut(&mut self, n: u8) -> &mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/.offset(n) / 4;
        unsafe { &mut *(&self.backing[word_offset] as *mut u32 as *mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*% elif reg_type.dim -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/_mut(&mut self, i: u8) -> &mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/(i).offset() / 4;
        unsafe { &mut *(&self.backing[word_offset] as *mut u32 as *mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*% else -%*/
    pub fn get_/*{ cluster_type.name.lower() }*/_/*{ reg_type.name.lower() }*/_mut(&mut self) -> &mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/ {
        let word_offset = /*{ _reg_type(cluster_type) }*/::/*{ reg_type.name }*/.offset() / 4;
        unsafe { &mut *(&self.backing[word_offset] as *mut u32 as *mut /*{ _cluster_mod(cluster_type) }*/::/*{ reg_type.struct }*/) }
    }
    /*%- endif %*/
    /*% endfor -%*/
    /*% endfor %*/
}