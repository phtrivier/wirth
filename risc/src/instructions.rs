/**
 * Computer Instruction Parsing and Encoding.
 */

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
/*
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    MOV = 0,
    MVN = 1,
//    ADD = 2
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Mov {a: Register, b: u32, c: Register},
    Mvn {a: Register, b: u32, c: Register}
}

#[derive(Debug)]
pub enum InstructionParseError {
    InvalidInstruction(i32)
}

impl Instruction {
    pub fn encode(i : Instruction) -> i32 {
        match i {
            Instruction::Mov{a, b, c} => Instruction::encode_f0(OpCode::MOV, a, b, c),
            Instruction::Mvn{a, b, c} => Instruction::encode_f0(OpCode::MVN, a, b, c)
        }
    }

    fn encode_f0(opcode: OpCode, a: Register, b: u32, c: Register) -> i32 {
        // 00(2) [op](4) a(4) b(4) padding(14) c(4)
        return (opcode as i32) * (2 as i32).pow(32 - 2 - 4)
            |  (a as i32) * (2 as i32).pow(32 - 2 - 4 - 4)
            |  (b as i32) * (2 as i32).pow(32 - 2 - 4 - 4 - 4)
            |  (c as i32)
            as i32;
    }

    pub fn parse(i : i32) -> Result<Instruction, InstructionParseError> {
        /*
         opc := IR DIV 4000000H MOD 40H;
        a := IR DIV 400000H MOD 10H;
        b := IR DIV 40000H MOD 10H;
        c := IR MOD 40000H
        */

        let fail = Err(InstructionParseError::InvalidInstruction(i));

        let raw_op = u8::try_from(i / 0x4000000);
        if raw_op.is_err() {
            return fail;
        }
        let parsed_op = OpCode::try_from(raw_op.unwrap());

        let raw_a = u8::try_from((i / 0x400000) % 0x10);
        if raw_a.is_err() {
            return fail;
        }
        let parsed_a = Register::try_from(raw_a.unwrap());

        let b = ((i / 0x40000) % 0x10) as u32;

        let raw_c = u8::try_from(i % 0x40000);
        if raw_c.is_err() {
            return fail;
        }
        let parsed_c = Register::try_from(raw_c.unwrap());

        if let (Ok(op), Ok(a), Ok(c)) = (parsed_op, parsed_a, parsed_c) {
            match op {
                OpCode::MOV => return Ok(Instruction::Mov{a: a, b: b, c: c}),
                OpCode::MVN => return Ok(Instruction::Mvn{a: a, b: b, c: c}),
            }
        }

        return fail;

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode_f0_instructions() {
        assert_both(Instruction::Mov{a: Register::R2, b: 5, c: Register::R1}, 0b00_0000_0010_0101_00000000000000_0001);
        assert_both(Instruction::Mvn{a: Register::R3, b: 2, c: Register::R4}, 0b00_0001_0011_0010_00000000000000_0100);
    }

    fn assert_both(inst: Instruction, i: i32) {
        assert_encoded(inst, i);
        assert_parsed(inst, i);
    }

    fn assert_encoded(inst: Instruction, expected : i32) {
        let actual = Instruction::encode(inst);
        assert_eq!(format!("{:032b}", expected), format!("{:032b}", actual))
    }

    fn assert_parsed(expected: Instruction, i: i32) {
        let parsed = Instruction::parse(i).unwrap();
        assert_eq!(expected, parsed)
    }

}
