//! sc.rs
//! 
//! single channel dft executor harness
use libcme::{
    self,
    prelude::*,
    programdb::ProgramDB,
};

use libafl::{
    executors::{Executor, ExitKind},
    state::{HasCorpus, HasExecutions},
    inputs::HasTargetBytes,
};
use libafl_bolts::ownedref::OwnedSlice;

pub type HaltFn = fn(&DftExecutor, &mut dft::Context) -> Option<ExitKind>;
pub type InputFn = fn(&mut DftExecutor, OwnedSlice<u8>) -> Result<(), super::Error>;


/// a dft executor for channel-based peripherals
/// 
/// the base_context should be initialized at the point where
/// fuzzing should begin, so it must already be initialized for execution.
/// 
/// if cycle limit is None, then there is no limit.
pub struct DftExecutor<'policy, 'backend, 'irb> {
    /// an optional cycle count limit
    limit: Option<usize>,
    /// an optional halt condition callback
    halt_fn: Option<HaltFn>,
    /// a mandatory function for defining how the executor should feed the
    /// input stream to the context
    input_fn: InputFn,
    evaluator: dft::Evaluator<'policy>,
    base_context: dft::Context<'backend>,
    pdb: ProgramDB<'irb>,
}

impl<'policy, 'backend, 'irb> DftExecutor<'policy, 'backend, 'irb> {
    pub fn new_with(
        evaluator: dft::Evaluator<'policy>,
        base_context: dft::Context<'backend>,
        pdb: programdb::ProgramDB<'irb>,
        input_fn: InputFn,
        limit: Option<usize>,
        halt_fn: Option<HaltFn>,
    ) -> Self {
        Self { evaluator, base_context, pdb, limit, halt_fn, input_fn }
    }
}


impl<'p, 'b, 'a, EM, I, S, Z> Executor<EM, I, S, Z> for DftExecutor<'p, 'b, 'a>
where
    S: HasCorpus<I> + HasExecutions,
    I: HasTargetBytes,
{
    #[instrument(skip_all)]
    fn run_target(
        &mut self,
        _fuzzer: &mut Z,
        state: &mut S,
        _mgr: &mut EM,
        input: &I,
    ) -> Result<ExitKind, libafl::Error> {
        // track execution count
        *state.executions_mut() += 1;

        let mut context = self.base_context.clone();
        let should_halt = self.halt_fn.unwrap_or(|_, _| None);

        let input_bytes = input.target_bytes();
        (self.input_fn)(self, input_bytes)
            .map_err(|err| {
                libafl::Error::unknown(format!("{err:?}"))
            })?;

        let mut cycles: usize = 0;
        while self.limit.is_none() || cycles < self.limit.unwrap() {
            let result = self.evaluator.step(&mut context, &mut self.pdb);
            match result {
                Err(dft::eval::Error::Policy(err)) => {
                    // policy violation
                    error!("execution {:>4}: policy violation: {err:?}",
                        *state.executions());
                    return Ok(ExitKind::Crash);
                }
                Err(err) => {
                    // other evaluation/emulation error
                    error!("execution {:>4}: other error: {err:?}",
                        *state.executions());
                    return Ok(ExitKind::Crash);
                }
                _ => {
                    cycles += 1;
                    if let Some(kind) = should_halt(&self, &mut context) {
                        return Ok(kind);
                    }
                }
            }
        }
        Ok(ExitKind::Timeout)
    }
}