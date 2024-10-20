//! system.rs
//! 
//! system-related context functionality

use bitfield_struct::bitfield;

use super::*;



impl<'irb> Context<'irb> {

    fn _current_mode_is_privileged(&self) -> bool {
        self.mode == Mode::Handler || !self.control.npriv()
    }

    /// derived from LookUpSP() pseudocode in B1.4.7
    fn _is_sp_main(&self) -> bool {
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
    fn _take_reset(&mut self) -> Result<(), context::Error> {
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
