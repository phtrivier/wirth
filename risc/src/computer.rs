// A RISC Computer.
use crate::instructions::Instruction;

pub struct Computer {
    pub regs: [i32; 16],
    // NOTE(pht) memory is represented as an array of words, so byte-addressing is implicit.
    pub mem: [i32; 4096],

    // Test flags
    pub z_test: bool,
    pub neg_test: bool
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            regs: [0; 16],
            mem: [0; 4096],
            z_test: false,
            neg_test: false
        }
    }

    pub fn dump_regs(&self) {
        for (index, reg) in self.regs.iter().enumerate() {
            println!("REG {:02}: 0x{:04X} 0b{:032b}", index, reg, reg)
        }
    }

    pub fn execute(&mut self) {
        let next_pc_address = self.regs[15] + 1; // memory is byte-address;
        let ir : i32 = self.mem[next_pc_address as usize];

        // NOTE(pht): we panic if instruction is invalid.
        let instruction = Instruction::parse(ir).unwrap();

        self.execute_instruction(instruction);

        // TODO(pht) now add a loop with some kind of end condition, and
        // you have yourself an actual computer !
    }

    pub fn execute_instruction(&mut self, i: Instruction) {
        match i {
            Instruction::Mov{a, b, c} => {
                let index_a = a as usize;
                let index_c = c as usize;
                self.regs[index_a] = self.regs[index_c] << b;
            }
            Instruction::Mvn{a, b, c} => {
                let index_a = a as usize;
                let index_c = c as usize;
                self.regs[index_a] = - (self.regs[index_c] << b);
            }
            Instruction::Add{a, b, c} => {
                self.regs[a as usize] = self.regs[b as usize] + self.regs[c as usize];
            }
            Instruction::Sub{a, b, c} => {
                self.regs[a as usize] = self.regs[b as usize] - self.regs[c as usize];
            }
            Instruction::Mul{a, b, c} => {
                self.regs[a as usize] = self.regs[b as usize] * self.regs[c as usize];
            }
            Instruction::Div{a, b, c} => {
                self.regs[a as usize] = self.regs[b as usize] / self.regs[c as usize];
            }
            Instruction::Mod{a, b, c} => {
                self.regs[a as usize] = self.regs[b as usize] % self.regs[c as usize];
            }
            Instruction::Cmp{b, c} => {
                let (reg_b, reg_c) = (self.regs[b as usize], self.regs[c as usize]);
                println!("Comparing {:?}={:?} with {:?}={:?}", b, reg_b, c, reg_c);
                self.z_test = reg_b == reg_c;
                self.neg_test = reg_b < reg_c;
                println!("Z={:?}, N={:?}", self.z_test, self.neg_test);
            }
            Instruction::Movi{a, b, im} => {
                self.regs[a as usize] = im << b;
            }
            Instruction::Mvni{a, b, im} => {
                self.regs[a as usize] = -(im << b);
            }
            Instruction::Addi{a, b, im} => {
                self.regs[a as usize] = self.regs[b as usize] + im;
            }
            Instruction::Subi{a, b, im} => {
                self.regs[a as usize] = self.regs[b as usize] - im;
            }
            Instruction::Muli{a, b, im} => {
                self.regs[a as usize] = self.regs[b as usize] * im;
            }
            Instruction::Divi{a, b, im} => {
                self.regs[a as usize] = self.regs[b as usize] / im;
            }
            Instruction::Modi{a, b, im} => {
                self.regs[a as usize] = self.regs[b as usize] % im;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::instructions::Register;

    mod execute_instruction {

        use super::*;

        #[test]
        fn test_execute_register_move_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[2] = 42;
            c.execute_instruction(Instruction::Mov{a: Register::R0, b: 1, c: Register::R2});
            assert_eq!(84, c.regs[0]);

            c.execute_instruction(Instruction::Mvn{a: Register::R0, b: 2, c: Register::R2});
            assert_eq!(-168, c.regs[0])
        }

        #[test]
        fn test_execute_immediate_move_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.execute_instruction(Instruction::Movi{a: Register::R0, b: 1, im: 42});
            assert_eq!(84, c.regs[0]);

            let mut c = Computer::new();
            c.regs[0] = 0;
            c.execute_instruction(Instruction::Mvni{a: Register::R0, b: 2, im: 42});
            assert_eq!(-168, c.regs[0]);
        }

        #[test]
        fn test_execute_register_arithmetic_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;
            // R.a = R.b + R.c
            c.execute_instruction(Instruction::Add{a: Register::R0, b: Register::R1, c: Register::R2});
            assert_eq!(42, c.regs[0]);

            // R.a = R.b - R.c
            c.execute_instruction(Instruction::Sub{a: Register::R0, b: Register::R1, c: Register::R2});
            assert_eq!(-22, c.regs[0]);

            // R.a = R.b * R.c
            c.execute_instruction(Instruction::Mul{a: Register::R0, b: Register::R1, c: Register::R2});
            assert_eq!(320, c.regs[0]);

            // R.a = R.b / R.c
            c.execute_instruction(Instruction::Div{a: Register::R0, b: Register::R2, c: Register::R1});
            assert_eq!(3, c.regs[0]);

            // R.a = R.b % R.c
            c.execute_instruction(Instruction::Mod{a: Register::R0, b: Register::R2, c: Register::R1});
            assert_eq!(2, c.regs[0]);

        }

        #[test]
        fn test_execute_immediate_arithmetic_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;
            // R.a = R.b + im
            c.execute_instruction(Instruction::Addi{a: Register::R0, b: Register::R1, im: 32});
            assert_eq!(42, c.regs[0]);

            // R.a = R.b - im
            c.execute_instruction(Instruction::Subi{a: Register::R0, b: Register::R1, im: 32});
            assert_eq!(-22, c.regs[0]);

            // R.a = R.b * im
            c.execute_instruction(Instruction::Muli{a: Register::R0, b: Register::R1, im: 32});
            assert_eq!(320, c.regs[0]);

            // R.a = R.b / im
            c.execute_instruction(Instruction::Divi{a: Register::R0, b: Register::R2, im: 10});
            assert_eq!(3, c.regs[0]);

            // R.a = R.b % im
            c.execute_instruction(Instruction::Modi{a: Register::R0, b: Register::R2, im: 10});
            assert_eq!(2, c.regs[0]);

        }


        #[test]
        fn test_execute_register_compare(){
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;

            c.execute_instruction(Instruction::Cmp{b: Register::R1, c: Register::R2});
            // R.b == R.c ?
            assert_eq!(false, c.z_test);
            // R.b < R.c ?
            assert_eq!(true, c.neg_test);

            c.regs[1] = 10;
            c.regs[2] = 10;
            c.execute_instruction(Instruction::Cmp{b: Register::R1, c: Register::R2});
            // R.b == R.c ?
            assert_eq!(true, c.z_test);
            // R.b < R.c ?
            assert_eq!(false, c.neg_test);

            c.regs[1] = -32;
            c.regs[2] = 10;
            c.execute_instruction(Instruction::Cmp{b: Register::R1, c: Register::R2});
            // R.b == R.c ?
            assert_eq!(false, c.z_test);
            // R.b < R.c ?
            assert_eq!(true, c.neg_test);
        }
    }

    #[test]
    fn test_execute_next_instruction() {
        let mut c = Computer::new();

        // Prepare memory
        let instruction = Instruction::Mov{a: Register::R0, b: 1, c: Register::R2};
        let instruction_data = Instruction::encode(instruction);
        c.mem[1] = instruction_data;

        // TODO(pht) c.mem[2] = Instruction::Ret(0)

        // Prepare registers
        c.regs[2] = 42;
        c.regs[15] = 0;

        // Run the code
        c.execute();

        // Check that the register was changed
        assert_eq!(84, c.regs[0])
    }

    #[test]
    fn test_program_execution_to_end() {

        // Memory:
        // 1: Mov{a: R0, b: 1, c: R2}

        // Should do the same as ear

    }


}
