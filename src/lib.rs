const ZERO_FLAG_BYTE: u8 = 7;
const SUBTRACT_FLAG_BYTE: u8 = 6;
const HALF_CARRY_FLAG_BYTE: u8 = 5;
const CARRY_FLAG_BYTE: u8 = 4;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8,
}

impl Registers {
    // some instructions write 2 bytes at a time so we need to read and write from these "virtual" 16 bit registers
    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
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

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    // fn write_byte(&mut self, address: u16, value: u8) {
    //     self.memory[address as usize] = value;
    // }
}
struct CPU {
    registers: Registers,
    program_counter: u16,
    stack_pointer: u16,
    bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.program_counter);

        let next_program_counter =
            if let Some(instruction) = Instruction::from_byte(instruction_byte) {
                self.execute(instruction);
            } else {
                panic!("Unknown instruction for: 0x{:x}", instruction_byte);
            };

        self.program_counter = next_program_counter;
    }
    fn add(&mut self, value: u8) -> u8 {
        let (result, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = result == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;
        result
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::A => {
                        // do something
                    }
                    ArithmeticTarget::B => {
                        // do something
                    }
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.program_counter.wrapping_add(1);
                    }
                    ArithmeticTarget::D => {
                        // do something
                    }
                    ArithmeticTarget::E => {
                        // do something
                    }
                    ArithmeticTarget::H => {
                        // do something
                    }
                    ArithmeticTarget::L => {
                        // do something
                    }
                }
            }
        }
    }
}
