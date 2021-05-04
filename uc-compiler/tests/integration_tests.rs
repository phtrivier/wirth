use risc::instructions::*;
use risc::instructions::OpCode::*;

#[test]
fn compile_program() {

  // Technically, this is a unit test, because it's not an entirely valid program *yet*,
  // but I suppose the full proper test would be:
  // let content_full = "
  //   MODULE Test;
  //     VAR x,y: INTEGER;
  //     BEGIN
  //       y:= 42;
  //       x:=y;
  //     END
  //   END Test
  // ";
  // And it should generate _almost_ the same instructions, or something very close.
  
  let content = String::from("y:=42;x:=y");
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
        offset: 1
      },
      Instruction::Memory {
        u: MemoryMode::Load,
        a: 0,
        b: 14,
        offset: 1
      },
      Instruction::Memory {
        u: MemoryMode::Store,
        a: 0,
        b: 14,
        offset: 0
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
