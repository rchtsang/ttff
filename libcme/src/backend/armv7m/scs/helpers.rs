//! helpers.rs
//! 
//! separating system control space helper functions

use crate::backend::armv7m::system::{BASEPRI, FAULTMASK, PRIMASK};

use super::*;

impl SysCtrlSpace {
    /// current exception priority
    /// from B1.5.4 page B1-529
    #[allow(unused)]
    pub fn current_priority(&self,
        basepri: &BASEPRI,
        primask: &PRIMASK,
        faultmask: &FAULTMASK,
    ) -> i16 {
        // priority of thread mode with no active exceptions
        // this value is PriorityMax + 1 = 256
        // (configurable priority maximum bit field is 8 bits)
        let mut highestpri: i16 = 256;
        // priority influence of basepri, primask, and faultmask
        let mut boostedpri: i16 = 256;

        let subgroupshift = self.get_aircr().prigroup();
        // used by priority grouping
        let groupvalue  = 0b10 << subgroupshift;

        // valid ipsr values should be in range of 2 to 511
        // to save time, we keep a list of active exceptions
        // instead of looping over the full range of exception values.
        // if desired, we can switch to looping to save memory and
        // removing nvic.active list
        for excp_type in self.exceptions.active() {
            let excp_num = u32::from(excp_type) as u8;
            let pri = self.nvic_regs()
                .get_ipr(excp_num / 4)
                .pri_n(excp_num % 4);
            if (pri as i16) < highestpri {
                highestpri = pri as i16;

                // include prigroup effect
                highestpri -= highestpri % groupvalue;
            }
        }

        if basepri.basepri() != 0 {
            boostedpri = basepri.basepri() as i16;

            // include prigroup effect
            boostedpri -= boostedpri % groupvalue;
        }

        if primask.pm() {
            boostedpri = 0;
        }

        if faultmask.fm() {
            boostedpri = -1;
        }

        if boostedpri < highestpri {
            boostedpri
        } else {
            highestpri
        }
    }

    pub fn get_exception_priority(
        &self,
        typ: ExceptionType,
    ) -> Option<i16> {
        let excp_num = u32::from(&typ);
        match excp_num {
             1 => { Some(-3) }
             2 => { Some(-2) }
             3 => { Some(-1) }
             4 => { Some(self.get_shpr1().pri_4() as i16) }
             5 => { Some(self.get_shpr1().pri_5() as i16) }
             6 => { Some(self.get_shpr1().pri_6() as i16) }
             7 => { Some(self.get_shpr1().pri_7() as i16) }
             8 => { Some(self.get_shpr2().pri_8() as i16) }
             9 => { Some(self.get_shpr2().pri_9() as i16) }
            10 => { Some(self.get_shpr2().pri_10() as i16) }
            11 => { Some(self.get_shpr2().pri_11() as i16) }
            12 => { Some(self.get_shpr3().pri_12() as i16) }
            13 => { Some(self.get_shpr3().pri_13() as i16) }
            14 => { Some(self.get_shpr3().pri_14() as i16) }
            15 => { Some(self.get_shpr3().pri_15() as i16) }
            16..=511 => {
                let int_num = excp_num - 16;
                let n = (int_num / 32) as u8;
                let i = (int_num % 32) as u8;
                Some(self.nvic_regs().get_ipr(n).pri_n(i) as i16)
            }
            _ => { panic!("invalid exception number: {excp_num}") }
        }
    }

    pub fn set_exception_priority(
        &mut self,
        typ: ExceptionType,
        pri: u8,
    ) {
        let excp_num = u32::from(&typ);
        match excp_num {
            1 => { panic!("cannot change {typ:?} priority") }
            2 => { panic!("cannot change {typ:?} priority") }
            3 => { panic!("cannot change {typ:?} priority") }
            4 => { self.get_shpr1_mut().set_pri_4(pri) }
            5 => { self.get_shpr1_mut().set_pri_5(pri) }
            6 => { self.get_shpr1_mut().set_pri_6(pri) }
            7 => { self.get_shpr1_mut().set_pri_7(pri) }
            8 => { self.get_shpr2_mut().set_pri_8(pri) }
            9 => { self.get_shpr2_mut().set_pri_9(pri) }
           10 => { self.get_shpr2_mut().set_pri_10(pri) }
           11 => { self.get_shpr2_mut().set_pri_11(pri) }
           12 => { self.get_shpr3_mut().set_pri_12(pri) }
           13 => { self.get_shpr3_mut().set_pri_13(pri) }
           14 => { self.get_shpr3_mut().set_pri_14(pri) }
           15 => { self.get_shpr3_mut().set_pri_15(pri) }
           16..=511 => {
               let int_num = excp_num - 16;
               let n = (int_num / 32) as u8;
               let i = (int_num % 32) as u8;
               self.nvic_regs_mut().get_ipr_mut(n).set_pri_n(i, pri)
           }
           _ => { panic!("invalid exception number: {excp_num}") }
        }
    }

    pub fn update_regs(&mut self) -> Result<(), super::Error> {
        // see B1.5.6 ExceptionTaken
        // don't know what status registers need to be updated
        Ok(())
    }
}


impl SysCtrlSpace {
    pub fn set_exception_pending(&mut self, typ: ExceptionType) {
        self.exceptions.set_pending(typ);
        if let Some(typ) = self.exceptions.pending().first().cloned() {
            self.get_icsr_mut().set_vectpending(u32::from(&typ));
        }
    }

    pub fn enable_exception(&mut self, typ: ExceptionType) {
        self.exceptions.enable(typ)
    }

    pub fn disable_exception(&mut self, typ: ExceptionType) {
        self.exceptions.disable(typ)
    }

    pub fn clr_exception_pending(&mut self, typ: ExceptionType) {
        self.exceptions.clr_pending(typ)
    }

    pub fn set_exception_active(&mut self, typ: ExceptionType) {
        self.exceptions.set_active(typ);
        let vectactive = self.exceptions.active()
            .first()
            .map(|t| u32::from(t))
            .unwrap_or(0);
        self.get_icsr_mut().set_vectactive(vectactive);
        let rettobase = self.exceptions.active().len() < 2;
        self.get_icsr_mut().set_rettobase(rettobase);
    }

    pub fn clr_exception_active(&mut self, typ: ExceptionType) {
        self.exceptions.clr_active(typ);
        let vectactive = self.exceptions.active()
            .first()
            .map(|t| u32::from(t))
            .unwrap_or(0);
        self.get_icsr_mut().set_vectactive(vectactive);
        let rettobase = self.exceptions.active().len() < 2;
        self.get_icsr_mut().set_rettobase(rettobase);
    }
}