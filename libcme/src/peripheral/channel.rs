//! channel.rs
//! 
//! a peripheral implementation that sends and receives
//! via crossbeam channel
use std::collections::VecDeque;
use anyhow;
use thiserror::Error;
use crossbeam::channel::{
    Sender,
    Receiver,
    TrySendError,
    TryRecvError,
};

use fugue_core::prelude::*;
use crate::peripheral::{
    self,
    PeripheralState,
    Event,
};

#[derive(Debug, Error)]
pub enum ChannelStateError {
    #[error("log error: {0:?}")]
    Log(Address, TrySendError<(Address, usize, bool)>),
    #[error("send error: {0:?}")]
    Send(Address, TrySendError<u8>),
    #[error("recv error: {0:?}")]
    Recv(Address, TryRecvError),
}

impl From<ChannelStateError> for peripheral::Error {
    fn from(err: ChannelStateError) -> Self {
        Self::State(anyhow::Error::from(err))
    }
}

#[derive(Clone)]
pub struct ChannelPeripheral {
    base: Address,
    size: usize,
    access_log: Sender<(Address, usize, bool)>,
    read_src: Receiver<u8>,
    write_dst: Sender<u8>,
}

impl ChannelPeripheral {
    pub fn new_with(
        base: impl Into<Address>,
        size: usize,
        access_log: Sender<(Address, usize, bool)>,
        read_src: Receiver<u8>,
        write_dst: Sender<u8>,
    ) -> Self {
        let base = base.into();
        Self { base, size, access_log, read_src, write_dst }
    }
}

impl PeripheralState for ChannelPeripheral {
    fn base_address(&self) -> Address {
        self.base.clone()
    }

    fn size(&self) -> u64 {
        self.size as u64
    }

    fn tick(&mut self) -> Result<Option<Event>, peripheral::Error> {
        Ok(None)
    }

    fn read_bytes(
        &mut self,
        address: &Address,
        dst: &mut [u8],
        _events: &mut VecDeque<Event>,
    ) -> Result<(), peripheral::Error> {
        assert!(self.base <= *address && *address <= self.base + self.size,
            "address out of range: {address:#x?}");
        self.access_log.try_send((address.clone(), dst.len(), false))
            .map_err(|err| {
                ChannelStateError::Log(address.clone(), err)
            })?;
        for i in 0..dst.len() {
            dst[i] = self.read_src.try_recv()
                .map_err(|err| {
                    let addr = *address + i as u64;
                    ChannelStateError::Recv(addr, err)
                })?;
        }
        Ok(())
    }

    fn write_bytes(
        &mut self,
        address: &Address,
        src: &[u8],
        _events: &mut VecDeque<Event>,
    ) -> Result<(), peripheral::Error> {
        assert!(self.base <= *address && *address <= self.base + self.size,
            "address out of range: {address:#x?}");
        self.access_log.try_send((address.clone(), src.len(), true))
            .map_err(|err| {
                ChannelStateError::Log(address.clone(), err)
            })?;
        for (i, byte) in src.iter().cloned().enumerate() {
            self.write_dst.try_send(byte)
                .map_err(|err| {
                    let addr = *address + i as u64;
                    ChannelStateError::Send(addr, err)
                })?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use anyhow;
    use crate::prelude::*;
    use crate::utils::*;
    use super::*;

    #[test]
    fn test_channel_state_in_context() -> Result<(), anyhow::Error> {
        use crossbeam::channel::unbounded;
        use fugue_core::prelude::*;
        use crate::backend::{self, armv7m};

        let global_sub = compact_dbg_logger();
        set_global_default(global_sub)
            .expect("failed to set tracing default logger");

        info!("creating language builder...");
        let builder = LanguageBuilder::new("data/processors")?;

        info!("building backend...");
        let mut backend = armv7m::Backend::new_with(&builder, None)?;

        info!("mapping channel peripheral...");
        let (access_log_send, access_log_recv) = unbounded();
        let (read_src_send, read_src_recv) = unbounded();
        let (write_dst_send, write_dst_recv) = unbounded();

        // initializing data for peripheral byte reads
        let bytes: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        for byte in bytes.iter().cloned() {
            read_src_send.try_send(byte)?;
        }

        let channel_peripheral = ChannelPeripheral::new_with(
            Address::from(0x40001000u32),
            0x1000,
            access_log_send,
            read_src_recv,
            write_dst_send,
        );
        let channel_peripheral = Peripheral::new_with(Box::new(channel_peripheral));
        backend.map_mmio(channel_peripheral)?;

        info!("testing load bytes");
        let mut dst: [u8; 4] = [0, 0, 0, 0];
        let read_addr = Address::from(0x40001040u32);
        backend.load_bytes(
            &read_addr,
            &mut dst,
        )?;
        let access = access_log_recv.try_recv()?;
        assert_eq!(access, (read_addr, 4, false),
            "unexpected access log value: {:#x?}", access);
        assert_eq!(dst, bytes,
            "read bytes do not match expected: {:#x?} != {:#x?}",
            dst, bytes);
        
        let src: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
        let write_addr = Address::from(0x40001048u32);
        backend.store_bytes(
            &write_addr,
            &src,
        )?;
        let access = access_log_recv.try_recv()?;
        let mut bytes: [u8; 4] = [0, 0, 0, 0];
        for byte in bytes.iter_mut() {
            *byte = write_dst_recv.try_recv()?;
        }
        assert_eq!(access, (write_addr, 4, true),
            "unexpected access log value: {:#x?}", access);
        assert_eq!(src, bytes,
            "write bytes do not match expected: {:#x?} != {:#x?}",
            src, bytes);

        match backend.load_bytes(&read_addr, &mut dst) {
            Err(err) => {
                assert!(matches!(err, backend::Error::Peripheral(_)),
                    "expected peripheral error");
            }
            Ok(()) => {
                let msg = "expected peripheral error, got Ok(())";
                return Err(anyhow::Error::msg(msg));
            }
        }
        Ok(())
    }
}