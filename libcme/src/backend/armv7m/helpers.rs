//! helpers.rs
//! 
//! implementation of helper functions that require full backend context

use super::*;

impl Backend {
    /// returns true if the event is a WFE wakeup event based on current
    /// processor state (see B1.5.18)
    #[instrument]
    pub fn is_wfe_wakeup_evt(&self, evt: &Event) -> bool {
        match evt {
            // execution of a SEV instruction on any processor in a multiprocessor system
            Event::SEVInstructionExecuted => {
                true
            }
            // any exception entering the pending state if SCR.SEVONPEND is set.
            Event::ExceptionSetPending(_typ, true)
            if self.scs.get_scr().sevonpend() => {
                true
            }
            // an asynchronous exception at a priority that preempts any currently
            // active exceptions
            Event::ExceptionSetActive(typ, true)
            | Event::ExceptionSetPending(typ, true) => {
                let Some(priority) = self.scs.get_exception_priority(*typ) else {
                    warn!("processor may be in inconsistent state: no exception registered for exception: {typ:?}");
                    return false;
                };
                
                priority < self.scs.current_priority(&self.basepri, &self.primask, &self.faultmask)
            }
            // a debug event with debug enabled
            Event::Debug(_) => {
                let offset = DebugRegType::DHCSR.offset() / 4;
                let backing: &[u32; 0x400] = self.scs.as_ref();
                let dhcsr = DHCSR::from_bits(backing[offset]);
                dhcsr.s_halt()
            }
            _ => { false }
        }
    }

    /// returns true if the event is a WFI wakeup event based on current
    /// processor state (see B1.5.19)
    pub fn is_wfi_wakeup_evt(&self, evt: &Event) -> bool {
        match evt {
            // reset
            Event::ExternSysResetRequest
            | Event::LocalSysResetRequest => {
                true
            }
            // asynchronous exception at a priority that would preempt any 
            // currently active exception if PRIMASK were 0, (actual value of
            // PRIMASK is ignored)
            Event::ExceptionSetActive(typ, true)
            | Event::ExceptionSetPending(typ, true) => {
                let Some(priority) = self.scs.get_exception_priority(*typ) else {
                    warn!("processor may be in inconsistent state: no exception registered for exception: {typ:?}");
                    return false;
                };
                
                let vecactive = self.scs.get_icsr().vectactive();
                let current_typ: ExceptionType = vecactive.into();
                let Some(unmasked_current_priority) = self.scs.get_exception_priority(current_typ) else {
                    panic!("processor is in inconsistent state: no exception registered for current exception: {current_typ:?}");
                };

                priority < unmasked_current_priority
            }
            // a debug event with debug enabled
            Event::Debug(_) => {
                let offset = DebugRegType::DHCSR.offset() / 4;
                let backing: &[u32; 0x400] = self.scs.as_ref();
                let dhcsr = DHCSR::from_bits(backing[offset]);
                dhcsr.s_halt()
            }
            // other implementation-defined events

            _ => { false }
        }
    }
}