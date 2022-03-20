#![feature(assert_matches)]
use std::result::Result;

mod assembler;
mod assembler_test;

pub use crate::assembler::AssembleError;

use crate::assembler::*;
use risc::instructions::*;

pub fn assemble(input: &str) -> Result<Vec<Instruction>, AssembleError> {
    let mut assembler = Assembler::new();

    match assembler.assemble(input) {
        Ok(AssembleResult::Program) => Ok(assembler.instructions),
        Err(err) => Err(err),
    }
}
