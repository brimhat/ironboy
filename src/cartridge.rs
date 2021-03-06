use std::result::Result;

pub const KILOBYTE: u32 = 1024;
pub const MEGABYTE: u32 = 1024 * KILOBYTE;
pub const ROM_BANK_SIZE: u32 = 0x4000;
pub const RAM_BANK_SIZE: u32 = 0x2000;

#[derive(Debug, Copy, Clone)]
pub struct Mbc1 {
    pub ram_enabled: bool,
    pub bank1: u8,
    pub bank2: u8,
    pub mode: bool,
}

impl Mbc1 {
    pub fn setup() -> Mbc1 {
        Mbc1 {
            ram_enabled: false,
            bank1: 0b00001,
            bank2: 0b00,
            mode: false,
        }
    }

    pub fn get_rom_offsets(&self) -> (u32, u32) {
        let bits = self.bank2 << 5;
        let lower = if self.mode { bits } else { 0 } as u32;
        let upper = (bits | self.bank1) as u32;
        (ROM_BANK_SIZE * lower, ROM_BANK_SIZE * upper)
    }

    pub fn get_ram_offsets(&self) -> u32 {
        let bits = if self.mode { self.bank2 } else { 0 };
        RAM_BANK_SIZE * bits as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mbc2 {
    pub ram_enabled: bool,
    pub bank: u8,
}

impl Mbc2 {
    pub fn setup() -> Mbc2 {
        Mbc2 {
            ram_enabled: false,
            bank: 0b0001,
        }
    }

    pub fn get_rom_offsets(&self) -> (u32, u32) {
        (0, ROM_BANK_SIZE * self.bank as u32)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mbc3 {
    pub ram_enabled: bool,
    pub romb: u8,
    pub ramb: u8,
}

impl Mbc3 {
    pub fn setup() -> Mbc3 {
        Mbc3 {
            ram_enabled: false,
            romb: 0b0000_0001,
            ramb: 0b0000_0000,
        }
    }

    pub fn get_rom_offsets(&self) -> (u32, u32) {
        (0, ROM_BANK_SIZE * self.romb as u32)
    }

    pub fn get_ram_offset(&self) -> u32 {
        RAM_BANK_SIZE * self.ramb as u32
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Mbc5 {
    pub ram_enabled: bool,
    pub bank0: u8,
    pub bank1: u8,
    pub ramb: u8,
}

impl Mbc5 {
    pub fn setup() -> Mbc5 {
        Mbc5 {
            ram_enabled: false,
            bank0: 0,
            bank1: 0,
            ramb: 0,
        }
    }

    pub fn get_rom_offsets(&self) -> (u32, u32) {
        let bank = ((self.bank1 as u32) << 8) | (self.bank0 as u32);
        (0, ROM_BANK_SIZE * bank)
    }

    pub fn get_ram_offset(&self) -> u32 {
        RAM_BANK_SIZE * self.ramb as u32
    }
}

#[derive(Debug)]
pub enum Mbc {
    NoMBC,
    MBC1 { mbc: Mbc1 },
    MBC2 { mbc: Mbc2 },
    MBC3 { mbc: Mbc3 },
    MBC5 { mbc: Mbc5 },
}

#[derive(Debug)]
pub enum CartridgeError {
    MissingHeaderInformation,
    UnsupportedROMType,
    UnsupportedROMSize,
    UnsupportedRAMSize,
    UnsupportedMBC,
}

#[derive(Debug)]
pub struct Cartridge {
    pub title: String,
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbc: Mbc,
    pub rom_offsets: (u32, u32),
    ram_offset: u32,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Result<Cartridge, CartridgeError> {
        if data.len() < 0x150 {
            return Err(CartridgeError::MissingHeaderInformation);
        }

        if data[0x143] == 0xC0 {
            return Err(CartridgeError::UnsupportedROMType)
        }

        let rom_size = match data[0x148] {
            0x00 => 32 * KILOBYTE,  // no ROM banking
            0x01 => 64 * KILOBYTE,  // 4 banks
            0x02 => 128 * KILOBYTE, // 8 banks
            0x03 => 256 * KILOBYTE, // 16 banks
            0x04 => 512 * KILOBYTE, // 32 banks
            0x05 => MEGABYTE,       // 64 banks (only 63 banks used by MBC1
            0x06 => 2 * MEGABYTE,   // 128 banks
            0x07 => 4 * MEGABYTE,   // 256 banks
            0x08 => 8 * MEGABYTE,   // 512 banks
            _ => return Err(CartridgeError::UnsupportedROMSize)
        } as usize;

        let mut ram_size = match data[0x149] {
            0x0 => 0,
            0x1 => 2 * KILOBYTE,
            0x2 => 8 * KILOBYTE,
            0x3 => 32 * KILOBYTE,  // 4 banks of 8kb each
            0x4 => 128 * KILOBYTE, // 16 banks of 8kb each
            0x5 => 64 * KILOBYTE,  // 8 banks of 8kb each
            _ => return Err(CartridgeError::UnsupportedRAMSize)
        } as usize;

        let mbc = match data[0x147] {
            0x00 => Mbc::NoMBC,
            0x01..=0x03 => Mbc::MBC1 { mbc: Mbc1::setup() },
            0x05..=0x06 => {
                ram_size = 512;
                Mbc::MBC2 { mbc: Mbc2::setup() }
            },
            0x11..=0x13 => Mbc::MBC3 { mbc: Mbc3::setup() },
            0x19..=0x1E => Mbc::MBC5 { mbc: Mbc5::setup() },
            _ => return Err(CartridgeError::UnsupportedMBC)
        };

        let title = match String::from_utf8(data[0x134..0x13F].to_owned()) {
            Err(e) => panic!("Error loading cartridge title: {}", e),
            Ok(s) => s,
        };

        let rom = Cartridge::load_rom(data, rom_size);

        Ok(Cartridge {
            title,
            mbc,
            rom,
            ram: vec![0; ram_size],
            rom_offsets: (0x0000, 0x4000),
            ram_offset: 0x0000,
        })
    }

    pub fn load_rom(data: Vec<u8>, rom_size: usize) -> Vec<u8> {
        let mut i: usize = 0;
        let mut rom = vec![0; rom_size];
        for &byte in data.iter() {
            if i == rom_size {
                panic!("Actual ROM size not matching header information");
            }
            rom[i] = byte;
            i += 1;
        }
        rom
    }

    pub fn read_lower_rom(&self, address: u16) -> u8 {
        let (lower, _) = self.rom_offsets;
        let address_in_bank = (address & 0x3FFF) as usize;
        self.rom[(lower as usize | address_in_bank) & (self.rom.len() - 1)]
    }

    pub fn read_upper_rom(&self, address: u16) -> u8 {
        let (_, upper) = self.rom_offsets;
        let address_in_bank = (address & 0x3FFF) as usize;
        self.rom[(upper as usize | address_in_bank) & (self.rom.len() - 1)]
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        match self.mbc {
            Mbc::NoMBC => {},
            Mbc::MBC1 { ref mut mbc } => {
                match address {
                    0x0000..=0x1FFF => mbc.ram_enabled = (value & 0b1111) == 0b1010,
                    0x2000..=0x3FFF => {
                        mbc.bank1 = if value & 0b1_1111 == 0b0 {
                            0b1
                        } else {
                            value & 0b1_1111
                        };
                        self.rom_offsets = mbc.get_rom_offsets();
                    },
                    0x4000..=0x5FFF => {
                        mbc.bank2 = value & 0b11;
                        self.rom_offsets = mbc.get_rom_offsets();
                        self.ram_offset = mbc.get_ram_offsets();
                    },
                    0x6000..=0x7FFF => {
                        mbc.mode = (value & 0b1) == 0b1;
                        self.rom_offsets = mbc.get_rom_offsets();
                        self.ram_offset = mbc.get_ram_offsets();
                    },
                    _ => panic!("Virtual address overflow: {:#X}", address)
                }
            },
            Mbc::MBC2 { ref mut mbc } => {
                match address {
                    0x0000..=0x3FFF => {
                        if (address & 0x100) == 0 {
                            mbc.ram_enabled = (value & 0b1111) == 0b1010
                        } else {
                            let v = value & 0b1111;
                            mbc.bank = if v == 0 { 1 } else { v };
                            self.rom_offsets = mbc.get_rom_offsets();
                        }
                    },
                    _ => ()
                }
            },
            Mbc::MBC3 { ref mut mbc } => {
                match address {
                    0x0000..=0x1FFF => mbc.ram_enabled = (value & 0b1111) == 0b1010,
                    0x2000..=0x3FFF => {
                        let v = value & 0x7F;
                        mbc.romb = if v == 0 { 1 } else { v };
                        self.rom_offsets = mbc.get_rom_offsets();
                    },
                    0x4000..=0x5FFF => {
                        mbc.ramb = value & 0b11;
                        self.ram_offset = mbc.get_ram_offset();
                    },
                    _ => ()
                }
            },
            Mbc::MBC5 { ref mut mbc } => {
                match address {
                    0x0000..=0x1FFF => mbc.ram_enabled = value == 0b0000_1010,
                    0x2000..=0x2FFF => {
                        mbc.bank0 = value;
                        self.rom_offsets = mbc.get_rom_offsets();
                    },
                    0x3000..=0x3FFF => {
                        mbc.bank1 = value & 0b1;
                        self.rom_offsets = mbc.get_rom_offsets();
                    },
                    0x4000..=0x5FFF => {
                        mbc.ramb = value & 0x0F;
                        self.ram_offset = mbc.get_ram_offset();
                    },
                    _ => ()
                }
            }
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        let undefined = 0xFF;
        let physical_address = (self.ram_offset | (address as u32 & 0x1FFF)) as usize;
        match self.mbc {
            Mbc::NoMBC => 0xFD, // NULL opcode
            Mbc::MBC1 { ref mbc } => {
                if mbc.ram_enabled {
                    self.ram[physical_address & (self.ram.len() - 1)]
                } else {
                    undefined
                }
            },
            Mbc::MBC2 { ref mbc } => {
                if mbc.ram_enabled {
                    self.ram[physical_address & 0x1FF] & 0x0F
                } else {
                    undefined
                }
            },
            Mbc::MBC3 { ref mbc } => {
                if mbc.ram_enabled {
                    match mbc.ramb {
                        0x00..=0x03 => self.ram[physical_address & (self.ram.len() - 1)],
                        _ => undefined
                    }
                } else {
                    undefined
                }
            },
            Mbc::MBC5 { ref mbc } => {
                if mbc.ram_enabled {
                    self.ram[physical_address & (self.ram.len() - 1)]
                } else {
                    undefined
                }
            }
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        let physical_address = (self.ram_offset | (address as u32 & 0x1FFF)) as usize;
        match self.mbc {
            Mbc::NoMBC => {},
            Mbc::MBC1 { ref mbc } => {
                if mbc.ram_enabled {
                    let len = self.ram.len() - 1;
                    self.ram[physical_address & len] = value;
                }
            },
            Mbc::MBC2 { ref mbc } => {
                if mbc.ram_enabled {
                    self.ram[physical_address & 0x1FF] = value & 0x0F;
                }
            },
            Mbc::MBC3 { ref mbc } => {
                if mbc.ram_enabled {
                    let len = self.ram.len() - 1;
                    match mbc.ramb {
                        0x00..=0x03 => self.ram[physical_address & len] = value,
                        _ => (),
                    }
                }
            }
            Mbc::MBC5 { ref mbc } => {
                if mbc.ram_enabled {
                    let len = self.ram.len() - 1;
                    self.ram[physical_address & len] = value;
                }
            }
        }
    }
}
