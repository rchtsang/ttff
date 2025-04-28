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
        let mut highestpri: i16 = 256;
        let mut boostedpri: i16 = 256;
        let subgroupshift = self.get_aircr().prigroup();
        let groupvalue  = 0b10 << subgroupshift;

        for excp_type in self.nvic.active() {
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
}