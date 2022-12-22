//TODO: add more instructions
enum Instruction {
    ADD(ArithmeticTarget),
}

// TODO: reference the instruction guide
impl Instruction {
    fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),
            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            _ => None,
        }
    }
}

enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}
