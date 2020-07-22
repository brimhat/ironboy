use crate::cartridge::Cartridge;

const MEM_SIZE: usize = 0xFFFF + 1;

pub struct MMU<'a> {
    pub boot: [u8; 0x100],
    cartridge: &'a mut Cartridge,
    mem: [u8; MEM_SIZE]
}

impl<'a> MMU<'a> {
    pub fn new(cartridge: &'a mut Cartridge) -> MMU {
        MMU {
            boot: [0; 0x100],
            cartridge,
            mem: [0; MEM_SIZE]
        }
    }

    pub fn read_boot(&mut self, boot: &Vec<u8>) {
        let mut i: usize = 0x0000;
        for &byte in boot.iter() {
            self.boot[i] = byte;
            i += 1;
        }
    }

    pub fn rb(&self, address: u16) -> u8 {
        if address < 0x100 {
            return if self.rb(0xFF50) == 0 {
                self.boot[address as usize]
            } else {
                self.cartridge.read_lower_rom(address)
            }
        }

        match address {
            0x0000..=0x3FFF => self.cartridge.read_lower_rom(address),
            0x4000..=0x7FFF => self.cartridge.read_upper_rom(address),
            0xA000..=0xBFFF => self.cartridge.read_ram(address),
            _ => self.mem[address as usize],
        }
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        if address == 0xFF04 {
            // if divider is written to, it is reset to 0
            self.mem[address as usize] = 0;
            return;
        }

        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0xA000..=0xBFFF => self.cartridge.write_ram(address, value),
            _ => self.mem[address as usize] = value,
        }
    }
}