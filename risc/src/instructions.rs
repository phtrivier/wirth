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
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    MOD = 6,
    CMP = 7,
    MOVI = 16,
    MVNI = 17,
    ADDI = 18,
    SUBI = 19,
    MULI = 20,
    DIVI = 21,
    MODI = 22
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Mov {a: Register, b: u8, c: Register},
    Mvn {a: Register, b: u8, c: Register},
    Add {a: Register, b: Register, c: Register},
    Sub {a: Register, b: Register, c: Register},
    Mul {a: Register, b: Register, c: Register},
    Div {a: Register, b: Register, c: Register},
    Mod {a: Register, b: Register, c: Register},
    Cmp {b: Register, c: Register},
    Movi { a: Register, b: u8, im: i32 },
    Mvni { a: Register, b: u8, im: i32 },
    Addi {a: Register, b: Register, im: i32},
    Subi {a: Register, b: Register, im: i32},
    Muli {a: Register, b: Register, im: i32},
    Divi {a: Register, b: Register, im: i32},
    Modi {a: Register, b: Register, im: i32},

}

#[derive(Debug)]
pub enum InstructionParseError {
    InvalidInstruction(i32)
}

impl Instruction {
    pub fn encode(i : Instruction) -> i32 {
        match i {
            Instruction::Mov{a, b, c} => Instruction::encode_f0(OpCode::MOV, a, b, c),
            Instruction::Mvn{a, b, c} => Instruction::encode_f0(OpCode::MVN, a, b, c),
            Instruction::Add{a, b, c} => Instruction::encode_f0(OpCode::ADD, a, b as u8, c),
            Instruction::Sub{a, b, c} => Instruction::encode_f0(OpCode::SUB, a, b as u8, c),
            Instruction::Mul{a, b, c} => Instruction::encode_f0(OpCode::MUL, a, b as u8, c),
            Instruction::Div{a, b, c} => Instruction::encode_f0(OpCode::DIV, a, b as u8, c),
            Instruction::Mod{a, b, c} => Instruction::encode_f0(OpCode::MOD, a, b as u8, c),
            Instruction::Cmp{b, c} => Instruction::encode_f0(OpCode::CMP, Register::R0, b as u8, c),

            Instruction::Movi{a, b, im} => Instruction::encode_f1(OpCode::MOVI, a, b, im),
            Instruction::Mvni{a, b, im} => Instruction::encode_f1(OpCode::MVNI, a, b, im),
            Instruction::Addi{a, b, im} => Instruction::encode_f1(OpCode::ADD, a, b as u8, im),
            Instruction::Subi{a, b, im} => Instruction::encode_f1(OpCode::SUB, a, b as u8, im),
            Instruction::Muli{a, b, im} => Instruction::encode_f1(OpCode::MUL, a, b as u8, im),
            Instruction::Divi{a, b, im} => Instruction::encode_f1(OpCode::DIV, a, b as u8, im),
            Instruction::Modi{a, b, im} => Instruction::encode_f1(OpCode::MOD, a, b as u8, im),
        }
    }

    fn encode_f0(opcode: OpCode, a: Register, b: u8, c: Register) -> i32 {
        // 00(2) [op](4) a(4) b(4) padding(14) c(4)
        return (opcode as i32) * (2 as i32).pow(32 - 2 - 4)
            |  (a as i32) * (2 as i32).pow(32 - 2 - 4 - 4)
            |  (b as i32) * (2 as i32).pow(32 - 2 - 4 - 4 - 4)
            |  (c as i32)
            as i32;
    }

    fn encode_f1(opcode: OpCode, a: Register, b: u8, im: i32) -> i32 {
        // 00(2) [op](4) a(4) b(4) padding(14) c(4)
        return 0b01_0000_0000_0000_00_00_00_00_00_00_00_00_00
            | (opcode as i32) * (2 as i32).pow(32 - 2 - 4)
            |  (a as i32) * (2 as i32).pow(32 - 2 - 4 - 4)
            |  (b as i32) * (2 as i32).pow(32 - 2 - 4 - 4 - 4)
            |  im
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

        let b = ((i / 0x40000) % 0x10) as u8;

        let raw_c = u8::try_from(i % 0x40000);
        if raw_c.is_err() {
            return fail;
        }
        let parsed_c = Register::try_from(raw_c.unwrap());

        if parsed_op.is_err() || parsed_a.is_err() || parsed_c.is_err() {
            return fail;
        }

        let mut im = i % 0x40000;
        if im > 0x20000 {
            im = im - 0x40000;
        }

        let (op, a, c) = (parsed_op.unwrap(), parsed_a.unwrap(), parsed_c.unwrap());

        match op {
            OpCode::MOV => return Ok(Instruction::Mov{a: a, b: b, c: c}),
            OpCode::MVN => return Ok(Instruction::Mvn{a: a, b: b, c: c}),

            OpCode::ADD => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Add{a, b, c});
                }
            },
            OpCode::SUB => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Sub{a, b, c});
                }
            },
            OpCode::MUL => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Mul{a, b, c});
                }
            },
            OpCode::DIV => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Div{a, b, c});
                }
            },
            OpCode::MOD => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Mod{a, b, c});
                }
            }
            OpCode::CMP => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Cmp{b, c});
                }
            }

            OpCode::MOVI => {
                return Ok(Instruction::Movi{a, b, im});
            }
            OpCode::MVNI => {
                return Ok(Instruction::Mvni{a, b, im});
            }

            OpCode::ADDI => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Addi{a, b, im});
                }
            }
            OpCode::SUBI => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Subi{a, b, im});
                }
            }
            OpCode::MULI => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Muli{a, b, im});
                }
            }
            OpCode::DIVI => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Divi{a, b, im});
                }
            }
            OpCode::MODI => {
                if let Ok(b) = Register::try_from(b) {
                    return Ok(Instruction::Modi{a, b, im});
                }
            }
        }

        return fail;

    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_im_disp_c_part() {
        let inst = 0b00_0001_0001_0001_10_00_00_00_00_00_00_1110;

        // Get c as a register index (the first four bits)
        let c_as_register_index = inst % 0x10;
        assert_eq!(0b00_0000_0000_0000_00_00_00_00_00_00_00_1110, c_as_register_index);

        // Get the 18 bits part after the 'f', 'op', 'a' and 'b'
        let im = inst % 0x40000;
        assert_eq!(0b00_0000_0000_0000_10_00_00_00_00_00_00_1110, im);
        // The immediate value can ve seen as an integer, by default the sign is in the first bit of the whole thing, I suppose
        assert_eq!(131086, im);

        // The value after which we need to do sign extension
        let f1_sign_limit = 0x20000;
        assert_eq!(131072, f1_sign_limit);
        assert_eq!(0b00_0000_0000_0000_10_00_00_00_00_00_00_0000, f1_sign_limit);

        let neg_im = f1_sign_limit + 1;
        assert_eq!(131073, neg_im);
        let neg_im_with_sign_extension = neg_im - 0x40000;
        assert_eq!("11111111111111100000000000000001", format!("{:032b}", neg_im_with_sign_extension));
        assert_eq!(-131071, neg_im_with_sign_extension);

        // NOTE: same thing goes for extension for the bigger instructions
        // except sign_limit = 0x2000000 and you decrement by 0x4000000

        // Yes, remember that sign extension is a weird businnes...
        assert_eq!("11111111111111111111111111111111", format!("{:032b}", -1));
    }

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
