// A RISC Computer.
use crate::instructions::*;

pub struct Computer {
    pub regs: [i32; 16],
    // NOTE(pht) memory is represented as an array of words, so byte-addressing is implicit.
    pub mem: [i32; 4096],

    // Test flags
    pub z_test: bool,
    pub neg_test: bool,

    // Done flag to stop execution loop
    pub done_flag: bool,

    // Dirty hack
    pub next: i32,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            regs: [0; 16],
            mem: [0; 4096],
            z_test: false,
            neg_test: false,
            done_flag: false,
            next: 0, // dirty hack
        }
    }

    pub fn load_instructions_at(&mut self, instructions: Vec<Instruction>, base: usize) {
        for (index, instruction) in instructions.iter().enumerate() {
            self.mem[base + index] = Instruction::encode(*instruction) as i32;
        }
    }

    pub fn load_instructions(&mut self, instructions: Vec<Instruction>) {
        self.load_instructions_at(instructions, 0);
    }

    pub fn dump_regs(&self) {
        for (index, reg) in self.regs.iter().enumerate() {
            println!("REG {:04}: 0x{:08X} 0b{:032b} {:12}", index, reg, reg, reg)
        }
    }

    pub fn dump_mem(&self, from: usize, count: usize) {
        let to = from + count;
        for index in from..to {
            let content = self.mem[index];
            println!("MEM {:04}: 0x{:08X} 0b{:032b} {:12?}", index, content, content, content);
        }
    }

    pub fn execute_at(&mut self, max: u32, base: usize, debug: bool) {
        self.done_flag = false;
        self.regs[15] = base as i32;

        let mut i = 0;

        loop {
            let pc_address = self.regs[15]; // memory is byte-address;

            // NOTE(pht) Next instructions, unless told otherwise.
            self.next = self.regs[15] + 1;

            let ir: i32 = self.mem[pc_address as usize];

            // NOTE(pht): we panic if instruction is invalid.
            let instruction = Instruction::parse(ir as u32).unwrap();

            if debug {
                println!("Instruction {:?}", instruction);
            }

            self.execute_instruction(instruction, debug);

            if debug {
                println!("PC ? {:?}", self.regs[15]);
                println!("Done ? {:?}", self.done_flag);
            }

            if i > max || self.done_flag {
                break;
            }
            i = i + 1;

            self.regs[15] = self.next;
        }
    }

    pub fn execute(&mut self, max: u32, debug: bool) {
        self.execute_at(max, 0, debug);
    }

    pub fn execute_instruction(&mut self, i: Instruction, debug: bool) {
        match i {
            Instruction::Register { o, a, b, c } => match o {
                RegisterOpCode::MOV => {
                    self.regs[a] = self.regs[c] << b;
                }
                RegisterOpCode::MVN => {
                    self.regs[a] = -(self.regs[c] << b);
                }
                RegisterOpCode::ADD => {
                    self.regs[a] = self.regs[b] + self.regs[c];
                }
                RegisterOpCode::SUB => {
                    self.regs[a] = self.regs[b] - self.regs[c];
                }
                RegisterOpCode::MUL => {
                    self.regs[a] = self.regs[b] * self.regs[c];
                }
                RegisterOpCode::DIV => {
                    self.regs[a] = self.regs[b] / self.regs[c];
                }
                RegisterOpCode::MOD => {
                    self.regs[a] = self.regs[b] % self.regs[c];
                }
                RegisterOpCode::CMP => {
                    let (reg_b, reg_c) = (self.regs[b], self.regs[c]);
                    self.z_test = reg_b == reg_c;
                    self.neg_test = reg_b < reg_c;
                }
            },
            Instruction::RegisterIm { o, a, b, im } => match o {
                RegisterImOpCode::MOVI => {
                    self.regs[a] = im << b;
                }
                RegisterImOpCode::MVNI => {
                    self.regs[a] = -(im << b);
                }
                RegisterImOpCode::ADDI => {
                    self.regs[a] = self.regs[b] + im;
                }
                RegisterImOpCode::SUBI => {
                    self.regs[a] = self.regs[b] - im;
                }
                RegisterImOpCode::MULI => {
                    self.regs[a] = self.regs[b] * im;
                }
                RegisterImOpCode::DIVI => {
                    self.regs[a] = self.regs[b] / im;
                }
                RegisterImOpCode::MODI => {
                    self.regs[a] = self.regs[b] % im;
                }
                RegisterImOpCode::CMPI => {
                    let reg_b = self.regs[b];
                    self.z_test = reg_b == im;
                    self.neg_test = reg_b < im;
                }
                RegisterImOpCode::CHKI => {
                    let reg_a = self.regs[a];
                    if reg_a < 0 || reg_a > im {
                        self.regs[a] = 0;
                    }
                }
            },
            Instruction::Memory { o, a, b, disp } => match o {
                MemoryOpCode::LDW => {
                    if debug {
                        println!("Loading from register {:?} ", b);
                        println!("Loading with displacement {:?} ", disp);
                    }
                    let b_add = self.regs[b];
                    if debug {
                        println!("b address {:?}", b_add);
                    }
                    let b_plus_disp_add = (b_add as i32 + disp) as usize;
                    if debug {
                        println!("b + disp address, {:?}", b_plus_disp_add);
                    }
                    self.regs[a] = self.mem[b_plus_disp_add];
                }
                MemoryOpCode::POP => {
                    self.regs[a] = self.mem[self.regs[b] as usize];
                    self.regs[b] = ((self.regs[b] as i32) + disp) as i32;
                }
                MemoryOpCode::PSH => {
                    self.regs[b] = self.regs[b] - disp;
                    self.mem[self.regs[b] as usize] = self.regs[a];
                }
                MemoryOpCode::STW => {
                    self.mem[(self.regs[b] + disp) as usize] = self.regs[a];
                }
            },
            Instruction::Branch { o, disp } => match o {
                BranchOpCode::BEQ => {
                    if self.z_test {
                        self.next = self.regs[15] + (disp as i32);
                    }
                }
                BranchOpCode::BLT => {
                    if self.neg_test {
                        self.next = self.regs[15] + (disp as i32);
                    }
                }
                BranchOpCode::BLE => {
                    if self.neg_test || self.z_test {
                        self.next = self.regs[15] + (disp as i32);
                    }
                }
                BranchOpCode::BNE => {
                    if !self.z_test {
                        self.next = self.regs[15] + (disp as i32);
                    }
                }
                BranchOpCode::BGE => {
                    if !self.neg_test {
                        self.next = self.regs[15] + (disp as i32);
                    }
                }
                BranchOpCode::BGT => {
                    if !self.neg_test && !self.z_test {
                        self.next = self.regs[15] + (disp as i32);
                    }
                }
                BranchOpCode::BR => {
                    self.next = self.regs[15] + (disp as i32);
                }
                BranchOpCode::BSR => {
                    self.regs[14] = self.regs[15];
                    self.next = self.regs[15] + (disp as i32);
                }
                BranchOpCode::RET => {
                    let index = (disp % 0x10) as usize;
                    self.next = self.regs[index];
                    if self.next == 0 {
                        self.done_flag = true;
                    }
                }
            },
        }
    }
}
