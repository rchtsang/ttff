//! regs.rs
//! 
//! system control registers
use derive_more::{Into, From, TryFrom, TryInto};

use super::*;

/// system control register enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SCRegType {
    /// cpuid base register
    CPUID,
    /// interrupt control and state register 
    ICSR,
    /// vector table offset register
    VTOR,
    /// application interrupt and reset control register
    AIRCR,
    /// system control register
    SCR,
    /// configuration and control register
    CCR,
    /// system handler priority register 1 (with handler number)
    SHPR1(u8),
    /// system handler priority register 2 (with handler number)
    SHPR2(u8),
    /// system handler priority register 3 (with handler number)
    SHPR3(u8),
    /// system handler control and state register
    SHCSR,
    /// configurable fault status register (with offset to bus subregisters)
    CFSR,
    /// hardfault status register
    HFSR,
    /// debug fault status register
    DFSR,
    /// memmanage fault address register
    MMFAR,
    /// busfault address register
    BFAR,
    /// auxiliary fault status register
    AFSR,
    /// coprocessor access control register
    CPACR,
    
    /// floating point context control register
    FPCCR,
    /// floating point context address register
    FPCAR,
    /// floating point default status control register
    FPDSCR,
    /// media and fp feature register 0
    MVFR0,
    /// media and fp feature register 1
    MVFR1,
    /// media and fp feature register 2
    MVFR2,

    /// main control register, reserved
    MCR,
    /// interrupt controller type register
    ICTR,
    /// auxiliary control register
    ACTLR,
    /// software triggered interrupt register
    STIR,

    /// systick register
    SysTick(SysTickRegType),
    /// nvic register
    NVIC(NVICRegType),
    /// mpu register
    MPU(MPURegType),
    
    // todo: floating point extension scb registers
    // todo: cache and branch predictor maintenance

    /// peripheral identification register 4
    PID4,
    /// peripheral identification register 5
    PID5,
    /// peripheral identification register 6
    PID6,
    /// peripheral identification register 7
    PID7,
    /// peripheral identification register 0
    PID0,
    /// peripheral identification register 1
    PID1,
    /// peripheral identification register 2
    PID2,
    /// peripheral identification register 3
    PID3,

    /// component identification register 0
    CID0,
    /// component identification register 1
    CID1,
    /// component identification register 2
    CID2,
    /// component identification register 3
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
                SysTickRegType::lookup_offset(offset)
                    .map(|systick_reg| SCRegType::SysTick(systick_reg))
            }
            0x100 ..= 0xcff => {
                NVICRegType::lookup_offset(offset)
                    .map(|nvic_reg| SCRegType::NVIC(nvic_reg))
            }
            0xd90 ..= 0xdef => {
                MPURegType::lookup_offset(offset)
                    .map(|mpu_reg| SCRegType::MPU(mpu_reg))
            }

            _ => { None }
        }
    }
}

