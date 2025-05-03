/// policy module
/// 
/// implements dynamic taint policies

pub mod jump;
pub use jump::*;
pub mod address;
pub use address::*;
pub mod overflow;
pub use overflow::*;