//! shadow.rs
//! 
//! a generic implementation of shadow memory given a language
use std::fmt;
use std::ops::Range;

use thiserror::Error;
use iset::IntervalMap;

use fugue_ir::{Address, VarnodeData};
use fugue_core::language::Language;

// use crate::backend;

use super::tag::{
    self,
    Tag,
    state::{FixedTagState, FixedTagStateError},
};

/// shadow state errors
#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("language builder error: {0}")]
    Builder(String),
    #[error(transparent)]
    State(#[from] FixedTagStateError),
    #[error("unmapped access: {0:#x?}")]
    Unmapped(u64),
    #[error("mapped regions conflict: {0:#x?} and {1:#x?}")]
    MapConflict(Range<u64>, Range<u64>),
    #[error("invalid register: {0}")]
    InvalidRegister(&'static str),
}



/// a shadow state for pcode context
#[derive(Clone)]
pub struct ShadowState {
    pub lang: Language,
    regs: FixedTagState,
    tmps: FixedTagState,
    mmap: IntervalMap<u64, FixedTagState>,
}

impl ShadowState {
    pub fn new_with(lang: Language) -> Self {
        let t = lang.translator();
        let regs = FixedTagState::new(t.register_space_size());
        let tmps = FixedTagState::new(t.unique_space_size());
        let mmap = IntervalMap::default();

        Self { lang, regs, tmps, mmap }
    }

    #[inline(always)]
    pub fn get_pc_tag(&self) -> Result<Tag, Error> {
        let pc_vnd = self._pc_vnd();
        self.regs.read_tag(pc_vnd.offset() as usize, pc_vnd.size())
            .map_err(|e| e.into())
    }

    #[inline(always)]
    pub fn set_pc_tag(&mut self, tag: impl AsRef<Tag>) -> Result<(), Error> {
        let tag = tag.as_ref();
        let pc_vnd = self._pc_vnd();
        self.regs.write_tag(pc_vnd.offset() as usize, pc_vnd.size(), *tag)
            .map_err(|e| e.into())
    }

    #[inline(always)]
    pub fn get_sp_tag(&self) -> Result<Tag, Error> {
        let sp_vnd = self._sp_vnd();
        self.regs.read_tag(sp_vnd.offset() as usize, sp_vnd.size())
            .map_err(|e| e.into())
    }

    #[inline(always)]
    pub fn set_sp_tag(&mut self, tag: impl AsRef<Tag>) -> Result<(), Error> {
        let tag = tag.as_ref();
        let sp_vnd = self._sp_vnd();
        self.regs.write_tag(sp_vnd.offset() as usize, sp_vnd.size(), *tag)
            .map_err(|e| e.into())
    }

    /// map a taint state region corresponding to concrete memory
    pub fn map_mem(&mut self,
        base: impl Into<Address>,
        size: usize,
        tag: Option<Tag>,
    ) -> Result<(), Error> {
        let tag = tag.unwrap_or(Tag::from(tag::UNACCESSED)).into();
        let base: Address = base.into();
        // mapped memory must be word-aligned
        assert_eq!(base.offset() & 0b11, 0, "base {base:#x?} is not word-aligned!");
        assert_eq!(size & 0b11, 0, "size {size:#x} is not word-aligned!");

        // check for collision
        let range = base.offset()..(base.offset() + size as u64);
        if let Some(colliding) = self.mmap.intervals(range.clone()).next() {
            return Err(Error::MapConflict(range, colliding));
        }

        let mem = FixedTagState::new_with(size, tag);
        self.mmap.insert(range, mem);

        Ok(())
    }

    pub fn read_tag(&self, vnd: &VarnodeData) -> Result<Tag, Error> {
        let spc = vnd.space();
        if spc.is_constant() {
            Ok(Tag::from(tag::UNACCESSED))
        } else if spc.is_register() {
            self.regs.read_tag(vnd.offset() as usize, vnd.size())
                .map_err(|e| e.into())
        } else if spc.is_unique() {
            self.tmps.read_tag(vnd.offset() as usize, vnd.size())
                .map_err(|e| e.into())
        } else if spc.is_default() {
            self.read_mem_tags(Address::from(vnd.offset()), vnd.size())
        } else {
            panic!("read from {spc:?} unsupported!")
        }
    }

    pub fn write_tag(&mut self, vnd: &VarnodeData, tag: &Tag) -> Result<(), Error> {
        let spc = vnd.space();
        if spc.is_register() {
            self.regs.write_tag(vnd.offset() as usize, vnd.size(), tag)
                .map_err(|e| e.into())
        } else if spc.is_unique() {
            self.tmps.write_tag(vnd.offset() as usize, vnd.size(), tag)
                .map_err(|e| e.into())
        } else if spc.is_default() {
            self.write_mem_tags(Address::from(vnd.offset()), vnd.size(), tag)
        } else if spc.is_constant() {
            panic!("cannot write to constant varnode")
        } else {
            panic!("write to {spc:?} unsupported!")
        }
    }

    pub fn read_mem_tags(&self, address: impl AsRef<Address>, size: usize) -> Result<Tag, Error> {
        let tag_mem = self.view_mem_tags(address, size)?;
        Ok(tag_mem.iter().fold(Tag::new(), |result, t| result | t))
    }

    pub fn view_mem_tags(&self, address: impl AsRef<Address>, size: usize) -> Result<&[Tag], Error> {
        let address = address.as_ref();
        let (range, mem) = self._get_mem_tagstate(address)?;
        let offset = (address.offset() - range.start) as usize;
        mem.view(offset, size)
            .map_err(|e| e.into())
    }

    pub fn write_mem_tags(&mut self, address: impl AsRef<Address>, size: usize, tag: impl AsRef<Tag>) -> Result<(), Error> {
        let tag = tag.as_ref();
        let tag_mem = self.view_mem_tags_mut(address, size)?;
        for t in tag_mem.iter_mut() {
            t.set_raw(tag.get_raw());
        }
        Ok(())
    } 

    pub fn view_mem_tags_mut(&mut self, address: impl AsRef<Address>, size: usize) -> Result<&mut [Tag], Error> {
        let address = address.as_ref();
        let (range, mem) = self._get_mem_tagstate_mut(address)?;
        let offset = (address.offset() - range.start) as usize;
        mem.view_mut(offset, size)
            .map_err(|e| e.into())
    }
}



impl ShadowState {
    fn _pc_vnd(&self) -> &VarnodeData {
        self.lang.translator().program_counter()
    }

    fn _sp_vnd(&self) -> &VarnodeData {
        self.lang.convention().stack_pointer().varnode()
    }

    fn _get_mem_tagstate(&self, address: &Address) -> Result<(Range<u64>, &FixedTagState), Error> {
        let mut overlaps = self.mmap.overlap(address.offset());
        let (range, mem) = overlaps.next()
            .ok_or(Error::Unmapped(address.offset()))?;
        if let Some((conflict, _)) = overlaps.next() {
            return Err(Error::MapConflict(range, conflict));
        }
        Ok((range, mem))
    }

    fn _get_mem_tagstate_mut(&mut self, address: &Address) -> Result<(Range<u64>, &mut FixedTagState), Error> {
        let mut overlaps = self.mmap.overlap_mut(address.offset());
        let (range, mem) = overlaps.next()
            .ok_or(Error::Unmapped(address.offset()))?;
        if let Some((conflict, _)) = overlaps.next() {
            return Err(Error::MapConflict(range, conflict));
        }
        Ok((range, mem))
    }
}


impl fmt::Debug for ShadowState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ShadowState {{ {:?} }}", self.lang.translator().architecture())
    }
}