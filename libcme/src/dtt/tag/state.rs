//! state.rs
//! 
//! a simple fixed tag state struct
use thiserror::Error;

use super::Tag;

#[derive(Clone, Debug, Error)]
pub enum FixedTagStateError {
    #[error("out-of-bounds read access; {size} bytes at {offset:#x}")]
    OOBRead { offset: usize, size: usize },
    #[error("out-of-bounds write access; {size} bytes at {offset:#x}")]
    OOBWrite { offset: usize, size: usize },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct FixedTagState {
    pub(crate) backing: Box<[Tag]>,
}

impl FixedTagState {
    pub fn new(size: usize) -> Self {
        Self {
            backing: vec![Tag::from(0); size].into_boxed_slice(),
        }
    }

    /// create a fixed state initialized with the given Tag
    pub fn new_with(size: usize, tag: Tag) -> Self {
        Self {
            backing: vec![tag; size].into_boxed_slice(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.backing.len()
    }

    #[inline(always)]
    pub fn read_tag(
        &self,
        offset: impl Into<usize>,
        size: usize,
    ) -> Result<Tag, FixedTagStateError> {
        let (offset, end) = self._check_bounds(offset, size)?;
        Ok(self.backing[offset..end]
            .iter()
            .fold(Tag::default(), |result, t| result | t))
    }

    #[inline(always)]
    pub fn write_tag(
        &mut self,
        offset: impl Into<usize>,
        size: usize,
        tag: impl AsRef<Tag>,
    ) -> Result<(), FixedTagStateError> {
        let (offset, end) = self._check_bounds(offset, size)?;
        let tag = tag.as_ref();

        for i in offset..end {
            self.backing[i] = *tag;
        }

        Ok(())
    }

    #[inline(always)]
    pub fn view(
        &self,
        offset: impl Into<usize>,
        size: usize,
    ) -> Result<&[Tag], FixedTagStateError> {
        let (offset, end) = self._check_bounds(offset, size)?;
        Ok(&self.backing[offset..end])
    }

    #[inline(always)]
    pub fn view_mut(
        &mut self,
        offset: impl Into<usize>,
        size: usize,
    ) -> Result<&mut [Tag], FixedTagStateError> {
        let (offset, end) = self._check_bounds(offset, size)?;
        Ok(&mut self.backing[offset..end])
    }
}

impl FixedTagState {
    #[inline(always)]
    fn _check_bounds(
        &self,
        offset: impl Into<usize>,
        size: usize,
    ) -> Result<(usize, usize), FixedTagStateError> {
        let offset = offset.into();
        let end = offset
            .checked_add(size)
            .ok_or(FixedTagStateError::OOBRead { offset, size })?;
        if end > self.backing.len() {
            return Err(FixedTagStateError::OOBRead { offset, size });
        }
        Ok((offset, end))
    }
}

impl From<Vec<Tag>> for FixedTagState {
    fn from(backing: Vec<Tag>) -> Self {
        Self {
            backing: backing.into_boxed_slice(),
        }
    }
}

impl From<Vec<u8>> for FixedTagState {
    fn from(backing: Vec<u8>) -> Self {
        let backing: Vec<Tag> = backing.into_iter()
            .map(|v| v.into())
            .collect();
        Self { 
            backing: backing.into_boxed_slice(),
        }
    }
}