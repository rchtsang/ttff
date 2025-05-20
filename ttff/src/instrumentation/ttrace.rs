//! ttrace.rs
//! 
//! taint trace plugin
use libcme::prelude::*;

use fugue_ir::disassembly::PCodeData;

#[derive(Debug, Default)]
pub struct TaintTracePlugin {}

impl EvalPlugin for TaintTracePlugin {

    #[instrument(skip_all)]
    fn post_pcode_cb<'irb, 'backend>(
        &mut self,
        loc: &Location,
        pcode: &PCodeData<'irb>,
        context: &mut dtt::Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), dtt::plugin::Error> {
        let mut is_tainted = false;
        for vnd in pcode.inputs.iter() {
            let (_val, tag) = context.read(vnd)
                .map_err(|e| {
                    dtt::plugin::Error(e.into())
                })?;
            is_tainted |= tag.is_tainted();
        }
        if let Some(ref vnd) = pcode.output {
            let (_val, tag) = context.read(vnd)
                .map_err(|e| {
                    dtt::plugin::Error(e.into())
                })?;
            is_tainted |= tag.is_tainted();
        }

        if is_tainted {
            warn!("TAINTED LOCATION: {:#010x}-{}",
                loc.address().offset(), loc.position());
        }
        Ok(())
    }
}