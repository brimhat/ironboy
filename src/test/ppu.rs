#![allow(dead_code, unused_imports)]

use crate::timer::Timer;
use crate::mmu::MMU;
use crate::ppu::{PPU, Mode};
use crate::cartridge::Cartridge;
use crate::interrupts::IntReq;
use std::cell::RefCell;
use std::rc::Rc;

const ROM: [u8; 32768] = [0; 32768];

fn cartridge() -> Cartridge {
    let cartridge = match Cartridge::new(ROM.to_vec()) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c,
    };
    return cartridge;
}

#[test]
fn tick_clocks() {
    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    let mut ppu = PPU::new(intr.clone());

    mmu.wb(0xFF40, 0x80);

    for _ in 0..20 {
        ppu.tick(&mut mmu);
        assert_eq!(ppu.stat.mode, Mode::OAMSearch);
    }

    ppu.tick(&mut mmu);
    assert_eq!(ppu.stat.mode, Mode::PixelTransfer);

    for _ in 0..42 {
        ppu.tick(&mut mmu);
        assert_eq!(ppu.stat.mode, Mode::PixelTransfer);
    }

    ppu.tick(&mut mmu);
    assert_eq!(ppu.stat.mode, Mode::HBlank);

    for _ in 0..49 {
        ppu.tick(&mut mmu);
        assert_eq!(ppu.stat.mode, Mode::HBlank);
    }

    ppu.tick(&mut mmu);
    assert_eq!(ppu.stat.mode, Mode::OAMSearch);
    assert_eq!(mmu.rb(0xFF44), 1);
}

#[test]
fn vblank_clocks() {
    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    let mut ppu = PPU::new(intr.clone());

    mmu.wb(0xFF40, 0x80);

    assert_eq!(ppu.stat.mode, Mode::OAMSearch);
    for _ in 0..16415 {
        ppu.tick(&mut mmu);
        assert_ne!(ppu.stat.mode, Mode::VBlank);
    }

    ppu.tick(&mut mmu);
    assert_eq!(ppu.stat.mode, Mode::VBlank);
    assert_eq!(mmu.rb(0xFF44), 144);
    for _ in 0..1139 {
        ppu.tick(&mut mmu);
        assert_eq!(ppu.stat.mode, Mode::VBlank);
    }

    assert_eq!(mmu.rb(0xFF44), 153);
    ppu.tick(&mut mmu);
    assert_eq!(ppu.stat.mode, Mode::OAMSearch);
    assert_eq!(mmu.rb(0xFF44), 0);
}