impl SCRegType {
    fn _data(&self) -> &'static SCRegTypeData {
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

// todo: write a proc-macro that can generate the Ref and Mut versions of this
// since i seem to be using this pattern quite a bit.
#[derive(Debug, Clone, PartialEq, Eq, From, TryInto, TryFrom)]
#[try_into(owned, ref, ref_mut)]
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

#[derive(Debug, Clone, From, TryInto, TryFrom)]
#[try_into(owned, ref, ref_mut)]
pub enum SCRegRef<'a> {
    CPUID(&'a CPUID),
    ICSR(&'a ICSR),
    VTOR(&'a VTOR),
    AIRCR(&'a AIRCR),
    SCR(&'a SCR),
    CCR(&'a CCR),
    SHPR1(&'a SHPR1),
    SHPR2(&'a SHPR2),
    SHPR3(&'a SHPR3),
    SHCSR(&'a SHCSR),
    CFSR(&'a CFSR),
    HFSR(&'a HFSR),
    DFSR(&'a DFSR),
    MMFAR(&'a MMFAR),
    BFAR(&'a BFAR),
    // AFSR(&'a AFSR),
    CPACR(&'a CPACR),
    // FPCCR(&'a FPCCR),
    // FPCAR(&'a FPCAR),
    // FPDSCR(&'a FPDSCR),
    // MVFR0(&'a MVFR0),
    // MVFR1(&'a MVFR1),
    // MVFR2(&'a MVFR2),
    // MCR(&'a MCR),
    ICTR(&'a ICTR),
    // ACTLR(&'a ACTLR),
    STIR(&'a STIR),
    SysTick(SysTickRegRef<'a>),
    NVIC(NVICRegRef<'a>),
    MPU(MPURegRef<'a>),
}

#[derive(Debug, From, TryInto, TryFrom)]
#[try_into(owned, ref, ref_mut)]
pub enum SCRegMut<'a> {
    CPUID(&'a mut CPUID),
    ICSR(&'a mut ICSR),
    VTOR(&'a mut VTOR),
    AIRCR(&'a mut AIRCR),
    SCR(&'a mut SCR),
    CCR(&'a mut CCR),
    SHPR1(&'a mut SHPR1),
    SHPR2(&'a mut SHPR2),
    SHPR3(&'a mut SHPR3),
    SHCSR(&'a mut SHCSR),
    CFSR(&'a mut CFSR),
    HFSR(&'a mut HFSR),
    DFSR(&'a mut DFSR),
    MMFAR(&'a mut MMFAR),
    BFAR(&'a mut BFAR),
    // AFSR(&'a mut AFSR),
    CPACR(&'a mut CPACR),
    // FPCCR(&'a mut FPCCR),
    // FPCAR(&'a mut FPCAR),
    // FPDSCR(&'a mut FPDSCR),
    // MVFR0(&'a mut MVFR0),
    // MVFR1(&'a mut MVFR1),
    // MVFR2(&'a mut MVFR2),
    // MCR(&'a mut MCR),
    ICTR(&'a mut ICTR),
    // ACTLR(&'a mut ACTLR),
    STIR(&'a mut STIR),
    SysTick(SysTickRegMut<'a>),
    NVIC(NVICRegMut<'a>),
    MPU(MPURegMut<'a>),
}

#[allow(unused)]
impl SCReg {
    fn _write_evt(&self, context: &mut Context) -> Result<(), context::Error> {
        match self {
            SCReg::CPUID(cpuid) => { todo!() }
            SCReg::ICSR(icsr) => { todo!() }
            SCReg::VTOR(vtor) => { todo!() }
            SCReg::AIRCR(aircr) => { todo!() }
            SCReg::SCR(scr) => { todo!() }
            SCReg::CCR(ccr) => { todo!() }
            SCReg::SHPR1(shpr1) => { todo!() }
            SCReg::SHPR2(shpr2) => { todo!() }
            SCReg::SHPR3(shpr3) => { todo!() }
            SCReg::SHCSR(shcsr) => { todo!() }
            SCReg::CFSR(cfsr) => { todo!() }
            SCReg::HFSR(hfsr) => { todo!() }
            SCReg::DFSR(dfsr) => { todo!() }
            SCReg::MMFAR(mmfar) => { todo!() }
            SCReg::BFAR(bfar) => { todo!() }
            // SCReg::AFSR(afsr) => { todo!() }
            SCReg::CPACR(cpacr) => { todo!() }
            
            // SCReg::FPCCR(fpccr) => { todo!() }
            // SCReg::FPCAR(fpcar) => { todo!() }
            // SCReg::FPDSCR(fpdscr) => { todo!() }
            // SCReg::MVFR0(mvfr0) => { todo!() }
            // SCReg::MVFR1(mvfr1) => { todo!() }
            // SCReg::MVFR2(mvfr2) => { todo!() }
        
            // SCReg::MCR(mcr) => { todo!() }
            SCReg::ICTR(ictr) => { todo!() }
            // SCReg::ACTLR(actlr) => { todo!() }
            SCReg::STIR(stir) => { todo!() }
        
            SCReg::SysTick(systickreg) => { todo!() }
            SCReg::NVIC(nvicreg) => { todo!() }
            SCReg::MPU(mpureg) => { todo!() }
        }
    }
}

/// provides identification information for the processor.
/// software can use CPUID registers to find out more about the processor.
/// word-accessible only. read-only. implementation-defined.
/// 
/// see B3.2.3
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CPUID {
    /// implementation-defined revision number
    #[bits(4)]
    pub revision: u32,
    /// implementation-defined part number
    #[bits(12)]
    pub partno: u32,
    /// architecture (always reads as 0xF)
    #[bits(4, default = 0xF)]
    pub architecture: u32,
    /// implementation defined variant number
    #[bits(4)]
    pub variant: u32,
    /// implmentor code assigned by Arm (reads as 0x41 if Arm-implemented)
    #[bits(8, default = 0x41)]
    pub implementer: u32,
}

/// provides software control of the NMI, PendSV, and SysTick Exceptions
/// and provides interrupt status information.
/// 
/// see B3.2.4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ICSR {
    /// exception number of current executing exception.
    /// (0 if in thread mode)
    /// read-only.
    #[bits(9)]
    pub vectactive: u32,
    #[bits(2)]
    __: u32,
    /// in handler mode, indicates whether there is an active exception other
    /// than the exception indicated by the current value of the IPSR.
    /// (0 = other active exception, 1 = no other active exception)
    /// in thread mode, unknown value.
    /// read-only.
    #[bits(1)]
    pub rettobase: bool,
    /// exception number of the highest prioirty pending and enabled interrupt.
    /// (0 = no pending exception)
    /// note: if DHCSR.C_MASKINTS is set, then PendSV, SysTick, and configurable
    /// external interrupts are masked and will not be shown as pending.
    /// read-only.
    #[bits(9)]
    pub vectpending: u32,
    #[bits(1)]
    __: bool,
    /// indicates whether an external interrupt, generated by the NVIC, is pending.
    /// (0 = no pending, 1 = pending)
    /// read-only.
    #[bits(1)]
    pub isrpending: bool,
    /// indicates whether a pending exception will be serviced on exit from debug
    /// halt state.
    /// (0 = will not service, 1 = will service)
    /// read-only.
    #[bits(1)]
    pub isrpreempt: bool,
    #[bits(1)]
    __: bool,
    /// removes pending status of the SysTick exception
    /// (0 = no effect, 1 = remove pending status)
    /// write-only
    #[bits(1)]
    pub pendstclr: bool,
    /// on writes, sets SysTick exception as pending. on reads, indicates
    /// the current state of the exception.
    /// write (0 = no effect, 1 = set SysTick pending)
    /// read (0 = SysTick not pending, 1 = SysTick pending)
    #[bits(1)]
    pub pendstset: bool,
    /// removes pending status of PendSV exception
    /// (0 = no effect, 1 = remove pending status)
    /// write-only
    #[bits(1)]
    pub pendsvclr: bool,
    /// on writes, sets PendSV exception as pending. on reads, indicates
    /// the current state of the exception.
    /// write (0 = no effect, 1 = set PendSV pending)
    /// read (0 = PendSV not pending, 1 = PendSV pending)
    #[bits(1)]
    pub pendsvset: bool,
    #[bits(2)]
    __: u32,
    /// on writes, makes NMI exception active. on reads, indicates
    /// current state of the exception.
    /// write (0 = no effect, 1 = set NMI active)
    /// read (0 = NMI inactive, 1 = NMI active)
    /// 
    /// since NMI is highest priority, if processor not already executing
    /// in the NMI handler, it enters NMI exception handler as soon as 
    /// it recognizes the write to this bit.
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

/// Provides information about the interrupt controller. 
/// Word-accessible only. Read-only.
///
/// See B3.4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct ICTR {
    /// Number of interrupt lines supported by the implementation.
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
        self.intid() + 16
    }

    pub fn write_evt(&self) -> Vec<Event> {
        vec![Event::ExceptionSetActive(ExceptionType::ExternalInterrupt(self.exception_number()), true)]
    }
}





impl SCRegType {
    pub(super) unsafe fn to_reg_ref<'a>(&self, int_ref: &'a u32) -> Result<SCRegRef<'a>, Error> {
        match self {
            SCRegType::CPUID => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const CPUID)).unwrap())
            }
            SCRegType::ICSR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const ICSR)).unwrap())
            }
            SCRegType::VTOR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const VTOR)).unwrap())
            }
            SCRegType::AIRCR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const AIRCR)).unwrap())
            }
            SCRegType::SCR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const SCR)).unwrap())
            }
            SCRegType::CCR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const CCR)).unwrap())
            }
            SCRegType::SHPR1(_) => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const SHPR1)).unwrap())
            }
            SCRegType::SHPR2(_) => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const SHPR2)).unwrap())
            }
            SCRegType::SHPR3(_) => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const SHPR3)).unwrap())
            }
            SCRegType::SHCSR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const SHCSR)).unwrap())
            }
            SCRegType::CFSR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const CFSR)).unwrap())
            }
            SCRegType::HFSR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const HFSR)).unwrap())
            }
            SCRegType::DFSR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const DFSR)).unwrap())
            }
            SCRegType::MMFAR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const MMFAR)).unwrap())
            }
            SCRegType::BFAR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const BFAR)).unwrap())
            }
            SCRegType::CPACR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const CPACR)).unwrap())
            }
            SCRegType::ICTR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const ICTR)).unwrap())
            }
            SCRegType::STIR => {
                Ok(SCRegRef::try_from(&*(int_ref as *const u32 as *const STIR)).unwrap())
            }
            SCRegType::SysTick(systick_regtype) => {
                Ok(SCRegRef::SysTick(systick_regtype.to_reg_ref(int_ref)))
            }
            SCRegType::NVIC(nvic_regtype) => {
                Ok(SCRegRef::NVIC(nvic_regtype.to_reg_ref(int_ref)))
            }
            SCRegType::MPU(mpu_regtype) => {
                Ok(SCRegRef::MPU(mpu_regtype.to_reg_ref(int_ref)))
            }
            _ => { Err(Error::UnimplementedSysCtrlReg(self.clone())) }
        }
    }

    pub(super) unsafe fn to_reg_mut<'a>(&self, int_ref: &'a mut u32) -> Result<SCRegMut<'a>, Error> {
        match self {
            SCRegType::CPUID => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CPUID)).unwrap())
            }
            SCRegType::ICSR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut ICSR)).unwrap())
            }
            SCRegType::VTOR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut VTOR)).unwrap())
            }
            SCRegType::AIRCR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut AIRCR)).unwrap())
            }
            SCRegType::SCR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut SCR)).unwrap())
            }
            SCRegType::CCR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CCR)).unwrap())
            }
            SCRegType::SHPR1(_) => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut SHPR1)).unwrap())
            }
            SCRegType::SHPR2(_) => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut SHPR2)).unwrap())
            }
            SCRegType::SHPR3(_) => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut SHPR3)).unwrap())
            }
            SCRegType::SHCSR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut SHCSR)).unwrap())
            }
            SCRegType::CFSR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CFSR)).unwrap())
            }
            SCRegType::HFSR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut HFSR)).unwrap())
            }
            SCRegType::DFSR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut DFSR)).unwrap())
            }
            SCRegType::MMFAR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut MMFAR)).unwrap())
            }
            SCRegType::BFAR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut BFAR)).unwrap())
            }
            SCRegType::CPACR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut CPACR)).unwrap())
            }
            SCRegType::ICTR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut ICTR)).unwrap())
            }
            SCRegType::STIR => {
                Ok(SCRegMut::try_from(&mut *(int_ref as *mut u32 as *mut STIR)).unwrap())
            }
            SCRegType::SysTick(systick_regtype) => {
                Ok(SCRegMut::SysTick(systick_regtype.to_reg_mut(int_ref)))
            }
            SCRegType::NVIC(nvic_regtype) => {
                Ok(SCRegMut::NVIC(nvic_regtype.to_reg_mut(int_ref)))
            }
            SCRegType::MPU(mpu_regtype) => {
                Ok(SCRegMut::MPU(mpu_regtype.to_reg_mut(int_ref)))
            }
            _ => { Err(Error::UnimplementedSysCtrlReg(self.clone())) }
        }
    }
}