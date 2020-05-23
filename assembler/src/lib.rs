/*
A simple assembler for the risc computer.

The goal is to simplify writing programs for tests.

 I want to be able to write things like:

  ```
  * Compute the first 3 squares.
  * R0 accumulates the results
  * R1 is the loop index
  * Results are put in memory starting at address 100 (arbitrarily)
  #MAX      3
  #OUT      100 ; locations for the squares
            MOVI R1,0,1     ; Init Loop index
  @LOOP     MOV  R0,0,R1    ; R0 <- R1
            MUL  R0,R0,R0   ; R0 <- R0 * R0
            STW  R0,R1,#OUT ; MEM[OUT+I] <- R0
            CMPI R1,#MAX    ; Test loop finished
            BGT  @END       ; If R1 > 3, terminates
            ADD  R1,0,1     ; Otherwise, Increase R1 and continue loop
            RET  @LOOP
  @END      MOVI R0,0 ; Terminates
            RET  R0
  ```

The goal is to parse a list of lines, and fror each line:
* ignore those that starts with * -> Some(Comment)
* scrap everything in the lines that's after a `;`
* treat a line line #X Value as adding a symbol in a map (symbol vs line) -> Some(SymbolDefinition("X", 3)
* treat the first line that's not a symbold def as the `origin` address of the program
* treat a line line starting with @Y as adding an well known address (and keep its displacement from the origin)
* than try to parse the content of the line as an instruction
  - Separate by any form of spaces, you should have [opcode, content]
  - To parse the content:
    - Replace anything like Rx by the corresponding integer
    - Replace anything like @Y by the difference between the current instruction index and Y

This should generate, in the end, a vec of Instruction.
We can then `encode` those instructions into a vec of i32 that can be memcopied into a Computer structure, for execution.
*/

use risc::instructions::*;
use std::str::FromStr;

use std::collections::HashMap;

