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
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            regs: [0; 16],
            mem: [0; 4096],
            z_test: false,
            neg_test: false,
            done_flag: false,
        }
    }

    pub fn dump_regs(&self) {
        for (index, reg) in self.regs.iter().enumerate() {
            println!("REG {:02}: 0x{:04X} 0b{:032b}", index, reg, reg)
        }
    }

    pub fn execute(&mut self, max: u32) {
        self.done_flag = false;
        self.regs[15] = 0;

        let mut i = 0;

        loop {
            // NOTE(pht) I might be doing this loop wrong... anyway
            self.regs[15] = self.regs[15] + 1;
            let next_pc_address = self.regs[15]; // memory is byte-address;
            let ir: i32 = self.mem[next_pc_address as usize];

            // NOTE(pht): we panic if instruction is invalid.
            let instruction = Instruction::parse(ir as u32).unwrap();

            println!("Instruction {:?}", instruction);

            self.execute_instruction(instruction);

            println!("PC ? {:?}", self.regs[15]);
            println!("Done ? {:?}", self.done_flag);

            if i > max || self.done_flag {
                break;
            }
            i = i + 1;
        }
    }

    pub fn execute_instruction(&mut self, i: Instruction) {
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
                    let b_add = self.regs[b] as usize;
                    self.regs[a] = self.mem[(b_add as i32 + disp) as usize];
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
            Instruction::Branch { o, disp } => {
                match o {
                    BranchOpCode::BEQ => {
                        if self.z_test {
                            self.regs[15] = self.regs[15] + (disp as i32);
                        }
                    }
                    BranchOpCode::BLT => {
                        if self.neg_test {
                            self.regs[15] = self.regs[15] + (disp as i32);
                        }
                    }
                    BranchOpCode::BLE => {
                        if self.neg_test || self.z_test {
                            self.regs[15] = self.regs[15] + (disp as i32);
                        }
                    }
                    BranchOpCode::BNE => {
                        if !self.z_test {
                            self.regs[15] = self.regs[15] + (disp as i32);
                        }
                    }
                    BranchOpCode::BGE => {
                        if !self.neg_test {
                            self.regs[15] = self.regs[15] + (disp as i32);
                        }
                    }
                    BranchOpCode::BGT => {
                        if !self.neg_test && !self.z_test {
                            self.regs[15] = self.regs[15] + (disp as i32);
                        }
                    }
                    BranchOpCode::BR => {
                        self.regs[15] = self.regs[15] + (disp as i32);
                    }
                    BranchOpCode::BSR => {
                        self.regs[14] = self.regs[15];
                        self.regs[15] = self.regs[15] + (disp as i32);
                    }
                    BranchOpCode::RET => {
                        // TODO(pht) If I ever get bitten, make a Ret{c: Register} command instead of taking the end of disp...
                        let index = (disp % 0x10) as usize;
                        self.regs[15] = self.regs[index];
                        if self.regs[15] == 0 {
                            self.done_flag = true;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::instructions::BranchOpCode::*;
    use crate::instructions::Instruction::*;
    use crate::instructions::MemoryOpCode::*;
    use crate::instructions::RegisterImOpCode::*;
    use crate::instructions::RegisterOpCode::*;

    mod execute_registers_instruction {

        use super::*;

        #[test]
        fn test_execute_register_move_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[2] = 42;
            c.execute_instruction(Register {
                o: MOV,
                a: 0,
                b: 1,
                c: 2,
            });
            assert_eq!(84, c.regs[0]);

            c.execute_instruction(Register {
                o: MVN,
                a: 0,
                b: 2,
                c: 2,
            });
            assert_eq!(-168, c.regs[0])
        }

        #[test]
        fn test_execute_immediate_move_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.execute_instruction(RegisterIm {
                o: MOVI,
                a: 0,
                b: 1,
                im: 42,
            });
            assert_eq!(84, c.regs[0]);

            let mut c = Computer::new();
            c.regs[0] = 0;
            c.execute_instruction(RegisterIm {
                o: MVNI,
                a: 0,
                b: 2,
                im: 42,
            });
            assert_eq!(-168, c.regs[0]);
        }

        #[test]
        fn test_execute_register_arithmetic_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;
            // R.a = R.b + R.c
            c.execute_instruction(Register {
                o: ADD,
                a: 0,
                b: 1,
                c: 2,
            });
            assert_eq!(42, c.regs[0]);

            // R.a = R.b - R.c
            c.execute_instruction(Register {
                o: SUB,
                a: 0,
                b: 1,
                c: 2,
            });
            assert_eq!(-22, c.regs[0]);

            // R.a = R.b * R.c
            c.execute_instruction(Register {
                o: MUL,
                a: 0,
                b: 1,
                c: 2,
            });
            assert_eq!(320, c.regs[0]);

            // R.a = R.b / R.c
            c.execute_instruction(Register {
                o: DIV,
                a: 0,
                b: 2,
                c: 1,
            });
            assert_eq!(3, c.regs[0]);

            // R.a = R.b % R.c
            c.execute_instruction(Register {
                o: MOD,
                a: 0,
                b: 2,
                c: 1,
            });
            assert_eq!(2, c.regs[0]);
        }

        #[test]
        fn test_execute_immediate_arithmetic_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;

            c.execute_instruction(RegisterIm {
                o: ADDI,
                a: 0,
                b: 1,
                im: 32,
            });
            assert_eq!(42, c.regs[0]);

            // R.a = R.b - im
            c.execute_instruction(RegisterIm {
                o: SUBI,
                a: 0,
                b: 1,
                im: 32,
            });
            assert_eq!(-22, c.regs[0]);

            // R.a = R.b * im
            c.execute_instruction(RegisterIm {
                o: MULI,
                a: 0,
                b: 1,
                im: 32,
            });
            assert_eq!(320, c.regs[0]);

            // R.a = R.b / im
            c.execute_instruction(RegisterIm {
                o: DIVI,
                a: 0,
                b: 1,
                im: 3,
            });
            assert_eq!(3, c.regs[0]);

            // R.a = R.b % im
            c.execute_instruction(RegisterIm {
                o: MODI,
                a: 0,
                b: 1,
                im: 3,
            });
            assert_eq!(1, c.regs[0]);
        }

        #[test]
        fn test_execute_register_compare() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;

            c.execute_instruction(Register {
                o: CMP,
                a: 0,
                b: 1,
                c: 2,
            });
            // R.b == R.c ?
            assert_eq!(false, c.z_test);
            // R.b < R.c ?
            assert_eq!(true, c.neg_test);

            c.regs[1] = 10;
            c.regs[2] = 10;
            c.execute_instruction(Register {
                o: CMP,
                a: 0,
                b: 1,
                c: 2,
            });
            // R.b == R.c ?
            assert_eq!(true, c.z_test);
            // R.b < R.c ?
            assert_eq!(false, c.neg_test);

            c.regs[1] = -32;
            c.regs[2] = 10;
            c.execute_instruction(Register {
                o: CMP,
                a: 0,
                b: 1,
                c: 2,
            });
            // R.b == R.c ?
            assert_eq!(false, c.z_test);
            // R.b < R.c ?
            assert_eq!(true, c.neg_test);
        }

        #[test]
        fn test_execute_immediate_compare() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;

            c.execute_instruction(RegisterIm {
                o: CMPI,
                a: 0,
                b: 1,
                im: -32,
            });
            assert_eq!(false, c.z_test);
            assert_eq!(false, c.neg_test);

            c.execute_instruction(RegisterIm {
                o: CMPI,
                a: 0,
                b: 1,
                im: 10,
            });
            assert_eq!(true, c.z_test);
            assert_eq!(false, c.neg_test);

            c.execute_instruction(RegisterIm {
                o: CMPI,
                a: 0,
                b: 1,
                im: 32,
            });
            assert_eq!(false, c.z_test);
            assert_eq!(true, c.neg_test);
        }

        #[test]
        fn test_execute_chki() {
            let mut c = Computer::new();
            c.regs[0] = 30;
            c.execute_instruction(RegisterIm {
                o: CHKI,
                b: 0,
                a: 0,
                im: 32,
            });
            assert_eq!(c.regs[0], 30);

            c.execute_instruction(RegisterIm {
                o: CHKI,
                b: 0,
                a: 0,
                im: 15,
            });
            assert_eq!(c.regs[0], 0);

            c.regs[0] = -10;
            c.execute_instruction(RegisterIm {
                o: CHKI,
                b: 0,
                a: 0,
                im: 15,
            });
            assert_eq!(c.regs[0], 0);
        }
    }

    mod execute_memory_instruction {

        use super::*;

        #[test]
        fn test_execute_load_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;

            // Mem[R.b + disp] = Mem[10 + 4] = Mem[14] ?
            // Or at least I'm goint to assume that
            c.mem[14] = 42;

            c.execute_instruction(Memory {
                o: LDW,
                a: 0,
                b: 1,
                disp: 4,
            });
            assert_eq!(c.regs[0], 42);

            // TODO(pht) LDB is not implemented, but will it be needed ?
        }

        #[test]
        fn test_execute_stack_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 42;
            c.regs[1] = 10;
            c.mem[10] = 0;

            // NOTE(pht) : the instruction description for PSH and POP are
            // not the same as the implementation.
            // We'll trust the implementations:
            // ```
            // PSH:
            //  DEC(R[b], c);
            //  M[(R[b]) DIV 4] := R[a]
            //
            // ```
            c.execute_instruction(Memory {
                o: PSH,
                a: 0,
                b: 1,
                disp: 1,
            });
            assert_eq!(c.regs[1], 9);
            assert_eq!(c.mem[9], 42);

            c.regs[0] = 0;
            // ```
            // POP:
            //  R[a] := M[(R[b]) DIV 4];
            //  INC(R[b], c)
            // ```
            c.execute_instruction(Memory {
                o: POP,
                a: 0,
                b: 1,
                disp: 1,
            });
            assert_eq!(c.regs[0], 42);
            assert_eq!(c.regs[1], 10);
        }

        #[test]
        fn test_execute_store_memory_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 42;
            c.regs[1] = 10;

            // M[(R[b] + c) DIV 4] := R[a]
            c.execute_instruction(Memory {
                o: STW,
                a: 0,
                b: 1,
                disp: 2,
            });
            assert_eq!(c.mem[12], 42);
        }
    }

    mod execute_branch_instructions {
        use super::*;

        #[test]
        fn test_branch_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 42;
            c.regs[1] = 10;
            c.regs[15] = 40;

            c.z_test = true;
            c.execute_instruction(Branch { o: BEQ, disp: 1 });
            assert_eq!(c.regs[15], 41);

            c.neg_test = true;
            c.execute_instruction(Branch { o: BLT, disp: 2 });
            assert_eq!(c.regs[15], 43);

            c.execute_instruction(Branch { o: BLE, disp: 3 });
            assert_eq!(c.regs[15], 46);

            c.z_test = false;
            c.neg_test = false;

            c.execute_instruction(Branch { o: BNE, disp: 1 });
            assert_eq!(c.regs[15], 47);

            c.execute_instruction(Branch { o: BGE, disp: 2 });
            assert_eq!(c.regs[15], 49);

            c.execute_instruction(Branch { o: BGT, disp: 3 });
            assert_eq!(c.regs[15], 52);

            c.execute_instruction(Branch { o: BR, disp: 12 });
            assert_eq!(c.regs[15], 64);

            c.execute_instruction(Branch { o: BSR, disp: 10 });
            assert_eq!(c.regs[14], 64);
            assert_eq!(c.regs[15], 74);

            c.execute_instruction(Branch { o: RET, disp: 1 });
            assert_eq!(c.regs[15], 10);

            c.regs[1] = 0;
            c.execute_instruction(Branch { o: RET, disp: 1 });
            assert_eq!(c.regs[15], 0);
            assert_eq!(c.done_flag, true);
        }
    }

    #[test]
    fn test_program_execution_to_end() {
        let mut c = Computer::new();

        // MOVI $0, 5
        let mut instruction = RegisterIm {
            o: MOVI,
            a: 0,
            b: 0,
            im: 5,
        };
        let mut instruction_data = Instruction::encode(instruction);
        c.mem[1] = instruction_data as i32;

        // MOVI $1, 10
        instruction = RegisterIm {
            o: MOVI,
            a: 1,
            b: 0,
            im: 10,
        };
        instruction_data = Instruction::encode(instruction);
        c.mem[2] = instruction_data as i32;

        // RET $2
        instruction = Branch { o: RET, disp: 2 };
        instruction_data = Instruction::encode(instruction);
        c.mem[3] = instruction_data as i32;

        c.execute(5);

        assert_eq!(c.done_flag, true);
        assert_eq!(c.regs[0], 5);
        assert_eq!(c.regs[1], 10);
    }

    #[test]
    fn test_ambitious_program() {
        // TODO(pht) a more ambitious program
        // a = 5
        // if a > 3 {
        //    return 0
        // } else {
        //    return 1
        // }

        // Prepare memory

        // MAIN:  MOVI $0, 5 ; R.0 = 5
        // let instruction = RegisterIm{o: MOVI, a: 0, b: 0, im: 5};
        // let instruction_data = Instruction::encode(instruction);
        // c.mem[1] = instruction_data as i32;

        //        CMPI $0, 3; Z <- R.0 == 3 ; N <-R.0 < 3
        //        BLE  $0, RET1
        // RET0   MOVI $0, 0 ; load result
        //        MOVI $1, 0 ; load return address to end
        //        RET  $0
        // RET1   MOVI $0, 1 ; load result
        //        MOVI $1, 0 ; load return address to end
        //        RET  $0
    }
}
