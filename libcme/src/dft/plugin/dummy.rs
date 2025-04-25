//! dummy.rs
//! 
//! a dummy plugin implementation

use crate::utils::*;
use super::*;

/// a dummy plugin with a number of various counters
#[derive(Debug, Default)]
pub struct DummyPlugin {
    pub pre_insn_cnt: usize,
    pub post_insn_cnt: usize,
    pub pre_pcode_cnt: usize,
    pub post_pcode_cnt: usize,
    pub read_access_cnt: usize,
    pub write_access_cnt: usize,
}

impl Plugin for DummyPlugin {
    #[instrument(skip_all)]
    fn pre_insn_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _insn: &Insn<'irb>,
        _context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        self.pre_insn_cnt += 1;
        info!("pre_insn_cb calls: {}", self.pre_insn_cnt);
        Ok(())
    }

    #[instrument(skip_all)]
    fn post_insn_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _insn: &Insn<'irb>,
        _context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        self.post_insn_cnt += 1;
        info!("post_insn_cb calls: {}", self.post_insn_cnt);
        Ok(())
    }

    #[instrument(skip_all)]
    fn pre_pcode_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _pcode: &PCodeData<'irb>,
        _context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        self.pre_pcode_cnt += 1;
        info!("pre_pcode_cb calls: {}", self.pre_pcode_cnt);
        Ok(())
    }

    #[instrument(skip_all)]
    fn post_pcode_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _pcode: &PCodeData<'irb>,
        _context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        self.post_pcode_cnt += 1;
        info!("post_pcode_cb calls: {}", self.post_pcode_cnt);
        Ok(())
    }

    #[instrument(skip_all)]
    fn mem_access_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _mem_address: &Address,
        _mem_size: usize,
        access_type: Permission,
        _value: &mut (BitVec, Tag),
        _context: &mut Context<'backend>,
    ) -> Result<(), Error> {
        match access_type {
            Permission::R => {
                self.read_access_cnt += 1;
                info!("read access count: {}", self.read_access_cnt);
            }
            Permission::W => {
                self.write_access_cnt += 1;
                info!("write access count: {}", self.write_access_cnt);
            }
            _ => { panic!("expected read or write permission to indicate load or store") }
        }
        Ok(())
    }
}