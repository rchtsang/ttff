//! systick.rs
//! 
//! implementation of system timer for armv7m
//! 
//! note: assumes implementation MUST be little endian
//! which _should_ be Rust's default endianness (i think. big assumption...)

use derive_more::{From, TryFrom, TryInto};
use bitfield_struct::bitfield;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SysTickRegType {
    CSR,     // systick control and status register
    RVR,     // systick reload value register
    CVR,     // systick current value register
    CALIB,   // systick calibration value register
}

#[derive(Debug, Clone)]
struct SysTickRegData {
    pub offset: usize,
    pub perms: u8,
    pub reset: Option<u32>,
}

impl SysTickRegType {
    pub fn lookup_offset(offset: usize) -> Option<SysTickRegType> {
        match offset {
            0x010 => { Some(SysTickRegType::CSR) }
            0x014 => { Some(SysTickRegType::RVR) }
            0x018 => { Some(SysTickRegType::CVR) }
            0x01C => { Some(SysTickRegType::CALIB) }
            _ => { None }
        }
    }

    /// returns the register's address
    pub fn address(&self) -> Address {
        (0xe000e000 + self.offset() as u32).into()
    }

    /// returns the byte offset into the system control space of the 
    /// systick register type
    pub fn offset(&self) -> usize {
        self._data().offset
    }

    /// returns access permissions of systick register type
    pub fn permissions(&self) -> u8 {
        self._data().perms
    }

    /// returns systick register reset value
    pub fn reset_value(&self) -> Option<u32> {
        self._data().reset
    }

