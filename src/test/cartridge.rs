use crate::cartridge::{Cartridge, KILOBYTE, MEGABYTE, ROM_BANK_SIZE, RAM_BANK_SIZE};
use crate::mmu::MMU;

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

#[test]
fn mbc1_banking_mode_on() {
    let mut cartridge = cartridge_4b();
    let mut mmu = MMU::new(&mut cartridge);
    mmu.wb(0x3FFF, 0b0001_0010); // Bank 1 register
    mmu.wb(0x5FFF, 0b1111_0101); // Bank 2 register
    mmu.wb(0x7FFF, 0b1000_0001); // Mode register ON
    let (lower, upper) = cartridge.rom_offsets;
    assert_eq!(0b0010_0000, lower / ROM_BANK_SIZE);
    assert_eq!(0b0011_0010, upper / ROM_BANK_SIZE);
}

#[test]
fn mbc1_banking_mode_off() {
    let mut cartridge = cartridge_4b();
    let mut mmu = MMU::new(&mut cartridge);
    mmu.wb(0x3FFF, 0b0001_0010); // Bank 1 register
    mmu.wb(0x5FFF, 0b1111_0101); // Bank 2 register
    mmu.wb(0x7FFF, 0b1000_0000); // Mode register OFF
    let (lower, upper) = cartridge.rom_offsets;
    assert_eq!(0b0000_0000, lower / ROM_BANK_SIZE);
    assert_eq!(0b0011_0010, upper / ROM_BANK_SIZE);
}

#[test]
fn mbc1_read_bank() {
    let mut cartridge = cartridge_128b();
    let mut mmu = MMU::new(&mut cartridge);
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
