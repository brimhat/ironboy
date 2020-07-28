use crate::cartridge::Cartridge;

const MEM_SIZE: usize = 0xFFFF + 1;

pub struct MMU<'a> {
    pub boot: [u8; 0x100],
    cartridge: &'a mut Cartridge,
    mem: [u8; MEM_SIZE],
    pub reset_div: bool,
    pub update_screen: bool,
}

impl<'a> MMU<'a> {
    pub fn new(cartridge: &'a mut Cartridge) -> MMU {
        MMU {
            boot: [0; 0x100],
            cartridge,
            mem: [0; MEM_SIZE],
            reset_div: false,
            update_screen: false,
        }
    }

    pub fn read_boot(&mut self, boot: &Vec<u8>) {
        let mut i: usize = 0x0000;
        for &byte in boot.iter() {
            self.boot[i] = byte;
            i += 1;
        }
    }

    pub fn write_div(&mut self, value: u8) {
        self.mem[0xFF04] = value;
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
        if address == 0xFF0F {
            self.mem[0xFF0F] = 0b1110_0000 | value;
            return;
        }
//        if self.mem[0xFF50] != 0 {
//            println!("MEM[{:#X}] = {:#X}", address, value);
//        }
//        if address >= 0x9800 && address <= 0x9BFF && self.mem[0xFF50] != 0 {
////            if value != 0 && value != 0x2F {
////                println!("MAP0[{:#X}] = {:#X}", address, value);
////            }
//            if value == 0x2F {
//                print!(".");
//            } else if value == 0 {
//                print!("_");
//            } else {
//                println!("MAP0[{:#X}] = {:#X}", address, value);
//            }
//        }

        if address == 0xFF40 {
            if (value & 0x80) != 0 && (self.mem[0xFF40] & 0x80) == 0 {
                println!("LCD ON");
                self.update_screen = true;
            } else if (value & 0x80) == 0 && (self.mem[0xFF40] & 0x80) != 0 {
                println!("LCD OFF");
            }
        }

        if address == 0xFF04 {
            // if divider is written to, div and system internal counter set to 0
            self.reset_div = true;
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