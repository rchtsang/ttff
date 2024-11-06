//! events.rs
//! 
//! armv7m architectural events and event processing

use super::*;

/// armv7m architecture event
/// 
/// these events can be triggered on writes to system control
/// and must be dealt with immediately with an update to the
/// context state if necessary
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    // ICSR and SHCSR
    ExceptionSetActive(ExceptionType, bool),
    ExceptionSetPending(ExceptionType, bool),
    ExceptionEnabled(ExceptionType, bool),

    // VTOR
    VectorTableOffsetWrite(u32),

    // AIRCR
    ExternSysResetRequest,      // (SYSRESETREQ) external system reset request
    LocalSysResetRequest,       // (VECTRESET) local system reset
    ExceptionClrAllActive,      // (VECTCLRACTIVE) clear all active state info for fixed and configurable exceptions, clear ipsr to 0
    VectorKeyWrite,             // (VECTKEY) 0x05fa written to vector key register
    SetPriorityGrouping(u8),    // (PRIGROUP) set the priority grouping according to Table B1-7

    // SCR keeps state that influences execution
    SetTransitionWakupEvent(bool),  // transitions from inactive to pending are/aren't wakeup events
    SetDeepSleep(bool),             // selected sleep state is/isn't deep sleep
    SetSleepOnExit(bool),           // enter/don't enter sleep state

    // CCR keeps state that influences executon
    ThreadModeExceptionsEnabled(bool),      // (NONBASETHRDENA) allow/disallow enter/return to thread mode with active exceptions (except priority boosting)
    STIRUnprivilegedAccessAllowed(bool),    // (USERSETMPEND) allow/disallow unprivileged access to the STIR
    UnalignedAccessTrapEnabled(bool),       // (UNALIGN_TRP) enable/disable trapping on unaligned word/halfword accesses
    DivideByZeroTrapEnabled(bool),          // (DIV_0_TRP) enable/disable trapping on divide by 0
    PreciseDataAccessFaultIgnored(bool),    // (BFHFNMIGN) set lockup/ignored for precise data access faults at priorities -1 or -2
    Stack8ByteAligned(bool),                // (STKALIGN) guarantee 4-byte/8-byte stack alignment w/ SP adjustment
    DataCacheEnabled(bool),                 // (DC) enable/disable data and unified caches
    InsnCacheEnabled(bool),                 // (IC) enable/disable instruction caches
    BranchPredictionEnabled(bool),          // (BP) enable/disable program flow prediction

    // SHPR sets system handler priorities, needed by exception/interrupt handling system
    SetSystemHandlerPriority { id: u8, priority: u8}, // (PRI_x in SHPR1, SHPR2, or SHPR3) set system handler x's priority level
    
    // CFSR, MMFSR, BFSR, UFSR, HFSR
    FaultStatusClr(Fault),

    // special purpose registers PRIMASK, FAULTMASK, BASEPRI, CONTROL
    // SetCurrentExecPriority(i32)
}

impl<'irb> Context<'irb> {
    fn _process_events(&mut self) -> Result<(), context::Error> {
        while let Some(evt) = self.events.pop_front() {
            self._handle_event(evt).map_err(Into::<context::Error>::into)?;
        }
        Ok(())
    }

    #[allow(unused)]
    fn _handle_event(&mut self, evt: Event) -> Result<(), Error> {
        match evt {
            Event::ExceptionSetActive(exception_type, val) => {
                todo!()
            }
            Event::ExceptionSetPending(exception_type, val) => {
                todo!()
            }
            Event::ExceptionEnabled(exception_type, val) => {
                todo!()
            }
            Event::VectorTableOffsetWrite(offset) => {
                todo!()
            }
            Event::ExternSysResetRequest => {
                todo!()
            }
            Event::LocalSysResetRequest => {
                todo!()
            }
            Event::ExceptionClrAllActive => {
                // TODO: this must also clear the IPSR
                todo!()
            }
            Event::VectorKeyWrite => {
                todo!()
            }
            Event::SetPriorityGrouping(group) => {
                todo!()
            }
            Event::SetTransitionWakupEvent(val) => {
                todo!()
            }
            Event::SetDeepSleep(val) => {
                todo!()
            }
            Event::SetSleepOnExit(val) => {
                todo!()
            }
            Event::ThreadModeExceptionsEnabled(val) => {
                todo!()
            }
            Event::STIRUnprivilegedAccessAllowed(val) => {
                todo!()
            }
            Event::UnalignedAccessTrapEnabled(val) => {
                todo!()
            }
            Event::DivideByZeroTrapEnabled(val) => {
                todo!()
            }
            Event::PreciseDataAccessFaultIgnored(val) => {
                todo!()
            }
            Event::Stack8ByteAligned(val) => {
                todo!()
            }
            Event::DataCacheEnabled(val) => {
                todo!()
            }
            Event::InsnCacheEnabled(val) => {
                todo!()
            }
            Event::BranchPredictionEnabled(val) => {
                todo!()
            }
            Event::SetSystemHandlerPriority{ id, priority } => {
                todo!()
            }
            Event::FaultStatusClr(fault) => {
                todo!()
            }
        }
    }
}