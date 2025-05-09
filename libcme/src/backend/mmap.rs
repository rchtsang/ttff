//! mmap.rs
//! 
//! memory map module
use std::ops::Range;
use std::collections::VecDeque;
use iset::IntervalMap;

use fugue_core::prelude::*;
use fugue_core::eval::fixed_state::FixedState;

use crate::types::*;
use crate::utils::*;
use crate::backend;
use crate::peripheral::{
    self,
    Peripheral,
};


#[derive(Clone, Copy, Debug)]
enum MapIx {
    Mem(usize),
    Mmio(usize),
}

/// memory map
#[derive(Default, Clone)]
pub struct MemoryMap {
    mmap: IntervalMap<Address, MapIx>,
    mem: Vec<FixedState>,
    mmio: Vec<Peripheral>,
}



impl MemoryMap {
    pub fn map_mem(
        &mut self,
        base: &Address,
        size: usize,
    ) -> Result<(), backend::Error> {
        let base = base.clone();
        // mapped memory must be word-aligned
        assert_eq!(base.offset() & 0b11, 0, "base {base:#x?} is not word-aligned!");
        assert_eq!(size & 0b11, 0, "size {size:#x} is not word-aligned!");

        // check for collision with existing mapped regions
        let range = base..(base + size as u64);
        if let Some(colliding) = self.mmap.intervals(range.clone()).next() {
            return Err(backend::Error::MapConflict(range, colliding));
        }

        // create memory and add to map
        let mem = FixedState::new(size);
        let idx = MapIx::Mem(self.mem.len());
        self.mem.push(mem);
        self.mmap.insert(range, idx);

        Ok(())
    }

    pub fn map_mmio(
        &mut self,
        peripheral: Peripheral,
    ) -> Result<(), backend::Error> {
        assert_eq!(peripheral.base_address().offset() & 0b11, 0,
            "peripheral is not word-aligned!");

        // check for collision with existing mapped regions
        for range in peripheral.ranges().iter() {
            if let Some(colliding) = self.mmap.intervals(range.clone()).next() {
                return Err(backend::Error::MapConflict(range.clone(), colliding));
            }
        }

        // add peripheral to map
        let idx = MapIx::Mmio(self.mmio.len());
        for range in peripheral.ranges().iter() {
            self.mmap.insert(range.clone(), idx);
        }
        self.mmio.push(peripheral);

        Ok(())
    }

    /// tick peripherals
    pub fn tick<E>(
        &mut self,
        events: &mut VecDeque<E>,
    ) -> Result<(), backend::Error>
    where
        E: From<peripheral::Event>,
    {
        for peripheral in self.mmio.iter_mut() {
            if let Some(evt) = peripheral.tick()? {
                events.push_back(E::from(evt));
            }
        }
        Ok(())
    }

    pub fn mapped(&self) -> impl Iterator<Item=MappedRange> + use<'_> {
        self.mmap.iter(..)
            .map(|(range, ix)| {
                match ix {
                    MapIx::Mem(_) => { MappedRange::Mem(range.clone()) }
                    MapIx::Mmio(_) => { MappedRange::Mmio(range.clone()) }
                }
            })
    }

    #[instrument(skip_all)]
    pub fn load_bytes<E>(
        &mut self,
        address: &Address,
        dst: &mut [u8],
        events: &mut VecDeque<E>,
    ) -> Result<(), backend::Error>
    where
        E: From<peripheral::Event>,
    {
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.read_bytes(offset, dst)
                    .map_err(backend::Error::from)
            }
            MapIx::Mmio(idx) => {
                let peripheral = self.mmio.get_mut(idx).unwrap();
                let mut peripheral_events = VecDeque::new();
                let result = peripheral.read_bytes(address, dst, &mut peripheral_events);
                for peripheral_event in peripheral_events {
                    events.push_back(peripheral_event.into());
                }
                if let Err(peripheral::Error::InvalidPeripheralReg(address)) = result {
                    let offset = address.offset();
                    warn!("warning: ignoring unimplemented peripheral register @ {offset:#x}");
                    Ok(())
                } else {
                    result.map_err(backend::Error::from)
                }
            }
        }
    }

    #[instrument(skip_all)]
    pub fn store_bytes<E>(
        &mut self,
        address: &Address,
        src: &[u8],
        events: &mut VecDeque<E>,
    ) -> Result<(), backend::Error>
    where
        E: From<peripheral::Event>,
    {
        let (range, val) = self._get_mapped_region(address.clone())?;
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get_mut(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.write_bytes(offset, src)
                    .map_err(backend::Error::from)
            }
            MapIx::Mmio(idx) => {
                let peripheral = self.mmio.get_mut(idx).unwrap();
                let mut peripheral_events = VecDeque::new();
                let result = peripheral.write_bytes(address, src, &mut peripheral_events);
                for peripheral_event in peripheral_events {
                    events.push_back(peripheral_event.into());
                }
                if let Err(peripheral::Error::InvalidPeripheralReg(address)) = result {
                    let offset = address.offset();
                    warn!("warning: ignoring unimplemented peripheral register @ {offset:#x}");
                    Ok(())
                } else {
                    result.map_err(backend::Error::from)
                }
            }
        }
    }

    pub fn mem_view_bytes(&self, address: &Address, size: Option<usize>) -> Result<&[u8], backend::Error> {
        let (range, val) = self._get_mapped_region(address.clone())?;
        let size = size.unwrap_or((range.end.offset() - range.start.offset()) as usize);
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes(offset, size)
                    .map_err(backend::Error::from)
            }
            MapIx::Mmio(_idx) => {
                panic!("mmio peripherals can't implement view_bytes due to their send/receive data model")
            }
        }
    }

    pub fn mem_view_bytes_mut(&mut self, address: &Address, size: Option<usize>) -> Result<&mut [u8], backend::Error> {
        let (range, val) = self._get_mapped_region(address.clone())?;
        let size = size.unwrap_or((range.end.offset() - range.start.offset()) as usize);
        match val {
            MapIx::Mem(idx) => {
                let state = self.mem.get_mut(idx).unwrap();
                let offset = (*address - range.start).offset() as usize;
                state.view_bytes_mut(offset, size)
                    .map_err(backend::Error::from)
            }
            MapIx::Mmio(_idx) => {
                panic!("mmio peripherals can't implement view_bytes_mut due to their send/receive data model")
            }
        }
    }


    fn _get_mapped_region(&self, address: impl Into<Address>) -> Result<(Range<Address>, MapIx), backend::Error> {
        let address: Address = address.into();
        let mut overlaps = self.mmap.overlap(address.clone());
        let (range, val) = overlaps.next()
            .ok_or(backend::Error::Unmapped(address.clone()))?;
        if let Some((other_range, _)) = overlaps.next() {
            return Err(backend::Error::MapConflict(range, other_range));
        }
        Ok((range, val.clone()))
    }

}