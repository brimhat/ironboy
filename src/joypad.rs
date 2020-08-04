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
    Down    = 0b1000_0000,
    Up      = 0b0100_0000,
    Left    = 0b0010_0000,
    Right   = 0b0001_0000,
}

pub struct Joypad {
    intr: Rc<RefCell<IntReq>>,
}

impl Joypad {
    pub fn new(intr: Rc<RefCell<IntReq>>) -> Joypad {
        Joypad {
            intr,
        }
    }

    pub fn button_down(&mut self, mmu: &mut MMU, button: Button) {
        self.intr.borrow_mut().set_flag(IntFlag::Joypad);
        let select = mmu.rb(0xFF00) & 0b0011_0000;
        let value = if select == 0b0010_0000 {
            select | !(button as u8) >> 4
        } else {
            select | !(button as u8) & 0x0F
        };

        mmu.wb(0xFF00, value);
//        if value != select {
//            println!("{:?} {:#b}", button, value);
//        }
    }

    pub fn button_up(&mut self, mmu: &mut MMU, button: Button) {
        let down = mmu.rb(0xFF00);
        let select = down & 0b0011_0000;
        let nibble = if select == 0b0010_0000 {
            (button as u8) >> 4
        } else {
            (button as u8) & 0x0F
        };

        mmu.wb(0xFF00, down | nibble as u8);
    }
}
