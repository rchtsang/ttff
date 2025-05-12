//! nfc.rs
//!
//! NFC module
//! 

use bitfield_struct::bitfield;

use libcme::types::RegInfo;
use super::*;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NFCRegType {
    /// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
    TAGHEADER0,
    /// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
    TAGHEADER1,
    /// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
    TAGHEADER2,
    /// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
    TAGHEADER3,
    
}

impl NFCRegType {

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

    pub fn lookup_address(base: impl Into<u64>, address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(base.into())
            .expect("address not in peripheral!");
        Self::lookup_offset(offset as usize)
    }

    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(offset < 16, "address not in peripheral!");
        match offset {
            0x0 => { Some(NFCRegType::TAGHEADER0) }
            0x4 => { Some(NFCRegType::TAGHEADER1) }
            0x8 => { Some(NFCRegType::TAGHEADER2) }
            0xc => { Some(NFCRegType::TAGHEADER3) }
            
            _ => { None }
        }
    }

    pub fn list() -> Vec<Self> {
        let types = vec![
            NFCRegType::TAGHEADER0,
            NFCRegType::TAGHEADER1,
            NFCRegType::TAGHEADER2,
            NFCRegType::TAGHEADER3,
            
        ];
        
        types
    }
}

impl NFCRegType {
    pub(super) fn _data(&self) -> &'static RegInfo {
        match self {
            NFCRegType::TAGHEADER0 => { &RegInfo { offset: 0x0, perms: 0b100, reset: Some(0xFFFFFF5F) } }
            NFCRegType::TAGHEADER1 => { &RegInfo { offset: 0x4, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            NFCRegType::TAGHEADER2 => { &RegInfo { offset: 0x8, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            NFCRegType::TAGHEADER3 => { &RegInfo { offset: 0xc, perms: 0b100, reset: Some(0xFFFFFFFF) } }
            

            #[allow(unreachable_patterns)]
            reg => { panic!("data for {reg:?} not implemented!") }
        }
    }
}

/// TAGHEADER0
///
/// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TAGHEADER0 {
    /// Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F
    #[bits(8)]
    pub mfgid: u8,
    /// Unique identifier byte 1
    #[bits(8)]
    pub ud1: u8,
    /// Unique identifier byte 2
    #[bits(8)]
    pub ud2: u8,
    /// Unique identifier byte 3
    #[bits(8)]
    pub ud3: u8,
    
}

/// TAGHEADER1
///
/// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TAGHEADER1 {
    /// Unique identifier byte 4
    #[bits(8)]
    pub ud4: u8,
    /// Unique identifier byte 5
    #[bits(8)]
    pub ud5: u8,
    /// Unique identifier byte 6
    #[bits(8)]
    pub ud6: u8,
    /// Unique identifier byte 7
    #[bits(8)]
    pub ud7: u8,
    
}

/// TAGHEADER2
///
/// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TAGHEADER2 {
    /// Unique identifier byte 8
    #[bits(8)]
    pub ud8: u8,
    /// Unique identifier byte 9
    #[bits(8)]
    pub ud9: u8,
    /// Unique identifier byte 10
    #[bits(8)]
    pub ud10: u8,
    /// Unique identifier byte 11
    #[bits(8)]
    pub ud11: u8,
    
}

/// TAGHEADER3
///
/// Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct TAGHEADER3 {
    /// Unique identifier byte 12
    #[bits(8)]
    pub ud12: u8,
    /// Unique identifier byte 13
    #[bits(8)]
    pub ud13: u8,
    /// Unique identifier byte 14
    #[bits(8)]
    pub ud14: u8,
    /// Unique identifier byte 15
    #[bits(8)]
    pub ud15: u8,
    
}

