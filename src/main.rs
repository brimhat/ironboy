mod mmu;
mod registers;
mod cpu;
mod ppu;
mod cartridge;
mod timer;
mod test {
    mod cpu;
    mod timer;
    mod cartridge;
}

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::result::Result;
use minifb::{Key, Window, WindowOptions};
use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::ppu::{PPU, SCREEN_W, SCREEN_H};
use crate::cartridge::Cartridge;
use crate::timer::Timer;

fn main() {
    let mut test = Vec::<u8>::new();
    let path = "ROMS/blargg-test-roms/cpu_instrs/individual/02-interrupts.gb";
    let mut file = match File::open(path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f,
    };
    match file.read_to_end(&mut test) {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    };
    let mut cartridge = match Cartridge::new(test) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c,
    };

    let mut timer = Timer::new();
    let mut mmu = MMU::new(&mut cartridge);
    let mut cpu = CPU::new();
    let mut ppu = PPU::new();

    let mut buffer: [u32; SCREEN_W * SCREEN_H] = [0; SCREEN_H * SCREEN_W];
    let mut window = Window::new(
        "Press ESC to exit",
        SCREEN_W,
        SCREEN_H,
        WindowOptions::default()
    ).unwrap_or_else(|e| { panic!("{}", e) });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let clocks = cpu.step(&mut mmu);
        timer.step(&mut mmu, clocks);
        ppu.step(&mut mmu, clocks);

        if ppu.update_screen {
            let mut i = 0;
            for row in ppu.data.iter() {
                for pixel in row.iter() {
                    buffer[i as usize] = *pixel;
                    i += 1;
                }
            }
            window.update_with_buffer(&buffer, SCREEN_W, SCREEN_H).unwrap();
            ppu.update_screen = false;
        }
    }
}
