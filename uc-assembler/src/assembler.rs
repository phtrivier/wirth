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
            MOV  R1,#MAX    ; Init Loop index, loop will decrease to 0
  @LOOP     MOV  R0,R1      ; R0 <- R1
            MUL  R0,R0,R0   ; R0 <- R0 * R0
            STW  R0,R1,#OUT ; MEM[OUT+I] <- R0
            ADD  R1,-1      ; R1 <- R1 - 1
            BEQ  @END       ; IF R1 == 0 GOTO END
            BAW  @LOOP      ; ELSE CONTINUE
  @END      MOV  R15,0      ; Terminates
            BAW  R15
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

use std::collections::HashMap;

pub struct Assembler {
    pub instructions: Vec<Instruction>,
    pub instruction_indexes: HashMap<String, i32>, // Map of symbols like @BAR to instruction indices
    pub symbols: HashMap<String, u32>,             // Map of symbols like #FOO to their values
    pub line_index: u32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseError {
    SyntaxError { line_index: u32 },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseResult {
    Comment,
    SymbolDef,
    Instruction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AssembleResult {
    Program,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssembleError {
    SyntaxError { line_index: u32, line: String },
}

impl Assembler {
    #[allow(dead_code)]
    pub fn new() -> Assembler {
        let symbols = HashMap::new();

        Assembler {
            instructions: vec![],
            symbols,
            instruction_indexes: HashMap::new(),
            line_index: 0,
        }
    }

    #[allow(dead_code)]
    pub fn assemble(&mut self, program: &str) -> Result<AssembleResult, AssembleError> {
        // Load all symbols and addresses
        let mut instruction_index: u32 = 0;
        for (_line_index, line) in program.lines().enumerate() {
            let l = line.trim_start().trim_end_matches(';').trim_end();

            if l.is_empty() || l.starts_with('*') || l.starts_with('#') {
                continue;
            } else {
                let mut tokens = line.split_ascii_whitespace();
                if let Some(symbol) = tokens.next() {
                    if symbol.starts_with('@') {
                        self.instruction_indexes.insert(symbol.to_string(), instruction_index as i32);
                    }
                }
                instruction_index += 1;
            }
        }

        // Load actual code
        for (line_index, line) in program.lines().enumerate() {
            self.line_index = line_index as u32;
            let parsed = self.parse_line(line_index as u32, line);
            // TODO(pht) add more info into the ParseErrors to help diagnostic
            if let Err(ParseError::SyntaxError { line_index: error_line_index }) = parsed {
                return Err(AssembleError::SyntaxError {
                    line_index: error_line_index,
                    line: line.to_string(),
                });
            }
        }
        Ok(AssembleResult::Program)
    }

    #[allow(dead_code)]
    pub fn parse_line(&mut self, line_index: u32, line: &str) -> Result<ParseResult, ParseError> {
        let l = line.trim_start().trim_end_matches(';').trim_end();

        if l.is_empty() || l.starts_with('*') {
            return Ok(ParseResult::Comment);
        } else if l.starts_with('#') {
            return self.parse_symbol_def(line_index, l);
        }

        if let Ok(instruction) = self.parse_instruction(l) {
            self.instructions.push(instruction);
            return Ok(ParseResult::Instruction);
        }
        Err(ParseError::SyntaxError { line_index })
    }

    fn parse_symbol_def(&mut self, line_index: u32, line: &str) -> Result<ParseResult, ParseError> {
        let mut tokens = line.split_ascii_whitespace();

        if let Some(symbol) = tokens.next() {
            if let Some(value_str) = tokens.next() {
                if let Ok(value) = value_str.parse::<u32>() {
                    self.symbols.insert(symbol.to_string(), value);
                    return Ok(ParseResult::SymbolDef);
                }
            }
        }
        Err(ParseError::SyntaxError { line_index })
    }

    pub fn parse_instruction(&mut self, line: &str) -> Result<Instruction, ParseError> {
        let mut tokens = line.split_ascii_whitespace();
        let instruction_index = self.instructions.len() as u32;
        if let Some(symbol) = tokens.next() {
            if symbol.starts_with('@') {
                if let Some(op) = tokens.next() {
                    if let Some(params) = tokens.next() {
                        return self.parse_op_params(instruction_index, op, params);
                    }
                }
            } else {
                let op = symbol;
                if let Some(params) = tokens.next() {
                    return self.parse_op_params(instruction_index, op, params);
                }
            }
        }
        self.syntax_error()
    }

    fn parse_op_params(&mut self, instruction_index: u32, op: &str, params: &str) -> Result<Instruction, ParseError> {
        if let Some(op) = self.parse_register_opcode(op) {
            if op == OpCode::MOV {
                if let Some((a, c)) = self.parse_a_c(params) {
                    return Ok(Instruction::Register { o: op, a, b: 0, c });
                }
                if let Some((a, im)) = self.parse_a_im(params) {
                    return Ok(Instruction::RegisterIm { o: op, a, b: 0, im });
                }
            } else {
                if let Some((a, b, c)) = self.parse_a_b_c(params) {
                    return Ok(Instruction::Register { o: op, a, b, c });
                }
                if let Some((a, b, im)) = self.parse_a_b_im(params) {
                    return Ok(Instruction::RegisterIm { o: op, a, b, im });
                }
            }
        }

        if op == "LDW" || op == "STW" {
            if let Some((a, b, im)) = self.parse_a_b_im(params) {
                if im < 0 {
                    return self.syntax_error();
                }
                let mut mode = MemoryMode::Load;
                if op == "STW" {
                    mode = MemoryMode::Store;
                }
                return Ok(Instruction::Memory { u: mode, a, b, offset: im as u32 });
            }
        }

        if let Some((cond, link)) = self.parse_condition_link(op) {
            if let Ok(c) = self.parse_register(params) {
                return Ok(Instruction::Branch { cond, link, c });
            }
            if let Ok(offset) = self.parse_branch_offset(instruction_index, params) {
                return Ok(Instruction::BranchOff { cond, link, offset });
            }
        }

        self.syntax_error()
    }

    fn parse_a_im(&self, s: &str) -> Option<(usize, i32)> {
        let mut split = s.split(',');
        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            if let (Ok(a), Ok(im)) = (self.parse_register(left), self.parse_im(right)) {
                return Some((a, im));
            }
        }
        None
    }

    fn parse_a_b_c(&self, s: &str) -> Option<(usize, usize, usize)> {
        let mut split = s.split(',');
        if let (Some(left), Some(middle), Some(right)) = (split.next(), split.next(), split.next()) {
            if let (Ok(a), Ok(b), Ok(c)) = (self.parse_register(left), self.parse_register(middle), self.parse_register(right)) {
                return Some((a, b, c));
            }
        }
        None
    }

    fn parse_a_b_im(&self, s: &str) -> Option<(usize, usize, i32)> {
        let mut split = s.split(',');
        if let (Some(left), Some(middle), Some(right)) = (split.next(), split.next(), split.next()) {
            if let (Ok(a), Ok(b), Ok(im)) = (self.parse_register(left), self.parse_register(middle), self.parse_im(right)) {
                return Some((a, b, im));
            }
        }
        None
    }

    fn parse_a_c(&self, s: &str) -> Option<(usize, usize)> {
        let mut split = s.split(',');
        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            if let (Ok(a), Ok(c)) = (self.parse_register(left), self.parse_register(right)) {
                return Some((a, c));
            }
        }
        None
    }

    fn parse_register(&self, s: &str) -> Result<usize, ParseError> {
        let mut chars = s.chars();
        if let Some('R') = chars.next() {
            let remainer = chars.collect::<String>();
            if let Ok(c) = remainer.parse::<u32>() {
                if c < 16 {
                    return Ok(c as usize);
                }
            }
        }
        Err(ParseError::SyntaxError { line_index: self.line_index })
    }

    fn parse_im(&self, s: &str) -> Result<i32, std::num::ParseIntError> {
        if let Some(&symbol) = self.symbols.get(s) {
            return Ok(symbol as i32);
        }
        s.parse::<i32>()
    }

    fn parse_branch_offset(&self, instruction_index: u32, s: &str) -> Result<i32, std::num::ParseIntError> {
        if let Some(param_instruction_index) = self.instruction_indexes.get(s) {
            let offset: i32 = param_instruction_index - instruction_index as i32;
            if offset < 0 {
                return Ok(offset - 1);
            } else {
                return Ok(offset);
            }
        }
        if let Some(&symbol) = self.symbols.get(s) {
            return Ok(symbol as i32);
        }
        s.parse::<i32>()
    }

    fn syntax_error(&self) -> Result<Instruction, ParseError> {
        Err(ParseError::SyntaxError { line_index: self.line_index })
    }

    fn parse_register_opcode(&self, s: &str) -> Option<OpCode> {
        match s {
            "MOV" => Some(OpCode::MOV), // R.a = n
            "LSL" => Some(OpCode::LSL), // R.a = R.b <- n (shift left)
            "ASR" => Some(OpCode::ASR), // R.a = R.b -> n (shift right with sign extension)
            "ROR" => Some(OpCode::ROR),
            "AND" => Some(OpCode::AND),
            "ANN" => Some(OpCode::ANN), // R.a = R.b & ~n
            "IOR" => Some(OpCode::IOR), // R.a = R.b or n
            "XOR" => Some(OpCode::XOR), // R.a = R.b xor n
            "ADD" => Some(OpCode::ADD),
            "SUB" => Some(OpCode::SUB),
            "MUL" => Some(OpCode::MUL),
            "DIV" => Some(OpCode::DIV),
            "MOD" => Some(OpCode::MOD),
            _ => None,
        }
    }

    fn parse_condition_link(&self, s: &str) -> Option<(BranchCondition, bool)> {
        match s {
            // versions with no link. BNV is not parsed because it would not male any sense, really ?
            "BMI" => Some((BranchCondition::MI, false)),
            "BEQ" => Some((BranchCondition::EQ, false)),
            "BLT" => Some((BranchCondition::LT, false)),
            "BLE" => Some((BranchCondition::LE, false)),
            "B" => Some((BranchCondition::AW, false)),
            "BPL" => Some((BranchCondition::PL, false)),
            "BNE" => Some((BranchCondition::NE, false)),
            "BGE" => Some((BranchCondition::GE, false)),
            "BGT" => Some((BranchCondition::GT, false)),
            // versions with link (no sure if all of them are needed)
            "BMIL" => Some((BranchCondition::MI, true)),
            "BEQL" => Some((BranchCondition::EQ, true)),
            "BLTL" => Some((BranchCondition::LT, true)),
            "BLEL" => Some((BranchCondition::LE, true)),
            "BL" => Some((BranchCondition::AW, true)),
            "BPLL" => Some((BranchCondition::PL, true)),
            "BNEL" => Some((BranchCondition::NE, true)),
            "BGEL" => Some((BranchCondition::GE, true)),
            "BGTL" => Some((BranchCondition::GT, true)),
            _ => None,
        }
    }
}
