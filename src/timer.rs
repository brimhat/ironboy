use crate::mmu::MMU;
use crate::interrupts::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Timer {
    pub counter: u16,
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub intr: Rc<RefCell<IntReq>>,
}

impl Timer {
    pub fn new(intr: Rc<RefCell<IntReq>>) -> Timer {
        Timer {
            counter: 0,
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            intr
        }
    }

    pub fn tick_n(&mut self, m_clocks: u8) {
        if m_clocks == 0 {
            return;
        }

        for _ in 0..m_clocks {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        self.counter = self.counter.wrapping_add(4);
        self.div = (self.counter >> 8) as u8;

        if (self.tac & 0b0100) == 0 {
            return;
        }

        let threshold: u16 = match self.tac & 0b11 {
            0b00 => 1024, // increment every 256 cycles (where 1 cycle == 4 T-clocks == NOP)
            0b01 => 16,   // increment every 4 cycles
            0b10 => 64,   // increment every 16 cycles
            0b11 => 256,  // increment every 64 cycles
            _ => panic!("Unrecognized threshold: {:b}", self.tac & 0b11)
        };

        while self.counter >= threshold {
            self.counter -= threshold;
            self.check_overflow();
        }

        self.div = (self.counter >> 8) as u8;
    }

    fn check_overflow(&mut self) {
        let (tima, overflow) = self.tima.overflowing_add(1);

        // at overflow, reset to tma and request an interrupt
        if overflow {
            self.tima = self.tma;
            self.intr.borrow_mut().set_flag(IntFlag::Timer);
        } else {
            self.tima = tima;
        }
    }
}