#[allow(dead_code)]
struct Assembler {
    pub instructions: Vec<Instruction>,
    pub origin: Option<u32>,           // First line in the program that contains instructions
    pub symbols: HashMap<String, u32>, // Map of symbols like #FOO to their values
    pub disps: HashMap<String, u32>,   // Map of symbols like @BAR to displacements from origin
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseError {
    SyntaxError { index: u32 },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseResult {
    Comment,
    SymbolDef,
    Instruction,
}

impl Assembler {
    #[allow(dead_code)]
    pub fn new<'a>() -> Assembler {
        let mut symbols = HashMap::new();

        for i in 0..16 {
            symbols.insert(format!("R{}", i), i);
        }

        println!("{:?}", symbols["R0"]);

        Assembler {
            origin: None,
            instructions: vec![],
            symbols: symbols,
            disps: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn parse_line(&mut self, index: u32, line: &str) -> Result<ParseResult, ParseError> {
        let l = line.trim_start().trim_end_matches(';').trim_end();

        if l.starts_with("*") {
            return Ok(ParseResult::Comment);
        } else if l.starts_with("#") {
            return self.parse_symbol_def(index, l);
        }

        if self.origin == None {
            self.origin = Some(index);
        }

        if let Ok(instruction) = self.parse_instruction(index, l) {
            self.instructions.push(instruction);
            return Ok(ParseResult::Instruction);
        }
        return Err(ParseError::SyntaxError { index: index });
    }

    fn parse_symbol_def(&mut self, index: u32, line: &str) -> Result<ParseResult, ParseError> {
        let mut tokens = line.split_ascii_whitespace();

        if let Some(symbol) = tokens.next() {
            if let Some(value_str) = tokens.next() {
                if let Ok(value) = value_str.parse::<u32>() {
                    self.symbols.insert(symbol.to_string(), value);
                    return Ok(ParseResult::SymbolDef);
                }
            }
        }
        return Err(ParseError::SyntaxError { index: index });
    }

    fn parse_instruction(&mut self, index: u32, line: &str) -> Result<Instruction, ParseError> {
        let mut tokens = line.split_ascii_whitespace();

        if let Some(symbol) = tokens.next() {
            if symbol.starts_with("@") {
                if let Some(origin) = self.origin {
                    let disp = index - origin;
                    self.disps.insert(symbol.to_string(), disp);
                    if let Some(op) = tokens.next() {
                        if let Some(params) = tokens.next() {
                            return self.parse_op_params(index, op, params);
                        }
                    }
                }
            } else {
                let op = symbol;
                if let Some(params) = tokens.next() {
                    return self.parse_op_params(index, op, params);
                }
            }
        }
        return Err(ParseError::SyntaxError { index: index });
    }

    fn parse_op_params(&mut self, index: u32, op: &str, params: &str) -> Result<Instruction, ParseError> {
        if let Ok(op) = RegisterOpCode::from_str(op) {

            // TODO(pht) CMP

            if let Some((a,b,c)) = self.parse_params_a_b_c(params) {
                let instruction = Instruction::Register { o: op, a, b, c };
                return Ok(instruction);
            }
        }

        if let Ok(op) = RegisterImOpCode::from_str(op) {

            match op {
                RegisterImOpCode::CHKI => if let Some((a,b,im)) = self.parse_params_a_im(params) {
                    let instruction = Instruction::RegisterIm { o: op, a, b, im: im as i32 };
                    return Ok(instruction);    
                }
                RegisterImOpCode::CMPI => if let Some((a,b,im)) = self.parse_params_b_im(params) {
                    let instruction = Instruction::RegisterIm { o: op, a, b, im: im as i32 };
                    return Ok(instruction);    
                }
                _ => {
                    if let Some((a,b,im)) = self.parse_params_a_b_c(params) {
                        let instruction = Instruction::RegisterIm { o: op, a, b, im: im as i32 };
                        return Ok(instruction);
                    }
                }
            }

        }

        if let Ok(op) = MemoryOpCode::from_str(op) {
            if let Some((a,b,disp)) = self.parse_params_a_b_im(params) {
                let instruction = Instruction::Memory { o: op, a, b, disp };
                return Ok(instruction);
            }
        }

        if let Ok(op) = BranchOpCode::from_str(op) {
            if let Ok(disp) = params.parse::<i32>() {
                let instruction = Instruction::Branch { o: op, disp};
                return Ok(instruction)
            }
        }

        return Err(ParseError::SyntaxError { index: index });
    }

    fn parse_params_a_b_c(&self, params: &str) -> Option<(usize, usize, usize)> {
        let mut split = params.split(",");
        let (a, b, c) = (split.next(), split.next(), split.next());
        println!("After splitting ({:?},{:?},{:?}", a, b, c);
        if let (Some(a), Some(b), Some(c)) = (a, b, c) {
            let (a, b, c) = (self.parse_register(a), self.parse_register(b), self.parse_register(c));
            println!("After parsing value ({:?},{:?},{:?}", a, b, c);
            if let (Ok(a), Ok(b), Ok(c)) = (a, b, c) {
                return Some((a,b,c))
            }
        }
        return None;
    }

    fn parse_params_a_b_im(&self, params: &str) -> Option<(usize, usize, i32)> {
        let mut split = params.split(",");
        let (a, b, c) = (split.next(), split.next(), split.next());
        println!("After splitting ({:?},{:?},{:?}", a, b, c);
        if let (Some(a), Some(b), Some(c)) = (a, b, c) {
            let (a, b, c) = (self.parse_register(a), self.parse_register(b), self.parse_im(c));
            println!("After parsing value ({:?},{:?},{:?}", a, b, c);
            if let (Ok(a), Ok(b), Ok(c)) = (a, b, c) {
                return Some((a,b,c))
            }
        }
        return None;
    }

    fn parse_params_b_im(&self, params: &str) -> Option<(usize, usize, i32)> {
        let mut split = params.split(",");
        let (b, c) = (split.next(), split.next());
        if let (Some(b), Some(c)) = (b, c) {
            let (b, c) = (self.parse_register(b), self.parse_im(c));
            if let (Ok(b), Ok(c)) = (b, c) {
                return Some((0,b,c))
            }
        }
        return None;
    }

    fn parse_params_a_im(&self, params: &str) -> Option<(usize, usize, i32)> {
        let mut split = params.split(",");
        let (a, c) = (split.next(), split.next());
        if let (Some(a), Some(c)) = (a, c) {
            let (a, c) = (self.parse_register(a), self.parse_im(c));
            if let (Ok(a), Ok(c)) = (a, c) {
                return Some((a,0,c))
            }
        }
        return None;
    }

    fn parse_register(&self, s: &str) -> Result<usize, std::num::ParseIntError> {
        if let Some(&symbol) = self.symbols.get(s) {
            return Ok(symbol as usize);
        }
        return s.parse::<usize>();
    }

    fn parse_im(&self, s: &str) -> Result<i32, std::num::ParseIntError> {
        if let Some(&symbol) = self.symbols.get(s) {
            return Ok(symbol as i32);
        }
        return s.parse::<i32>();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_ignores_comment_lines() {
        let mut a = Assembler::new();
        let parsed = a.parse_line(0, "* A comment should be ignored");
        assert_eq!(Ok(ParseResult::Comment), parsed);
        assert_eq!(None, a.origin);
        assert!(a.instructions.is_empty());
    }

    #[test]
    fn it_parses_symbol_definition() {
        let mut a = Assembler::new();
        let parsed = a.parse_line(0, "#FOO 42");
        assert_eq!(Ok(ParseResult::SymbolDef), parsed);
        assert_eq!(None, a.origin);
        assert!(a.instructions.is_empty());
        assert_eq!(a.symbols["#FOO"], 42);
    }

    #[test]
    #[allow(unused)]
    fn it_maintains_origin() {
        let mut a = Assembler::new();
        a.parse_line(0, "#FOO 42");
        a.parse_line(1, "#BAR 50");
        a.parse_line(2, "MOVI R0,42,32");
        assert_eq!(Some(2), a.origin);
    }

    #[test]
    #[allow(unused)]
    fn it_parses_an_instruction_with_address() {
        let mut a = Assembler::new();
        a.parse_line(0, "#FOO 42");
        a.parse_line(1, "#BAR 50");
        a.parse_line(2, "MOVI R0,42,32");
        let parsed = a.parse_line(3, "@START MOVI R1,42,32 ; do some stuff");
        assert_eq!(Some(2), a.origin);
        assert_eq!(a.disps["@START"], 1);
        assert_eq!(Ok(ParseResult::Instruction), parsed);

        assert_eq!(
            Instruction::RegisterIm {
                o: RegisterImOpCode::MOVI,
                a: 1,
                b: 42,
                im: 32
            },
            a.instructions[1]
        )
    }

    #[test]
    fn it_converts_lines_to_instructions() {
        let mut a = Assembler::new();
        a.symbols.insert("#FOO".to_string(), 32);

        let tests = [
            (
                "MOVI R1,42,32",
                Instruction::RegisterIm {
                    o: RegisterImOpCode::MOVI,
                    a: 1,
                    b: 42,
                    im: 32,
                },
            ),
            (
                "MVNI R0,2,#FOO ; Some comment",
                Instruction::RegisterIm {
                    o: RegisterImOpCode::MVNI,
                    a: 0,
                    b: 2,
                    im: 32,
                },
            ),
            (
                "ADDI R5,R12,#FOO ; Some comment",
                Instruction::RegisterIm {
                    o: RegisterImOpCode::ADDI,
                    a: 5,
                    b: 12,
                    im: 32,
                },
            ),
            (    "CMPI R1,3",
                Instruction::RegisterIm {
                    o: RegisterImOpCode::CMPI,
                    a: 0,
                    b: 1,
                    im: 3,
                }),
            (    "CHKI R1,#FOO",
                Instruction::RegisterIm {
                    o: RegisterImOpCode::CHKI,
                    a: 1,
                    b: 0,
                    im: 32,
                }),
            (
                "ADD R0,R1,R3",
                Instruction::Register {
                    o: RegisterOpCode::ADD,
                    a: 0,
                    b: 1,
                    c: 3,
                },
            ),
            (
                "LDW R0,R1,#FOO",
                Instruction::Memory{
                    o: MemoryOpCode::LDW,
                    a: 0,
                    b: 1,
                    disp: 32
                }
            ),
            (
                "BNE 15",
                Instruction::Branch{
                    o: BranchOpCode::BNE,
                    disp: 15
                }
            )            
        ];
        for test in tests.iter() {
            let (line, expected) = test;
            assert_eq!(a.parse_instruction(0, line), Ok(*expected))
        }
    }
    #[test]
    #[allow(unused)]
    fn it_computes_relative_displacements() {
        let mut a = Assembler::new();
        a.parse_line(0, "#FOO 42");
        a.parse_line(1, "MOVI R0,0,1");
        a.parse_line(2, "@LOOP SUBI R0,0,0");
        a.parse_line(3, "CMPI R0,0");
        let parsed = a.parse_instruction(4, "BNE @LOOP");
        assert_eq!(Some(1), a.origin);
        // TODO(pht) make this pass by interpreting the @LOOP as a displacement...
        assert_eq!(Ok(Instruction::Branch{o: BranchOpCode::BNE, disp: -2}), parsed)
    }
}
