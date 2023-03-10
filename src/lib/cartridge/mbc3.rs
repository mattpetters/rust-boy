use crate::lib::cartridge::cartridge_base::CartridgeBase;
use crate::lib::cartridge::{get_ram_size, Cartridge, RamDumper, CARTRIDGE_TYPE_ADDRESS};

use super::regions::{ERAM_REGION_END, ERAM_REGION_START};

// enum Mode {
//     RomBankingMode,
//     RamBankingMode,
// }

pub struct Mbc3 {
    cartridge_base: CartridgeBase,
    // selected_mode: Mode,
    rtc_mode: bool,
    ram_timer_enabled: bool,
    reg_rtc: u8,
}

impl Mbc3 {
    pub fn new(rom: Vec<u8>, ram_dumper: Option<Box<dyn RamDumper + Send>>) -> Self {
        let cartridge_type = rom[CARTRIDGE_TYPE_ADDRESS];
        let has_ram = true;
        let has_battery = cartridge_type == 0x13;
        let ram_size = get_ram_size(&rom);

        let cartridge_base = CartridgeBase::new(rom, has_ram, ram_size, has_battery, ram_dumper);

        Mbc3 {
            cartridge_base,
            // selected_mode: Mode::RomBankingMode,
            rtc_mode: false,
            ram_timer_enabled: false,
            reg_rtc: 0,
        }
    }
}

impl Cartridge for Mbc3 {
    fn read(&self, address: u16) -> u8 {
        self.cartridge_base.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0..=0x1FFF => {
                self.cartridge_base.ram_enabled = value == 0x0A;
            }
            //Address range for rom bank number
            0x2000..=0x3FFF => {
                self.cartridge_base.rom_bank = value;
            }
            //Address range for RAM bank number
            // 0x4000..=0x5FFF => match self.selected_mode {
            //     Mode::RamBankingMode => {
            //         self.cartridge_base.ram_bank = value;
            //     }
            //     Mode::RomBankingMode => {
            //         //Only set upper 2 bits
            //         self.cartridge_base.rom_bank =
            //             self.cartridge_base.rom_bank | (value & 0x03) << 5;
            //     }
            // },
            0x4000..=0x5FFF => {
                if value <= 0x03 {
                    self.rtc_mode = false;
                    self.cartridge_base.ram_bank = value;
                } else if (0x08..=0x0C).contains(&value) {
                    self.rtc_mode = true;
                }
            }
            //Select Mode
            // 0x6000..=0x7FFF => match value {
            //     0 => self.selected_mode = Mode::RomBankingMode,
            //     1 => self.selected_mode = Mode::RamBankingMode,
            //     _ => {}
            // },
            ERAM_REGION_START..=ERAM_REGION_END => {
                if self.ram_timer_enabled {
                    if self.rtc_mode {
                        self.reg_rtc = value;
                    } else {
                        let offset = address - ERAM_REGION_START;
                        self.write_ram(offset, value);
                    }
                }
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
