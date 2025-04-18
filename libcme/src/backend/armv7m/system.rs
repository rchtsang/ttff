//! system.rs
//! 
//! system-related context functionality
//! 
//! read-only functions shall be public, but functions that update
//! context internal state shall be private to crate implmentation.

use bitfield_struct::bitfield;

use crate::backend;
use super::*;

impl Backend {

    /// derived from CurrentModeIsPrivileged() pseudocode in B1.3.1
    pub fn current_mode_is_privileged(&self) -> bool {
        matches!(self.mode, Mode::Handler(_)) || !self.control.npriv()
    }

    /// derived from LookUpSP() pseudocode in B1.4.7
    pub fn is_sp_main(&self) -> bool {
        if self.control.spsel() {
            if self.mode == Mode::Thread {
                false
            } else {
                panic!("sp state undefined!")
            }
        } else {
            true
        }
    }

    /// reset processor following pseudocode in B1.5.5
    pub(crate) fn _take_reset(&mut self) -> Result<(), backend::Error> {
        todo!("many more things need implementation before this will work!");
        /* 
         * TakeReset() pseudocode B1.5.5
         * CurrentMode = Mode_Thread;
         * PRIMASK<0> = '0';                   /* priority mask cleared at reset */
         * FAULTMASK<0> = '0';                 /* fault mask cleared at reset */
         * BASEPRI<7:0> = Zeros(8);            /* base priority disabled at reset */
         * if HavFPExt() {
         *     CONTROL<2:0> = '000';
         *     CPACR.cp10 = '00';
         *     CPACR.cp11 = '00';
         *     FPDSCR.AHP = '0';
         *     FPDSCR.DN = '0';
         *     FPDSCR.FZ = '0';
         *     FPDSCR.RMode = '00';
         *     FPCCR.ASPEN = '1';
         *     FPCCR.LSPEN = '1';
         *     FPCCR.LSPACT = '0';
         *     FPCAR = bits(32) UNKNOWN;
         *     FPFSR = bits(32) UNKNOWN;
         *     for i = 0 to 31 {
         *         S[i] = bits(32) UNKNOWN;
         *     }
         * } else {
         *     CONTROL<1:0> = '00';            /* current stack is Main, thread is privileged */
         * }
         * for i = 0 to 511 {                  /* all exceptions inactive */
         *     ExceptionActive[i] = '0';
         * }
         * ResetSCSRegs();                     /* system control space reset */
         * ClearExclusiveLocal(ProcessorID()); /* Synchronization (LDREX* / STREX*) monitor support */
         * ClearEventregister();               /* see WFE instruction */
         * for i = 0 to 12 {
         *     R[i] = bits(32) UNKNOWN;
         * }
         * 
         * bits(32) vectortable = VTOR<31:7>:'0000000';    /* initialize vector table */
         * SP_main = MemA_with_priv[vectortable, 4, AccType_VECTABLE] AND 0xFFFFFFFC<31:0>;
         * SP_process = ((bits(30) UNKNOWN:'00');
         * LR = 0xFFFFFFFF<31:0>;              /* preset to illegal exception return value */
         * reset_entry = MemA_with_priv[vectortable+4, 4, AccType_VECTABLE];
         * tbit = reset_entry<0>;
         * APSR = bits(32) UNKNOWN;            /* flags UNPREDICTABLE from reset */
         * IPSR<8:0> = Zeros(9);               /* Exception Number cleared */
         * EPSR.T = tbit;                      /* T bit set from entry vector */
         * EPSR.IT<7:0> = Zeros(8);            /* IT/ICI bits cleared */
         * BranchTo(reset_entry AND 0xFFFFFFFE<31:0>); /* branch to reset service routine */
         * 
         * // ExceptionActive is an array of active flag bits for all exceptions
         * // it has active flags for the fixed-priority system exception, 
         * // config-priority system exceptions, and external interrupts.
         * // The active flags for the fixed-priority exceptions are conceptual only and
         * // do not need to exist in a system register
         * 
         * // see B1.4.7 for register-related global pseudocode definitions
         */


    }
}


/// special-purpose control register
/// 
/// defined in B1.4.4
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct CONTROL {
    /// defines execution privilege in Thread mode.
    /// (0 = privileged, 1 = unprivileged)
    #[bits(1)]
    pub npriv: bool,
    /// defines stack to be used.
    /// (0 = SP_main, 1 = SP_process)
    #[bits(1)]
    pub spsel: bool,
    /// defines whether FP extension is active in current context.
    /// (0 = inactive, 1 = active)
    #[bits(1)]
    pub fpca: bool,

    #[bits(29)]
    __: u32,
}

/// special-purpose PRIMASK register
/// 
/// defined in B1.4.3
/// 
/// set to 1 on CPSID i
/// cleared to 0 on CPSIE i
/// 
/// prevent preemption from configurable-priority exceptons.
/// also affects WFI
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct PRIMASK {
    /// raise current execution priority to 0 if set to 1
    #[bits(1)]
    pub pm: bool,
    #[bits(31)]
    __: u32,
}

/// special-purpose FAULTMASK register
/// 
/// defined in B1.4.3
/// 
/// set to 1 on CPSID f
/// cleared to 0 on CPSIE f
/// 
/// prevent preemption from any exception except NMI.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct FAULTMASK {
    /// raise current execution priority to -1 if set to 1
    /// (only allowed by privileged software at prioirty below -1)
    /// return from any exception except NMI clears FM
    #[bits(1)]
    pub fm: bool,
    #[bits(31)]
    __: u32,
}

