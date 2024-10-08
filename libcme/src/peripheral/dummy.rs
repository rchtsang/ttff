//! dummy.rs
//! 
//! a dummy peripheral implementation that acts like regular memory

use fugue_core::prelude::*;
use fugue_core::eval::fixed_state::FixedState;

use super::*;

#[derive(Clone)]
pub struct DummyState(FixedState);

impl Default for DummyState {
    fn default() -> Self {
        DummyState(FixedState::new(0x400usize))
    }
}

impl PeripheralState for DummyState {
    fn read_bytes(&mut self,
        address: &Address,
        dst: &mut [u8],
    ) -> Result<(), Error> {
        self.0.read_bytes(address.offset() as usize, dst)
            .map_err(|err| Error::state(err))
    }

    fn write_bytes(&mut self,
        address: &Address,
        src: &[u8],
    ) -> Result<(), Error> {
        self.0.write_bytes(address.offset() as usize, src)
            .map_err(|err| Error::state(err))
    }
}