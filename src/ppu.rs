use crate::mmu::MMU;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

const DARKEST: u32  = 0xFF0F380F;
const DARK: u32     = 0xFF306230;
const LIGHT: u32    = 0xFF8BAC0F;
const LIGHTEST: u32 = 0xFF9BBC0F;

const TILE_MAP0: u16 = 0x9800;
const TILE_MAP1: u16 = 0x9C00;
const VRAM0_START: u16 = 0x8800;
const VRAM1_START: u16 = 0x8000;
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

        let i_f = mmu.rb(0xFF0F);
        mmu.wb(0xFF0F, i_f | 0x02);
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
                let i_f = mmu.rb(0xFF0F);
                mmu.wb(0xFF0F, i_f | 0x01);
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
        let lcdc = mmu.rb(0xFF40);
        let y = scy.wrapping_add(self.get_ly(mmu));

        let tile_start = if lcdc & (1 << 4) == 0 {
            VRAM0_START
        } else {
            VRAM1_START
        };

        let map_start = if lcdc & (1 << 3) == 0 {
            TILE_MAP0
        } else {
            TILE_MAP1
        };

        for i in 0..SCREEN_W {
            let x = scx.wrapping_add(i as u8);

            // grab tile num from tile map
            // tile map is 32x32 tiles in length
            let map_y = (y / 8) as u16;
            let map_x = (x / 8) as u16;
            let tile_map_address = map_start + (map_y * 32 + map_x);
            let tile_map_index = mmu.rb(tile_map_address) as u16;
            // grab two bytes
            // each tile is 16 bytes long (8x8 pixels of 2-bit color)
            let tile_idx = tile_map_index * 16;
            let tile_row = (y as u16 % 8) * 2;
            let index = tile_start + tile_idx + tile_row;
            let byte1 = mmu.rb(index);
            let byte2 = mmu.rb(index + 1);
            // convert bits to color
            let mask = 1 << (7 - (x % 8));
            let bit1 = byte1 & mask != 0;
            let bit2 = byte2 & mask != 0;
            let pair = ((bit1 as u8) << 1) | bit2 as u8;

            let color = PPU::get_color_from(pair);
            self.data[self.get_ly(mmu) as usize][i] = color;
        }
    }
}