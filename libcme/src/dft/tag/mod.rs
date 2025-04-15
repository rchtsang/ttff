//! tag module
//! 
//! implement tags for dynamic flow tracking
use std::ops;

use bitfield_struct::bitfield;

pub mod state;
pub use state::FixedTagState;

/// a data tag type
/// meant to be used as a bitflag container
#[bitfield(u8)]
#[derive(PartialEq, Eq, Hash)]
pub struct Tag {
    /// data has been accessed
    #[bits(1)]
    pub accessed: bool,
    /// data value is directly tainted
    #[bits(1)]
    pub tainted_val: bool,
    #[bits(1)]
    pub tainted_loc: bool,
    #[bits(5)]
    __: u8,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.get_raw())
    }
}

impl Tag {
    pub fn set_raw(&mut self, val: impl Into<u8>) {
        self.0 = val.into();
    }

    pub fn get_raw(&self) -> u8 {
        self.0
    }

    pub fn is_tainted(&self) -> bool {
        (self.0 & 0b11111110) != 0
    }

    pub fn simplify(&self) -> Self {
        Tag::new().with_tainted_val(self.0 > 1)
    }
}

pub const UNACCESSED:   u8 = 0;
pub const ACCESSED:     u8 = 0b00000001;
pub const TAINTED_VAL:  u8 = 0b00000010;
pub const TAINTED_LOC:  u8 = 0b00000100;

impl ops::BitAnd<Tag> for Tag {
    type Output = Tag;
    fn bitand(self, rhs: Tag) -> Self::Output {
        (self.0 & rhs.0).into()
    }
}

impl ops::BitAnd<&Tag> for Tag {
    type Output = Tag;
    fn bitand(self, rhs: &Tag) -> Self::Output {
        (self.0 & rhs.0).into()
    }
}

impl ops::BitAnd<u8> for Tag {
    type Output = Tag;
    fn bitand(self, rhs: u8) -> Self::Output {
        (self.0 & rhs).into()
    }
}

impl ops::BitAndAssign<Tag> for Tag {
    fn bitand_assign(&mut self, rhs: Tag) {
        self.0 = self.0 & rhs.0
    }
}

impl ops::BitAndAssign<&Tag> for Tag {
    fn bitand_assign(&mut self, rhs: &Tag) {
        self.0 = self.0 & rhs.0
    }
}

impl ops::BitAndAssign<u8> for Tag {
    fn bitand_assign(&mut self, rhs: u8) {
        self.0 = self.0 & rhs
    }
}

impl ops::BitOr<Tag> for Tag {
    type Output = Tag;
    fn bitor(self, rhs: Tag) -> Self::Output {
        (self.0 | rhs.0).into()
    }
}

impl ops::BitOr<&Tag> for Tag {
    type Output = Tag;
    fn bitor(self, rhs: &Tag) -> Self::Output {
        (self.0 | rhs.0).into()
    }
}

impl ops::BitOr<u8> for Tag {
    type Output = Tag;
    fn bitor(self, rhs: u8) -> Self::Output {
        (self.0 | rhs).into()
    }
}

impl ops::BitOrAssign<Tag> for Tag {
    fn bitor_assign(&mut self, rhs: Tag) {
        self.0 |= rhs.0
    }
}

impl ops::BitOrAssign<&Tag> for Tag {
    fn bitor_assign(&mut self, rhs: &Tag) {
        self.0 |= rhs.0
    }
}

impl ops::BitOrAssign<u8> for Tag {
    fn bitor_assign(&mut self, rhs: u8) {
        self.0 |= rhs
    }
}

impl ops::BitXor<Tag> for Tag {
    type Output = Tag;
    fn bitxor(self, rhs: Tag) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl ops::BitXor<&Tag> for Tag {
    type Output = Tag;
    fn bitxor(self, rhs: &Tag) -> Self::Output {
        (self.0 ^ rhs.0).into()
    }
}

impl ops::BitXor<u8> for Tag {
    type Output = Tag;
    fn bitxor(self, rhs: u8) -> Self::Output {
        (self.0 ^ rhs).into()
    }
}

impl ops::BitXorAssign<Tag> for Tag {
    fn bitxor_assign(&mut self, rhs: Tag) {
        self.0 ^= rhs.0
    }
}

impl ops::BitXorAssign<&Tag> for Tag {
    fn bitxor_assign(&mut self, rhs: &Tag) {
        self.0 ^= rhs.0
    }
}

impl ops::BitXorAssign<u8> for Tag {
    fn bitxor_assign(&mut self, rhs: u8) {
        self.0 ^= rhs
    }
}

impl AsRef<Tag> for Tag {
    fn as_ref(&self) -> &Tag {
        &self
    }
}