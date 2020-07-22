use crate::mmu::MMU;

pub struct Timer {
    main: u16,
    sub: u8,
    div: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            main: 0,
            sub: 0,
            div: 0,
        }
    }

    pub fn inc(&mut self, mmu: &mut MMU, clocks: u8) {
        self.sub += clocks / 4;

        if self.sub >= 4 {
            self.main.wrapping_add(1);
            self.sub -= 4;

            self.div += 1;
            if self.div == 16 {
                let div = mmu.rb(0xFF04);
                mmu.wb(0xFF04, div.wrapping_add(1));
                self.div = 0;
            }
        }

        self.check(mmu);
    }

    fn check(&mut self, mmu: &mut MMU) {
        let tac = mmu.rb(0xFF06);

        // if bit 2 of tac is 0, timer is not running
        if (tac & 0b0100) == 0 {
            return;
        }

        let threshold: u16 = match tac & 0b11 {
            0b00 => 64,
            0b01 => 1,
            0b10 => 4,
            0b11 => 16,
            _ => panic!("Unrecognized threshold: {:b}", tac & 0b11)
        };

        if self.main >= threshold {
            self.step(mmu);
        }
    }

    fn step(&mut self, mmu: &mut MMU) {
        let tima = mmu.rb(0xFF05).wrapping_add(1);
        let tma = mmu.rb(0xFF06);
        let i_f = mmu.rb(0xFF0F);
        self.main = 0;

        // at overflow, set to default value and request an interrupt
        if tima == 0 {
            mmu.wb(0xFF05, tma);
            mmu.wb(0xFF0F, i_f | 0b0100);
        } else {
            mmu.wb(0xFF05, tima);
        }
    }
}
