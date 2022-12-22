pub struct CPU {
    registers: Registers,
    program_counter: u16,
    stack_pointer: u16,
    pub bus: MemoryBus,
    is_halted: bool,
    interrupts_enabled: bool,
}

impl CPU {
    pub fn new(boot_rom: Option<Vec<u8>>, game_rom: Vec<u8>) -> CPU {
        CPU {
            registers: Registers::new(),
            program_counter: 0x0,
            stack_pointer: 0x00,
            bus: MemoryBus::new(boot_rom, game_rom),
            is_halted: false,
            interrupts_enabled: false,
        }
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.program_counter);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.read_next_byte();
        }
        let (next_pc, mut cycles) =
            if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
                self.execute(instruction)
            } else {
                let description = format!(
                    "0x{}{:x}",
                    if prefixed { "cb" } else { "" },
                    instruction_byte
                );
                panic!(
                    "0x{:x}: Unknown instruction found - {}",
                    self.program_counter, description
                )
            };

        self.bus.step(cycles);

        if self.bus.has_interrupt() {
            self.is_halted = false;
        }
        if !self.is_halted {
            self.pc = next_pc;
        }

        let mut interrupted = false;
        if self.interrupts_enabled {
            if self.bus.interrupt_enable.vblank && self.bus.interrupt_flag.vblank {
                interrupted = true;
                self.bus.interrupt_flag.vblank = false;
                self.interrupt(VBLANK_VECTOR)
            }
            if self.bus.interrupt_enable.lcdstat && self.bus.interrupt_flag.lcdstat {
                interrupted = true;
                self.bus.interrupt_flag.lcdstat = false;
                self.interrupt(LCDSTAT_VECTOR)
            }
            if self.bus.interrupt_enable.timer && self.bus.interrupt_flag.timer {
                interrupted = true;
                self.bus.interrupt_flag.timer = false;
                self.interrupt(TIMER_VECTOR)
            }
        }
        if interrupted {
            cycles += 12;
        }
        cycles
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
