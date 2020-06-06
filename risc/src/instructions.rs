/**
 * Computer Instruction Parsing and Encoding.
 */
extern crate strum;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use strum_macros::EnumString;

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, EnumString)]
#[repr(u8)]
pub enum RegisterOpCode {
    MOV = 0,
    MVN = 1,
    ADD = 2,
    SUB = 3,
    MUL = 4,
    DIV = 5,
    MOD = 6,
    CMP = 7,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, EnumString)]
#[repr(u8)]
pub enum RegisterImOpCode {
    MOVI = 16,
    MVNI = 17,
    ADDI = 18,
    SUBI = 19,
    MULI = 20,
    DIVI = 21,
    MODI = 22,
    CMPI = 23,
    CHKI = 24,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, EnumString)]
#[repr(u8)]
pub enum MemoryOpCode {
    LDW = 32,
    POP = 34,
    STW = 36,
    PSH = 38,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive, EnumString)]
#[repr(u8)]
pub enum BranchOpCode {
    BEQ = 48,
    BNE = 49,
    BLT = 50,
    BGE = 51,
    BLE = 52,
    BGT = 53,
    BR = 56,
    BSR = 57,
    RET = 58,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Register {
        o: RegisterOpCode,
        a: usize,
        b: usize,
        c: usize,
    },
    RegisterIm {
        o: RegisterImOpCode,
        a: usize,
        b: usize,
        im: i32,
    },
    Memory {
        o: MemoryOpCode,
        a: usize,
        b: usize,
        disp: i32,
    },
    Branch {
        o: BranchOpCode,
        disp: i32,
    },
}

#[derive(Debug)]
pub enum InstructionParseError {
    InvalidInstruction(u32),
}

impl Instruction {
    pub fn encode(i: Instruction) -> u32 {
        match i {
            Instruction::Register { o, a, b, c } => Instruction::encode_f0(o, a, b, c),
            Instruction::RegisterIm { o, a, b, im } => Instruction::encode_f1(o, a, b, im),
            Instruction::Memory { o, a, b, disp } => Instruction::encode_f2(o, a, b, disp),
            Instruction::Branch { o, disp } => Instruction::encode_f3(o, disp),
        }
    }

    fn encode_f0(opcode: RegisterOpCode, a: usize, b: usize, c: usize) -> u32 {
        // 00(2) [op](4) a(4) b(4) im (18)
        return 0b00_0000_0000_0000_00_00_00_00_00_00_00_00_00
            | (opcode as u32) << (32 - 2 - 4)
            | (a as u32) << (32 - 2 - 4 - 4)
            | (b as u32) << (32 - 2 - 4 - 4 - 4)
            | c as u32;
    }

    // TODO(pht) rename to sign extend on 18 bits
    fn adjust_im(im: i32) -> u32 {
        let mut dd = im;
        if dd < 0 {
            dd = dd + 0x400000
        }
        return dd as u32;
    }

    // TODO(pht) rename to sign extend on 26 bits
    fn adjust_disp(disp: i32) -> u32 {
        let mut dd = disp;
        if dd < 0 {
            dd = dd + 0x4000000
        }
        return dd as u32;
    }

    fn encode_f1(opcode: RegisterImOpCode, a: usize, b: usize, im: i32) -> u32 {
        // 01(2) [op](4) a(4) b(4) im (18)
        return 0b01_0000_0000_0000_00_00_00_00_00_00_00_00_00
            | (opcode as u32) << (32 - 2 - 4)
            | (a as u32) << (32 - 2 - 4 - 4)
            | (b as u32) << (32 - 2 - 4 - 4 - 4)
            | Self::adjust_im(im) as u32;
    }

    fn encode_f2(opcode: MemoryOpCode, a: usize, b: usize, disp: i32) -> u32 {
        // 10(2) [op](4) a(4) b(4) disp (18)
        return 0b10_0000_0000_0000_00_00_00_00_00_00_00_00_00
            | (opcode as u32) << (32 - 2 - 4)
            | (a as u32) << (32 - 2 - 4 - 4)
            | (b as u32) << (32 - 2 - 4 - 4 - 4)
            | Self::adjust_im(disp) as u32;
    }

