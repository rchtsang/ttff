//! faults.rs
//! 
//! architecture-defined faults

#[allow(unused)]
use super::*;

/// fault types
#[derive(Debug, Clone)]
pub enum Fault {
    Mem(MemFault),
    Bus(BusFault),
    Usg(UsgFault),
    Hard(HardFault),
}

/// MemManage faults
#[derive(Debug, Clone)]
pub enum MemFault {
    InsnAccessViolation,    // (MMFSR.IACCVIOL)
    DataAccessViolation,    // (MMFSR.DACCVIOL)
    OnExceptionReturn,      // (MMFSR.MUNSTKERR)
    OnExceptionEntry,       // (MMFSR.MSTKERR)
    LazyStatePreservation,  // (MMFSR.MLSPERR)
}

/// Bus faults
#[derive(Debug, Clone)]
pub enum BusFault {
    InsnPrefetch,           // (BFSR.IBUSERR)
    PreciseDataAccess,      // (BFSR.PRECISERR)
    ImprciseDataAccess,     // (BFSR.IMPRCISERR)
    OnExceptionReturn,      // (BFSR.UNSTKERR)
    OnExceptionEntry,       // (BFSR.STKERR)
    LazyStatePreservation,  // (BFSR.LSPERR)
}

/// Usage faults
#[derive(Debug, Clone)]
pub enum UsgFault {
    UndefinedInsn,          // (UFSR.UNDEFINSTR)
    InvalidState,           // (UFSR.INVSTATE) invlid EPSR.T or EPSR.IT field
    IntegrityCheck,         // (UFSR.INVPC) integrity check error on EXC_RETURN
    CoprocessorAccess,      // (UFSR.NOCP) coprocessor disabled or absent
    UnalignedAccess,        // (UFSR.UNALIGNED)
    DivideByZero,           // (UFSR.DIVBYZERO) on sdiv or udiv
}

/// Hard fault
#[derive(Debug, Clone)]
pub enum HardFault {
    VectorTableRead,        // (HFSR.VECTTBL) vector table read error on exception processing
    EscalatedException,     // (HFSR.FORCED) escalated from configurable-priority exception
    DebugEvent,             // (HFSR.DEBUGEVT) DFSR should also be updated
}



impl From<MemFault> for Fault {
    fn from(value: MemFault) -> Self {
        Self::Mem(value)
    }
}

impl From<BusFault> for Fault {
    fn from(value: BusFault) -> Self {
        Self::Bus(value)
    }
}

impl From<UsgFault> for Fault {
    fn from(value: UsgFault) -> Self {
        Self::Usg(value)
    }
}

impl From<HardFault> for Fault {
    fn from(value: HardFault) -> Self {
        Self::Hard(value)
    }
}