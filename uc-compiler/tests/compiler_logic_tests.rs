use risc::instructions::*;
use risc::instructions::OpCode::*;

#[test]
fn compile_false_condition_test() {
  let content = String::from("
  MODULE Test;
      VAR x: INTEGER;
      BEGIN
        IF 0 = 1 THEN
          x:= 1
        END
    END Test.
  ");
  let instructions = compiler::compile(&content).unwrap();
  assert_eq!(
    instructions,
    vec![
      // Instructions for the program
      Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 0 },
      Instruction::RegisterIm { o: MOV, a: 1, b: 0, im: 1 },
      // NOTE(pht) SUB R2,R0,R1 or SUB R0,R0,R1 ??
      Instruction::Register{o: SUB, a: 0, b: 0, c: 1},
      Instruction::BranchOff{cond: BranchCondition::NE, offset: 2, link: false}, // NOTE(pht) the offset here means that we need to skip the instructions for the assignemnt, and will require fixup.
        // It's not obvious if the offset must be 2 or 3...

      // Instruction for the then branch, will not be taken
      Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 1 },
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

// TODO(pht) compile other parts