    fn _data(&self) -> &'static SysTickRegData {
        match self {
            SysTickRegType::CSR     => { &SysTickRegData { offset: 0x010, perms: 0b110, reset: None } }
            SysTickRegType::RVR     => { &SysTickRegData { offset: 0x014, perms: 0b110, reset: None } }
            SysTickRegType::CVR     => { &SysTickRegData { offset: 0x018, perms: 0b110, reset: None } }
            SysTickRegType::CALIB   => { &SysTickRegData { offset: 0x01C, perms: 0b110, reset: None } }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum SysTickReg {
    CSR(CSR),       // systick control and status register
    RVR(RVR),       // systick reload value register
    CVR(CVR),       // systick current value register
    CALIB(CALIB),   // systick calibration value register
}

#[derive(Debug, Clone, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum SysTickRegRef<'a> {
    CSR(&'a CSR),
    RVR(&'a RVR),
    CVR(&'a CVR),
    CALIB(&'a CALIB),
}

#[derive(Debug, From, TryFrom, TryInto)]
#[try_into(owned, ref, ref_mut)]
pub enum SysTickRegMut<'a> {
    CSR(&'a mut CSR),
    RVR(&'a mut RVR),
    CVR(&'a mut CVR),
    CALIB(&'a mut CALIB),
}

/// controls sytem timer and provides status data
/// 
/// see B3.3.3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CSR {
    /// indicates enabled status of systick counter.
    /// (0 = disabled, 1 = enabled)
    #[bits(1)]
    pub enable: bool,
    /// indicates whether counting to 0 causes status of systick exception to 
    /// change to pending (enable/disable systick interrupt).
    /// (0 = exception disabled, 1 = exception enabled)
    /// 
    /// note: writing 0 to the CVR register never triggers systick exception
    #[bits(1)]
    pub tickint: bool,
    /// indicates systick clock source.
    /// (0 = implementation defined external reference clock, 1 = processor clock)
    /// 
    /// note: if no external clock provided, RAO/WI
    #[bits(1)]
    pub clksource: bool,
    #[bits(13)]
    __: u32,
    /// indicates whether the counter has counted to 0 since the last read
    /// of this register.
    /// (0 = not counted to 0, 1 = counted to 0)
    /// 
    /// notes:
    /// - countflag is set to 1 by count transition from 1 to 0
    /// - countflag is cleared to 0 by software read to this register 
    ///   or any write to CVR
    #[bits(1)]
    pub countflag: bool,
    #[bits(15)]
    __: u32,
}

/// holds the reload value of the systick current value register (CVR).
/// see B3.3.4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct RVR {
    #[bits(24)]
    pub reload: u32,
    #[bits(8)]
    __: u8,
}

/// reads or clears the current counter value.
/// see B3.3.5
/// 
/// usage:
/// - any write to the register clears the register to zero
/// - the counter does not provide read-modify-write protection
/// - unsupported bits are read as zero
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CVR {
    #[bits(24)]
    pub current: u32,
    #[bits(8)]
    __: u8,
}

/// reads the calibration value and parameters for systick.
/// see B3.3.6
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CALIB {
    /// optionally holds a reload value to be used for 10ms timing,
    /// subject to system clock skew errors.
    /// if this field is zero, the calibration value is not known.
    #[bits(24)]
    pub tenms: u32,
    #[bits(6)]
    __: u8,
    /// indicates whether the 10ms calibration value is exact.
    /// (0 = exact, 1 = inexact due to clock frequency)
    #[bits(1)]
    pub skew: bool,
    /// indicates whether reference clock is implemented.
    /// (0 = implemented, 1 = not implemented)
    /// 
    /// note: when this bit is 1, CSR.CLKSOURCE is forced to 1 and cannot be cleared
    #[bits(1)]
    pub noref: bool,
}

/// systick wrapper struct
/// 
/// used as a temporary wrapper struct to interact with the 
/// systick registers in the scs and perform systick-related operations
pub struct SysTickRegs<'a> {
    backing: &'a mut [u32; 0x40],
}

impl<'a> SysTickRegs<'a> {
    pub fn new(backing: &'a mut [u32; 0x40]) -> Self {
        Self { backing }
    }

    pub fn get_reg_ref(&self, regtype: SysTickRegType) -> SysTickRegRef {
        match regtype {
            SysTickRegType::CSR => { SysTickRegRef::CSR(self.get_csr()) }
            SysTickRegType::RVR => { SysTickRegRef::RVR(self.get_rvr()) }
            SysTickRegType::CVR => { SysTickRegRef::CVR(self.get_cvr()) }
            SysTickRegType::CALIB => { SysTickRegRef::CALIB(self.get_calib()) }
        }
    }

    pub fn get_reg_mut(&mut self, regtype: SysTickRegType) -> SysTickRegMut {
        match regtype {
            SysTickRegType::CSR => { SysTickRegMut::CSR(self.get_csr_mut()) }
            SysTickRegType::RVR => { SysTickRegMut::RVR(self.get_rvr_mut()) }
            SysTickRegType::CVR => { SysTickRegMut::CVR(self.get_cvr_mut()) }
            SysTickRegType::CALIB => { SysTickRegMut::CALIB(self.get_calib_mut()) }
        }
    }

    /// decrement the systick counter, reload if necessary.
    /// returns true if systick exception should be triggered
    pub fn tick(&mut self) -> bool {
        let offset = SysTickRegType::CVR.offset() / 4;
        let maybe_current = self.backing[offset].checked_sub(1);
        if let Some(current) = maybe_current {
            self.backing[offset] = current;
            if current == 0 {
                self.get_csr_mut().set_countflag(true);
                return self.get_csr().tickint();
            }
        } else {
            let reload_offset = SysTickRegType::RVR.offset() / 4;
            self.backing[offset] = self.backing[reload_offset];
        }
        false
    }

    /// perform an event-triggering read of systick register bytes
    pub fn read_bytes(&mut self,
        offset: usize,
        dst: &mut [u8],
        _events: &mut VecDeque<Event>,
    ) -> Result<(), context::Error> {
        let reg = SysTickRegType::lookup_offset(offset)
            .ok_or_else( | | {
                let address = Address::from(0xe000e000 + offset as u32);
                ArchError::from(Error::InvalidSysCtrlReg(address))
            })?;
        let word_offset = offset / 4;
        match reg {
            SysTickRegType::CSR => {
                let reg_slice = unsafe {
                    &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
                };
                dst.copy_from_slice(reg_slice);
                // countflag is cleared to 0 by software read to register per B3.3.3
                let csr = self.get_csr_mut();
                csr.set_countflag(false);
            }
            _ => {
                let reg_slice = unsafe {
                    &*(&self.backing[word_offset] as *const u32 as *const [u8; 4])
                };
                dst.copy_from_slice(reg_slice);
            }
        }
        Ok(())
    }

    /// perform an event-triggering write of systick register bytes
    pub fn write_bytes(&mut self,
        offset: usize,
        src: &[u8],
        events: &mut VecDeque<Event>,
    ) -> Result<(), context::Error> {
        assert_eq!(src.len(), 4, "writes to systick registers must be word-aligned");
        let reg = SysTickRegType::lookup_offset(offset)
            .ok_or_else(| | {
                let address = Address::from(0xe000e000 + offset as u32);
                ArchError::from(Error::InvalidSysCtrlReg(address))
            })?;
        let write_val = src.iter()
            .enumerate().take(4)
            .fold(0u32, |val, (i, &byte)| {
                val | ((byte as u32) << i)
            });
        match reg {
            SysTickRegType::CSR => {
                let csr = self.get_csr_mut();
                let tickint = csr.tickint();
                let enable = csr.enable();

                // countflag and clksource are read-only

                let new_csr = CSR::from_bits(write_val);
                let new_tickint = new_csr.tickint();
                let new_enable = new_csr.enable();

                if enable ^ new_enable {
                    // enable/disable systick module
                    csr.set_enable(new_enable);
                }

                if new_enable && (tickint ^ new_tickint) {
                    // enable/disable systick exceptions
                    events.push_back(Event::ExceptionEnabled(ExceptionType::SysTick, new_tickint));
                    csr.set_tickint(new_tickint);
                }
            }
            SysTickRegType::RVR => {
                // RVR only supports 24 bits
                let rvr = self.get_rvr_mut();
                rvr.0 = write_val & 0x00FFFFFF;
            }
            SysTickRegType::CVR => {
                // any write to CVR sets it to 0
                let cvr = self.get_cvr_mut();
                cvr.0 = 0;
            }
            SysTickRegType::CALIB => {
                // CALIB is read-only and implementation-defined
                let address: Address = (BASE + offset as u32).into();
                let err = Error::WriteAccessViolation(address);
                return Err(ArchError::from(err).into());
            }
        }
        Ok(())
    }
}


impl SysTickRegType {
    pub(super) unsafe fn to_reg_ref<'a>(&self, int_ref: &'a u32) -> SysTickRegRef<'a> {
        match self {
            SysTickRegType::CSR => {
                SysTickRegRef::try_from(&*(int_ref as *const u32 as *const CSR)).unwrap()
            }
            SysTickRegType::RVR => {
                SysTickRegRef::try_from(&*(int_ref as *const u32 as *const RVR)).unwrap()
            }
            SysTickRegType::CVR => {
                SysTickRegRef::try_from(&*(int_ref as *const u32 as *const CVR)).unwrap()
            }
            SysTickRegType::CALIB => {
                SysTickRegRef::try_from(&*(int_ref as *const u32 as *const CALIB)).unwrap()
            }
        }
    }

    pub(super) unsafe fn to_reg_mut<'a>(&self, int_ref: &'a mut u32) -> SysTickRegMut<'a> {
        match self {
            SysTickRegType::CSR => {
                SysTickRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CSR)).unwrap()
            }
            SysTickRegType::RVR => {
                SysTickRegMut::try_from(&mut *(int_ref as *mut u32 as *mut RVR)).unwrap()
            }
            SysTickRegType::CVR => {
                SysTickRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CVR)).unwrap()
            }
            SysTickRegType::CALIB => {
                SysTickRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CALIB)).unwrap()
            }
        }
    }
}

