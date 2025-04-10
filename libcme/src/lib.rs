//! libcme: cortex-m emulation library
//! 
//! 

pub mod backend;
pub mod concrete;
pub mod dft;
pub mod peripheral;
pub mod utils;
pub mod types;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use fugue_core::prelude::*;
    // pub use super::concrete::{
    //     context::{
    //         self,
    //         Permission,
    //         Alignment,
    //         CtxRequest,
    //         CtxResponse,
    //         Context as ContextTrait,
    //     },
    //     context::arch::{
    //         self,
    //         armv7m,
    //     },
    //     eval::{
    //         self,
    //         FlowType,
    //         Evaluator,
    //         bv2addr,
    //         bool2bv,
    //     },
    // };
    pub use super::peripheral::{
        self,
        Peripheral,
        PeripheralState,
    };
    pub use super::utils::*;
}