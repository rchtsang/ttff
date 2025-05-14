//! mem.rs
//! 
//! a pre-memory access intercept plugin
use libcme::prelude::*;

pub type MemCallback = dyn FnMut(
    &mut dft::Context,
    &Address,
    usize,
) -> Result<(), dft::plugin::Error>;

pub struct MemInterceptPlugin<'a> {
    pub callback: &'a mut MemCallback,
}

impl<'a> std::fmt::Debug for MemInterceptPlugin<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MemInterceptPlugin")
    }
}

impl<'a> dft::EvalPlugin for MemInterceptPlugin<'a> {

    #[instrument(skip_all)]
    fn pre_mem_access_cb<'irb, 'backend>(
        &mut self,
        _loc: &Location,
        mem_address: &Address,
        mem_size: usize,
        _access_type: Permission,
        context: &mut dft::Context<'backend>,
        _pdb: &mut ProgramDB<'irb>,
    ) -> Result<(), dft::plugin::Error> {
        if !context.backend().mmap().has_mapped(mem_address) {
            error!("encountered unmapped access @ {:#x}", mem_address.offset());
            (self.callback)(context, mem_address, mem_size)?;
        }
        Ok(())
    }
}