    fn encode_f3(opcode: BranchOpCode, disp: i32) -> u32 {
        // 11(2) [op](4) dest (28)
        // But warning, the op code is too big to fit on 4 bits
        return 0b11_0000_00_00_00_00_00_00_00_00_00_00_00_00_00
            | ((opcode as u32) % 0x10) << (32 - 2 - 4)
            | Self::adjust_disp(disp) as u32;
    }

    pub fn parse(i: u32) -> Result<Instruction, InstructionParseError> {
        /*
         opc := IR DIV 4000000H MOD 40H;
        a := IR DIV 400000H MOD 10H;
        b := IR DIV 40000H MOD 10H;
        c := IR MOD 40000H
        */

        let op = ((i / 0x4000000) % 0x40) as u8;
        let a = ((i / 0x400000) % 0x10) as usize;
        let b = ((i / 0x40000) % 0x10) as usize;
        let c = (i % 0x40000) as usize;

        let mut im = (i % 0x40000) as i32;
        if im > 0x20000 {
            im = im - 0x40000;
        }

        let mut disp = (i % 0x4000000) as i32;
        if disp > 0x2000000 {
            disp = disp - 0x4000000;
        }

        if let Ok(o) = RegisterOpCode::try_from(op) {
            return Ok(Instruction::Register { o: o, a, b, c });
        }
        if let Ok(o) = RegisterImOpCode::try_from(op) {
            return Ok(Instruction::RegisterIm { o: o, a, b, im });
        }
        if let Ok(o) = MemoryOpCode::try_from(op) {
            return Ok(Instruction::Memory { o, a, b, disp });
        }
        if let Ok(o) = BranchOpCode::try_from(op) {
            return Ok(Instruction::Branch { o, disp });
        }

        return Err(InstructionParseError::InvalidInstruction(i));
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
        assert_eq!(
            0b00_0000_0000_0000_00_00_00_00_00_00_00_1110,
            c_as_register_index
        );

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
        assert_eq!(
            "11111111111111100000000000000001",
            format!("{:032b}", neg_im_with_sign_extension)
        );
        assert_eq!(-131071, neg_im_with_sign_extension);

        // NOTE: same thing goes for extension for the bigger instructions
        // except sign_limit = 0x2000000 and you decrement by 0x4000000

        // Yes, remember that sign extension is a weird businnes...
        assert_eq!("11111111111111111111111111111111", format!("{:032b}", -1));
    }

    #[test]
    fn test_encode_f0_instructions() {
        assert_both(
            Instruction::Register {
                o: RegisterOpCode::MOV,
                a: 2,
                b: 5,
                c: 1,
            },
            0b00_0000_0010_0101_00000000000000_0001,
        );
        assert_both(
            Instruction::Register {
                o: RegisterOpCode::MVN,
                a: 3,
                b: 2,
                c: 4,
            },
            0b00_0001_0011_0010_00000000000000_0100,
        );
    }

    #[test]
    fn test_encode_f3_instructions() {
        /*
        assert_both(
            Instruction::Branch {
                o: BranchOpCode::BEQ,
                disp: 4,
            },
            0b11_0000_00000000000000000000000100,
        );
        */

        /*
        println!("{:08b}", 48);
        println!("{:08b}", 49);

        println!("{:032b}", -2 as i32);
        println!("{:032b}", -2 + 0x4000000 as i32);
        */

        assert_encoded(
            Instruction::Branch {
                o: BranchOpCode::BNE,
                disp: -2,
            },
            0b11_0001_11111111111111111111111110
        );

        assert_parsed(
            Instruction::Branch {
                o: BranchOpCode::BNE,
                disp: -2,
            },
            0b11_0001_11111111111111111111111110
        );
    }


    fn assert_both(inst: Instruction, i: u32) {
        assert_encoded(inst, i);
        assert_parsed(inst, i);
    }

    fn assert_encoded(inst: Instruction, expected: u32) {
        let actual = Instruction::encode(inst);
        assert_eq!(format!("{:032b}", expected), format!("{:032b}", actual))
    }

    fn assert_parsed(expected: Instruction, i: u32) {
        let parsed = Instruction::parse(i).unwrap();
        assert_eq!(expected, parsed)
    }
}
