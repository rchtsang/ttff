//! scs.rs

/*
 * TODO:
 * - implement endian sensitivity
 * - replace struct/int/byte conversions with unsafe std::mem::transmute for performance
 */

use bitfield_struct::bitfield;

use super::*;
use context::Permission;

use crate::utils::*;

/// system control space
/// 
/// memory-mapped 4kb address space containing 32-bit registers for
/// configuration, status, and control [0xe000e000, 0xe000efff]
/// 
/// ARM DDI 0403E.e B3.2
#[derive(Clone)]
pub struct SysCtrlSpace {
    backing: FixedState,
}

impl AsRef<FixedState> for SysCtrlSpace {
    fn as_ref(&self) -> &FixedState {
        &self.backing
    }
}

impl AsMut<FixedState> for SysCtrlSpace {
    fn as_mut(&mut self) -> &mut FixedState {
        &mut self.backing
    }
}

impl SysCtrlSpace {
    pub fn new_from(config: SysCtrlConfig) -> Self {
        todo!("implement scs constructor")
    }

    pub fn read_bytes(&mut self, offset: impl Into<usize>, dst: &mut [u8]) -> Result<Vec<Event>, context::Error> {
        let offset = offset.into();
        if let Some(scr) = SCReg::lookup_offset(offset) {
            self.read_reg(&scr, dst, Some(offset - scr.offset()))
        } else {
            Ok(vec![])
        }
    }

    pub fn write_bytes(&mut self, offset: impl Into<usize>, src: &[u8]) -> Result<Vec<Event>, context::Error> {
        let offset = offset.into();
        if let Some(scr) = SCReg::lookup_offset(offset) {
            self.write_reg(&scr, src, Some(offset - scr.offset()))
        } else {
            Ok(vec![])
        }
    }

