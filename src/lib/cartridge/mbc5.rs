use crate::lib::cartridge::cartridge_base::CartridgeBase;
use crate::lib::cartridge::{get_ram_size, Cartridge, RamDumper, CARTRIDGE_TYPE_ADDRESS};

use super::regions::{
    RAM_BANK_SEL_END, RAM_BANK_SEL_START, RAM_ENABLE_END, ROM_BANK_SEL_END, ROM_BANK_SEL_START,
};

pub struct Mbc5 {
    cartridge_base: CartridgeBase,
}

impl Mbc5 {
    pub fn new(rom: Vec<u8>, ram_dumper: Option<Box<dyn RamDumper + Send>>) -> Self {
        let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
        let has_ram = cartridge_type == 0x02 || cartridge_type == 0x03;
        let has_battery = cartridge_type == 0x1B;
        let ram_size = get_ram_size(&rom);

        let cartridge_base = CartridgeBase::new(rom, has_ram, ram_size, has_battery, ram_dumper);

        Mbc5 { cartridge_base }
    }
}

impl Cartridge for Mbc5 {
    fn read(&self, address: u16) -> u8 {
        self.cartridge_base.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..=RAM_ENABLE_END => {
                self.cartridge_base.ram_enabled = value == 0x0A;
            }
            ROM_BANK_SEL_START..=0x2FFF => {
                // 8 least significant bits of ROM bank number
                //0 is also 1
                let bank_number = if value == 0 { 1 } else { value };
                //Only set lower 8 bits
                self.cartridge_base.rom_bank = bank_number & 0xFF;
            }
            0x3000..=ROM_BANK_SEL_END => {
                // 9th bit of ROM bank number
                self.cartridge_base.rom_bank = self.cartridge_base.rom_bank & 0xFF | (value & 0x01);
            }
            RAM_BANK_SEL_START..=RAM_BANK_SEL_END => {
                // RAM bank number
                /*
                As for the MBC1s RAM Banking Mode, writing a value in the range $00-$0F maps
                the corresponding external RAM bank (if any) into the memory area at A000-BFFF.
                */
                self.cartridge_base.ram_bank = value & 0x0F;
            }
            _ => {}
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.cartridge_base.write_ram(address, value);
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.cartridge_base.read_ram(address)
    }

    fn dump_savegame(&self) {
        self.cartridge_base.dump_savegame();
    }

    fn load_savegame(&mut self) {
        self.cartridge_base.load_savegame();
    }
}
