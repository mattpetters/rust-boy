use crate::audio_output::CpalAudioOutput;

use crate::graphics::gameboy_screen::GameboyScreen;
use crate::savegame::filesystem_ram_dumper::FilesystemRamDumper;
use crate::EmulationSignal;
use rust_boy::lib::apu::apu::Apu;
use rust_boy::lib::cartridge;
use rust_boy::lib::cpu::cpu::Cpu;
use rust_boy::lib::gpu::gpu::Gpu;
use rust_boy::lib::io::joypad::Joypad;
use rust_boy::lib::memory::mmu::Mmu;

use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::{fs, thread};

pub struct Emulation {
    gameboy_screen: Arc<GameboyScreen>,
    joypad: Arc<Mutex<Joypad>>,
}

impl Emulation {
    pub fn new(gameboy_screen: Arc<GameboyScreen>, joypad: Arc<Mutex<Joypad>>) -> Self {
        Emulation {
            gameboy_screen,
            joypad,
        }
    }

    pub fn start(&self, rom_path: &String) -> Result<Sender<EmulationSignal>, String> {
        let rom = read_rom_from_file(rom_path)?;
        let ram_dumper = FilesystemRamDumper::new(&rom_path);
        let mut cartridge = cartridge::new_cartridge(rom, Some(Box::new(ram_dumper)))?;

        let (emulation_signal_sender, emulation_signal_receiver) = channel();
        let cloned_sender = emulation_signal_sender.clone();

        let screen = Arc::clone(&self.gameboy_screen);
        let joypad = Arc::clone(&self.joypad);

        thread::Builder::new()
            .name("emulation".to_string())
            .spawn(move || {
                //Cpal needs to be startet from a different thread because of a winit bug on windows
                let mut audio_output = CpalAudioOutput::new(2048, Some(emulation_signal_sender));
                let default_device = audio_output.get_default_device_name();

                audio_output.start(default_device);

                let mut apu = Apu::new(&mut audio_output);
                let mut gpu = Gpu::new(screen);
                let mut mmu = Mmu::new(&mut *cartridge, &mut gpu, &mut apu);
                let mut cpu = Cpu::new();
                let mut emulation = rust_boy::lib::emulation::Emulation::new();

                loop {
                    let signal = emulation_signal_receiver.recv().unwrap();

                    if let EmulationSignal::Quit = signal {
                        mmu.save();
                        audio_output.stop();
                        break;
                    }

                    let joypad = joypad.lock().unwrap();

                    emulation.cycle(&mut cpu, &mut mmu, &joypad);
                }
            })
            .unwrap();

        Ok(cloned_sender)
    }
}

fn read_rom_from_file(rom_path: &String) -> Result<Vec<u8>, String> {
    match fs::read(rom_path) {
        Ok(rom) => Ok(rom),
        Err(_) => Err(format!("Could not open file {}", rom_path)),
    }
}
