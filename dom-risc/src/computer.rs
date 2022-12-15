// A RISC Computer.
use crate::instructions::*;

pub const MEMORY_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Computer {
    // Memory, represented as an array of 32-bit words ; byte-addressing is implicit
    // in the simulator.
    pub mem: [i32; MEMORY_SIZE],

    // Arithmetic unit
    pub regs: [i32; 16],
    pub pc: usize,

    // Condition codes
    pub z_test: bool,
    pub neg_test: bool,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            regs: [0; 16],
            mem: [0; 4096],
            pc: 0,
            z_test: false,
            neg_test: false,
        }
    }

    pub fn load_instructions(&mut self, instructions: Vec<Instruction>) {
        for (index, instruction) in instructions.iter().enumerate() {
            self.mem[index] = Instruction::encode(instruction) as i32;
        }
    }

    pub fn execute(&mut self, max_cycles: u32, debug: bool) {
        self.pc = 0;

        let mut cycles = 0;

        loop {
            if debug {
                println!("----------------- PC = {} --------------", { self.pc });
            }

            // Read current instruction
            let ir: i32 = self.mem[self.pc];
            // NOTE(pht): we panic if instruction is invalid, this could
            // be done by returning an error, etc...

            let instruction = Instruction::parse(ir as u32).unwrap();

            if debug {
                println!("Instruction {:?}", instruction);
            }
            // Set PC to the address of next instruction ; unless a branch instruction
            // is run, this will be the next instruction in memory.
            self.pc += 1;

            if debug {
                println!("Setting PC to next value {:?}", self.pc);
            }

            self.execute_instruction(instruction, debug);

            if self.pc == 0 {
                if debug {
                    println!("Program finished succesfully.")
                }
                break;
            }

            if cycles > max_cycles {
                if debug {
                    println!("Reached max cycles count {}, aborting.", max_cycles);
                }
                break;
            }
            cycles += 1;
        }
    }

    pub fn execute_instruction(&mut self, i: Instruction, _debug: bool) {
        match i {
            Instruction::Register { o, a, b, c } => self.execute_register(o, a, b, self.regs[c]),
            Instruction::RegisterIm { o, a, b, im } => self.execute_register(o, a, b, im),
            Instruction::Memory { u, a, b, offset } => self.execute_memory(u, a, b, offset),
            Instruction::Branch { cond, c, link } => self.execute_branch(cond, c, link),
            Instruction::BranchOff { cond, offset, link } => self.execute_branch_offset(cond, offset, link),
        }
    }

    fn execute_register(&mut self, o: OpCode, a: usize, b: usize, value: i32) {
        match o {
            OpCode::MOV => {
                self.regs[a] = value;
            }
            OpCode::LSL => {
                self.regs[a] = self.regs[b] << value;
            }
            OpCode::ASR => {
                self.regs[a] = self.regs[b] >> value;
            }
            OpCode::ROR => {
                if value > 0 {
                    self.regs[a] = (self.regs[b] as u32).rotate_right(value as u32) as i32;
                } else {
                    self.regs[a] = (self.regs[b] as u32).rotate_left(-value as u32) as i32;
                }
            }
            OpCode::AND => {
                self.regs[a] = self.regs[b] & value;
            }
            OpCode::ANN => {
                self.regs[a] = self.regs[b] & !value;
            }
            OpCode::IOR => {
                self.regs[a] = self.regs[b] | value;
            }
            OpCode::XOR => {
                self.regs[a] = self.regs[b] ^ value;
            }
            OpCode::ADD => {
                self.regs[a] = self.regs[b] + value;
            }
            OpCode::SUB => {
                self.regs[a] = self.regs[b] - value;
            }
            OpCode::MUL => {
                self.regs[a] = self.regs[b] * value;
            }
            OpCode::DIV => {
                self.regs[a] = self.regs[b] / value;
            }
            OpCode::MOD => {
                self.regs[a] = self.regs[b] % value;
            }
        }
        self.update_flags(a);
    }

    fn execute_memory(&mut self, u: MemoryMode, a: usize, b: usize, offset: u32) {
        match u {
            MemoryMode::Load => {
                let adr: i32 = self.regs[b] + offset as i32;
                if adr < 0 {
                    panic!("Attempt to load memory from negative address {}, not implemented.", adr);
                }
                if adr > MEMORY_SIZE as i32 {
                    panic!("Attempt to load memory from address {}, bigger than computer memory.", adr)
                }
                self.regs[a] = self.mem[adr as usize];
                self.update_flags(a);
            }
            MemoryMode::Store => {
                let adr: i32 = self.regs[b] + offset as i32;
                if adr < 0 {
                    panic!("Attempt to store data at negative address {}, not implemented.", adr);
                }
                if adr > MEMORY_SIZE as i32 {
                    panic!("Attempt to store data at address {}, bigger than computer memory.", adr)
                }
                self.mem[adr as usize] = self.regs[a];
            }
        }
    }

    fn execute_branch(&mut self, cond: BranchCondition, c: usize, link: bool) {
        if self.matches_cond(cond) {
            if link {
                self.regs[15] = self.pc as i32;
            }
            self.pc = self.regs[c] as usize;
        }
    }

    fn execute_branch_offset(&mut self, cond: BranchCondition, offset: i32, link: bool) {
        println!("Testing if branch condition {:?} is met", cond);
        if self.matches_cond(cond) {
            println!("Condition was met");
            if link {
                self.regs[15] = self.pc as i32;
            }

            let new_pc = (self.pc as i32 + offset as i32) as usize;
            println!("Will set pc to new value {:?}", new_pc);
            self.pc = new_pc;
        } else {
            println!("Condition was not met");
        }
    }

    pub fn matches_cond(&self, cond: BranchCondition) -> bool {
        match cond {
            BranchCondition::MI => self.neg_test,
            BranchCondition::EQ => self.z_test,
            BranchCondition::LT => self.neg_test,
            BranchCondition::LE => (self.neg_test || self.z_test),
            BranchCondition::AW => true,
            BranchCondition::PL => !self.neg_test,
            BranchCondition::NE => !self.z_test,
            BranchCondition::GE => !self.neg_test,
            BranchCondition::GT => !(self.neg_test || self.z_test),
            BranchCondition::NV => false,
        }
    }

    pub fn update_flags(&mut self, a: usize) {
        self.z_test = self.regs[a] == 0;
        self.neg_test = self.regs[a] < 0;
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}
