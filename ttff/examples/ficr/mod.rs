//! ficr.rs
//! 
//! FICR module
//! Factory Information Configuration Registers
use std::fmt;
use std::collections::VecDeque;

use thiserror::Error;
use bitfield_struct::bitfield;

use libcme::prelude::*;
use libcme::peripheral::{ Error, Event };

// use super::*;

mod registers;
pub use registers::*;

pub static FICR_BASE: u32 = 0x10000000;

#[derive(Debug, Error, Clone)]
pub enum FicrError {
    #[error("attempted to write to read-only register: {0:?}")]
    WriteViolation(FICRRegType),
}

#[derive(Clone)]
pub struct FICRState {
    pub base_address: u32,
    backing: Box<[u32; 0x400]>,
}

impl fmt::Debug for FICRState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FICR @ {:#x}", self.base_address)
    }
}

impl PeripheralState for FICRState {
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

impl AsRef<[u8]> for FICRState {
    fn as_ref(&self) -> &[u8] {
        unsafe { &*(self.backing.as_ref() as *const [u32] as *const [u8]) }
    }
}

impl AsMut<[u8]> for FICRState {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self.backing.as_mut() as *mut [u32] as *mut [u8]) }
    }
}

impl Default for FICRState {
    fn default() -> Self {
        let base_address = 0x10000000;
        let backing = Box::new([0u32; 0x400]);
        Self { base_address, backing }
    }
}

impl FICRState {
    pub fn new_with(base_address: u32) -> Self {
        let mut backing = Box::new([0u32; 0x400]);
        for reg_type in FICRRegType::list() {
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
    pub fn view_as_bytes(&self) -> &[u8; 0x1000] {
        let bytes: &[u8] = self.as_ref();
        unsafe { &*(bytes as *const [u8] as *const [u8; 0x1000]) }
    }

    /// direct mutable view as bytes
    pub fn view_as_bytes_mut(&mut self) -> &mut [u8; 0x1000] {
        let bytes: &mut [u8] = self.as_mut();
        unsafe { &mut *(bytes as *mut [u8] as *mut [u8; 0x1000]) }
    }

    #[instrument(skip_all)]
    pub fn _read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        _events: &mut VecDeque<Event>
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let Some(reg_type) = FICRRegType::lookup_offset(offset) else {
            // treat unimplemented registers as memory and issue warning
            let err = Error::InvalidPeripheralReg(address.into());
            warn!("{err:x?} (treated as memory)");
            let slice = &self.view_as_bytes()[offset..offset + dst.len()];
            dst.copy_from_slice(slice);
            return Err(err.into());
        };
        trace!("read from ficr reg: {reg_type:?}");
        let slice = &self.view_as_bytes()[offset..offset + dst.len()];
        dst.copy_from_slice(slice);
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn _write_bytes(&mut self,
        offset: usize,
        _src: &[u8],
        _events: &mut VecDeque<Event>,
    ) -> Result<(), Error> {
        let address = self.base_address + offset as u32;
        let Some(reg_type) = FICRRegType::lookup_offset(offset) else {
            let err = peripheral::Error::InvalidPeripheralReg(address.into());
            return Err(err.into());
        };
        let err = FicrError::WriteViolation(reg_type);
        Err(peripheral::Error::State(err.into()))
    }
}


impl FICRState {
    // register reference getters

    pub fn get_codepagesize(&self) -> &CODEPAGESIZE {
        let word_offset = FICRRegType::CODEPAGESIZE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CODEPAGESIZE) }
    }
    
