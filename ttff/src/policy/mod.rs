/// policy module
/// 
/// implements dynamic taint policies

pub mod jump;
pub use jump::{
    PolicyViolation as JumpPolicyViolation,
    TaintedJumpPolicy,
};
pub mod address;
pub use address::{
    PolicyViolation as AddressPolicyViolation,
    TaintedAddressPolicy,
};
pub mod overflow;
pub use overflow::{
    PolicyViolation as OverflowPolicyViolation,
    TaintedOverflowPolicy,
};