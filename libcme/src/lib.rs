//! libcme: cortex-m emulation library
//! 
//! 

pub mod backend;
// pub mod concrete;
pub mod programdb;
pub mod dft;
pub mod peripheral;
pub mod utils;
pub mod types;

#[cfg(test)]
mod test;

pub mod prelude {
    pub use fugue_core::prelude::*;
    pub use fugue_core::ir::Location;
    pub use fugue_ir::disassembly::IRBuilderArena;
    pub use super::peripheral::{
        self,
        Peripheral,
        PeripheralState,
    };
    pub use super::backend::{self, armv7m, Backend};
    pub use super::dft::{
        self,
        policy,
        tag,
    };
    pub use super::programdb::{
        self,
        ProgramDB,
        Block,
        Platform,
        Region,
    };
    pub use super::utils::*;
    pub use super::types::*;

    pub use super::programdb::plugin::AnalysisPlugin;
    pub use super::dft::plugin::EvalPlugin;
}