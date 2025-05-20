//! plugin.rs
//! 
//! architecture plugin system for the shadow
use std::fmt;
use dyn_clone::{DynClone, clone_trait_object};

use fugue_bytes::Endian;
use fugue_arch::ArchitectureDef;
use super::*;

pub mod armv7m;

/// plugins for handling tag propagation that are tied
/// to arch-specific operations.
pub trait ArchPlugin: DynClone + fmt::Debug {
    /// performs tag propagation on thread context switch
    /// and returns the tag of the target address
    fn maybe_thread_switch(
        &mut self,
        shadow: &mut ShadowState,
        ctx_switch: &backend::ThreadSwitch,
    ) -> Result<Tag, Error>;
}
clone_trait_object!(ArchPlugin);


impl<'plugin> ArchPlugin for Box<dyn ArchPlugin + 'plugin> {
    fn maybe_thread_switch(
        &mut self,
        shadow: &mut ShadowState,
        ctx_switch: &backend::ThreadSwitch,
    ) -> Result<Tag, Error> {
        (**self).maybe_thread_switch(shadow, ctx_switch)
    }
}

pub fn plugin_from<'plugin>(arch: &ArchitectureDef) -> Box<dyn ArchPlugin + 'plugin> {
    let details = (
        arch.processor(),
        arch.endian(),
        arch.bits(),
        arch.variant(),
    );
    match details {
        ("ARM", Endian::Little, 32, "Cortex") => {
            Box::new(armv7m::Armv7m)
        }
        _ => {
            panic!("arch shadow plugin not implemented for {arch}")
        }
    }
}