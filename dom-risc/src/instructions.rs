#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    MOV = 0, // R.a = n
    LSL = 1, // R.a = R.b <- n (shift left)
    ASR = 2, // R.a = R.b -> n (shift right with sign extension)
    ROR = 3,
    AND = 4,
    ANN = 5, // R.a = R.b & ~n
    IOR = 6, // R.a = R.b or n
    XOR = 7, // R.a = R.b xor n
    ADD = 8,
    SUB = 9,
    MUL = 10,
    DIV = 11,
    MOD = 12, // Not strictly in the book, but I could not do anything without it !
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MemoryMode {
    Load,
    Store,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BranchCondition {
    MI = 0,
    EQ = 1,
    // Ignored CS = 2, Ignored condition since we don't do signed arithmetic
    // Ignored VS = 3,
    // Ignored LS = 4,
    LT = 5,
    LE = 6,
    AW = 7, // Always
    PL = 8,
    NE = 9,
    // Ignored CC = 10,
    // Ignored VC = 11,
    // Ignored HI = 12,
    GE = 13,
    GT = 14,
    NV = 15, // Never
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Register { a: usize, b: usize, o: OpCode, c: usize },
    RegisterIm { a: usize, b: usize, o: OpCode, im: i32 },        // 'v' modified is implied by im > 0 or im < 0, I suppose ?
    Memory { a: usize, b: usize, offset: u32, u: MemoryMode },    // weirdly, memory offsets are positive
    Branch { cond: BranchCondition, c: usize, link: bool },       // I don't understand u/v enough to give them better names, yet
    BranchOff { cond: BranchCondition, offset: i32, link: bool }, // I don't understand u/v enough to give them better names, yet
}

#[derive(Debug)]
pub enum InstructionParseError {
    InvalidInstruction(u32),
    InvalidOpCode(u8),
    InvalidBranchCondition(u32),
}

impl Instruction {
    pub fn encode(i: &Instruction) -> u32 {
        match i {
            Instruction::Register { o, a, b, c } => Instruction::encode_register(*o, *a, *b, *c),
            Instruction::RegisterIm { o, a, b, im } => Instruction::encode_register_im(*o, *a, *b, *im),
            Instruction::Memory { a, b, offset, u } => Instruction::encode_memory(*a, *b, *offset, *u),
            Instruction::Branch { cond, c, link } => Instruction::encode_branch(*cond, *c, *link),
            Instruction::BranchOff { cond, offset, link } => Instruction::encode_branch_offset(*cond, *offset, *link),
        }
    }

    fn encode_register(op: OpCode, a: usize, b: usize, c: usize) -> u32 {
        //  0000(4) a(4) b(4) [op](4) 000000000000(12) c (4)
        c as u32 | (op as u32) << (4 + 12) | (b as u32) << (4 + 12 + 4) | (a as u32) << (4 + 4 + 12 + 4)
    }

    const F1_IM_EXTENSION_MASK: u32 = 0b0000_0000_0000_0000_11_11_11_11_11_11_11_11;

    fn encode_register_im(op: OpCode, a: usize, b: usize, im: i32) -> u32 {
        let mut v = 0;
        if im < 0 {
            v = 1;
        }

        //  0100(4) a(4) b(4) [op](4) im(16)
        0b0100_0000_0000_0000_00_00_00_00_00_00_00_00
            | ((im as u32) & Self::F1_IM_EXTENSION_MASK)
            | (op as u32) << (4 + 12)
            | (b as u32) << (4 + 12 + 4)
            | (a as u32) << (4 + 4 + 12 + 4)
            | (v as u32) << (32 - 4)
    }

    fn encode_memory(a: usize, b: usize, offset: u32, u: MemoryMode) -> u32 {
        let mut uv = 0b1000;
        if u == MemoryMode::Store {
            uv = 0b1010;
        }

        ((offset as u32) & 0b0000_0000_0000_1111_1111_1111_1111_1111) | (b as u32) << (20) | (a as u32) << (4 + 20) | (uv as u32) << (32 - 4)
    }

    fn encode_branch(cond: BranchCondition, c: usize, link: bool) -> u32 {
        let mut uv = 0b1100;
        if link {
            uv = 0b1101;
        }

        (c as u32) | (cond as u32) << (20 + 4) | (uv as u32) << (32 - 4)
    }

    fn encode_branch_offset(cond: BranchCondition, offset: i32, link: bool) -> u32 {
        let mut uv = 0b1110;
        if link {
            uv = 0b1111;
        }

        ((offset as u32) & 0b0000_0000_1111_1111_1111_1111_1111_1111) | (cond as u32) << 24 | (uv as u32) << (32 - 4)
    }

    pub fn parse(i: u32) -> Result<Instruction, InstructionParseError> {
        // NOTE(pht) there is definitely a clearer way to test for the kind of
        // instruction.
        // And I'm pretty sure I can improve the general look and feel of those functions,
        // as well as the organisation.
        // But that's a job for later !
        if (i / 0x80000000) % 2 == 0 {
            Instruction::parse_register(i)
        } else if (i / 0x40000000) % 2 == 0 {
            Instruction::parse_memory(i)
        } else if (i & 0b0010_0000_0000_0000_0000_0000_0000_0000) == 0 {
            Instruction::parse_branch(i)
        } else {
            Instruction::parse_branch_offset(i)
        }
    }

    fn parse_register(i: u32) -> Result<Instruction, InstructionParseError> {
        let a = ((i / 0x1000000) % 0x10) as usize;
        let b = ((i / 0x100000) % 0x10) as usize;
        let c = (i % 0x10) as usize;

        let op = ((i / 0x10000) % 0x10) as u8;
        let im = (i % 0x10000) as i32;

        let o = Instruction::parse_op_code(op)?;
        if ((i / 0x40000000) % 2) == 0 {
            Ok(Instruction::Register { a, b, o, c })
        } else if (i / 0x10000000) % 2 == 0 {
            Ok(Instruction::RegisterIm { a, b, o, im })
        } else {
            let im_extended = ((im as u32) | 0xFFFF0000) as i32;
            Ok(Instruction::RegisterIm { a, b, o, im: im_extended })
        }
    }

    fn parse_op_code(op: u8) -> Result<OpCode, InstructionParseError> {
        match op {
            0 => Ok(OpCode::MOV), // R.a n
            1 => Ok(OpCode::LSL), // R.a R.b <- n (shift left),
            2 => Ok(OpCode::ASR), // R.a R.b -> n (shift right with sign extension),
            3 => Ok(OpCode::ROR),
            4 => Ok(OpCode::AND),
            5 => Ok(OpCode::ANN), // R.a R.b & ~n
            6 => Ok(OpCode::IOR), // R.a R.b or n
            7 => Ok(OpCode::XOR), // R.a R.b xor n
            8 => Ok(OpCode::ADD),
            9 => Ok(OpCode::SUB),
            10 => Ok(OpCode::MUL),
            11 => Ok(OpCode::DIV),
            12 => Ok(OpCode::MOD),
            _ => Err(InstructionParseError::InvalidOpCode(op)),
        }
    }

    fn parse_memory(i: u32) -> Result<Instruction, InstructionParseError> {
        let a = ((i / 0x1000000) % 0x10) as usize;
        let b = ((i / 0x100000) % 0x10) as usize;
        let offset = (i & 0b0000_0000_0000_1111_1111_1111_1111_1111) as u32;
        let mut u = MemoryMode::Load;
        if i & 0b0010_0000_0000_0000_0000_0000_0000_0000 != 0 {
            u = MemoryMode::Store;
        }
        Ok(Instruction::Memory { a, b, offset, u })
    }

    fn parse_branch(i: u32) -> Result<Instruction, InstructionParseError> {
        let a = ((i / 0x1000000) % 0x10) as usize;
        let cond = Instruction::parse_cond(a)?;
        let c = (i % 0x10) as usize;
        let link = (i & 0b0001_0000_0000_0000_0000_0000_0000_0000) > 0;
        Ok(Instruction::Branch { cond, c, link })
    }

    fn parse_branch_offset(i: u32) -> Result<Instruction, InstructionParseError> {
        let a = ((i / 0x1000000) % 0x10) as usize;
        let cond = Instruction::parse_cond(a)?;

        let mut offset = (i as u32) & 0b0000_0000_1111_1111_1111_1111_1111_1111;
        if offset & 0b0000_0000_1000_0000_0000_0000_0000_0000 > 0 {
            offset |= 0b1111_1111_0000_0000_0000_0000_0000_0000;
        }

        let link = (i & 0b0001_0000_0000_0000_0000_0000_0000_0000) > 0;
        Ok(Instruction::BranchOff {
            cond,
            offset: offset as i32,
            link,
        })
    }

    fn parse_cond(a: usize) -> Result<BranchCondition, InstructionParseError> {
        match a {
            0 => Ok(BranchCondition::MI),
            1 => Ok(BranchCondition::EQ),
            // Ignored 2 => Ok(BranchCondition::CS),
            // Ignored 3 => Ok(BranchCondition::VS),
            // Ignored 4 => Ok(BranchCondition::LS),
            5 => Ok(BranchCondition::LT),
            6 => Ok(BranchCondition::LE),
            7 => Ok(BranchCondition::AW),
            8 => Ok(BranchCondition::PL),
            9 => Ok(BranchCondition::NE),
            // Ignored 10 => Ok(BranchCondition::CC),
            // Ignored 11 => Ok(BranchCondition::VC),
            // Ignored 12 => Ok(BranchCondition::HI),
            13 => Ok(BranchCondition::GE),
            14 => Ok(BranchCondition::GT),
            15 => Ok(BranchCondition::NV),
            _ => Err(InstructionParseError::InvalidBranchCondition(a as u32)),
        }
    }

    pub fn serialize_all(instructions: Vec<Instruction>) -> Vec<u8> {
        let instruction_bits: Vec<u32> = instructions.iter().map(|instruction| Instruction::encode(instruction)).collect();
        // FIXME(pht) the type of the serialize function can not be infered, it seems
        bincode::serialize(&instruction_bits).unwrap()
    }

    pub fn deserialize_all(bytes: &[u8]) -> Vec<Instruction> {
        let instructions_bits: Vec<u32> = bincode::deserialize_from(bytes).unwrap();
        return instructions_bits.iter().map(|i: &u32| Instruction::parse(*i).unwrap()).collect();
    }
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_register() {
        assert_both(Instruction::Register { o: OpCode::MOV, a: 2, b: 5, c: 1 }, 0b0000_0010_0101_0000_00_00_00_00_00_00_0001);
        assert_both(Instruction::Register { o: OpCode::AND, a: 3, b: 2, c: 4 }, 0b0000_0011_0010_0100_00_00_00_00_00_00_0100);
    }

    #[test]
    fn test_register_im() {
        assert_both(
            Instruction::RegisterIm {
                o: OpCode::LSL,
                a: 2,
                b: 5,
                im: 4,
            },
            0b0100_0010_0101_0001_00_00_00_00_00_00_01_00,
        );
        assert_both(
            Instruction::RegisterIm {
                o: OpCode::LSL,
                a: 2,
                b: 5,
                im: -4,
            },
            0b0101_0010_0101_0001_11_11_11_11_11_11_11_00,
        );
    }

    #[test]
    fn test_memory() {
        assert_both(
            Instruction::Memory {
                u: MemoryMode::Load,
                a: 1,
                b: 3,
                offset: 2,
            },
            0b1000_0001_0011_0000_0000_0000_0000_0010,
        );
    }

    #[test]
    fn test_branch() {
        assert_both(
            Instruction::Branch {
                cond: BranchCondition::EQ,
                c: 3,
                link: true,
            },
            0b1101_0001_0000_0000_0000_0000_0000_0011,
        );
    }

    #[test]
    fn test_branch_off_positive() {
        assert_both(
            Instruction::BranchOff {
                cond: BranchCondition::PL,
                offset: 3,
                link: false,
            },
            0b1110_1000_0000_0000_0000_0000_0000_0011,
        );
    }

    #[test]
    fn test_branch_off_negative() {
        assert_both(
            Instruction::BranchOff {
                cond: BranchCondition::PL,
                offset: -5,
                link: false,
            },
            0b1110_1000_1111_1111_1111_1111_1111_1011,
        );

        // 0b1110_1000_11111_11111_11111_11111_1011 ???
        // 0b1110_1000_11111_11111_11111_11111_1011

        let i = Instruction::encode(&Instruction::BranchOff {
            cond: BranchCondition::PL,
            offset: -5,
            link: false,
        });
        assert_eq!(0b1110_1000_1111_1111_1111_1111_1111_1011, i);
        println!("{}", i);
        println!("{:032b}", i);

        let inst = Instruction::parse(3909091323).unwrap();
        assert_matches!(inst, Instruction::BranchOff{cond: _cond, offset, link: _link} if offset == -5);
        println!("{:?}", inst);

        let i2 = Instruction::encode(&inst);
        assert_eq!(0b1110_1000_1111_1111_1111_1111_1111_1011, i2);
        println!("{:032b}", i2);

        assert_eq!(i, i2);
    }

    fn assert_both(inst: Instruction, i: u32) {
        assert_encoded(&inst, i);
        assert_parsed(&inst, i);
    }

    fn assert_encoded(inst: &Instruction, expected: u32) {
        let actual = Instruction::encode(inst);
        assert_eq!(format!("{:032b}", expected), format!("{:032b}", actual))
    }

    fn assert_parsed(expected: &Instruction, i: u32) {
        let parsed = Instruction::parse(i).unwrap();
        assert_eq!(*expected, parsed)
    }

    #[test]
    fn test_serde() {
        let instruction = Instruction::Register { o: OpCode::MOV, a: 2, b: 5, c: 1 };
        let mut instructions = vec![];
        instructions.push(instruction);
        let serialized = Instruction::serialize_all(instructions);
        let deserialized = Instruction::deserialize_all(&serialized[..]);
        assert_eq!(Instruction::encode(&instruction), Instruction::encode(&(deserialized[0])));
    }
}