/// special-purpose BASEPRI register
/// 
/// defined in B1.4.3
/// 
/// changes priority level required for exception preemption
/// only comes into effect when BASEPRI has a lower value than
/// the unmasked priority level of the currently executing software
/// (BASEPRI is set to higher priority than current priority)
/// 
/// zero disables masking by BASEPRI.
/// any nonzero value, based on AIRCR.PRIGROUP acts as priority mask.
/// 
/// unprivileged accesses are Read Always Zero/Write Ignored (RAZ/WI)
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct BASEPRI {
    #[bits(8)]
    pub basepri: u8,
    #[bits(24)]
    __: u32,
}

/// special-purpose program status register xPSR
/// defined in B1.4.2
/// 
/// XPSR is a combination of APSR, IPSR, and EPSR.
/// 
/// APSR SHOULD NOT BE ACCESSED FROM THIS STRUCT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct XPSR(pub u32);

#[allow(unused)]
impl XPSR {
    /// access xPSR as IPSR
    pub fn ipsr(&self) -> &IPSR {
        unsafe { &*(self as *const XPSR as *const u32 as *const IPSR) }
    }

    /// access xPSR as mutable IPSR
    pub fn ipsr_mut(&mut self) -> &mut IPSR {
        unsafe { &mut *(self as *mut XPSR as *mut u32 as *mut IPSR) }
    }

    /// access xPSR as EPSR
    pub fn epsr(&self) -> &EPSR {
        unsafe { &*(self as *const XPSR as *const u32 as *const EPSR) }
    }

    /// access xPSR as mutable EPSR
    pub fn epsr_mut(&mut self) -> &mut EPSR {
        unsafe { &mut *(self as *mut XPSR as *mut u32 as *mut EPSR) }
    }

    /// access xPSR as APSR
    pub fn apsr(&self) -> &APSR {
        unimplemented!("APSR should be accessed as defined by sleigh spec");
    }

    /// access xPSR as mutable APSR
    pub fn apsr_mut(&mut self) -> &mut APSR {
        unimplemented!("APSR should be accessed as defined by sleigh spec");
    }
}

/// application program status register.
/// 
/// note: this struct will be unused, since the sleigh spec defines an APSR
/// as the CPSR and manages the flags in spec.
/// 
/// note2: the sleigh spec ignores the GE bits.
/// 
/// holds flags that can be written by application-level software, that is, by
/// unprivileged software.
/// APSR handling of application-level writeable flags by the MSR and MRS 
/// instructions is consistent across all ARMv7 profiles.
/// 
/// see A2.3.2
/// 
/// reserved bits are allocated to system features or available for future
/// expansion. bits are defined as UNK/SBZP.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct APSR {
    #[bits(16)]
    __: u16,
    /// greater than or Equal flags, updated by SIMD instructions to indicate
    /// results from individual bytes or halfwords of the operations.
    /// DSP extension only.
    /// 
    /// software can use these flags to control a later SEL instruction
    /// (see SEL on A7-351).
    #[bits(4)]
    pub ge: u8,
    #[bits(7)]
    __: u8,
    /// set to 1 if a SSAT or USAT instruction changes the input value for
    /// the signed or unsigned range of the result.
    /// in a processor that implements the DSP extension, the processor sets
    /// this bit to 1 to indicate an overflow on some multiplies.
    /// 
    /// setting this bit to 1 is called saturation.
    #[bits(1)]
    pub q: bool,
    /// overflow condition flag.
    /// set to 1 if the instruction results in an overflow condition,
    /// e.g. a signed overflow on an addition.
    #[bits(1)]
    pub v: bool,
    /// carry condition flag.
    /// set to 1 if the instruction results in a carry condition,
    /// e.g. unsigned overflow on addition.
    #[bits(1)]
    pub c: bool,
    /// zero condition flag.
    /// set to 1 if result of instruction is zero, 0 otherwise.
    /// result of zero often indicates equality from comparison.
    #[bits(1)]
    pub z: bool,
    /// negative condition flag.
    /// set to bit[31] of result of the instruction.
    /// if result is regarded as a two's complement signed integer, then 
    /// N == 1 if result is negative and 0 if positive or zero.
    #[bits(1)]
    pub n: bool,

}

