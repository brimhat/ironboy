use crate::interrupts::*;
use crate::mmu::MMU;
use std::cell::RefCell;
use std::rc::Rc;

// 0 = pressed
#[derive(Debug, Copy, Clone)]
pub enum Button {
    Start   = 0b0000_1000,
    Select  = 0b0000_0100,
    B       = 0b0000_0010,
    A       = 0b0000_0001,
    Down    = 0b1111_1000,
    Up      = 0b1111_0100,
    Left    = 0b1111_0010,
    Right   = 0b1111_0001,
}

pub struct Joypad {
    intr: Rc<RefCell<IntReq>>,
    dpad: u8,
    bpad: u8,
    select: u8,
}

impl Joypad {
    pub fn new(intr: Rc<RefCell<IntReq>>) -> Joypad {
        Joypad {
            intr,
            dpad: 0,
            bpad: 0,
            select: 0,
        }
    }

    pub fn set_select(&mut self, value: u8) {
        self.select = value;
    }

    pub fn state(&self) -> u8 {
        let state = if (self.select & 0b0001_0000) == 0 {
            self.select | self.dpad
        } else if (self.select & 0b0010_0000) == 0 {
            self.select | self.bpad
        } else {
            self.select
        };

        0b1100_0000 | state
    }

    pub fn button_down(&mut self, button: Button) {
        self.intr.borrow_mut().set_flag(IntFlag::Joypad);
        if button as u8 > 0x0F {
            self.dpad &= !(button as u8) & 0x0F;
        } else {
            self.bpad &= !(button as u8) & 0x0F;
        }
    }

    pub fn button_up(&mut self, button: Button) {
        if button as u8 > 0x0F {
            self.dpad |= button as u8 & 0x0F;
        } else {
            self.bpad |= button as u8;
        }
    }
}
