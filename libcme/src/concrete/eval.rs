//! eval.rs
//! 
//! concrete pcode evaluator

use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    
}


/// the evaluator operates on an evaluator state
pub trait EvalState {

}