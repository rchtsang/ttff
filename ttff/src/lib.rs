/// ttff
/// 
/// taint-triggered firmware fuzzing

pub mod policy;
pub mod instrumentation;
pub mod harness;

pub mod prelude {
    pub use crate::policy::{self, *};
    pub use crate::instrumentation::{self, *};
    pub use crate::harness::{self, *};

    pub use libafl_bolts::ownedref::OwnedSlice;
}