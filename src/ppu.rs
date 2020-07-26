use crate::mmu::MMU;

pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

const DARKEST: u32  = 0xFF0F380F;
const DARK: u32     = 0xFF306230;
const LIGHT: u32    = 0xFF8BAC0F;
const LIGHTEST: u32 = 0xFF9BBC0F;

const TILE_MAP0: u16 = 0x9800;
const TILE_MAP1: u16 = 0x9C00;
const TILE_SET0: u16 = 0x8800;
const TILE_SET1: u16 = 0x8000;
const OAM_START: u16 = 0xF300;
const OAM_SEARCH_END: u16 = 80;
const PIXEL_TRANSFER_END: u16 = (80 + 172);

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
            mode: Mode::VBlank,
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

        let stat = mmu.rb(0xFF41);

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
                if lcdc & 0b10 != 0 {
                    self.draw_obj(mmu);
                }
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
        let wx = mmu.rb(0xFF4A).wrapping_sub(7);
        let wy = mmu.rb(0xFF4B);
        let lcdc = mmu.rb(0xFF40);
        let ly = self.get_ly(mmu);

        let tile_start = if lcdc & (1 << 4) == 0 {
            TILE_SET0
        } else {
            TILE_SET1
        };

        let tile_set0 = lcdc & (1 << 4) == 0;
        let win_map0 = lcdc & (1 << 6) == 0;
        let bg_map0 = lcdc & (1 << 3) == 0;

        let y_in_win = ly >= wy && lcdc & (1 << 5) != 0;
        let y = if y_in_win {
            ly.wrapping_sub(wy)
        } else {
            ly.wrapping_add(scy)
        };
        for i in 0..SCREEN_W {
            let writing_win = (i as u8) >= wx && y_in_win;
            let x = if writing_win {
                (i as u8) - wx
            } else {
                (i as u8).wrapping_add(scx)
            };

            let map_start = if (writing_win && win_map0) || (!writing_win && bg_map0) {
                TILE_MAP0
            } else {
                TILE_MAP1
            };

            // grab tile num from tile map
            // tile map is 32x32 tiles in length
            let map_y = (y / 8) as u16;
            let map_x = (x / 8) as u16;
            let tile_map_address = map_start + (map_y * 32 + map_x);
            let tile_map_index = mmu.rb(tile_map_address);
            // grab two bytes
            // each tile is 16 bytes long (8x8 pixels of 2-bit color)
            // if the TILE_SET0 in use, the indicies are signed
            let tile_map_offset = if tile_set0 {
                128 + ((tile_map_index as i8 as i16) + 128) as u16
            } else {
                tile_map_index as u16
            };

            let tile_idx = tile_map_offset * 16;
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

    pub fn draw_obj(&mut self, mmu: &mut MMU) {
        let lcdc = mmu.rb(0xFF40);
        let ly = mmu.rb(0xFF44);

        let sprite_size = if lcdc & (1 << 2) == 0 { 8 } else { 16 };

        for i in 0..40 {
            // each sprite has 4 bytes associated with it in the OAM:
            // byte 0: Y coordinate (minus 16)
            // byte 1: X coordinate (minus 8)
            // byte 2: Tile number
            // byte 3: Priority, y-flip, x-flip, palette
            let sprite_data_address = OAM_START + (i as u16) * 4;
            let sprite_y = mmu.rb(sprite_data_address).wrapping_sub(16);
            let sprite_x = mmu.rb(sprite_data_address + 1).wrapping_sub(8);
            let sprite_n = mmu.rb(sprite_data_address + 2);
            let sprite_o = mmu.rb(sprite_data_address + 3);

            println!("Y: {:#X}, X: {:#X}, N: {:#X}\nO: {:#b}",
                     sprite_y, sprite_x, sprite_n, sprite_o
            );

            if sprite_y <= ly && (sprite_y + sprite_size) > ly {
                let y_flip = sprite_o & (1 << 6) != 0;
                let x_flip = sprite_o & (1 << 5) != 0;

                let tile_y = if y_flip {
                    sprite_size - ly.wrapping_sub(sprite_y) - 1
                } else {
                    ly.wrapping_sub(sprite_y)
                };

                let tile_idx = TILE_SET1 + (sprite_n as u16) * 16;
                let tile_row = (tile_y as u16) * 2;
                let index = tile_idx + tile_row;
                let byte1 = mmu.rb(index);
                let byte2 = mmu.rb(index + 1);

                for x in 0..8 {
                    let bit_idx = if x_flip { 7 - x } else { x };
                    let mask = 1 << bit_idx;
                    let bit1 = byte1 & mask != 0;
                    let bit2 = byte2 & mask != 0;
                    let pair = ((bit1 as u8) << 1) | bit2 as u8;

                    if sprite_x.wrapping_add(7 - x) < (SCREEN_W as u8) && pair != 0b00 {
                        let sprite_has_priority = sprite_o & 0x80 == 0;

                        if sprite_has_priority {
                            let color = PPU::get_color_from(pair);
                            let column = sprite_x.wrapping_add(x) as usize;
                            self.data[ly as usize][column] = color;
                        }
                    }
                }
            }
        }
    }
}
