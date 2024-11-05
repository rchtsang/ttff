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
    /// debug register
    Debug(DebugRegType),

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
pub(super) struct SCRegTypeData {
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

    pub fn iter() -> impl Iterator<Item=Self> {
        (0x0..0x1000).step_by(4).filter_map(Self::lookup_offset)
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
            0xdf0 ..= 0xeff => {
                DebugRegType::lookup_offset(offset)
                    .map(|dbg_reg| SCRegType::Debug(dbg_reg))
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
    Debug(DebugReg),
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
    Debug(DebugRegRef<'a>),
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
    Debug(DebugRegMut<'a>),
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

/// holds the vector table address.
/// one or two of the high-order bits of the TBLOFF field can be implemented
/// as RAZ/WI, reducing the supported address range.
/// 
/// see B3.2.5
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct VTOR {
    #[bits(7)]
    __: u32,
    #[bits(25)]
    pub tbloff: u32,
}

impl VTOR {
    pub fn write_evt(&self) -> Vec<Event> {
        vec![Event::VectorTableOffsetWrite(self.tbloff() << 7)]
    }
}

/// sets or returns interrupt control data
/// 
/// see B3.2.6
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct AIRCR {
    /// writing 1 to this bit causes a local system reset.
    /// see B1-559. 
    /// this bit self-clears.
    /// writing a 1 to this bit if the processor is not halted
    /// in Debug state is unpredictable
    #[bits(1)]
    pub vectreset: bool,
    /// writing 1 to this bit clears all active state information for fixed
    /// and configurable exceptions. this will also clear the IPSR to 0.
    /// see B1-517.
    /// writing a 1 to this bit if the processor is not halted
    /// in Debug state is unpredictable
    #[bits(1)]
    pub vectclractive: bool,
    /// system reset request.
    /// (0 = no reset request, 1 = request reset)
    /// writing 1 to this bit asserts a signal to the external system
    /// to request a local reset.
    /// a local or power-on reset clears this bit to 0.
    #[bits(1, default = false)]
    pub sysresetreq: bool,
    #[bits(5)]
    __: u32,
    /// priority grouping, indicates binary point position.
    /// see B1-527.
    /// this field resets to 0.
    #[bits(3, default = 0)]
    pub prigroup: u8,
    #[bits(4)]
    __: u32,
    /// indicates memory system endianness.
    /// (0 = little endian, 1 = big endian).
    /// this bit is static or configured by hardware input on reset.
    /// this bit is read-only.
    #[bits(1, default = 0)]
    pub endianness: u8,
    /// vector key.
    /// (on write: VECTKEY, on read: VECTKEYSTAT)
    /// register writes must write 0x05FA to this field or write is ignored.
    /// on reads, always returns 0xFA05.
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

/// sets or returns system control data
/// 
/// see B3.2.7
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct SCR {
    #[bits(1)]
    __: bool,
    /// determines whether processor enters sleep state after ISR exits and returns
    /// to base level of execution.
    /// (0 = don't enter, 1 = enter sleep)
    /// 
    /// see power management on page B1-559
    #[bits(1)]
    pub sleeponexit: bool,
    /// provides a qualifying hint indicating that waking from sleep might take
    /// longer. implementation can use this bit to select between sleep states:
    /// (0 = not deep sleep, 1 = deep sleep)
    /// 
    /// details are IMPLEMENTATION DEFINED.
    /// if not implemented, RAZ/WI.
    #[bits(1)]
    pub sleepdeep: bool,
    #[bits(1)]
    __: bool,
    /// determines whether an interrupt transition from inactive state to pending
    /// is a wakeup event:
    /// (0 = not wakeup, 1 = wakeup)
    /// 
    /// see WFE wakeup events on page B1-591
    #[bits(1)]
    pub sevonpend: bool,
    #[bits(27)]
    __: u32,
}

/// sets or returns configuration and control data, provides control over
/// caching and branch prediction.
/// 
/// see B3.2.8
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CCR {
    /// controls whether processor can enter thread mode with exception active
    /// - 0: any attempt to return to thread mode results in exception if the
    ///      number of active exceptions is non-zero and does not rely on 
    ///      execution prioirty boosting, including BASEPRI, FAULTMASK, PRIMASK
    /// - 1: processor can enter thread mode with exceptions active because of
    ///      a controlled return value (see page B1-539)
    #[bits(1)]
    pub nonbasethrdena: bool,
    /// controls whether unprivileged software can access the STIR
    /// (0 = cannot access, 1 = can access)
    /// (see B3-619 for further info)
    #[bits(1)]
    pub usersetmpend: bool,
    #[bits(1)]
    __: bool,
    /// controls trapping of unaligned word or halfword accesses:
    /// (0 = trapping disabled, 1 = trapping enabled)
    /// Unaligned load-store multiple and word/halfword exclusive accesses
    /// always fault.
    #[bits(1)]
    pub unalign_trp: bool,
    /// controls trapping on divide-by-zero
    /// (0 = trapping disabled, 1 = trapping enabled)
    #[bits(1)]
    pub div_0_trp: bool,
    #[bits(3)]
    __: u32,
    /// determines effect of precise data access faults on handlers running
    /// at priority -1 or -2.
    /// (0 = precise data access fault causes lockup, 1 = handler ignores fault)
    /// (see Unrecoverable exception cases on page B1-555)
    #[bits(1)]
    pub bfhfnmign: bool,
    /// determines whether the exception entry sequence guarantees 8-byte stack
    /// frame alignment, adjusting SP if necessary before saving state.
    /// (0 = guaranteed 4-byte, no SP adjustment; 1 = guaranteed 8-byte, 
    /// SP adjusted as necessary)
    /// 
    /// whether bit is read-write or read-only is IMPLEMENTATION DEFINED
    /// (pick read-write as default)
    /// reset value of bit is IMPLEMENTATION DEFINED (1 recommended)
    /// (see Stack alignment on exception entry on page B1-535)
    #[bits(1)]
    pub stkalign: bool,
    #[bits(6)]
    __: u32,
    /// cache enable bit. global enable for data and unified caches.
    /// (0 = disabled, 1 = enabled)
    /// 
    /// if system doesn't implement caches that are processor-accessible,
    /// bit is RAZ/WI.
    /// if system implements such caches, it _must_ be possible to disable
    /// them by clearing this bit.
    #[bits(1)]
    pub dc: bool,
    /// instruction cache enable bit. global enable for instruction caches.
    /// (0 = disabled, 1 = enabled)
    /// 
    /// if system does not implement caches that are processor-accessible,
    /// bit is RAZ/WI.
    /// if system implements such caches, it _must_ be possible to disable
    /// them by clearing this bit.
    #[bits(1)]
    pub ic: bool,
    /// branch prediction enable bit.
    /// (0 = disabled, 1 = enabled)
    /// 
    /// if prediction cannot be disabled, this bit is RAO/WI.
    /// if prediction is not supported, this bit is RAZ/WI.
    #[bits(1)]
    pub bp: bool,
    #[bits(13)]
    __: u32,
}

/// sets or returns priority for system handlers 4-7
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

/// sets or returns priority for system handlers 8-11
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

/// sets or returns priority for system handlers 12-15
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

/// shows which debug event occured.
/// writing 1 to a register bit clears the bit to 0.
/// a read of the HALTED bit by an instruction executed by stepping returns an 
/// unknown value. see C1-695.
/// 
/// power-on reset clears register to 0.
/// local reset does not affect the register.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct DFSR {
    /// indicates a debug event generated by either:
    /// - a C_HALT or C_STEP request, triggered by a write to the DHCSR
    /// - a step request triggered by setting DEMCR.MON_STEP to 1
    /// (0 = no halt request debug event, 1 = halt request debug event)
    #[bits(1)]
    pub halted: bool,
    /// indicates a debug event generated by BKPT instruction execution or a
    /// breakpoint match in FPB.
    /// (0 = no breakpoint event, 1 = at least 1 breakpoint event)
    #[bits(1)]
    pub bkpt: bool,
    /// indicates a debug event generated by th DWT.
    /// (0 = no events, 1 = at least 1 event)
    #[bits(1)]
    pub dwttrap: bool,
    /// indicates triggering of a vector catch
    /// (0 = no catch triggered, 1 = vector catch triggered)
    #[bits(1)]
    pub vcatch: bool,
    /// indicates a debug event generated because of the assertion of an 
    /// external debug request
    /// (0 = no external request event, 1 = external debug requested)
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
            SCRegType::Debug(dbg_regtype) => {
                Ok(SCRegRef::Debug(dbg_regtype.to_reg_ref(int_ref)))
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
            SCRegType::Debug(dbg_regtype) => {
                Ok(SCRegMut::Debug(dbg_regtype.to_reg_mut(int_ref)))
            }
            _ => { Err(Error::UnimplementedSysCtrlReg(self.clone())) }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_iter() -> Result<(), ()> {
        for sc_regtype in SCRegType::iter() {
            println!("{sc_regtype:?}");
        }
        Ok(())
    }
}