/// interrupt program status register.
/// 
/// when processor is executing an exception handler, holds the exception
/// number of the exception being processed. Otherwise, IPSR value is zero.
/// 
/// processor writes to the IPSR on exception entry and exit.
/// Software can use an MRS instruction to read the IPSR, but the
/// processor ignores writes tot he IPSR by an MSR instruction.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct IPSR {
    /// exception number defined as:
    /// - 0 in thread mode
    /// - exception number of currently executing exception in handler mode
    /// 
    /// on reset, processor is in thread mode and exception number is
    /// cleared to 0. (1 is a transitory value and an invalid exception number)
    #[bits(9)]
    pub exception_number: u32,
    #[bits(23)]
    __: u32,
}

/// execution program status register.
/// 
/// holds execution state bits.
#[bitfield(u32)]
#[derive(PartialEq, Eq)]
pub struct EPSR {
    #[bits(10)]
    __: u16,
    /// ICI/IT bits
    #[bits(6)]
    ici_it_lower: u8,
    #[bits(8)]
    __: u8,
    /// thumb mode bit
    #[bits(1)]
    pub t: bool,
    /// more ICI/IT bits
    #[bits(2)]
    ici_it_upper: u8,
    #[bits(5)]
    __: u8,
}

impl EPSR {
    /// get bits as ITSTATE
    pub fn itstate(&mut self) -> &mut ITSTATE {
        unsafe { &mut *(self as *mut EPSR as *mut u32 as *mut ITSTATE) }
    }


}

/// holds the If-Then Execution state bits for the Thumb IT instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ITSTATE(u32);

#[allow(unused)]
impl ITSTATE {

    /// get current value of ITSTATE as expected bits
    pub fn value(&self) -> u8 {
        (((self.0 & 0x0000fc00) >>  8) | ((self.0 & 0x06000000) >> 25)) as u8
    }

    /// set current value of ITSTATE from expected bits
    pub fn set(&mut self, val: u8) {
        let val = val as u32;
        self.0 &= !0x0600fc00;
        self.0 |= ((val & 0x03) << 25) | ((val & 0xfc) << 8);
    }

    /// IT block size (number of instructions that are to be conditionally
    /// executed). Size is implied by position of least significant 1 in
    /// this field (see A7-180).
    /// Also encodes least significant bit of condition code for each 
    /// instruction in the block (see Table A7-2).
    pub fn code(&self) -> u8 {
        self.value() & 0b11111
    }
    pub fn set_code(&mut self, val: u8) {
        let masked = self.value() & 0b11100000;
        self.set((val & 0b11111) | masked);
    }

    /// base condition for current IT block (top 3 bits of the condition
    /// specified by the IT instruction).
    /// 
    /// subfield is 0b000 when no IT block is active.
    pub fn base_cond(&self) -> u8 {
        (self.value() & 0b11100000) >> 5
    }
    pub fn set_base_cond(&mut self, val: u8) {
        let masked = self.value() & 0b00011111;
        self.set(((val & 0b111) << 5) | masked);
    }
}

impl Into<u8> for ITSTATE {
    fn into(self) -> u8 {
        self.value()
    }
}

impl From<u8> for ITSTATE {
    fn from(value: u8) -> Self {
        let mut val = ITSTATE(0);
        val.set(value);
        val
    }
}

/// holds ICI bits that provide information on the outstanding register
/// list for an interrupted exception-continuable multicycle load or
/// store instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ICI(u32);

#[allow(unused)]
impl ICI {
    /// get current value of ICI bits
    pub fn value(&self) -> u8 {
        (((self.0 & 0x0000fc00) >>  10) | ((self.0 & 0x06000000) >> 15)) as u8
    }

    /// set current value of ICI bits
    pub fn set(&mut self, val: u8) {
        assert!((val & 0b11000011) == 0, "only ICI reg_num bits[5:2] should be nonzero");
        let val = val as u32;
        self.0 &= !0x0600fc00;
        self.0 |= ((val & 0xc0) << 15) | ((val & 0x3f) << 10);
    }

    /// register number to continue from in exception-continuable memory access
    /// after returning from an exception.
    /// processor should continue loading/storing any registers in instruction
    /// register list with number equal to or greater than this register number
    pub fn reg_num(&self) -> u8 {
        (self.value() & 0b00111100) >> 2
    }
    pub fn set_reg_num(&mut self, val: u8) {
        let masked = self.value() & 0b11000011;
        self.set(((val & 0xf) << 2) | masked);
    }
}

impl Into<u8> for ICI {
    fn into(self) -> u8 {
        self.value()
    }
}

impl From<u8> for ICI {
    fn from(value: u8) -> Self {
        let mut val = ICI(0);
        val.set(value);
        val
    }
}

/// event register representation
/// (defined in B1.5.18).
/// 
/// a single bit register for each processor in a multiprocessor system.
/// when set, an event register indicates that an event has occurred since
/// the register was last cleared, and which might prevent the processor
/// from suspending operation on issuing a WFE instruction.
/// 
/// the following conditions apply:
/// - a system reset clears the event register
/// - any WFE wakeup event or execution or an exception return instruction
///   sets the event register. (for definition of exception return instructions
///   see Exception return behavior on B1-539)
/// - a WFE instruction clears the event register
/// - software cannor read or write teh value of the EVENT register directly.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct EVENT(pub bool);