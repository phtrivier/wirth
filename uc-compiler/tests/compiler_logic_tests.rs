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