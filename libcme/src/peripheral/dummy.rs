//! dummy.rs
//! 
//! a dummy peripheral implementation that acts like regular memory

use fugue_core::prelude::*;
use fugue_core::eval::fixed_state::FixedState;

use super::*;

#[derive(Clone)]
pub struct DummyState {
    base: Address,
    backing: FixedState,
}

impl DummyState {
    pub fn new_with(base: impl Into<Address>, size: usize) -> Self {
        Self {
            base: base.into(),
            backing: FixedState::new(size),
        }
    }
}

impl PeripheralState for DummyState {
    fn base_address(&self) -> Address {
        self.base.clone()
    }

    fn size(&self) -> u64 {
        self.backing.len() as u64
    }

    fn read_bytes(&mut self,
        address: &Address,
        dst: &mut [u8],
    ) -> Result<(), Error> {
        self.backing.read_bytes(address.offset() as usize, dst)
            .map_err(|err| Error::state(err))
    }

    fn write_bytes(&mut self,
        address: &Address,
        src: &[u8],
    ) -> Result<(), Error> {
        self.backing.write_bytes(address.offset() as usize, src)
            .map_err(|err| Error::state(err))
    }
}