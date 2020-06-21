
pub struct MMU {
    pub boot: [u8; 0x100],
    mem: [u8; 0xFFFF]
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            boot: [0; 0x100],
            mem: [0; 0xFFFF]
        }
    }

    pub fn read_boot(&mut self, boot: Vec<u8>) {
        let mut i: usize = 0x0000;
        for &byte in boot.iter() {
            self.boot[i] = byte;
            i += 1;
        }
    }

    pub fn read_rom(&mut self, rom: Vec<u8>) {
        let mut i: u16 = 0x0000;
        for &byte in rom.iter() {
            self.wb(i, byte);
            i += 1;
        }
    }

    pub fn rb(&self, address: u16) -> u8 {
        if address < 0x100 && self.rb(0xFF50) == 0 {
            self.boot[address as usize]
        } else {
            self.mem[address as usize]
        }
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
}