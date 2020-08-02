mod mmu;
mod registers;
mod cpu;
mod ppu;
mod cartridge;
mod timer;
mod interrupts;
mod test {
    mod cpu;
    mod ppu;
    mod timer;
    mod cartridge;
}

use std::ffi::CStr;
use std::io::prelude::*;
use std::fs::File;
use std::result::Result;
use std::rc::Rc;
use std::cell::RefCell;
use minifb::{Key, Window, WindowOptions};
use crate::cpu::CPU;
use crate::mmu::MMU;
use crate::ppu::{PPU, SCREEN_W, SCREEN_H};
use crate::cartridge::Cartridge;
use crate::timer::Timer;
use crate::interrupts::IntReq;

fn main() {
    let mut test = Vec::<u8>::new();
    let path = "ROMS/tetris_jue1.1.gb";
    let mut file = match File::open(path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f,
    };
    match file.read_to_end(&mut test) {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    }
    let mut cartridge = match Cartridge::new(test) {
        Err(e) => panic!("Error loading cartridge: {:?}", e),
        Ok(c) => c,
    };

    let mut boot = Vec::<u8>::new();
    let boot_path = "ROMS/DMG_ROM.bin";
    file = match File::open(boot_path) {
        Err(e) => panic!("{}", e),
        Ok(f) => f,
    };
    match file.read_to_end(&mut boot) {
        Err(e) => panic!("{}", e),
        Ok(_) => (),
    }

    let title = {
        let mut end_of_title: usize = 1;
        let mut cart_title = cartridge.title.as_bytes();
        while cart_title[end_of_title - 1] != 0 {
            end_of_title += 1;
            if end_of_title == cartridge.title.len() {
                cartridge.title.push_str("\0");
                end_of_title += 1;
                break;
            }
        }
        let title_bytes = &cartridge.title[0..end_of_title];
        match CStr::from_bytes_with_nul(title_bytes.as_ref()) {
            Err(e) => panic!("{}", e),
            Ok(cstr) => match cstr.to_str() {
                Err(e) => panic!("{}", e),
                Ok(s) => s,
            },
        }
    };

    let mut buffer: [u32; SCREEN_W * SCREEN_H] = [0; SCREEN_H * SCREEN_W];
    let mut window = Window::new(
        title, //"Press ESC to exit",
        SCREEN_W,
        SCREEN_H,
        WindowOptions::default()
    ).unwrap_or_else(|e| { panic!("{}", e) });

    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    mmu.read_boot(&boot);
    let mut cpu = CPU::new(timer.clone());
    let mut ppu = PPU::new(intr.clone());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.step(&mut mmu);
        let m_clocks = cpu.clocks_elapsed;
        ppu.tick_n(&mut mmu, m_clocks);

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