    pub fn get_codesize(&self) -> &CODESIZE {
        let word_offset = FICRRegType::CODESIZE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const CODESIZE) }
    }
    
    pub fn get_deviceid(&self, i: u8) -> &DEVICEID {
        let word_offset = FICRRegType::DEVICEID(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DEVICEID) }
    }
    
    pub fn get_er(&self, i: u8) -> &ER {
        let word_offset = FICRRegType::ER(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const ER) }
    }
    
    pub fn get_ir(&self, i: u8) -> &IR {
        let word_offset = FICRRegType::IR(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const IR) }
    }
    
    pub fn get_deviceaddrtype(&self) -> &DEVICEADDRTYPE {
        let word_offset = FICRRegType::DEVICEADDRTYPE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DEVICEADDRTYPE) }
    }
    
    pub fn get_deviceaddr(&self, i: u8) -> &DEVICEADDR {
        let word_offset = FICRRegType::DEVICEADDR(i).offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const DEVICEADDR) }
    }
    
    

    pub fn get_codepagesize_mut(&mut self) -> &mut CODEPAGESIZE {
        let word_offset = FICRRegType::CODEPAGESIZE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CODEPAGESIZE) }
    }
    
    pub fn get_codesize_mut(&mut self) -> &mut CODESIZE {
        let word_offset = FICRRegType::CODESIZE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut CODESIZE) }
    }
    
    pub fn get_deviceid_mut(&mut self, i: u8) -> &mut DEVICEID {
        let word_offset = FICRRegType::DEVICEID(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DEVICEID) }
    }
    
    pub fn get_er_mut(&mut self, i: u8) -> &mut ER {
        let word_offset = FICRRegType::ER(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut ER) }
    }
    
    pub fn get_ir_mut(&mut self, i: u8) -> &mut IR {
        let word_offset = FICRRegType::IR(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut IR) }
    }
    
    pub fn get_deviceaddrtype_mut(&mut self) -> &mut DEVICEADDRTYPE {
        let word_offset = FICRRegType::DEVICEADDRTYPE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DEVICEADDRTYPE) }
    }
    
    pub fn get_deviceaddr_mut(&mut self, i: u8) -> &mut DEVICEADDR {
        let word_offset = FICRRegType::DEVICEADDR(i).offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut DEVICEADDR) }
    }
    
    

    pub fn get_info_part(&self) -> &info::PART {
        let word_offset = INFORegType::PART.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const info::PART) }
    }
    pub fn get_info_variant(&self) -> &info::VARIANT {
        let word_offset = INFORegType::VARIANT.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const info::VARIANT) }
    }
    pub fn get_info_package(&self) -> &info::PACKAGE {
        let word_offset = INFORegType::PACKAGE.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const info::PACKAGE) }
    }
    pub fn get_info_ram(&self) -> &info::RAM {
        let word_offset = INFORegType::RAM.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const info::RAM) }
    }
    pub fn get_info_flash(&self) -> &info::FLASH {
        let word_offset = INFORegType::FLASH.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const info::FLASH) }
    }
    pub fn get_temp_a0(&self) -> &temp::A0 {
        let word_offset = TEMPRegType::A0.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::A0) }
    }
    pub fn get_temp_a1(&self) -> &temp::A1 {
        let word_offset = TEMPRegType::A1.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::A1) }
    }
    pub fn get_temp_a2(&self) -> &temp::A2 {
        let word_offset = TEMPRegType::A2.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::A2) }
    }
    pub fn get_temp_a3(&self) -> &temp::A3 {
        let word_offset = TEMPRegType::A3.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::A3) }
    }
    pub fn get_temp_a4(&self) -> &temp::A4 {
        let word_offset = TEMPRegType::A4.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::A4) }
    }
    pub fn get_temp_a5(&self) -> &temp::A5 {
        let word_offset = TEMPRegType::A5.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::A5) }
    }
    pub fn get_temp_b0(&self) -> &temp::B0 {
        let word_offset = TEMPRegType::B0.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::B0) }
    }
    pub fn get_temp_b1(&self) -> &temp::B1 {
        let word_offset = TEMPRegType::B1.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::B1) }
    }
    pub fn get_temp_b2(&self) -> &temp::B2 {
        let word_offset = TEMPRegType::B2.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::B2) }
    }
    pub fn get_temp_b3(&self) -> &temp::B3 {
        let word_offset = TEMPRegType::B3.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::B3) }
    }
    pub fn get_temp_b4(&self) -> &temp::B4 {
        let word_offset = TEMPRegType::B4.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::B4) }
    }
    pub fn get_temp_b5(&self) -> &temp::B5 {
        let word_offset = TEMPRegType::B5.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::B5) }
    }
    pub fn get_temp_t0(&self) -> &temp::T0 {
        let word_offset = TEMPRegType::T0.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::T0) }
    }
    pub fn get_temp_t1(&self) -> &temp::T1 {
        let word_offset = TEMPRegType::T1.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::T1) }
    }
    pub fn get_temp_t2(&self) -> &temp::T2 {
        let word_offset = TEMPRegType::T2.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::T2) }
    }
    pub fn get_temp_t3(&self) -> &temp::T3 {
        let word_offset = TEMPRegType::T3.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::T3) }
    }
    pub fn get_temp_t4(&self) -> &temp::T4 {
        let word_offset = TEMPRegType::T4.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const temp::T4) }
    }
    pub fn get_nfc_tagheader0(&self) -> &nfc::TAGHEADER0 {
        let word_offset = NFCRegType::TAGHEADER0.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const nfc::TAGHEADER0) }
    }
    pub fn get_nfc_tagheader1(&self) -> &nfc::TAGHEADER1 {
        let word_offset = NFCRegType::TAGHEADER1.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const nfc::TAGHEADER1) }
    }
    pub fn get_nfc_tagheader2(&self) -> &nfc::TAGHEADER2 {
        let word_offset = NFCRegType::TAGHEADER2.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const nfc::TAGHEADER2) }
    }
    pub fn get_nfc_tagheader3(&self) -> &nfc::TAGHEADER3 {
        let word_offset = NFCRegType::TAGHEADER3.offset() / 4;
        unsafe { &*(&self.backing[word_offset] as *const u32 as *const nfc::TAGHEADER3) }
    }
    

    pub fn get_info_part_mut(&mut self) -> &mut info::PART {
        let word_offset = INFORegType::PART.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut info::PART) }
    }
    pub fn get_info_variant_mut(&mut self) -> &mut info::VARIANT {
        let word_offset = INFORegType::VARIANT.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut info::VARIANT) }
    }
    pub fn get_info_package_mut(&mut self) -> &mut info::PACKAGE {
        let word_offset = INFORegType::PACKAGE.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut info::PACKAGE) }
    }
    pub fn get_info_ram_mut(&mut self) -> &mut info::RAM {
        let word_offset = INFORegType::RAM.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut info::RAM) }
    }
    pub fn get_info_flash_mut(&mut self) -> &mut info::FLASH {
        let word_offset = INFORegType::FLASH.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut info::FLASH) }
    }
    pub fn get_temp_a0_mut(&mut self) -> &mut temp::A0 {
        let word_offset = TEMPRegType::A0.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::A0) }
    }
    pub fn get_temp_a1_mut(&mut self) -> &mut temp::A1 {
        let word_offset = TEMPRegType::A1.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::A1) }
    }
    pub fn get_temp_a2_mut(&mut self) -> &mut temp::A2 {
        let word_offset = TEMPRegType::A2.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::A2) }
    }
    pub fn get_temp_a3_mut(&mut self) -> &mut temp::A3 {
        let word_offset = TEMPRegType::A3.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::A3) }
    }
    pub fn get_temp_a4_mut(&mut self) -> &mut temp::A4 {
        let word_offset = TEMPRegType::A4.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::A4) }
    }
    pub fn get_temp_a5_mut(&mut self) -> &mut temp::A5 {
        let word_offset = TEMPRegType::A5.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::A5) }
    }
    pub fn get_temp_b0_mut(&mut self) -> &mut temp::B0 {
        let word_offset = TEMPRegType::B0.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::B0) }
    }
    pub fn get_temp_b1_mut(&mut self) -> &mut temp::B1 {
        let word_offset = TEMPRegType::B1.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::B1) }
    }
    pub fn get_temp_b2_mut(&mut self) -> &mut temp::B2 {
        let word_offset = TEMPRegType::B2.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::B2) }
    }
    pub fn get_temp_b3_mut(&mut self) -> &mut temp::B3 {
        let word_offset = TEMPRegType::B3.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::B3) }
    }
    pub fn get_temp_b4_mut(&mut self) -> &mut temp::B4 {
        let word_offset = TEMPRegType::B4.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::B4) }
    }
    pub fn get_temp_b5_mut(&mut self) -> &mut temp::B5 {
        let word_offset = TEMPRegType::B5.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::B5) }
    }
    pub fn get_temp_t0_mut(&mut self) -> &mut temp::T0 {
        let word_offset = TEMPRegType::T0.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::T0) }
    }
    pub fn get_temp_t1_mut(&mut self) -> &mut temp::T1 {
        let word_offset = TEMPRegType::T1.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::T1) }
    }
    pub fn get_temp_t2_mut(&mut self) -> &mut temp::T2 {
        let word_offset = TEMPRegType::T2.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::T2) }
    }
    pub fn get_temp_t3_mut(&mut self) -> &mut temp::T3 {
        let word_offset = TEMPRegType::T3.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::T3) }
    }
    pub fn get_temp_t4_mut(&mut self) -> &mut temp::T4 {
        let word_offset = TEMPRegType::T4.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut temp::T4) }
    }
    pub fn get_nfc_tagheader0_mut(&mut self) -> &mut nfc::TAGHEADER0 {
        let word_offset = NFCRegType::TAGHEADER0.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut nfc::TAGHEADER0) }
    }
    pub fn get_nfc_tagheader1_mut(&mut self) -> &mut nfc::TAGHEADER1 {
        let word_offset = NFCRegType::TAGHEADER1.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut nfc::TAGHEADER1) }
    }
    pub fn get_nfc_tagheader2_mut(&mut self) -> &mut nfc::TAGHEADER2 {
        let word_offset = NFCRegType::TAGHEADER2.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut nfc::TAGHEADER2) }
    }
    pub fn get_nfc_tagheader3_mut(&mut self) -> &mut nfc::TAGHEADER3 {
        let word_offset = NFCRegType::TAGHEADER3.offset() / 4;
        unsafe { &mut *(&mut self.backing[word_offset] as *mut u32 as *mut nfc::TAGHEADER3) }
    }
    
}