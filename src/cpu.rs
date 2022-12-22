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
