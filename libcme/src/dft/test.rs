//! test.rs
//! 
//! dft module tests

use crate::test::programs;

#[test]
fn test_smash_stack() -> Result<(), anyhow::Error> {
    let program = programs::STACK_SMASH_TEST;

    Ok(())
}