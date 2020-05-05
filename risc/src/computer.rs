pub enum OpCode {
    MOV = 0,
    /*
    MVN = 1,
    ADD = 2
*/
}

pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
/*
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15
*/
}

pub enum Intructions {
    Mov {a: Register, b: u32, c: Register}
}

// impl Instructuions
// parse(u32) -> u32
// encode(Instution) -> u32

pub struct Computer {
    pub regs: [u32; 16],
    pub mem: [u32; 4096]
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
            println!("REG {:02}: 0x{:04X} 0b{:32b}", index, reg, reg)
        }
    }

    pub fn execute(&self) {
        /*
        let nxt = self.regs[15] + 4;
        let ir = self.mem[ self.regs[15] % 4 ];
        let opc = (ir / 0x4000000) % 0x40;
        println!("nxt: {:x}, ir: {:x}, opc: {:x}", nxt, ir, opc)
        */
    }

    pub fn f0_instruction_raw(&self, opcode: OpCode, a: Register, b: u8, c: Register) -> u32 {
        // 00(2) [op](4) a(4) b(4) padding(14) c(4)
        // return (00 << 30) | (a as u8 << ;
        // shifts should do, I need to only take the 4 bits of what I need... so I need to put it on paper nicely :D
        return 0;
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_shift() {
        assert_eq!(3, 1 | (1 << 1));
    }

    // #[test]
    fn test_build_mov_instruction() {
        let mut c = Computer::new();
        c.regs[0] = 1;
        // TODO(pht) instruction::Mov::build ?
        let inst : u32 = c.f0_instruction_raw(OpCode::MOV, Register::R2, 0, Register::R1);
        assert_eq!(0, inst)
    }
}
