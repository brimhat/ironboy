use crate::cartridge::Cartridge;
use crate::timer::Timer;
use crate::interrupts::IntReq;
use crate::joypad::Joypad;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MMU<'a> {
    boot: [u8; 0x100],
    cartridge: &'a mut Cartridge,
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    io_reg: [u8; 0x80],
    hram: [u8; 0x80],
    ie: u8,
    pub timer: Rc<RefCell<Timer>>,
    pub intr: Rc<RefCell<IntReq>>,
    pub joypad: Joypad,
}

impl<'a> MMU<'a> {
    pub fn new(cartridge: &'a mut Cartridge, timer: Rc<RefCell<Timer>>) -> MMU {
        let intr = timer.borrow_mut().intr.clone();
        MMU {
            boot: [0; 0x100],
            cartridge,
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            io_reg: [0; 0x80],
            hram: [0; 0x80],
            ie: 0,
            timer,
            joypad: Joypad::new(intr.clone()),
            intr,
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
        let address = address as usize;
        if address < 0x100 {
            return if self.rb(0xFF50) == 0 {
                self.boot[address]
            } else {
                self.cartridge.read_lower_rom(address as u16)
            }
        }

        match address {
            0x0000..=0x3FFF => self.cartridge.read_lower_rom(address as u16),
            0x4000..=0x7FFF => self.cartridge.read_upper_rom(address as u16),
            0x8000..=0x9FFF => self.vram[address & 0x1FFF],
            0xA000..=0xBFFF => self.cartridge.read_ram(address as u16),
            0xC000..=0xDFFF => self.wram[address & 0x1FFF],
            0xE000..=0xFDFF => self.wram[address & 0x1FFF], // echo
            0xFE00..=0xFE9F => self.oam[address & 0xFF],
            0xFEA0..=0xFEFF => 0xFF, // unusable area returns FFh
            0xFF00 => self.joypad.state(),
            0xFF04 => self.timer.borrow_mut().div,
            0xFF05 => self.timer.borrow_mut().tima,
            0xFF06 => self.timer.borrow_mut().tma,
            0xFF07 => self.timer.borrow_mut().tac,
            0xFF0F => self.intr.borrow_mut().flags,
            0xFF00..=0xFF7F => self.io_reg[address & 0x7F],
            0xFF80..=0xFFFE => self.hram[address & 0x7F],
            0xFFFF => self.ie,
            _ => unreachable!()
        }
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address as u16, value),
            0x8000..=0x9FFF => self.vram[address & 0x1FFF] = value,
            0xA000..=0xBFFF => self.cartridge.write_ram(address as u16, value),
            0xC000..=0xDFFF => self.wram[address & 0x1FFF] = value,
            0xE000..=0xFDFF => (), // echo not writable
            0xFE00..=0xFE9F => self.oam[address & 0xFF] = value,
            0xFEA0..=0xFEFF => (), // writes to unusable area have no effect
            0xFF00 => self.joypad.set_select(value),
            0xFF04 => {
                self.timer.borrow_mut().counter = 0;
                self.timer.borrow_mut().div = 0;
            },
            0xFF05 => self.timer.borrow_mut().tima = value,
            0xFF06 => self.timer.borrow_mut().tma = value,
            0xFF07 => self.timer.borrow_mut().tac = value,
            0xFF0F => self.intr.borrow_mut().flags = 0b1110_0000 | value,
            0xFF46 => {
                assert!(value <= 0xF1);
                self.io_reg[address & 0x7F] = value;
                let hi = (value as u16) << 8;
                for lo in 0..0xA0 {
                    let byte = self.rb(hi | lo);
                    self.wb(0xFE00 | lo, byte);
                }
            }
            0xFF00..=0xFF7F => self.io_reg[address & 0x7F] = value,
            0xFF80..=0xFFFE => self.hram[address & 0x7F] = value,
            0xFFFF => self.ie = value,
            _ => unreachable!()
        }
    }
}
