use crate::mmu::MMU;

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    pub counter: u16,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            counter: 0,
        }
    }

    pub fn step(&mut self, mmu: &mut MMU, t_clocks: u8) {
        if mmu.reset_div {
            self.counter = 0;
            mmu.reset_div = false;
        }

        self.counter = self.counter.wrapping_add(t_clocks as u16);
        mmu.write_div((self.counter >> 8) as u8);

        let tac = mmu.rb(0xFF07);
        // if bit 2 of tac is 0, timer is not running (but div is ALWAYS counting)
        if (tac & 0b0100) == 0 {
            return;
        }

        let threshold: u16 = match tac & 0b11 {
            0b00 => 1024, // increment every 256 cycles (where 1 cycle == 4 T-clocks == NOP)
            0b01 => 16,   // increment every 4 cycles
            0b10 => 64,   // increment every 16 cycles
            0b11 => 256,  // increment every 64 cycles
            _ => panic!("Unrecognized threshold: {:b}", tac & 0b11)
        };

        while self.counter >= threshold {
            self.counter -= threshold;
            self.check_overflow(mmu);
        }

        mmu.write_div((self.counter >> 8) as u8);
    }

    fn check_overflow(&self, mmu: &mut MMU) {
        let (tima, overflow) = mmu.rb(0xFF05).overflowing_add(1);
        let tma = mmu.rb(0xFF06);
        let i_f = mmu.rb(0xFF0F);

        // at overflow, reset to tma and request an interrupt
        if overflow {
            mmu.wb(0xFF05, tma);
            mmu.wb(0xFF0F, i_f | 0b0100);
        } else {
            mmu.wb(0xFF05, tima);
        }
    }
}
