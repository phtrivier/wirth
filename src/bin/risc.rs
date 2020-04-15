enum Opcode {
    MOV = 0,
    MVN = 1,
    ADD = 2
}

struct Computer {
    regs: [u32; 16],
    mem: [u32; 4096]
}

impl Computer {
    fn new() -> Computer {
        Computer {
            regs: [0; 16],
            mem: [0; 4096]
        }
    }

    fn dumpRegs(&self) {
        for (index, reg) in self.regs.iter().enumerate() {
            println!("REG {:02}: 0x{:04X} 0b{:32b}", index, reg, reg)
        }
    }

    fn execute(&self) {
        /*
        let nxt = self.regs[15] + 4;
        let ir = self.mem[ self.regs[15] % 4 ];
        let opc = (ir / 0x4000000) % 0x40;
        println!("nxt: {:x}, ir: {:x}, opc: {:x}", nxt, ir, opc)
        */
    }

}

fn main() {
    let c = Computer::new();
    c.dumpRegs();
    c.execute();
}
