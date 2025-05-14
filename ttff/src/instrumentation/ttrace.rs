//! ttrace.rs
//! 
//! taint trace plugin

use libcme::prelude::*;

use fugue_ir::disassembly::PCodeData;

#[derive(Debug)]
pub struct TaintTracePlugin {
    trace: Vec<Location>,
}

impl EvalPlugin for TaintTracePlugin {
    fn pre_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut dft::Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), dft::plugin::Error> {
        let mut has_tainted_inputs = false;
        for vnd in pcode.inputs.iter() {
            let (_val, tag) = context.read(vnd)
                .map_err(|e| {
                    dft::plugin::Error(e.into())
                })?;
            has_tainted_inputs |= tag.is_tainted();
        }

        if has_tainted_inputs {
            self.trace.push(loc.clone());
        }
        Ok(())
    }
}