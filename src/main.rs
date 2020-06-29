mod mmu;
mod registers;
mod cpu;
mod ppu;
mod instructions;
mod test { mod cpu; }

use std::io;
use std::io::prelude::*;
use std::fs::File;
use minifb::{Key, Window, WindowOptions};
use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::ppu::{PPU, SCREEN_W, SCREEN_H};

fn main() {
    let path = "ROMS/blargg-test-roms/cpu_instrs/individual/01-special.gb";
    let mut file = File::open(path).unwrap();
    let mut test = Vec::<u8>::new();
    file.read_to_end(&mut test);
    let mut mmu = MMU::new();
    mmu.read_blargg(&test);

//    let mut boot = Vec::<u8>::new();
//    let mut rom = Vec::<u8>::new();
//    let mut file = File::open("ROMS/DMG_ROM.bin").unwrap();
//    file.read_to_end(&mut boot);
//    file = File::open("ROMS/tetris_jue1.1.gb").unwrap();
//    file.read_to_end(&mut rom);
//
//    let mut mmu = MMU::new();
//    mmu.read_boot(&boot);
//    mmu.read_rom(&rom);

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
//    loop {
//        let clocks = cpu.step(&mut mmu);
//        ppu.step(&mut mmu, clocks);
//    }
}
