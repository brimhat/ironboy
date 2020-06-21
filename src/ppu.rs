use crate::mmu::MMU;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

const DARKEST: u32  = 0xFF0F380F;
const DARK: u32     = 0xFF306230;
const LIGHT: u32    = 0xFF8BAC0F;
const LIGHTEST: u32 = 0xFF9BBC0F;

const TILE_MAP0: usize = 0x9800;
const TILE_MAP1: usize = 0x9C00;
const VRAM_START: u16 = 0x8000;
const OAM_SEARCH_END: u16 = 80;
const PIXEL_TRANSFER_END: u16 = (80 + 172);
const HBLANK_END: u16 = 456;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    OAMSearch       = 0b0010_0000,
    PixelTransfer,
    HBlank          = 0b0000_1000,
    VBlank          = 0b0001_0000,
}

pub struct PPU {
    mode: Mode,
    mode_clock: u16,
    pub data: [[u32; SCREEN_W]; SCREEN_H],
    pub update_screen: bool,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            mode: Mode::OAMSearch,
            mode_clock: 0,
            data: [[0; SCREEN_W]; SCREEN_H],
            update_screen: false,
        }
    }

    pub fn get_ly(&self, mmu: &MMU) -> u8 {
        mmu.rb(0xFF44)
    }

    pub fn inc_ly(&self, mmu: &mut MMU) {
        mmu.wb(0xFF44, (self.get_ly(mmu) + 1) % 154);
    }

    pub fn set_stat(&self, mmu: &mut MMU, mode: Mode) {
        let lyc = mmu.rb(0xFF45);
        let stat = mmu.rb(0xFF41);
        let clean_stat = stat & 0b1000_0111;
        let coincidence: u8 = if self.get_ly(mmu) == lyc { 0b0100_0000 } else { 0 };
        mmu.wb(0xFF41, clean_stat | coincidence | mode as u8);
    }

    pub fn get_color_from(pair: u8) -> u32 {
        match pair {
            0b00 => LIGHTEST,
            0b01 => LIGHT,
            0b10 => DARK,
            0b11 => DARKEST,
            _ => panic!("Error getting color from pair: {:#X}", pair)
        }
    }

    pub fn step(&mut self, mmu: &mut MMU, clocks: u8) {
        let lcdc = mmu.rb(0xFF40);
        if (lcdc >> 7) == 0 || clocks == 0 {
            return;
        }

        self.update_screen = false;
        let mut stat = mmu.rb(0xFF41);
        let lyc = mmu.rb(0xFF45);

        self.mode_clock += clocks as u16;
        if self.mode_clock >= 456 {
            self.inc_ly(mmu);
            self.mode_clock %= 456;
        }

        if self.get_ly(mmu) >= 144 {
            if self.mode != Mode::VBlank {
                self.update_screen = true;
                self.mode = Mode::VBlank;
                if (stat & Mode::VBlank as u8) == 0 {
                    self.set_stat(mmu, Mode::VBlank);
                }
            }
        } else if self.mode_clock <= OAM_SEARCH_END {
            self.mode = Mode::OAMSearch;
            if (stat & Mode::OAMSearch as u8) == 0 {
                self.set_stat(mmu, Mode::OAMSearch);
            }
        } else if self.mode_clock <= PIXEL_TRANSFER_END {
            // only write line once
            if self.mode != Mode::PixelTransfer {
                self.mode = Mode::PixelTransfer;
                // draw background line at LY
                self.draw_bg(mmu);
                // draw sprites line at LY
            }
        } else {
            self.mode = Mode::HBlank;
            if (stat & Mode::HBlank as u8) == 0 {
                self.set_stat(mmu, Mode::HBlank);
            }
        }
    }

    pub fn draw_bg(&mut self, mmu: &mut MMU) {
        let scx = mmu.rb(0xFF43);
        let scy = mmu.rb(0xFF42);
        let y = scy.wrapping_add(self.get_ly(mmu));
        let tile_map_start = TILE_MAP0 as u16;

        for i in 0..SCREEN_W {
            let x = scx.wrapping_add(i as u8);

            // grab tile num from tile map
            // tile map is 32x32 tiles in length
            let map_y = (y / 8) as u16;
            let map_x = (x / 8) as u16;
            let tile_map_address = tile_map_start + (map_y * 32 + map_x);
            let tile_index = mmu.rb(tile_map_address) as u16;
            if tile_map_address > 0x9BFF {
                panic!("index overflow: {:#X}", tile_map_address);
            }
            // grab 16 bits from character ram
            let normalized_index: u16 = (VRAM_START + tile_index * 16);
            let byte1 = mmu.rb(normalized_index);
            let byte2 = mmu.rb(normalized_index + 1);
            if byte1 != 0 || byte2 != 0 {
                println!("BYTE1: {:#b}, \nBYTE2: {:#b}\n", byte1, byte2);
            }
            // convert bits to color
            let mask = 1 << (7 - (x % 8));
            let bit1 = byte1 & mask != 0;
            let bit2 = byte2 & mask != 0;
            let pair = ((bit1 as u8) << 1) | bit2 as u8;

            let color = PPU::get_color_from(pair);
//            if color != LIGHTEST {
//                println!("Writing {:#X} at ROW: {} COL: {}", color, self.get_ly(mmu), i);
//            }
            self.data[self.get_ly(mmu) as usize][i] = color;
        }
    }
}