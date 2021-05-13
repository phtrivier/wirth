use risc::instructions::*;
use risc::instructions::OpCode::*;

#[test]
fn compile_assignement_program() {
  let content = String::from("
  MODULE Test;
      VAR x,y: INTEGER;
      BEGIN
        y:= 42;
        x:=y
    END Test.
  ");
  let instructions = compiler::compile(&content).unwrap();
  assert_eq!(
    instructions,
    vec![
      // Instructions for the program
      Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 42 },
      Instruction::Memory {
        u: MemoryMode::Store,
        a: 0,
        b: 14,
        offset: 2
      },
      Instruction::Memory {
        u: MemoryMode::Load,
        a: 0,
        b: 14,
        offset: 2
      },
      Instruction::Memory {
        u: MemoryMode::Store,
        a: 0,
        b: 14,
        offset: 1
      },
      // Footer to exit
      Instruction::RegisterIm {
        o: OpCode::MOV,
        a: 15,
        b: 0,
        im: 0
      },
      Instruction::Branch {
        cond: BranchCondition::AW,
        c: 15,
        link: false
      }
    ]
  )
}
