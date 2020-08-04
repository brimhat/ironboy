use crate::cartridge::{Cartridge, KILOBYTE, MEGABYTE, ROM_BANK_SIZE, RAM_BANK_SIZE};
use crate::mmu::MMU;
use crate::interrupts::IntReq;
use crate::timer::Timer;
use std::cell::RefCell;
use std::rc::Rc;

pub fn cartridge_0b() -> Cartridge {
    let mut rom: Vec<u8> = vec![0; 32 * KILOBYTE as usize];
    rom[0x147] = 0;
    rom[0x148] = 0;
    rom[0x149] = 0;
    rom[0x27EB] = 0x20;
    rom[0x3FFF] = 0x3E;
    rom[0x5A74] = 0x53;
    rom[0x7FFF] = 0x80;

    match Cartridge::new(rom) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c,
    }
}

pub fn cartridge_4b() -> Cartridge {
    let mut rom: Vec<u8> = vec![0; 64 * KILOBYTE as usize];
    rom[0x147] = 1;
    rom[0x148] = 1;
    rom[0x149] = 0;

    match Cartridge::new(rom) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c
    }
}

pub fn cartridge_128b() -> Cartridge {
    let mut rom: Vec<u8> = vec![0; 2 * MEGABYTE as usize];
    rom[0x147] = 1;
    rom[0x148] = 6;
    rom[0x149] = 0;
    rom[0x1132A7] = 0x20;

    match Cartridge::new(rom) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c
    }
}

pub fn cartridge_mbc2() -> Cartridge {
    let mut rom: Vec<u8> = vec![0; 64 * KILOBYTE as usize];
    rom[0x147] = 5;
    rom[0x148] = 1;
    rom[0x149] = 0;

    match Cartridge::new(rom) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c
    }
}

#[test]
fn no_mbc_read() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge_0b();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    let lower_1 = mmu.rb(0x27EB);
    let expected_lower_1 = 0x20;
    assert_eq!(lower_1, expected_lower_1);
    let lower_2 = mmu.rb(0x3FFF);
    let expected_lower_2 = 0x3E;
    assert_eq!(lower_2, expected_lower_2);
    let upper_1 = mmu.rb(0x5A74);
    let expected_upper_1 = 0x53;
    assert_eq!(upper_1, expected_upper_1);
    let upper_2 = mmu.rb(0x7FFF);
    let expected_upper_2 = 0x80;
    assert_eq!(upper_2, expected_upper_2);
}

#[test]
fn mbc1_bank_mode_on() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge_4b();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    mmu.wb(0x3FFF, 0b0001_0010); // Bank 1 register
    mmu.wb(0x5FFF, 0b1111_0101); // Bank 2 register
    mmu.wb(0x7FFF, 0b1000_0001); // Mode register ON
    let (lower, upper) = cartridge.rom_offsets;
    assert_eq!(0b0010_0000, lower / ROM_BANK_SIZE);
    assert_eq!(0b0011_0010, upper / ROM_BANK_SIZE);
}

#[test]
fn mbc1_bank_mode_off() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge_4b();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    mmu.wb(0x3FFF, 0b0001_0010); // Bank 1 register
    mmu.wb(0x5FFF, 0b1111_0101); // Bank 2 register
    mmu.wb(0x7FFF, 0b1000_0000); // Mode register OFF
    let (lower, upper) = cartridge.rom_offsets;
    assert_eq!(0b0000_0000, lower / ROM_BANK_SIZE);
    assert_eq!(0b0011_0010, upper / ROM_BANK_SIZE);
}

#[test]
fn mbc1_read_bank() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge_128b();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    mmu.wb(0x3FFF, 0b0000_0100); // Bank 1 register
    mmu.wb(0x5FFF, 0b0000_0010); // Bank 2 register
    mmu.wb(0x7FFF, 0b1000_0000); // Mode register OFF
    let read_value = mmu.rb(0x72A7);
    let (_, bank_number) = cartridge.rom_offsets;

    let expected_read_value: u8 = 0x20;        // see 'cartridge_4b' function
    let expected_bank_number: u32 = 0b0100_0100;
    assert_eq!(expected_bank_number, bank_number / ROM_BANK_SIZE);
    assert_eq!(expected_read_value, read_value);
}

#[test]
fn mbc2_read_ram() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge_mbc2();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    mmu.wb(0x3EFF, 0b1010); // should enable ram
    mmu.wb(0xA000, 0x2F);

    let expected_read_value = 0x0F;
    let read_value = mmu.rb(0xA200);
    assert_eq!(expected_read_value, read_value);

    mmu.wb(0x3000, 0); // should disable ram
    let expected_undefined = 0xFD;
    let read_value2 = mmu.rb(0xA400);
    assert_eq!(expected_undefined, read_value2);
}
