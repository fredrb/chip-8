
type Word = u16;

pub struct Chip8Instruction {
    pub raw: Word,
    pub parts: [u8;4],
}

fn join_2_bytes (left: u8, right: u8) -> u16 {
    return ((left as u16) << 8) + right as u16;
}

pub trait DecodableInstruction {
    fn new(left: u8, right: u8) -> Self;

    fn match_masked(&self, mask: u16, matcher: u16) -> bool;
}

impl DecodableInstruction for Chip8Instruction {
    fn new(left: u8, right: u8) -> Self {
        let raw = join_2_bytes(left, right);
        Chip8Instruction {
            raw,
            parts: [
                ((raw & 0xF000) >> 12)  as u8,
                ((raw & 0x0F00) >> 8)   as u8,
                ((raw & 0x00F0) >> 4)   as u8,
                (raw & 0x000F)          as u8
            ]
        }
    }

    fn match_masked(&self, mask: u16, matcher: u16) -> bool {
        (self.raw & mask) == matcher
    }
}