//! exception.rs
//! 
//! manage interrupt and exception state

use super::*;


#[allow(unused)]
/// state for nested vector interrupt controller
#[derive(Default, Debug, Clone)]
pub struct ExceptionState {
    enabled: Vec<ExceptionType>,
    pending: Vec<ExceptionType>,
    active: Vec<ExceptionType>,
}

impl ExceptionState {

    /// enable an exception
    #[instrument(skip_all)]
    pub fn enable(&mut self, typ: ExceptionType) {
        debug!("enable {typ:?}");
        let mut idx = 0;
        for t in self.enabled.iter() {
            if *t < typ {
                idx += 1;
                continue;
            } else if *t == typ {
                // exception already enabled
                return;
            } else {
                break;
            }
        }
        self.enabled.insert(idx, typ);
    }

    /// disable an exception
    #[instrument(skip_all)]
    pub fn disable(&mut self, typ: ExceptionType) {
        debug!("disable {typ:?}");
        let idx = self.enabled.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.enabled.remove(idx);
        }
    }

    /// set an exception as pending
    #[instrument(skip_all)]
    pub fn set_pending(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("set pending {typ:?}");
        let mut idx = 0;
        for t in self.pending.iter() {
            if *t < typ {
                idx += 1;
                continue;
            } else if *t == typ {
                // exception already pending
                return;
            } else {
                break;
            }
        }
        self.pending.insert(idx, typ);
    }

    /// clr a pending interrupt (does nothing if not pending)
    /// will not reorder exception queue
    #[instrument(skip_all)]
    pub fn clr_pending(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("clr pending {typ:?}");
        let idx = self.pending.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.pending.remove(idx);
        }
    }

    /// set an exception as active
    /// exception will not be set active unless it is enabled per B3.4.1
    #[instrument(skip_all)]
    pub fn set_active(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("set active {typ:?}");
        assert!(self.pending.contains(&typ),
            "interrupt must be pending before becoming active");
        self.clr_pending(typ);
        let mut idx = 0;
        for t in self.active.iter() {
            if *t < typ {
                idx += 1;
                continue;
            } else if *t == typ {
                return;
            } else {
                break;
            }
        }
        self.active.insert(idx, typ);
    }

    /// clear an exception
    #[instrument(skip_all)]
    pub fn clr_active(
        &mut self,
        typ: ExceptionType,
    ) {
        debug!("clr active {typ:?}");
        let idx = self.active.iter()
            .position(|t| *t == typ);
        if let Some(idx) = idx {
            self.active.remove(idx);
        }
    }

    pub fn active(&self) -> &[ExceptionType] {
        &self.active
    }

    pub fn pending(&self) -> &[ExceptionType] {
        &self.pending
    }

    pub fn enabled(&self) -> &[ExceptionType] {
        &self.enabled
    }
}