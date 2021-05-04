#[cfg(test)]
mod tests {

  use crate::assembler::*;
  use risc::instructions::*;
  use risc::instructions::Instruction::*;
  

  #[test]
  fn it_ignores_comment_lines() {
    let mut a = Assembler::new();
    let parsed = a.parse_line(0, "* A comment should be ignored");
    assert_eq!(Ok(ParseResult::Comment), parsed);
    assert!(a.instructions.is_empty());
  }

  #[test]
  fn it_parses_symbol_definition() {
    let mut a = Assembler::new();
    let parsed = a.parse_line(0, "#FOO 42");
    assert_eq!(Ok(ParseResult::SymbolDef), parsed);
    assert!(a.instructions.is_empty());
    assert_eq!(a.symbols["#FOO"], 42);
  }

  #[test]
  #[allow(unused)]
  fn it_maintains_origin() {
    let mut a = Assembler::new();
    a.parse_line(0, "#FOO 42");
    a.parse_line(1, "#BAR 50");
    a.parse_line(2, "MOV R0,42");
    assert_eq!(1, a.instructions.len());
  }

  #[test]
  #[allow(unused)]
  fn it_parses_an_instruction_with_address() {
    let mut a = Assembler::new();
    a.instruction_indexes.insert(String::from("@START"), 1);
    a.parse_line(0, "#FOO 42");
    a.parse_line(1, "#BAR 50");
    a.parse_line(2, "MOV R0,32");
    let parsed = a.parse_line(3, "@START MOV R1,42 ; do some stuff");
    assert_eq!(a.instruction_indexes["@START"], 1);
    assert_eq!(Ok(ParseResult::Instruction), parsed);

    assert_eq!(
      Instruction::RegisterIm {
        o: OpCode::MOV,
        a: 0,
        b: 0,
        im: 32
      },
      a.instructions[0]
    );

    assert_eq!(
      Instruction::RegisterIm {
        o: OpCode::MOV,
        a: 1,
        b: 0,
        im: 42
      },
      a.instructions[1]
    )
  }

  #[test]
  fn it_converts_lines_to_instructions() {
    let tests = [
      (
        "MOV R1,32",
        RegisterIm {
          o: OpCode::MOV,
          a: 1,
          b: 0,
          im: 32,
        },
      ),
      (
        "MOV R0,#FOO ; Some comment",
        RegisterIm {
          o: OpCode::MOV,
          a: 0,
          b: 0,
          im: 32,
        },
      ),
      ("MOV R1,R2", Register { o: OpCode::MOV, a: 1, b: 0, c: 2 }),
      (
        "ADD R5,R12,#FOO ; Some comment",
        RegisterIm {
          o: OpCode::ADD,
          a: 5,
          b: 12,
          im: 32,
        },
      ),
      (
        "ADD R5,R12,R2 ; Some comment",
        Register {
          o: OpCode::ADD,
          a: 5,
          b: 12,
          c: 2,
        },
      ),
      (
        "LDW R0,R1,#FOO",
        Memory {
          u: MemoryMode::Load,
          a: 0,
          b: 1,
          offset: 32,
        },
      ),
      (
        "STW R0,R1,42",
        Memory {
          u: MemoryMode::Store,
          a: 0,
          b: 1,
          offset: 42,
        },
      ),
      (
        "BNE R1",
        Branch {
          cond: BranchCondition::NE,
          c: 1,
          link: false
        }
      ),
      (
        "BEQL R1",
        Branch {
          cond: BranchCondition::EQ,
          c: 1,
          link: true
        }
      ),
      (
        "BNE #FOO",
        BranchOff {
          cond: BranchCondition::NE,
          offset: 32,
          link: false
        }
      ),
      (
        "BGTL -42",
        BranchOff {
          cond: BranchCondition::GT,
          offset: -42,
          link: true
        }
      ),
      (
        "B @END",
        BranchOff {
          cond: BranchCondition::AW,
          offset: 50, // A new assembler is created each one, so the instruction index is 0
          link: false
        }
      )
    ];
    for test in tests.iter() {
      let mut a = Assembler::new();
      a.symbols.insert("#FOO".to_string(), 32);
      a.instruction_indexes.insert("@END".to_string(), 50);
      a.line_index = 30;
      
      let (line, expected) = test;
      let parsed = a.parse_instruction(line);
      assert!(parsed.is_ok(), "Error parsing line: {}", line);
      assert_eq!(*expected, parsed.unwrap(), "Invalid translation for {}", line);
    }
  }

}
