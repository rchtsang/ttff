//! dummy.rs
//! 
//! a dummy plugin implementation
use nohash::IntMap;

use crate::utils::*;
use super::*;

/// a dummy plugin with a number of various counters
#[derive(Debug, Default)]
pub struct DummyEvalPlugin {
    pub thread_switch_cnt: usize,
    pub pre_insn_cnt: usize,
    pub post_insn_cnt: usize,
    pub pre_pcode_cnt: usize,
    pub post_pcode_cnt: usize,
    pub read_access_cnt: usize,
    pub write_access_cnt: usize,
    pub userops_called: IntMap<usize, usize>,
}

impl EvalPlugin for DummyEvalPlugin {
    #[instrument(skip_all)]
    fn post_thread_switch_cb<'irb, 'backend>(
        &mut self,
        _thd_switch: &ThreadSwitch,
        _context: &mut Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), Error> {
        self.thread_switch_cnt += 1;
        info!("post_thread_switch_cb calls: {}", self.thread_switch_cnt);
        Ok(())
    }

    #[instrument(skip_all)]
    fn pre_insn_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        _insn: &Insn<'irb>,
        _context: &mut Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
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
        _flow: &Flow,
        _context: &mut Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
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
        _pdb: &mut ProgramDB<'irb>,
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
        _pdb: &mut ProgramDB<'irb>,
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
        _pdb: &mut ProgramDB<'irb>,
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

    #[instrument(skip_all)]
    fn pre_userop_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        index: usize,
        _inputs: &[VarnodeData],
        _output: Option<&VarnodeData>,
        _context: &mut Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), Error> {
        info!("called userop: {}", index);
        *(self.userops_called.entry(index).or_insert(0)) += 1;
        Ok(())
    }
}