impl<'a> SysTickRegs<'a> {
    pub fn get_csr(&self) -> &CSR {
        let offset = SysTickRegType::CSR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const CSR) }
    }

    pub fn get_rvr(&self) -> &RVR {
        let offset = SysTickRegType::RVR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const RVR) }
    }

    pub fn get_cvr(&self) -> &CVR {
        let offset = SysTickRegType::CVR.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const CVR) }
    }

    pub fn get_calib(&self) -> &CALIB {
        let offset = SysTickRegType::CALIB.offset() / 4;
        unsafe { &*(&self.backing[offset] as *const u32 as *const CALIB) }
    }

    pub fn get_csr_mut(&mut self) -> &mut CSR {
        let offset = SysTickRegType::CSR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut CSR) }
    }

    pub fn get_rvr_mut(&mut self) -> &mut RVR {
        let offset = SysTickRegType::RVR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut RVR) }
    }

    pub fn get_cvr_mut(&mut self) -> &mut CVR {
        let offset = SysTickRegType::CVR.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut CVR) }
    }

    pub fn get_calib_mut(&mut self) -> &mut CALIB {
        let offset = SysTickRegType::CALIB.offset() / 4;
        unsafe { &mut *(&mut self.backing[offset] as *mut u32 as *mut CALIB) }
    }
}