    pub fn read_reg(&mut self, scr: &SCReg, dst: &mut [u8], byte_off: Option<usize>) -> Result<Vec<Event>, context::Error> {
        let byte_off = byte_off.unwrap_or(0);
        let offset = scr.offset() + byte_off;
        let view = self.backing.view_bytes(offset, dst.len())
            .map_err(context::Error::from)?;
        let read_val = view.iter()
            .enumerate()
            .take(4)
            .fold(0u32, |val, (i, &byte)| {
                dst[i] = byte; // read val into dst during fold
                val | (byte << (i * 8)) as u32
            });
        match scr {
            SCReg::AIRCR => {
                let prigroup = todo!("get prigroup value");
                let aircr = AIRCR::new()
                    .with_vectkey_stat(0xFA05)
                    .with_endianness(0)
                    .with_prigroup(prigroup)
                    .with_sysresetreq(false)
                    .with_vectclractive(false)
                    .with_vectreset(false)
                    .into_bits();
                dst.copy_from_slice(&aircr.to_le_bytes());
                Ok(vec![])
            }
            SCReg::MPU(mpu_reg) => {
                mpu_reg.read_evt(read_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCReg::NVIC(nvic_reg) => {
                nvic_reg.read_evt(read_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCReg::SysTick(systick_reg) => {
                systick_reg.read_evt(read_val)
                    .map_err(Into::<context::Error>::into)
            }
            _ => { Ok(vec![]) }
        }
    }

    pub fn write_reg(&mut self, sc_reg: &SCReg, src: &[u8], byte_off: Option<usize>) -> Result<Vec<Event>, context::Error> {
        let byte_off = byte_off.unwrap_or(0);
        assert!(((src.len() + byte_off) <= 4), "access must be within a single word-aligned region");
        let offset = sc_reg.offset();
        let write_val = bytes_as_u32_le(src);
        match sc_reg {
            SCReg::ICSR => {
                let icsr = ICSR::from_bits(write_val);
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                view.copy_from_slice(src);
                Ok(icsr.write_evt())
            }
            SCReg::VTOR => {
                let write_val = write_val & 0xFFFFFF80;
                let vtor = VTOR::from(write_val);
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                view.copy_from_slice(&write_val.to_le_bytes());
                Ok(vtor.write_evt())
            }
            SCReg::AIRCR => {
                let aircr = AIRCR::from_bits(write_val);
                Ok(aircr.write_evt())
            }
            SCReg::SCR => {
                // don't want to create events for things that didn't change
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                let current_val = bytes_as_u32_le(view);
                let scr = SCR::from_bits(write_val);
                view.copy_from_slice(src);
                Ok(scr.write_evt(current_val))
            }
            SCReg::CCR => {
                // don't want to create events for things that didn't change
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                let current_val = bytes_as_u32_le(view);
                let ccr = CCR::from(write_val);
                // TODO: implement RAO/WI and RAZ/WI for relvant bits as necessary. this will need config information
                view.copy_from_slice(&write_val.to_le_bytes());
                Ok(ccr.write_evt(current_val))
            }
            SCReg::SHPR1(idx) | SCReg::SHPR2(idx) | SCReg::SHPR3(idx) => {
                // note that `.offset()` of shpr registers returns the word-aligned register offset,
                // but the access may be relative to a byte or halfword.
                // in the future i may need to consider this for other registers as well.
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                let mut evts = vec![];
                for (i, &byte) in src.iter().enumerate().take(4 - byte_off) {
                    if view[i] != byte {
                        view[i] = byte;
                        evts.push(Event::SetSystemHandlerPriority { id: *idx + i as u8, priority: byte });
                    }
                }
                Ok(evts)
            }
            SCReg::SHCSR => {
                todo!()
            }
            SCReg::CPACR => {
                unimplemented!("coprocessor access not supported")
            }
            SCReg::STIR => {
                todo!()
            }
            SCReg::MPU(mpu_reg) => {
                mpu_reg.write_evt(write_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCReg::NVIC(nvic_reg) => {
                nvic_reg.write_evt(write_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCReg::SysTick(systick_reg) => {
                systick_reg.write_evt(write_val)
                    .map_err(Into::<context::Error>::into)
            }
            _ => {
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                view.copy_from_slice(src);
                Ok(vec![])
            }
        }
    }
}

impl Default for SysCtrlSpace {
    fn default() -> Self {
        Self {
            backing: FixedState::new(0x1000),
        }
    }
}

/// config containing reset values for scs registers
#[derive(Debug)]
pub struct SysCtrlConfig {
    // todo
}

impl Default for SysCtrlConfig {
    fn default() -> Self {
        todo!("need to implement default scs values")
    }
}

/// system control register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SCReg {
    CPUID,      // cpuid base register
    ICSR,       // interrupt control and state register 
    VTOR,       // vector table offset register
    AIRCR,      // application interrupt and reset control register
    SCR,        // system control register
    CCR,        // configuration and control register
    SHPR1(u8),  // system handler priority register 1 (with handler number)
    SHPR2(u8),  // system handler priority register 2 (with handler number)
    SHPR3(u8),  // system handler priority register 3 (with handler number)
    SHCSR,      // system handler control and state register
    CFSR,       // configurable fault status register
    HFSR,       // hardfault status register
    DFSR,       // debug fault status register
    MMFAR,      // memmanage fault address register
    BFAR,       // busfault address register
    AFSR,       // auxiliary fault status register
    CPACR,      // coprocessor access control register
    
    FPCCR,      // floating point context control register
    FPCAR,      // floating point context address register
    FPDSCR,     // floating point default status control register
    MVFR0,      // media and fp feature register 0
    MVFR1,      // media and fp feature register 1
    MVFR2,      // media and fp feature register 2

    MCR,        // main control register, reserved
    ICTR,       // interrupt controller type register
    ACTLR,      // auxiliary control register
    STIR,       // software triggered interrupt register

    SysTick(SysTickReg),    // systick register
    NVIC(NVICReg),          // nvic register
    MPU(MPUReg),            // mpu register
    // todo: floating point extension scb registers
    // todo: cache and branch predictor maintenance

    // peripheral identification registers
    PID4,
    PID5,
    PID6,
    PID7,
    PID0,
    PID1,
    PID2,
    PID3,

    // component identification registers
    CID0,
    CID1,
    CID2,
    CID3,
}

#[derive(Debug, Clone)]
struct SCRegData {
    pub offset: usize,
    pub perms: u8,
    pub reset: Option<u32>,
}

impl SCReg {

    pub fn address(&self) -> Address {
        Address::from(0xe000e000_u64 + (self._data().offset as u64))
    }

    pub fn offset(&self) -> usize {
        self._data().offset
    }

    pub fn perms(&self) -> FlagSet<Permission> {
        unsafe { FlagSet::<Permission>::new_unchecked(self._data().perms) }
    }

    pub fn reset(&self) -> Option<u32> {
        self._data().reset
    }

    pub fn lookup_address(address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(0xe000e000)
            .expect("address is not in scs!");
        Self::lookup_offset(offset as usize)
    }

    pub fn lookup_offset(offset: usize) -> Option<Self> {
        assert!(offset < 0x1000, "address is not in scs!");
        match offset {
            0xd00_usize => { Some(SCReg::CPUID) }
            0xd04_usize => { Some(SCReg::ICSR) }
            0xd08_usize => { Some(SCReg::VTOR) }
            0xd0c_usize => { Some(SCReg::AIRCR) }
            0xd10_usize => { Some(SCReg::SCR) }
            0xd14_usize => { Some(SCReg::CCR) }
            0xd18_usize..=0xd1b => { Some(SCReg::SHPR1((offset - 0xd18 +  4) as u8)) }
            0xd1c_usize..=0xd1f => { Some(SCReg::SHPR2((offset - 0xd1c +  8) as u8)) }
            0xd20_usize..=0xd23 => { Some(SCReg::SHPR3((offset - 0xd20 + 12) as u8)) }
            0xd24_usize => { Some(SCReg::SHCSR) }
            0xd28_usize => { Some(SCReg::CFSR) }
            0xd2c_usize => { Some(SCReg::HFSR) }
            0xd30_usize => { Some(SCReg::DFSR) }
            0xd34_usize => { Some(SCReg::MMFAR) }
            0xd38_usize => { Some(SCReg::BFAR) }
            0xd3c_usize => { Some(SCReg::AFSR) }
            0xd88_usize => { Some(SCReg::CPACR) }
            
            0xf34_usize => { Some(SCReg::FPCCR) }
            0xf38_usize => { Some(SCReg::FPCAR) }
            0xf3c_usize => { Some(SCReg::FPDSCR) }
            0xf40_usize => { Some(SCReg::MVFR0) }
            0xf44_usize => { Some(SCReg::MVFR1) }
            0xf48_usize => { Some(SCReg::MVFR2) }

            0x000_usize => { Some(SCReg::MCR) }
            0x004_usize => { Some(SCReg::ICTR) }
            0x008_usize => { Some(SCReg::ACTLR) }
            0xf00_usize => { Some(SCReg::STIR) }
            0xfd0_usize => { Some(SCReg::PID4) }
            0xfd4_usize => { Some(SCReg::PID5) }
            0xfd8_usize => { Some(SCReg::PID6) }
            0xfdc_usize => { Some(SCReg::PID7) }
            0xfe0_usize => { Some(SCReg::PID0) }
            0xfe4_usize => { Some(SCReg::PID1) }
            0xfe8_usize => { Some(SCReg::PID2) }
            0xfec_usize => { Some(SCReg::PID3) }
            0xff0_usize => { Some(SCReg::CID0) }
            0xff4_usize => { Some(SCReg::CID1) }
            0xff8_usize => { Some(SCReg::CID2) }
            0xffc_usize => { Some(SCReg::CID3) }

            0x010 ..= 0x0ff => {
                SysTickReg::lookup_offset(offset)
                    .map(|systick_reg| SCReg::SysTick(systick_reg))
            }
            0x100 ..= 0xcff => {
                NVICReg::lookup_offset(offset)
                    .map(|nvic_reg| SCReg::NVIC(nvic_reg))
            }
            0xd90 ..= 0xdef => {
                MPUReg::lookup_offset(offset)
                    .map(|mpu_reg| SCReg::MPU(mpu_reg))
            }

            _ => { None }
        }
    }
}

impl SCReg {
    fn _data(&self) -> &SCRegData {
        match self {
            SCReg::CPUID    => { &SCRegData { offset: 0xd00_usize, perms: 0b100, reset: None } }
            SCReg::ICSR     => { &SCRegData { offset: 0xd04_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::VTOR     => { &SCRegData { offset: 0xd08_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::AIRCR    => { &SCRegData { offset: 0xd0c_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SCR      => { &SCRegData { offset: 0xd10_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::CCR      => { &SCRegData { offset: 0xd14_usize, perms: 0b110, reset: None } }
            SCReg::SHPR1(_) => { &SCRegData { offset: 0xd18_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SHPR2(_) => { &SCRegData { offset: 0xd1c_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SHPR3(_) => { &SCRegData { offset: 0xd20_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SHCSR    => { &SCRegData { offset: 0xd24_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::CFSR     => { &SCRegData { offset: 0xd28_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::HFSR     => { &SCRegData { offset: 0xd2c_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::DFSR     => { &SCRegData { offset: 0xd30_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::MMFAR    => { &SCRegData { offset: 0xd34_usize, perms: 0b110, reset: None } }
            SCReg::BFAR     => { &SCRegData { offset: 0xd38_usize, perms: 0b110, reset: None } }
            SCReg::AFSR     => { &SCRegData { offset: 0xd3c_usize, perms: 0b110, reset: None } }
            SCReg::CPACR    => { &SCRegData { offset: 0xd88_usize, perms: 0b110, reset: None } }
            
            // SCReg::FPCCR     => { &SCRegData { offset: 0xf34_usize, perms: 0b110, reset: Some(0x0) } }
            // SCReg::FPCAR     => { &SCRegData { offset: 0xf38_usize, perms: 0b110, reset: None } }
            // SCReg::FPDSCR    => { &SCRegData { offset: 0xf3c_usize, perms: 0b110, reset: Some(0x0) } }
            // SCReg::MVFR0     => { &SCRegData { offset: 0xf40_usize, perms: 0b100, reset: Some(0x0) } }
            // SCReg::MVFR1     => { &SCRegData { offset: 0xf44_usize, perms: 0b100, reset: Some(0x0) } }
            // SCReg::MVFR2     => { &SCRegData { offset: 0xf48_usize, perms: 0b100, reset: Some(0x0) } }
            
            SCReg::MCR      => { &SCRegData { offset: 0x000_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::ICTR     => { &SCRegData { offset: 0x004_usize, perms: 0b100, reset: None } }
            SCReg::ACTLR    => { &SCRegData { offset: 0x008_usize, perms: 0b110, reset: None } }
            SCReg::STIR     => { &SCRegData { offset: 0xf00_usize, perms: 0b010, reset: None } }
            SCReg::PID4     => { &SCRegData { offset: 0xfd0_usize, perms: 0b100, reset: None } }
            SCReg::PID5     => { &SCRegData { offset: 0xfd4_usize, perms: 0b100, reset: None } }
            SCReg::PID6     => { &SCRegData { offset: 0xfd8_usize, perms: 0b100, reset: None } }
            SCReg::PID7     => { &SCRegData { offset: 0xfdc_usize, perms: 0b100, reset: None } }
            SCReg::PID0     => { &SCRegData { offset: 0xfe0_usize, perms: 0b100, reset: None } }
            SCReg::PID1     => { &SCRegData { offset: 0xfe4_usize, perms: 0b100, reset: None } }
            SCReg::PID2     => { &SCRegData { offset: 0xfe8_usize, perms: 0b100, reset: None } }
            SCReg::PID3     => { &SCRegData { offset: 0xfec_usize, perms: 0b100, reset: None } }
            SCReg::CID0     => { &SCRegData { offset: 0xff0_usize, perms: 0b100, reset: None } }
            SCReg::CID1     => { &SCRegData { offset: 0xff4_usize, perms: 0b100, reset: None } }
            SCReg::CID2     => { &SCRegData { offset: 0xff8_usize, perms: 0b100, reset: None } }
            SCReg::CID3     => { &SCRegData { offset: 0xffc_usize, perms: 0b100, reset: None } }

            screg => { panic!("data for {screg:?} not implemented!") }
        }
    }
}


#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CPUID {
    #[bits(4)]
    pub revision: u32,
    #[bits(12)]
    pub partno: u32,
    #[bits(4, default = 0xF)]
    pub architecture: u32,
    #[bits(4)]
    pub variant: u32,
    #[bits(8, default = 0x41)]
    pub implementer: u32,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ICSR {
    #[bits(9)]
    pub vectactive: u32,
    #[bits(2)]
    __: u32,
    #[bits(1)]
    pub rettobase: bool,
    #[bits(9)]
    pub vectpending: u32,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub isrpending: bool,
    #[bits(1)]
    pub isrpreempt: bool,
    __: bool,
    #[bits(1)]
    pub pendstclr: bool,
    #[bits(1)]
    pub pendstset: bool,
    #[bits(1)]
    pub pendsvclr: bool,
    #[bits(1)]
    pub pendsvset: bool,
    #[bits(2)]
    __: u32,
    #[bits(1)]
    pub nmipendset: bool,
}

impl ICSR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        if self.pendstclr() {
            evts.push(Event::ExceptionSetPending(ExceptionType::SysTick, false));
        }
        if self.pendstset() {
            evts.push(Event::ExceptionSetPending(ExceptionType::SysTick, true));
        }
        if self.pendsvclr() {
            evts.push(Event::ExceptionSetPending(ExceptionType::PendSV, false));
        }
        if self.pendsvset() {
            // writing 1 should be a way of requesting context switch
            evts.push(Event::ExceptionSetPending(ExceptionType::PendSV, true));
        }
        if self.nmipendset() {
            evts.push(Event::ExceptionSetActive(ExceptionType::NMI, self.nmipendset()));
        }
        evts
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct VTOR {
    #[bits(7, default = 0x0)]
    __: u32,
    #[bits(25)]
    pub tbloff: u32,
}

impl VTOR {
    pub fn write_evt(&self) -> Vec<Event> {
        vec![Event::VectorTableOffsetWrite(self.tbloff() << 7)]
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AIRCR {
    #[bits(1)]
    pub vectreset: bool,
    #[bits(1)]
    pub vectclractive: bool,
    #[bits(1, default = false)]
    pub sysresetreq: bool,
    #[bits(5)]
    __: u32,
    #[bits(3, default = 0)]
    pub prigroup: u32,
    #[bits(4)]
    __: u32,
    #[bits(1, default = 0)]
    pub endianness: u8,
    #[bits(16)]
    pub vectkey_stat: u32,
}

impl AIRCR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        if self.vectreset() {
            evts.push(Event::LocalSysResetRequest);
        }
        if self.vectclractive() {
            evts.push(Event::ExceptionClrAllActive);
        }
        if self.sysresetreq() {
            evts.push(Event::ExternSysResetRequest);
        }
        if self.vectkey_stat() == 0x05fa {
            evts.push(Event::VectorKeyWrite);
        }
        evts
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCR {
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub sleeponexit: bool,
    #[bits(1)]
    pub sleepdeep: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub sevonpend: bool,
    #[bits(27)]
    __: u32,
}

impl SCR {
    pub fn write_evt(&self, current_val: u32) -> Vec<Event> {
        let mut evts = vec![];
        let changed = Self::from(self.into_bits() ^ current_val);
        if changed.sleeponexit() {
            evts.push(Event::SetSleepOnExit(self.sleeponexit()));
        }
        if changed.sleepdeep() {
            evts.push(Event::SetDeepSleep(self.sleepdeep()));
        }
        if changed.sevonpend() {
            evts.push(Event::SetTransitionWakupEvent(self.sevonpend()));
        }
        evts
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CCR {
    #[bits(1)]
    pub nonbasethrdena: bool,
    #[bits(1)]
    pub usersetmpend: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub unalign_trp: bool,
    #[bits(1)]
    pub div_0_trp: bool,
    #[bits(3)]
    __: u32,
    #[bits(1)]
    pub bfhfnmign: bool,
    #[bits(1)]
    pub stkalign: bool,
    #[bits(6)]
    __: u32,
    #[bits(1)]
    pub dc: bool,
    #[bits(1)]
    pub ic: bool,
    #[bits(1)]
    pub bp: bool,
    #[bits(13)]
    __: u32,
}

impl CCR {
    pub fn write_evt(&self, current_val: u32) -> Vec<Event> {
        let mut evts = vec![];
        let changed = Self::from(self.into_bits() ^ current_val);
        if changed.nonbasethrdena() {
            evts.push(Event::ThreadModeExceptionsEnabled(self.nonbasethrdena()));
        }
        if changed.usersetmpend() {
            evts.push(Event::STIRUnprivilegedAccessAllowed(self.usersetmpend()));
        }
        if changed.unalign_trp() {
            evts.push(Event::UnalignedAccessTrapEnabled(self.unalign_trp()));
        }
        if changed.div_0_trp() {
            evts.push(Event::DivideByZeroTrapEnabled(self.div_0_trp()));
        }
        if changed.bfhfnmign() {
            evts.push(Event::PreciseDataAccessFaultIgnored(self.bfhfnmign()));
        }
        if changed.stkalign() {
            evts.push(Event::Stack8ByteAligned(self.stkalign()));
        }
        if changed.dc() {
            evts.push(Event::DataCacheEnabled(self.dc()));
        }
        if changed.ic() {
            evts.push(Event::InsnCacheEnabled(self.ic()));
        }
        if changed.bp() {
            evts.push(Event::BranchPredictionEnabled(self.bp()));
        }
        evts
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SHPR1 {
    #[bits(8)]
    pub pri_4: u8,
    #[bits(8)]
    pub pri_5: u8,
    #[bits(8)]
    pub pri_6: u8,
    #[bits(8)]
    pub pri_7: u8,
}

impl SHPR1 {
    #[inline]
    pub fn base() -> usize { 0xd18_usize }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SHPR2 {
    #[bits(8)]
    pub pri_8: u8,
    #[bits(8)]
    pub pri_9: u8,
    #[bits(8)]
    pub pri_10: u8,
    #[bits(8)]
    pub pri_11: u8,
}

impl SHPR2 {
    #[inline]
    pub fn base() -> usize { 0xd1c_usize }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SHPR3 {
    #[bits(8)]
    pub pri_12: u8,
    #[bits(8)]
    pub pri_13: u8,
    #[bits(8)]
    pub pri_14: u8,
    #[bits(8)]
    pub pri_15: u8,
}

impl SHPR3 {
    #[inline]
    pub fn base() -> usize { 0xd20_usize }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SHCSR {
    #[bits(1)]
    pub memfaultact: bool,
    #[bits(1)]
    pub busfaultact: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub usgfaultact: bool,
    #[bits(3)]
    __: u32,
    #[bits(1)]
    pub svcallact: bool,
    #[bits(1)]
    pub monitoract: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub pendsvact: bool,
    #[bits(1)]
    pub systickact: bool,
    #[bits(1)]
    pub usgfaultpended: bool,
    #[bits(1)]
    pub memfaultpended: bool,
    #[bits(1)]
    pub busfaultpended: bool,
    #[bits(1)]
    pub svcallpended: bool,
    #[bits(1)]
    pub memfaultena: bool,
    #[bits(1)]
    pub busfaultena: bool,
    #[bits(1)]
    pub usgfaultena: bool,
    #[bits(13)]
    __: u32,
}

impl SHCSR {
    pub fn write_evt(&self, current_val: u32) -> Vec<Event> {
        let mut evts = vec![];
        let changed = Self::from_bits(current_val);
        if changed.memfaultact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::MemManage, self.memfaultact()));
        }
        if changed.busfaultact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::BusFault, self.busfaultact()));
        }
        if changed.usgfaultact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::UsageFault, self.usgfaultact()));
        }
        if changed.svcallact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::SVCall, self.svcallact()));
        }
        if changed.monitoract() {
            evts.push(Event::ExceptionSetActive(ExceptionType::DebugMonitor, self.monitoract()));
        }
        if changed.pendsvact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::PendSV, self.pendsvact()));
        }
        if changed.systickact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::SysTick, self.systickact()));
        }
        if changed.usgfaultpended() {
            evts.push(Event::ExceptionSetPending(ExceptionType::UsageFault, self.usgfaultpended()));
        }
        if changed.memfaultpended() {
            evts.push(Event::ExceptionSetPending(ExceptionType::MemManage, self.memfaultpended()));
        }
        if changed.busfaultpended() {
            evts.push(Event::ExceptionSetPending(ExceptionType::BusFault, self.busfaultpended()));
        }
        if changed.svcallpended() {
            evts.push(Event::ExceptionSetPending(ExceptionType::SVCall, self.svcallpended()));
        }
        if changed.memfaultena() {
            evts.push(Event::ExceptionEnabled(ExceptionType::MemManage, self.memfaultena()));
        }
        if changed.busfaultena() {
            evts.push(Event::ExceptionEnabled(ExceptionType::BusFault, self.busfaultena()));
        }
        if changed.usgfaultena() {
            evts.push(Event::ExceptionEnabled(ExceptionType::UsageFault, self.usgfaultena()));
        }
        evts
    }
}

#[bitfield(u32)]
pub struct CFSR {
    #[bits(8)]
    pub memmanage: MMFSR,
    #[bits(8)]
    pub busfault: BFSR,
    #[bits(16)]
    pub usagefault: UFSR,
}

/// UsageFault status register
#[bitfield(u16)]
pub struct UFSR {
    #[bits(1)]
    pub undefinstr: bool,
    #[bits(1)]
    pub invstate: bool,
    #[bits(1)]
    pub invpc: bool,
    #[bits(1)]
    pub nocp: bool,
    #[bits(4)]
    __: u32,
    #[bits(1)]
    pub unaligned: bool,
    #[bits(1)]
    pub divbyzero: bool,
    #[bits(6)]
    __: u32,
}

/// BusFault status register
#[bitfield(u8)]
pub struct BFSR {
    #[bits(1)]
    pub ibuserr: bool,
    #[bits(1)]
    pub preciserr: bool,
    #[bits(1)]
    pub impreciserr: bool,
    #[bits(1)]
    pub unstkerr: bool,
    #[bits(1)]
    pub stkerr: bool,
    #[bits(1)]
    pub lsperr: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub bfarvalid: bool,
}

/// MemManage fault status register
#[bitfield(u8)]
pub struct MMFSR {
    #[bits(1)]
    pub iaccviol: bool,
    #[bits(1)]
    pub daccviol: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub munstkerr: bool,
    #[bits(1)]
    pub mstkerr: bool,
    #[bits(1)]
    pub mlsperr: bool,
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub mmarvalid: bool,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct HFSR {
    #[bits(1)]
    __: bool,
    #[bits(1)]
    pub vecttbl: bool,
    #[bits(28)]
    __: u32,
    #[bits(1)]
    pub forced: bool,
    #[bits(1)]
    pub debugevt: bool,
}

// writing 1 clears bit to 0.
// read halted bit by instruction during stepping returns unknown 
/// C1.6.1 debug fault status register
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DFSR {
    #[bits(1)]
    pub halted: bool,
    #[bits(1)]
    pub bkpt: bool,
    #[bits(1)]
    pub dwttrap: bool,
    #[bits(1)]
    pub vcatch: bool,
    #[bits(1)]
    pub external: bool,
    #[bits(27)]
    __: u32,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct MMFAR {
    #[bits(32)]
    pub address: u32,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BFAR {
    #[bits(32)]
    pub address: u32,
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CPACR {
    #[bits(2)]
    pub cp0: CPACRAccess,
    #[bits(2)]
    pub cp1: CPACRAccess,
    #[bits(2)]
    pub cp2: CPACRAccess,
    #[bits(2)]
    pub cp3: CPACRAccess,
    #[bits(2)]
    pub cp4: CPACRAccess,
    #[bits(2)]
    pub cp5: CPACRAccess,
    #[bits(2)]
    pub cp6: CPACRAccess,
    #[bits(2)]
    pub cp7: CPACRAccess,
    #[bits(4)]
    __: u8,
    #[bits(2)]
    pub cp10: CPACRAccess,
    #[bits(2)]
    pub cp11: CPACRAccess,
    #[bits(8)]
    __: u8,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CPACRAccess {
    Denied = 0b00,
    Privileged = 0b01,
    Full = 0b11,
}

impl CPACRAccess {
    const fn into_bits(self) -> u8 {
        self as _
    }
    const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => Self::Denied,
            0b01 => Self::Privileged,
            0b11 => Self::Full,
            _ => { panic!("invalid access bits!") }
        }
    }
}

#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ICTR {
    #[bits(4)]
    pub intlinesnum: usize,
    #[bits(28)]
    __: u32,
}

impl ICTR {
    pub fn num_int_lines(&self) -> usize {
        (self.intlinesnum() + 1) * 32
    }
}

/// software triggered interrupt
/// 
/// same efect as setting NVIC IPSR interrupt bit to 1
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct STIR {
    #[bits(9)]
    pub intid: u32,
    #[bits(23)]
    __: u32,
}

impl STIR {
    pub fn exception_number(&self) -> usize {
        (self.intid() + 16) as usize
    }
}