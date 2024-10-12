//! scs.rs

use context::Permission;

use super::*;


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

    pub fn read_bytes(&self, offset: impl Into<usize>, dst: &mut [u8]) -> Result<Option<Event>, context::Error> {
        todo!()
    }

    pub fn write_bytes(&mut self, offset: impl Into<usize>, src: &[u8]) -> Result<Option<Event>, context::Error> {
        todo!()
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
    CPUID,  // cpuid base register
    ICSR,   // interrupt control and state register 
    VTOR,   // vector table offset register
    AIRCR,  // application interrupt and reset control register
    SCR,    // system control register
    CCR,    // configuration and control register
    SHPR1,  // system handler priority register 1
    SHPR2,  // system handler priority register 2
    SHPR3,  // system handler priority register 3
    SHCSR,  // system handler control and state register
    CFSR,   // configurable fault status register
    HFSR,   // hardfault status register
    DFSR,   // debug fault status register
    MMFAR,  // memmanage fault address register
    BFAR,   // busfault address register
    AFSR,   // auxiliary fault status register
    CPACR,  // coprocessor access control register
    
    FPCCR,  // floating point context control register
    FPCAR,  // floating point context address register
    FPDSCR, // floating point default status control register
    MVFR0,  // media and fp feature register 0
    MVFR1,  // media and fp feature register 1
    MVFR2,  // media and fp feature register 2

    MCR,    // main control register, reserved
    ICTR,   // interrupt controller type register
    ACTLR,  // auxiliary control register
    STIR,   // software triggered interrupt register

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

    fn _data(&self) -> &SCRegData {
        match self {
            SCReg::CPUID    => { &SCRegData { offset: 0xd00_usize, perms: 0b100, reset: None } }
            SCReg::ICSR     => { &SCRegData { offset: 0xd04_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::VTOR     => { &SCRegData { offset: 0xd08_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::AIRCR    => { &SCRegData { offset: 0xd0c_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SCR      => { &SCRegData { offset: 0xd10_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::CCR      => { &SCRegData { offset: 0xd14_usize, perms: 0b110, reset: None } }
            SCReg::SHPR1    => { &SCRegData { offset: 0xd18_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SHPR2    => { &SCRegData { offset: 0xd1c_usize, perms: 0b110, reset: Some(0x0) } }
            SCReg::SHPR3    => { &SCRegData { offset: 0xd20_usize, perms: 0b110, reset: Some(0x0) } }
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

    pub fn lookup(address: impl AsRef<Address>) -> Option<Self> {
        let address = address.as_ref();
        let offset = address.offset()
            .checked_sub(0xe000e000)
            .expect("address is not in scs!");
        assert!(offset < 0x1000, "address is not in scs!");

        match offset as usize {
            0xd00_usize => { Some(SCReg::CPUID) }
            0xd04_usize => { Some(SCReg::ICSR) }
            0xd08_usize => { Some(SCReg::VTOR) }
            0xd0c_usize => { Some(SCReg::AIRCR) }
            0xd10_usize => { Some(SCReg::SCR) }
            0xd14_usize => { Some(SCReg::CCR) }
            0xd18_usize => { Some(SCReg::SHPR1) }
            0xd1c_usize => { Some(SCReg::SHPR2) }
            0xd20_usize => { Some(SCReg::SHPR3) }
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

            _ => { None }
        }
    }
}