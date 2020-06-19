#[cfg(test)]
mod tests {
    use crate::computer::Computer;
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
            c.execute_instruction(Register { o: MOV, a: 0, b: 1, c: 2 });
            assert_eq!(84, c.regs[0]);

            c.execute_instruction(Register { o: MVN, a: 0, b: 2, c: 2 });
            assert_eq!(-168, c.regs[0])
        }

        #[test]
        fn test_execute_immediate_move_instruction() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.execute_instruction(RegisterIm { o: MOVI, a: 0, b: 1, im: 42 });
            assert_eq!(84, c.regs[0]);

            let mut c = Computer::new();
            c.regs[0] = 0;
            c.execute_instruction(RegisterIm { o: MVNI, a: 0, b: 2, im: 42 });
            assert_eq!(-168, c.regs[0]);
        }

        #[test]
        fn test_execute_register_arithmetic_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;
            // R.a = R.b + R.c
            c.execute_instruction(Register { o: ADD, a: 0, b: 1, c: 2 });
            assert_eq!(42, c.regs[0]);

            // R.a = R.b - R.c
            c.execute_instruction(Register { o: SUB, a: 0, b: 1, c: 2 });
            assert_eq!(-22, c.regs[0]);

            // R.a = R.b * R.c
            c.execute_instruction(Register { o: MUL, a: 0, b: 1, c: 2 });
            assert_eq!(320, c.regs[0]);

            // R.a = R.b / R.c
            c.execute_instruction(Register { o: DIV, a: 0, b: 2, c: 1 });
            assert_eq!(3, c.regs[0]);

            // R.a = R.b % R.c
            c.execute_instruction(Register { o: MOD, a: 0, b: 2, c: 1 });
            assert_eq!(2, c.regs[0]);
        }

        #[test]
        fn test_execute_immediate_arithmetic_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;

            c.execute_instruction(RegisterIm { o: ADDI, a: 0, b: 1, im: 32 });
            assert_eq!(42, c.regs[0]);

            // R.a = R.b - im
            c.execute_instruction(RegisterIm { o: SUBI, a: 0, b: 1, im: 32 });
            assert_eq!(-22, c.regs[0]);

            // R.a = R.b * im
            c.execute_instruction(RegisterIm { o: MULI, a: 0, b: 1, im: 32 });
            assert_eq!(320, c.regs[0]);

            // R.a = R.b / im
            c.execute_instruction(RegisterIm { o: DIVI, a: 0, b: 1, im: 3 });
            assert_eq!(3, c.regs[0]);

            // R.a = R.b % im
            c.execute_instruction(RegisterIm { o: MODI, a: 0, b: 1, im: 3 });
            assert_eq!(1, c.regs[0]);
        }

        #[test]
        fn test_execute_register_compare() {
            let mut c = Computer::new();
            c.regs[0] = 0;
            c.regs[1] = 10;
            c.regs[2] = 32;

            c.execute_instruction(Register { o: CMP, a: 0, b: 1, c: 2 });
            // R.b == R.c ?
            assert_eq!(false, c.z_test);
            // R.b < R.c ?
            assert_eq!(true, c.neg_test);

            c.regs[1] = 10;
            c.regs[2] = 10;
            c.execute_instruction(Register { o: CMP, a: 0, b: 1, c: 2 });
            // R.b == R.c ?
            assert_eq!(true, c.z_test);
            // R.b < R.c ?
            assert_eq!(false, c.neg_test);

            c.regs[1] = -32;
            c.regs[2] = 10;
            c.execute_instruction(Register { o: CMP, a: 0, b: 1, c: 2 });
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

            c.execute_instruction(RegisterIm { o: CMPI, a: 0, b: 1, im: -32 });
            assert_eq!(false, c.z_test);
            assert_eq!(false, c.neg_test);

            c.execute_instruction(RegisterIm { o: CMPI, a: 0, b: 1, im: 10 });
            assert_eq!(true, c.z_test);
            assert_eq!(false, c.neg_test);

            c.execute_instruction(RegisterIm { o: CMPI, a: 0, b: 1, im: 32 });
            assert_eq!(false, c.z_test);
            assert_eq!(true, c.neg_test);
        }

        #[test]
        fn test_execute_chki() {
            let mut c = Computer::new();
            c.regs[0] = 30;
            c.execute_instruction(RegisterIm { o: CHKI, b: 0, a: 0, im: 32 });
            assert_eq!(c.regs[0], 30);

            c.execute_instruction(RegisterIm { o: CHKI, b: 0, a: 0, im: 15 });
            assert_eq!(c.regs[0], 0);

            c.regs[0] = -10;
            c.execute_instruction(RegisterIm { o: CHKI, b: 0, a: 0, im: 15 });
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

            c.execute_instruction(Memory { o: LDW, a: 0, b: 1, disp: 4 });
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
            c.execute_instruction(Memory { o: PSH, a: 0, b: 1, disp: 1 });
            assert_eq!(c.regs[1], 9);
            assert_eq!(c.mem[9], 42);

            c.regs[0] = 0;
            // ```
            // POP:
            //  R[a] := M[(R[b]) DIV 4];
            //  INC(R[b], c)
            // ```
            c.execute_instruction(Memory { o: POP, a: 0, b: 1, disp: 1 });
            assert_eq!(c.regs[0], 42);
            assert_eq!(c.regs[1], 10);
        }

        #[test]
        fn test_execute_store_memory_instructions() {
            let mut c = Computer::new();
            c.regs[0] = 42;
            c.regs[1] = 10;

            // M[(R[b] + c) DIV 4] := R[a]
            c.execute_instruction(Memory { o: STW, a: 0, b: 1, disp: 2 });
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
            assert_eq!(c.next, 41);

            c.neg_test = true;
            c.execute_instruction(Branch { o: BLT, disp: 2 });
            assert_eq!(c.next, 42);

            c.execute_instruction(Branch { o: BLE, disp: 3 });
            assert_eq!(c.next, 43);

            c.z_test = false;
            c.neg_test = false;

            c.execute_instruction(Branch { o: BNE, disp: 1 });
            assert_eq!(c.next, 41);

            c.execute_instruction(Branch { o: BGE, disp: -2 });
            assert_eq!(c.next, 38);

            c.execute_instruction(Branch { o: BGT, disp: 3 });
            assert_eq!(c.next, 43);

            c.execute_instruction(Branch { o: BR, disp: 12 });
            assert_eq!(c.next, 52);

            c.execute_instruction(Branch { o: BSR, disp: 10 });
            assert_eq!(c.regs[14], 40);
            assert_eq!(c.next, 50);

            c.execute_instruction(Branch { o: RET, disp: 1 });
            assert_eq!(c.next, 10);

            c.regs[1] = 0;
            c.execute_instruction(Branch { o: RET, disp: 1 });
            assert_eq!(c.next, 0);
            assert_eq!(c.done_flag, true);
        }
    }

    #[test]
    fn test_program_execution_to_end() {
        let mut c = Computer::new();

        // MOVI $0, 5
        let mut instruction = RegisterIm { o: MOVI, a: 0, b: 0, im: 5 };
        let mut instruction_data = Instruction::encode(instruction);
        c.mem[1] = instruction_data as i32;

        // MOVI $1, 10
        instruction = RegisterIm { o: MOVI, a: 1, b: 0, im: 10 };
        instruction_data = Instruction::encode(instruction);
        c.mem[2] = instruction_data as i32;

        // RET $2
        instruction = Branch { o: RET, disp: 2 };
        instruction_data = Instruction::encode(instruction);
        c.mem[3] = instruction_data as i32;

        c.execute_at(5, 1, true);

        assert_eq!(c.done_flag, true);
        assert_eq!(c.regs[0], 5);
        assert_eq!(c.regs[1], 10);
    }
}
