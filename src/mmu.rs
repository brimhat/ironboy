use crate::cartridge::Cartridge;
use crate::timer::Timer;
use crate::interrupts::IntReq;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

const MEM_SIZE: usize = 0xFFFF + 1;

pub struct MMU<'a> {
    boot: [u8; 0x100],
    cartridge: &'a mut Cartridge,
    mem: [u8; MEM_SIZE],
    pub timer: Rc<RefCell<Timer>>,
    pub intr: Rc<RefCell<IntReq>>,
}

impl<'a> MMU<'a> {
    pub fn new(cartridge: &'a mut Cartridge, timer: Rc<RefCell<Timer>>) -> MMU {
        let intr = timer.borrow_mut().intr.clone();
        MMU {
            boot: [0; 0x100],
            cartridge,
            mem: [0; MEM_SIZE],
            timer,
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
            0xFF04 => self.timer.borrow_mut().div,
            0xFF05 => self.timer.borrow_mut().tima,
            0xFF06 => self.timer.borrow_mut().tma,
            0xFF07 => self.timer.borrow_mut().tac,
            0xFF0F => self.intr.borrow_mut().flags,
            _ => self.mem[address as usize],
        }
    }

    pub fn wb(&mut self, address: u16, value: u8) {
//        if address == 0xFF40 {
//            if (value & 0x80) == 0 && (self.mem[0xFF40] & 0x80) != 0 {
//                println!("LCD OFF")
//            } else if (value & 0x80) != 0 && (self.mem[0xFF40] & 0x80) == 0 {
//                println!("LCD ON")
//            }
//        }
//        if self.mem[0xFF50] != 0 {
//            if address >= 0x9800 && address <= 0x9FFF {
//                if value != 0 && value != 0x2F {
//                    println!("[{:#X}] = {:#X}", address, value);
//                } else {
//                    print!(".");
//                }
//            }
//        }
        if address == 0xFF04 {
            // if divider is written to, div and system internal counter set to 0
            self.timer.borrow_mut().counter = 0;
            self.timer.borrow_mut().div = 0;
            return;
        }

        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0xA000..=0xBFFF => self.cartridge.write_ram(address, value),
            0xFF05 => self.timer.borrow_mut().tima = value,
            0xFF06 => self.timer.borrow_mut().tma = value,
            0xFF07 => self.timer.borrow_mut().tac = value,
            0xFF0F => self.intr.borrow_mut().flags = 0b1110_0000 | value,
            _ => self.mem[address as usize] = value,
        }
    }
}
