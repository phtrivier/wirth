#![feature(assert_matches)]
use std::result::Result;

mod assembler_test;
mod assembler;

pub use crate::assembler::AssembleError;

use risc::instructions::*;
use crate::assembler::*;

pub fn assemble(input: &str) -> Result<Vec<Instruction>, AssembleError> {
  let mut assembler = Assembler::new();

  return match assembler.assemble(&input) {
    Ok(AssembleResult::Program) => {
      return Ok(assembler.instructions);
    },
    Err(err) => Err(err)
  }
}