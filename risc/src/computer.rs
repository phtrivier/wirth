// A RISC Computer.
use crate::instructions::Instruction;

pub struct Computer {
    pub regs: [i32; 16],
    // NOTE(pht) memory is represented as an array of words, so byte-addressing is implicit.
    pub mem: [i32; 4096]
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            regs: [0; 16],
            mem: [0; 4096]
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
            // _ => {}
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
        fn test_execute_mov_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[2] = 42;
            c.execute_instruction(Instruction::Mov{a: Register::R0, b: 1, c: Register::R2});
            assert_eq!(84, c.regs[0])
        }

        #[test]
        fn test_execute_mvn_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[2] = 42;
            c.execute_instruction(Instruction::Mvn{a: Register::R0, b: 2, c: Register::R2});
            assert_eq!(-168, c.regs[0])
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
