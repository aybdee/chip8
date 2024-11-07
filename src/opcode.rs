use crate::utils::{str_to_u16, str_to_u8};

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    SYS(u16),
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SEx(u8, u8),
    SNEx(u8, u8),
    SExy(u8, u8),
    LDx(u8, u8),
    ADDx(u8, u8),
    LDxy(u8, u8),
    ORxy(u8, u8),
    ANDxy(u8, u8),
    XORxy(u8, u8),
    ADDxy(u8, u8),
    SUBxy(u8, u8),
    SHRxy(u8, u8),
    SUBNxy(u8, u8),
    SHLxy(u8, u8),
    SNExy(u8, u8),
    LDI(u16),
    JPV0(u16),
    RNDx(u8, u8),
    DRWxy(u8, u8, u8),
    SKPx(u8),
    SKNPx(u8),
    LDxDT(u8),
    LDxK(u8),
    LDDTx(u8),
    LDSTx(u8),
    ADDIx(u8),
    LDFx(u8),
    LDBx(u8),
    LDIx(u8),
    LDxI(u8),
}

impl From<u16> for Opcode {
    fn from(instruction: u16) -> Self {
        // Match the first nibble (the first 4 bits)
        match (instruction >> 12) & 0xF {
            0 => match instruction {
                0x00E0 => Opcode::CLS,
                0x00EE => Opcode::RET,
                _ => Opcode::SYS(0),
            },
            1 => Opcode::JP(instruction & 0x0FFF),
            2 => Opcode::CALL(instruction & 0x0FFF),
            3 => Opcode::SEx(
                ((instruction >> 8) & 0x0F) as u8,
                (instruction & 0x00FF) as u8,
            ),
            4 => Opcode::SNEx(
                ((instruction >> 8) & 0x0F) as u8,
                (instruction & 0x00FF) as u8,
            ),
            5 => Opcode::SExy(
                ((instruction >> 8) & 0x0F) as u8,
                ((instruction >> 4) & 0x0F) as u8,
            ),
            6 => Opcode::LDx(
                ((instruction >> 8) & 0x0F) as u8,
                (instruction & 0x00FF) as u8,
            ),
            7 => Opcode::ADDx(
                ((instruction >> 8) & 0x0F) as u8,
                (instruction & 0x00FF) as u8,
            ),
            8 => match (instruction & 0x000F) {
                0 => Opcode::LDxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                1 => Opcode::ORxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                2 => Opcode::ANDxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                3 => Opcode::XORxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                4 => Opcode::ADDxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                5 => Opcode::SUBxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                6 => Opcode::SHRxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                7 => Opcode::SUBNxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                0xE => Opcode::SHLxy(
                    ((instruction >> 8) & 0x0F) as u8,
                    ((instruction >> 4) & 0x0F) as u8,
                ),
                _ => panic!("invalid opcode {}", instruction),
            },
            9 => Opcode::SNExy(
                ((instruction >> 8) & 0x0F) as u8,
                ((instruction >> 4) & 0x0F) as u8,
            ),
            0xA => Opcode::LDI(instruction & 0x0FFF),
            0xB => Opcode::JPV0(instruction & 0x0FFF),
            0xC => Opcode::RNDx(
                ((instruction >> 8) & 0x0F) as u8,
                (instruction & 0x00FF) as u8,
            ),
            0xD => Opcode::DRWxy(
                ((instruction >> 8) & 0x0F) as u8,
                ((instruction >> 4) & 0x0F) as u8,
                (instruction & 0x0F) as u8,
            ),
            0xE => match instruction & 0x00FF {
                0x9E => Opcode::SKPx(((instruction >> 8) & 0x0F) as u8),
                0xA1 => Opcode::SKNPx(((instruction >> 8) & 0x0F) as u8),
                _ => panic!("invalid opcode {}", instruction),
            },
            0xF => match instruction & 0x00FF {
                0x07 => Opcode::LDxDT(((instruction >> 8) & 0x0F) as u8),
                0x0A => Opcode::LDxK(((instruction >> 8) & 0x0F) as u8),
                0x15 => Opcode::LDDTx(((instruction >> 8) & 0x0F) as u8),
                0x18 => Opcode::LDSTx(((instruction >> 8) & 0x0F) as u8),
                0x1E => Opcode::ADDIx(((instruction >> 8) & 0x0F) as u8),
                0x29 => Opcode::LDFx(((instruction >> 8) & 0x0F) as u8),
                0x33 => Opcode::LDBx(((instruction >> 8) & 0x0F) as u8),
                0x55 => Opcode::LDIx(((instruction >> 8) & 0x0F) as u8),
                0x65 => Opcode::LDxI(((instruction >> 8) & 0x0F) as u8),
                _ => panic!("invalid opcode {}", instruction),
            },
            _ => {
                panic!("invalid opcode {}", instruction)
            }
        }
    }
}
