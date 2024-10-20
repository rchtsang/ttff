//! scs.rs

/*
 * TODO:
 * - implement endian sensitivity
 * - replace struct/int/byte conversions with unsafe std::mem::transmute for performance
 */

use bitfield_struct::bitfield;
use unwrap_enum::{EnumAs, EnumIs};

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
        if let Some(scr) = SCRegType::lookup_offset(offset) {
            self.read_reg(&scr, dst, Some(offset - scr.offset()))
        } else {
            Ok(vec![])
        }
    }

    pub fn write_bytes(&mut self, offset: impl Into<usize>, src: &[u8]) -> Result<Vec<Event>, context::Error> {
        let offset = offset.into();
        if let Some(scr) = SCRegType::lookup_offset(offset) {
            self.write_reg(&scr, src, Some(offset - scr.offset()))
        } else {
            Ok(vec![])
        }
    }

    pub fn get_reg(&self, scr: &SCRegType) -> Result<SCReg, context::Error> {
        match scr {
            _ => unimplemented!()
        }
    }

    pub fn read_reg(&mut self, scr: &SCRegType, dst: &mut [u8], byte_off: Option<usize>) -> Result<Vec<Event>, context::Error> {
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
            SCRegType::AIRCR => {
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
            SCRegType::VTOR => {
                todo!()
            }
            SCRegType::MPU(mpu_reg) => {
                mpu_reg.read_evt(read_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCRegType::NVIC(nvic_reg) => {
                nvic_reg.read_evt(read_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCRegType::SysTick(systick_reg) => {
                systick_reg.read_evt(read_val)
                    .map_err(Into::<context::Error>::into)
            }
            _ => { Ok(vec![]) }
        }
    }

    pub fn write_reg(&mut self, sc_reg: &SCRegType, src: &[u8], byte_off: Option<usize>) -> Result<Vec<Event>, context::Error> {
        let byte_off = byte_off.unwrap_or(0);
        assert!((src.len() != 3) && (src.len() <= 4), "access must be byte, half-word, or word aligned");
        assert!(((src.len() + byte_off) <= 4), "access must be within a single word-aligned region");
        let offset = sc_reg.offset();
        let write_bytes = &mut [0u8; 4];
        write_bytes[byte_off..].copy_from_slice(src);
        let write_val = bytes_as_u32_le(write_bytes);
        match sc_reg {
            SCRegType::ICSR => {
                let icsr = ICSR::from_bits(write_val);
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                view.copy_from_slice(src);
                Ok(icsr.write_evt())
            }
            SCRegType::VTOR => {
                let write_val = write_val & 0xFFFFFF80;
                let vtor = VTOR::from(write_val);
                Ok(vtor.write_evt())
            }
            SCRegType::AIRCR => {
                // let current_val = self.nvic.priority_grouping;
                let current_val = todo!();
                let aircr = AIRCR::from_bits(write_val);
                Ok(aircr.write_evt(current_val))
            }
            SCRegType::SCR => {
                // don't want to create events for things that didn't change
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                let current_val = bytes_as_u32_le(view);
                let scr = SCR::from_bits(write_val);
                view.copy_from_slice(src);
                Ok(scr.write_evt(current_val))
            }
            SCRegType::CCR => {
                // don't want to create events for things that didn't change
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                let current_val = bytes_as_u32_le(view);
                let ccr = CCR::from(write_val);
                // TODO: implement RAO/WI and RAZ/WI for relvant bits as necessary. this will need config information
                view.copy_from_slice(&write_val.to_le_bytes());
                Ok(ccr.write_evt(current_val))
            }
            SCRegType::SHPR1(idx) | SCRegType::SHPR2(idx) | SCRegType::SHPR3(idx) => {
                // note that `.offset()` of shpr registers returns the word-aligned register offset,
                // but the access may be relative to a byte or halfword.
                // in the future i may need to consider this for other registers as well.
                let view = self.backing.view_bytes_mut(offset, src.len())
                    .map_err(context::Error::from)?;
                let mut evts = vec![];
                for (i, &byte) in src.iter().enumerate().take(4 - byte_off) {
                    let i = i + byte_off;
                    if view[i] != byte {
                        view[i] = byte;
                        evts.push(Event::SetSystemHandlerPriority { id: *idx + i as u8, priority: byte });
                    }
                }
                Ok(evts)
            }
            SCRegType::CFSR => {
                let cfsr = CFSR::from_bits(write_val);
                Ok(cfsr.write_evt())
            }
            SCRegType::SHCSR => {
                let hfsr = HFSR::from_bits(write_val);
                Ok(hfsr.write_evt())
            }
            SCRegType::CPACR => {
                unimplemented!("coprocessor access not supported")
            }
            SCRegType::STIR => {
                let stir = STIR::from_bits(write_val);
                Ok(stir.write_evt())
            }
            SCRegType::MPU(mpu_reg) => {
                mpu_reg.write_evt(write_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCRegType::NVIC(nvic_reg) => {
                nvic_reg.write_evt(write_val)
                    .map_err(Into::<context::Error>::into)
            }
            SCRegType::SysTick(systick_reg) => {
                systick_reg.write_evt(write_val)
                    .map_err(Into::<context::Error>::into)
            }
            _ => {
                // TODO: add logging to print out warning that some registers aren't implemented
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
pub enum SCRegType {
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
    CFSR,       // configurable fault status register (with offset to bus subregisters)
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
struct SCRegTypeData {
    pub offset: usize,
    pub perms: u8,
    pub reset: Option<u32>,
}

impl SCRegType {

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
            0xd00_usize => { Some(SCRegType::CPUID) }
            0xd04_usize => { Some(SCRegType::ICSR) }
            0xd08_usize => { Some(SCRegType::VTOR) }
            0xd0c_usize => { Some(SCRegType::AIRCR) }
            0xd10_usize => { Some(SCRegType::SCR) }
            0xd14_usize => { Some(SCRegType::CCR) }
            0xd18_usize..=0xd1b => { Some(SCRegType::SHPR1((offset - 0xd18 +  4) as u8)) }
            0xd1c_usize..=0xd1f => { Some(SCRegType::SHPR2((offset - 0xd1c +  8) as u8)) }
            0xd20_usize..=0xd23 => { Some(SCRegType::SHPR3((offset - 0xd20 + 12) as u8)) }
            0xd24_usize => { Some(SCRegType::SHCSR) }
            0xd28_usize => { Some(SCRegType::CFSR) }
            0xd2c_usize => { Some(SCRegType::HFSR) }
            0xd30_usize => { Some(SCRegType::DFSR) }
            0xd34_usize => { Some(SCRegType::MMFAR) }
            0xd38_usize => { Some(SCRegType::BFAR) }
            0xd3c_usize => { Some(SCRegType::AFSR) }
            0xd88_usize => { Some(SCRegType::CPACR) }
            
            0xf34_usize => { Some(SCRegType::FPCCR) }
            0xf38_usize => { Some(SCRegType::FPCAR) }
            0xf3c_usize => { Some(SCRegType::FPDSCR) }
            0xf40_usize => { Some(SCRegType::MVFR0) }
            0xf44_usize => { Some(SCRegType::MVFR1) }
            0xf48_usize => { Some(SCRegType::MVFR2) }

            0x000_usize => { Some(SCRegType::MCR) }
            0x004_usize => { Some(SCRegType::ICTR) }
            0x008_usize => { Some(SCRegType::ACTLR) }
            0xf00_usize => { Some(SCRegType::STIR) }
            0xfd0_usize => { Some(SCRegType::PID4) }
            0xfd4_usize => { Some(SCRegType::PID5) }
            0xfd8_usize => { Some(SCRegType::PID6) }
            0xfdc_usize => { Some(SCRegType::PID7) }
            0xfe0_usize => { Some(SCRegType::PID0) }
            0xfe4_usize => { Some(SCRegType::PID1) }
            0xfe8_usize => { Some(SCRegType::PID2) }
            0xfec_usize => { Some(SCRegType::PID3) }
            0xff0_usize => { Some(SCRegType::CID0) }
            0xff4_usize => { Some(SCRegType::CID1) }
            0xff8_usize => { Some(SCRegType::CID2) }
            0xffc_usize => { Some(SCRegType::CID3) }

            0x010 ..= 0x0ff => {
                SysTickReg::lookup_offset(offset)
                    .map(|systick_reg| SCRegType::SysTick(systick_reg))
            }
            0x100 ..= 0xcff => {
                NVICReg::lookup_offset(offset)
                    .map(|nvic_reg| SCRegType::NVIC(nvic_reg))
            }
            0xd90 ..= 0xdef => {
                MPUReg::lookup_offset(offset)
                    .map(|mpu_reg| SCRegType::MPU(mpu_reg))
            }

            _ => { None }
        }
    }
}

impl SCRegType {
    fn _data(&self) -> &SCRegTypeData {
        match self {
            SCRegType::CPUID    => { &SCRegTypeData { offset: 0xd00_usize, perms: 0b100, reset: None } }
            SCRegType::ICSR     => { &SCRegTypeData { offset: 0xd04_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::VTOR     => { &SCRegTypeData { offset: 0xd08_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::AIRCR    => { &SCRegTypeData { offset: 0xd0c_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::SCR      => { &SCRegTypeData { offset: 0xd10_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::CCR      => { &SCRegTypeData { offset: 0xd14_usize, perms: 0b110, reset: None } }
            SCRegType::SHPR1(_) => { &SCRegTypeData { offset: 0xd18_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::SHPR2(_) => { &SCRegTypeData { offset: 0xd1c_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::SHPR3(_) => { &SCRegTypeData { offset: 0xd20_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::SHCSR    => { &SCRegTypeData { offset: 0xd24_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::CFSR     => { &SCRegTypeData { offset: 0xd28_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::HFSR     => { &SCRegTypeData { offset: 0xd2c_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::DFSR     => { &SCRegTypeData { offset: 0xd30_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::MMFAR    => { &SCRegTypeData { offset: 0xd34_usize, perms: 0b110, reset: None } }
            SCRegType::BFAR     => { &SCRegTypeData { offset: 0xd38_usize, perms: 0b110, reset: None } }
            SCRegType::AFSR     => { &SCRegTypeData { offset: 0xd3c_usize, perms: 0b110, reset: None } }
            SCRegType::CPACR    => { &SCRegTypeData { offset: 0xd88_usize, perms: 0b110, reset: None } }
            
            // SCRegType::FPCCR     => { &SCRegTypeData { offset: 0xf34_usize, perms: 0b110, reset: Some(0x0) } }
            // SCRegType::FPCAR     => { &SCRegTypeData { offset: 0xf38_usize, perms: 0b110, reset: None } }
            // SCRegType::FPDSCR    => { &SCRegTypeData { offset: 0xf3c_usize, perms: 0b110, reset: Some(0x0) } }
            // SCRegType::MVFR0     => { &SCRegTypeData { offset: 0xf40_usize, perms: 0b100, reset: Some(0x0) } }
            // SCRegType::MVFR1     => { &SCRegTypeData { offset: 0xf44_usize, perms: 0b100, reset: Some(0x0) } }
            // SCRegType::MVFR2     => { &SCRegTypeData { offset: 0xf48_usize, perms: 0b100, reset: Some(0x0) } }
            
            SCRegType::MCR      => { &SCRegTypeData { offset: 0x000_usize, perms: 0b110, reset: Some(0x0) } }
            SCRegType::ICTR     => { &SCRegTypeData { offset: 0x004_usize, perms: 0b100, reset: None } }
            SCRegType::ACTLR    => { &SCRegTypeData { offset: 0x008_usize, perms: 0b110, reset: None } }
            SCRegType::STIR     => { &SCRegTypeData { offset: 0xf00_usize, perms: 0b010, reset: None } }
            SCRegType::PID4     => { &SCRegTypeData { offset: 0xfd0_usize, perms: 0b100, reset: None } }
            SCRegType::PID5     => { &SCRegTypeData { offset: 0xfd4_usize, perms: 0b100, reset: None } }
            SCRegType::PID6     => { &SCRegTypeData { offset: 0xfd8_usize, perms: 0b100, reset: None } }
            SCRegType::PID7     => { &SCRegTypeData { offset: 0xfdc_usize, perms: 0b100, reset: None } }
            SCRegType::PID0     => { &SCRegTypeData { offset: 0xfe0_usize, perms: 0b100, reset: None } }
            SCRegType::PID1     => { &SCRegTypeData { offset: 0xfe4_usize, perms: 0b100, reset: None } }
            SCRegType::PID2     => { &SCRegTypeData { offset: 0xfe8_usize, perms: 0b100, reset: None } }
            SCRegType::PID3     => { &SCRegTypeData { offset: 0xfec_usize, perms: 0b100, reset: None } }
            SCRegType::CID0     => { &SCRegTypeData { offset: 0xff0_usize, perms: 0b100, reset: None } }
            SCRegType::CID1     => { &SCRegTypeData { offset: 0xff4_usize, perms: 0b100, reset: None } }
            SCRegType::CID2     => { &SCRegTypeData { offset: 0xff8_usize, perms: 0b100, reset: None } }
            SCRegType::CID3     => { &SCRegTypeData { offset: 0xffc_usize, perms: 0b100, reset: None } }

            sc_reg => { panic!("data for {sc_reg:?} not implemented!") }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, EnumAs, EnumIs)]
pub enum SCReg {
    CPUID(CPUID),
    ICSR(ICSR),
    VTOR(VTOR),
    AIRCR(AIRCR),
    SCR(SCR),
    CCR(CCR),
    SHPR1(SHPR1),
    SHPR2(SHPR2),
    SHPR3(SHPR3),
    SHCSR(SHCSR),
    CFSR(CFSR),
    HFSR(HFSR),
    DFSR(DFSR),
    MMFAR(MMFAR),
    BFAR(BFAR),
    // AFSR(AFSR),
    CPACR(CPACR),
    
    // FPCCR(FPCCR),
    // FPCAR(FPCAR),
    // FPDSCR(FPDSCR),
    // MVFR0(MVFR0),
    // MVFR1(MVFR1),
    // MVFR2(MVFR2),

    // MCR(MCR),
    ICTR(ICTR),
    // ACTLR(ACTLR),
    STIR(STIR),

    SysTick(SysTickReg),
    NVIC(NVICReg),
    MPU(MPUReg),
    // todo: floating point extension scb registers
    // todo: cache and branch predictor maintenance
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
    pub fn write_evt(&self, current_val: u32) -> Vec<Event> {
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
        let current = Self::from_bits(current_val);
        if self.prigroup() != current.prigroup() {
            evts.push(Event::SetPriorityGrouping(self.prigroup() as u8));
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
        let changed = Self::from_bits(self.into_bits() ^ current_val);
        if changed.memfaultact() {
            evts.push(Event::ExceptionSetActive(ExceptionType::MemFault, self.memfaultact()));
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
            evts.push(Event::ExceptionSetPending(ExceptionType::MemFault, self.memfaultpended()));
        }
        if changed.busfaultpended() {
            evts.push(Event::ExceptionSetPending(ExceptionType::BusFault, self.busfaultpended()));
        }
        if changed.svcallpended() {
            evts.push(Event::ExceptionSetPending(ExceptionType::SVCall, self.svcallpended()));
        }
        if changed.memfaultena() {
            evts.push(Event::ExceptionEnabled(ExceptionType::MemFault, self.memfaultena()));
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
#[derive(PartialEq, Eq)]
pub struct CFSR {
    #[bits(8)]
    pub memmanage: MMFSR,
    #[bits(8)]
    pub busfault: BFSR,
    #[bits(16)]
    pub usagefault: UFSR,
}

impl CFSR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        evts.extend(self.memmanage().write_evt());
        evts.extend(self.busfault().write_evt());
        evts.extend(self.usagefault().write_evt());
        evts
    }
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

impl UFSR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        if self.undefinstr() {
            evts.push(Event::FaultStatusClr(UsgFault::UndefinedInsn.into()));
        }
        if self.invstate() {
            evts.push(Event::FaultStatusClr(UsgFault::InvalidState.into()));
        }
        if self.invpc() {
            evts.push(Event::FaultStatusClr(UsgFault::IntegrityCheck.into()));
        }
        if self.nocp() {
            evts.push(Event::FaultStatusClr(UsgFault::CoprocessorAccess.into()));
        }
        if self.unaligned() {
            evts.push(Event::FaultStatusClr(UsgFault::UnalignedAccess.into()));
        }
        if self.divbyzero() {
            evts.push(Event::FaultStatusClr(UsgFault::DivideByZero.into()));
        }
        evts
    }
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

impl BFSR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        if self.ibuserr() {
            evts.push(Event::FaultStatusClr(BusFault::InsnPrefetch.into()));
        }
        if self.preciserr() {
            evts.push(Event::FaultStatusClr(BusFault::PreciseDataAccess.into()));
        }
        if self.impreciserr() {
            evts.push(Event::FaultStatusClr(BusFault::ImprciseDataAccess.into()));
        }
        if self.unstkerr() {
            evts.push(Event::FaultStatusClr(BusFault::OnExceptionReturn.into()));
        }
        if self.stkerr() {
            evts.push(Event::FaultStatusClr(BusFault::OnExceptionEntry.into()));
        }
        if self.lsperr() {
            evts.push(Event::FaultStatusClr(BusFault::LazyStatePreservation.into()));
        }
        if self.bfarvalid() {
            unimplemented!("what happens on write to bfarvalid?");
        }
        evts
    }
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

impl MMFSR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        if self.iaccviol() {
            evts.push(Event::FaultStatusClr(MemFault::InsnAccessViolation.into()));
        }
        if self.daccviol() {
            evts.push(Event::FaultStatusClr(MemFault::DataAccessViolation.into()));
        }
        if self.munstkerr() {
            evts.push(Event::FaultStatusClr(MemFault::OnExceptionReturn.into()));
        }
        if self.mstkerr() {
            evts.push(Event::FaultStatusClr(MemFault::OnExceptionEntry.into()));
        }
        if self.mlsperr() {
            evts.push(Event::FaultStatusClr(MemFault::LazyStatePreservation.into()));
        }
        if self.mmarvalid() {
            unimplemented!("what happens on write to mmarvalid?");
        }
        evts
    }
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

impl HFSR {
    pub fn write_evt(&self) -> Vec<Event> {
        let mut evts = vec![];
        if self.vecttbl() {
            evts.push(Event::FaultStatusClr(HardFault::VectorTableRead.into()));
        }
        if self.forced() {
            evts.push(Event::FaultStatusClr(HardFault::EscalatedException.into()));
        }
        if self.debugevt() {
            evts.push(Event::FaultStatusClr(HardFault::DebugEvent.into()));
        }
        evts
    }
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
    pub fn exception_number(&self) -> u32 {
        (self.intid() + 16)
    }

    pub fn write_evt(&self) -> Vec<Event> {
        vec![Event::ExceptionSetActive(ExceptionType::ExternalInterrupt(self.exception_number()), true)]
    }
}