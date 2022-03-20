#[cfg(test)]
mod tests {
    use crate::computer::Computer;
    use crate::instructions::BranchCondition::*;
    use crate::instructions::Instruction;
    use crate::instructions::Instruction::*;
    use crate::instructions::MemoryMode;
    use crate::instructions::OpCode::*;

    fn exec(c: &mut Computer, i: Instruction) {
        c.execute_instruction(i, false);
    }

    #[test]
    fn test_mov() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 9999; // b will be ignored
        c.regs[2] = 42;

        // R.a := R.c
        exec(&mut c, Register { o: MOV, a: 0, b: 1, c: 2 });
        assert_eq!(42, c.regs[0]);
        assert_eq!(false, c.z_test);
        assert_eq!(false, c.neg_test);
        exec(&mut c, RegisterIm { o: MOV, a: 0, b: 1, im: -84 });
        assert_eq!(-84, c.regs[0]);
        assert_eq!(false, c.z_test);
        assert_eq!(true, c.neg_test);

        exec(&mut c, RegisterIm { o: MOV, a: 0, b: 1, im: 0 });
        assert_eq!(0, c.regs[0]);
        assert_eq!(true, c.z_test);
        assert_eq!(false, c.neg_test);
    }

    #[test]
    fn test_lsl() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 42;
        c.regs[2] = 1;
        // R.a := R.b << R.c
        exec(&mut c, Register { o: LSL, a: 0, b: 1, c: 2 });
        assert_eq!(84, c.regs[0]);

        exec(&mut c, RegisterIm { o: LSL, a: 0, b: 1, im: 2 });
        assert_eq!(168, c.regs[0]);
    }

    #[test]
    fn test_asr() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 42;
        c.regs[2] = 1;
        // R.a := R.b >> R.c
        exec(&mut c, Register { o: ASR, a: 0, b: 1, c: 2 });
        assert_eq!(21, c.regs[0]);

        c.regs[1] = 42;
        exec(&mut c, RegisterIm { o: ASR, a: 0, b: 1, im: 1 });
        assert_eq!(21, c.regs[0]);

        c.regs[1] = -42;
        // R.a := R.b >> R.c (with sign extension)
        exec(&mut c, Register { o: ASR, a: 0, b: 1, c: 2 });
        assert_eq!(-21, c.regs[0]);

        c.regs[1] = -42;
        // R.a := R.b >> R.c (with sign extension)
        exec(&mut c, RegisterIm { o: ASR, a: 0, b: 1, im: 1 });
        assert_eq!(-21, c.regs[0]);
    }

    #[test]
    fn test_ror() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 0b0001_0010_0011_0100_0101_0110_0111_1000;
        c.regs[2] = 4;
        // R.a := R.b rot R.c
        exec(&mut c, Register { o: ROR, a: 0, b: 1, c: 2 });
        assert_eq!(0b1000_0001_0010_0011_0100_0101_0110_0111_u32 as i32, c.regs[0]);

        c.regs[2] = -4;
        exec(&mut c, Register { o: ROR, a: 0, b: 1, c: 2 });
        assert_eq!(0b0010_0011_0100_0101_0110_0111_1000_0001_u32 as i32, c.regs[0]);

        exec(&mut c, RegisterIm { o: ROR, a: 0, b: 1, im: 4 });
        assert_eq!(0b1000_0001_0010_0011_0100_0101_0110_0111_u32 as i32, c.regs[0]);

        exec(&mut c, RegisterIm { o: ROR, a: 0, b: 1, im: -4 });
        assert_eq!(0b0010_0011_0100_0101_0110_0111_1000_0001_u32 as i32, c.regs[0]);
    }
    #[test]
    fn test_logical_register() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 0b0001_0010_0011_0100_0101_0110_0111_1000;
        c.regs[2] = 0b0000_0000_0000_0000_0000_0000_0000_1111;
        // R.a := R.b & R.c
        exec(&mut c, Register { o: AND, a: 0, b: 1, c: 2 });
        assert_eq!(0b0000_0000_0000_0000_0000_0000_0000_1000, c.regs[0] as u32);

        // R.a := R.b & not R.c
        exec(&mut c, Register { o: ANN, a: 0, b: 1, c: 2 });
        assert_eq!(0b0001_0010_0011_0100_0101_0110_0111_0000, c.regs[0] as u32);

        // R.a := R.b | R.c
        exec(&mut c, Register { o: IOR, a: 0, b: 1, c: 2 });
        assert_eq!(0b0001_0010_0011_0100_0101_0110_0111_1111, c.regs[0] as u32);

        // R.a := R.b xor R.c
        exec(&mut c, Register { o: XOR, a: 0, b: 1, c: 2 });
        assert_eq!(0b0001_0010_0011_0100_0101_0110_0111_0111, c.regs[0] as u32);

        // I'm a bit lazy, and I trust my implementation for the "imediate" part ;)
    }

    #[test]
    fn test_arithmetic_register() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 10;
        c.regs[2] = 32;
        // R.a = R.b + R.c
        exec(&mut c, Register { o: ADD, a: 0, b: 1, c: 2 });
        assert_eq!(42, c.regs[0]);

        // R.a = R.b - R.c
        exec(&mut c, Register { o: SUB, a: 0, b: 1, c: 2 });
        assert_eq!(-22, c.regs[0]);

        // R.a = R.b * R.c
        exec(&mut c, Register { o: MUL, a: 0, b: 1, c: 2 });
        assert_eq!(320, c.regs[0]);

        // R.a = R.b / R.c
        exec(&mut c, Register { o: DIV, a: 0, b: 2, c: 1 });
        assert_eq!(3, c.regs[0]);

        c.regs[1] = 21;
        c.regs[2] = 10;

        // R.a = R.b % R.c
        exec(&mut c, Register { o: MOD, a: 0, b: 1, c: 2 });
        assert_eq!(1, c.regs[0]);

        // I'm a bit lazy, and I trust my implementation for the "imediate" part ;)
    }

    #[test]
    fn test_execute_memory_instruction() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 10;

        c.mem[14] = -42;

        // R.a := Mem[R.b + off]
        exec(
            &mut c,
            Memory {
                u: MemoryMode::Load,
                a: 0,
                b: 1,
                offset: 4,
            },
        );
        assert_eq!(c.regs[0], -42);
        assert_eq!(true, c.neg_test);
        assert_eq!(false, c.z_test);

        exec(
            &mut c,
            Memory {
                u: MemoryMode::Store,
                a: 0,
                b: 1,
                offset: 5,
            },
        );
        assert_eq!(c.mem[15], -42);
    }

    #[test]
    fn test_branch_instructions() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 10;
        c.regs[15] = 0;

        // In practice, in the loop,
        // `c.pc` would be incremented by one at each instruction.
        c.pc = 40;

        // Branch to R.c if N
        c.neg_test = true;
        exec(&mut c, Branch { cond: MI, c: 1, link: false });
        assert_eq!(c.pc, 10);
        assert_eq!(c.regs[15], 0);

        // Branch to PC + offset if Z
        exec(&mut c, BranchOff { cond: MI, offset: 2, link: false });
        assert_eq!(c.pc, 12);
        assert_eq!(c.regs[15], 0);

        // Branch to R.c if Z and store return address
        exec(&mut c, Branch { cond: MI, c: 1, link: true });
        assert_eq!(c.pc, 10);
        assert_eq!(c.regs[15], 12); // Address has been stored

        // Branch to PC + offset if Z and store return address
        exec(&mut c, BranchOff { cond: MI, offset: -3, link: true });
        assert_eq!(c.pc, 7);
        assert_eq!(c.regs[15], 10);
    }

    // NOTE(pht) previous test gives me enough confidence that exhaustively
    // checking `matches_cond` is enough to test all branch cases.
    #[test]
    fn match_conditions_against_flags() {
        let mut c = Computer::new();
        c.regs[0] = -42;
        c.update_flags(0);
        assert_eq!(true, c.neg_test);
        assert_eq!(false, c.z_test);
        assert_eq!(true, c.matches_cond(MI)); // -42 < 0 ?
        assert_eq!(false, c.matches_cond(EQ)); // -42 == 0 ?
        assert_eq!(true, c.matches_cond(LT)); // -42 < 0 (implicitely since we don't have carry and overflow)
        assert_eq!(true, c.matches_cond(LE)); // -42 <= 0 ?
        assert_eq!(true, c.matches_cond(AW)); // true ?
        assert_eq!(false, c.matches_cond(PL)); // -42 > 0 ?
        assert_eq!(true, c.matches_cond(NE)); // -42 != 0 ?
        assert_eq!(false, c.matches_cond(GE)); // -42 > 0 ?
        assert_eq!(false, c.matches_cond(GT)); // -42 >= 0 ?
        assert_eq!(false, c.matches_cond(NV)); // false

        c.regs[0] = 0;
        c.update_flags(0);
        assert_eq!(false, c.neg_test);
        assert_eq!(true, c.z_test);
        assert_eq!(false, c.matches_cond(MI)); // 0 < 0 ?
        assert_eq!(true, c.matches_cond(EQ)); // 0 == 0 ?
        assert_eq!(false, c.matches_cond(LT)); // 0 < 0 (implicitely since we don't have carry and overflow)
        assert_eq!(true, c.matches_cond(LE)); // 0 <= 0 ?
        assert_eq!(true, c.matches_cond(AW)); // true ?
        assert_eq!(true, c.matches_cond(PL)); // 0 > 0 ? (this is a special case)
        assert_eq!(false, c.matches_cond(NE)); // 0 != 0 ?
        assert_eq!(true, c.matches_cond(GE)); // 0 > 0 ? (special case)
        assert_eq!(false, c.matches_cond(GT)); // 0 >= 0 ? (not sure how to interpret this one...)
        assert_eq!(false, c.matches_cond(NV)); // false

        c.regs[0] = 42;
        c.update_flags(0);
        assert_eq!(false, c.neg_test);
        assert_eq!(false, c.z_test);
        assert_eq!(false, c.matches_cond(MI)); // 42 < 0 ?
        assert_eq!(false, c.matches_cond(EQ)); // 42 == 0 ?
        assert_eq!(false, c.matches_cond(LT)); // 42 < 0 (implicitely since we don't have carry and overflow)
        assert_eq!(false, c.matches_cond(LE)); // 42 <= 0 ?
        assert_eq!(true, c.matches_cond(AW)); // true ?
        assert_eq!(true, c.matches_cond(PL)); // 42 > 0 ?
        assert_eq!(true, c.matches_cond(NE)); // 42 != 0 ?
        assert_eq!(true, c.matches_cond(GE)); // 42 > 0 ?
        assert_eq!(true, c.matches_cond(GT)); // 42 >= 0 ?
        assert_eq!(false, c.matches_cond(NV)); // false
    }

    #[test]
    fn test_program_execution_to_end() {
        let mut c = Computer::new();
        c.regs[0] = 0;
        c.regs[1] = 0;
        c.regs[2] = 0;

        // MOVI $0, 5
        let mut instruction = RegisterIm { o: MOV, a: 0, b: 0, im: 5 };
        let mut instruction_data = Instruction::encode(&instruction);
        c.mem[0] = instruction_data as i32;

        // MOVI $1, 10
        instruction = RegisterIm { o: MOV, a: 1, b: 0, im: 10 };
        instruction_data = Instruction::encode(&instruction);
        c.mem[1] = instruction_data as i32;

        // BPL $2 -> Which happens to be 0, so this will end execution
        instruction = Branch { cond: PL, c: 2, link: false };
        instruction_data = Instruction::encode(&instruction);
        c.mem[2] = instruction_data as i32;

        let max_cycles = 5;
        let debug = false;
        c.execute(max_cycles, debug);

        assert_eq!(c.regs[0], 5);
        assert_eq!(c.regs[1], 10);
        assert_eq!(c.pc, 0);
    }

    #[test]
    fn test_assembled_program() {
        // NOTE(pht) this is the same program as show in `assembler_test.rs`.
        // Normally, it should increment R1 to 6.
        let instructions = vec![
            RegisterIm { o: MOV, a: 0, b: 0, im: 3 },       //
            RegisterIm { o: MOV, a: 1, b: 0, im: 0 },       //
            RegisterIm { o: ADD, a: 1, b: 1, im: 2 },       //
            RegisterIm { o: SUB, a: 0, b: 0, im: 1 },       //
            BranchOff { cond: EQ, link: false, offset: 2 }, //
            BranchOff {
                cond: AW,
                link: false,
                offset: -4,
            }, // FIXME(pht) assembler gives -3 for this, see if it is a bug in the assembler for negative offset
            RegisterIm { o: MOV, a: 2, b: 0, im: 0 },       //
            Branch { cond: AW, link: false, c: 2 },         //
        ];

        let mut c = Computer::new();
        c.load_instructions(instructions);

        let max_cycles = 50;
        let debug = true;
        c.execute(max_cycles, debug);

        assert_eq!(c.regs[0], 0);
        assert_eq!(c.regs[1], 6);
        assert_eq!(c.regs[2], 0);
        assert_eq!(c.pc, 0);
    }
}
