use crate::cartridge::Cartridge;
use crate::timer::Timer;
use crate::interrupts::IntReq;
use crate::joypad::Joypad;
use std::rc::Rc;
use std::cell::RefCell;

const MEM_SIZE: usize = 0xFFFF + 1;

pub struct MMU<'a> {
    boot: [u8; 0x100],
    cartridge: &'a mut Cartridge,
    mem: [u8; MEM_SIZE],
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
            mem: [0; MEM_SIZE],
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
        if address == 0xFF00 {
            println!("JOYPAD STATE: {:#b}", self.joypad.state());
        }
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
            0xFF00 => self.joypad.state(),
            0xFF04 => self.timer.borrow_mut().div,
            0xFF05 => self.timer.borrow_mut().tima,
            0xFF06 => self.timer.borrow_mut().tma,
            0xFF07 => self.timer.borrow_mut().tac,
            0xFF0F => self.intr.borrow_mut().flags,
            _ => self.mem[address as usize],
        }
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        if address == 0xFF00 {
            println!("JOYPAD SELECT: {:#b}", value);
        }
        if address == 0xFF46 { // DMA Transfer
            assert!(value <= 0xF1);
            self.mem[0xFF46] = value;
            let hi = (value as u16) << 8;
            for lo in 0..0xA0 {
                let byte = self.rb(hi | lo);
                self.wb(0xFE00 | lo, byte);
            }
        }

        if address == 0xFF04 {
            // if divider is written to, div and system internal counter set to 0
            self.timer.borrow_mut().counter = 0;
            self.timer.borrow_mut().div = 0;
            return;
        }

        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0xA000..=0xBFFF => self.cartridge.write_ram(address, value),
            0xFF00 => self.joypad.set_select(value),
            0xFF05 => self.timer.borrow_mut().tima = value,
            0xFF06 => self.timer.borrow_mut().tma = value,
            0xFF07 => self.timer.borrow_mut().tac = value,
            0xFF0F => self.intr.borrow_mut().flags = 0b1110_0000 | value,
            _ => self.mem[address as usize] = value,
        }
    }
}
