//! sort.rs
//! 
//! misc sorting utils

use std::collections::VecDeque;

/// sort a vecdeque in-place.
/// https://stackoverflow.com/questions/74873575
pub fn sort_vecdeque<T: Ord>(vec: &mut VecDeque<T>) {
    vec.rotate_right(vec.as_slices().1.len());
    assert!(vec.as_slices().1.is_empty());
    vec.as_mut_slices().0.sort();
}