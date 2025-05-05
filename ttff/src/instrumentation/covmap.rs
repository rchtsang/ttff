//! covmap.rs
//! 
//! a coverage map interface that wraps a raw mut pointer

use std::ops::{Index, IndexMut};



#[derive(Debug, Clone)]
pub struct CovMap {
    ptr: *mut u8,
    size: usize,
}

impl CovMap {
    pub fn new(ptr: *mut [u8], size: usize) -> Self {
        let ptr = ptr as *mut u8;
        Self { ptr, size }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Index<usize> for CovMap {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size { panic!("CovMap index out of bounds") }
        unsafe { &*(self.ptr.add(index) as *mut u8 as *const u8) }
    }
}

impl IndexMut<usize> for CovMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size { panic!("CovMap index out of bounds") }
        unsafe { &mut *(self.ptr.add(index) as *mut u8) }
    }
}

impl AsRef<[u8]> for CovMap {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }
}

impl AsMut<[u8]> for CovMap {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}