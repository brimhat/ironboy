use crate::mmu::MMU;
use crate::registers::Registers;
use crate::registers::Flag;
use crate::instructions::{CLOCKS, CB_CLOCKS};

pub struct CPU {
    pub reg: Registers,
    pub ime: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: Registers::new(),
            ime: false,
        }
    }

    pub fn execute(&mut self, mmu: &mut MMU, instr: Instruction) {
        match instr {
            Instruction::LD(t1, t2) => {
                match (t1, t2) {
                    (Target::SP, Target::IMM16) => {
                        self.reg.sp = self.get_imm16(mmu);
                        self.reg.pc += 3;
                    },
                    (Target::DE, Target::IMM16) => {
                        self.reg.set_de(self.get_imm16(mmu));
                        self.reg.pc += 3;
                    },
                    (Target::HL, Target::IMM16) => {
                        self.reg.set_hl(self.get_imm16(mmu));
                        self.reg.pc += 3;
                    },
                    (Target::HLI, Target::A) => {
                        mmu.wb(self.reg.hli(), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::HLD, Target::A) => {
                        mmu.wb(self.reg.hld(), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::A) => {
                        mmu.wb(self.reg.hl(), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::DE) => {
                        self.reg.a = mmu.rb(self.reg.de());
                        self.reg.pc += 1;
                    }
                    (Target::C, Target::IMM8) => {
                        self.reg.c = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::A, Target::IMM8) => {
                        self.reg.a = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::B, Target::IMM8) => {
                        self.reg.b = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::D, Target::IMM8) => {
                        self.reg.d = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::E, Target::IMM8) => {
                        self.reg.e = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::L, Target::IMM8) => {
                        self.reg.l = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::C, Target::A) => {
                        self.reg.c = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::A) => {
                        self.reg.d = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::A) => {
                        self.reg.h = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::B) => {
                        self.reg.a = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::E) => {
                        self.reg.a = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::H) => {
                        self.reg.a = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::L) => {
                        self.reg.a = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::FFIMM8, Target::A) => {
                        mmu.wb(0xFF00 | (self.get_imm8(mmu) as u16), self.reg.a);
                        self.reg.pc += 2;
                    },
                    (Target::FFC, Target::A) => {
                        mmu.wb(0xFF00 | (self.reg.c as u16), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::FFIMM8) => {
                        self.reg.a = mmu.rb(0xFF00 | (self.get_imm8(mmu) as u16));
                        self.reg.pc += 2;
                    }
                    (Target::IMM16, Target::A) => {
                        mmu.wb(self.get_imm16(mmu), self.reg.a);
                        self.reg.pc += 3;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::XOR(t) => {
                match t {
                    Target::A => {
                        self.reg.a ^= self.reg.a;
                        self.reg.set_flag(Flag::Z, true);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, false);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::INC(t) => {
                match t {
                    Target::C => {
                        let c = self.reg.c.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, c == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.c & 0x0F) + 1 > 0x0F);
                        self.reg.c = c;
                        self.reg.pc += 1;
                    },
                    Target::B => {
                        let b = self.reg.b.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, b == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.b & 0x0F) + 1 > 0x0F);
                        self.reg.b = b;
                        self.reg.pc += 1;
                    },
                    Target::H => {
                        let h = self.reg.h.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, h == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.h & 0x0F) + 1 > 0x0F);
                        self.reg.h = h;
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        self.reg.set_hl(self.reg.hl().wrapping_add(1));
                        self.reg.pc += 1;
                    },
                    Target::DE => {
                        self.reg.set_de(self.reg.de().wrapping_add(1));
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::ADD(t) => {
                match t {
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let (v, c) = self.reg.a.overflowing_add(at_hl);
                        let hc = (self.reg.a & 0x0F) + (at_hl & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::DEC(t) => {
                match t {
                    Target::A => {
                        let a = self.reg.a.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, a == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) == 0);
                        self.reg.a = a;
                        self.reg.pc += 1;
                    },
                    Target::B => {
                        let b = self.reg.b.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, b == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.b & 0x0F) == 0);
                        self.reg.b = b;
                        self.reg.pc += 1;
                    },
                    Target::C => {
                        let c = self.reg.c.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, c == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.c & 0x0F) == 0);
                        self.reg.c = c;
                        self.reg.pc += 1;
                    },
                    Target::D => {
                        let d = self.reg.d.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, d == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.d & 0x0F) == 0);
                        self.reg.d = d;
                        self.reg.pc += 1;
                    },
                    Target::E => {
                        let e = self.reg.e.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, e == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.e & 0x0F) == 0);
                        self.reg.e = e;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::SUB(t) => {
                match t {
                    Target::B => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.b);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::CP(t) => {
                match t {
                    Target::IMM8 => {
                        let imm8 = self.get_imm8(mmu);
                        let (v, c) = self.reg.a.overflowing_sub(imm8);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.pc += 2;
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let (v, c) = self.reg.a.overflowing_sub(at_hl);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::BIT(i, t) => {
                match t {
                    Target::H => {
                        self.reg.set_flag(Flag::Z, self.reg.h & (1 << i) == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, true);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::JR(f) => {
                match f {
                    JumpFlag::NZ => {
                        let mut pc = self.reg.pc + 2;
                        if !self.reg.get_flag(Flag::Z) {
                            let n = self.get_imm8(mmu) as i8;
                            pc = pc.wrapping_add(n as u16);
                        }
                        self.reg.pc = pc;
                    }
                    JumpFlag::Z => {
                        let mut pc = self.reg.pc + 2;
                        if self.reg.get_flag(Flag::Z) {
                            let n = self.get_imm8(mmu) as i8;
                            pc = pc.wrapping_add(n as u16);
                        }
                        self.reg.pc = pc;
                    }
                    JumpFlag::A => {
                        let mut pc = self.reg.pc + 2;
                        let n = self.get_imm8(mmu) as i8;
                        self.reg.pc = pc.wrapping_add(n as u16);
                    }
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::JP(f) => {
                match f {
                    JumpFlag::A => {
                        let jump = self.get_imm16(mmu);
                        self.reg.pc = jump;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::CALL(f) => {
                match f {
                    JumpFlag::A => {
                        let pc = self.reg.pc + 3;
                        self.push(mmu, pc);
                        self.reg.sp -= 2;
                        self.reg.pc = self.get_imm16(mmu);
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::RET(f) => {
                match f {
                    JumpFlag::A => {
                        self.reg.pc = self.pop(mmu);
                        self.reg.sp += 2;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::PUSH(t) => {
                match t {
                    Target::BC => {
                        self.push(mmu, self.reg.bc());
                        self.reg.sp -= 2;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::POP(t) => {
                match t {
                    Target::BC => {
                        self.reg.set_bc(self.pop(mmu));
                        self.reg.sp += 2;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::RL(t) => {
                match t {
                    Target::C => {
                        let c = (self.reg.c & 0x80) != 0;
                        self.reg.c = (self.reg.c << 1) + (self.reg.get_flag(Flag::C) as u8);
                        self.reg.set_flag(Flag::Z, self.reg.c == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::RLA => {
                let c = (self.reg.a & 0x80) != 0;
                self.reg.a = (self.reg.a << 1) + (self.reg.get_flag(Flag::C) as u8);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.pc += 1;
            },
            Instruction::NOP => self.reg.pc += 1,
            _ => panic!("Unrecognized instr: {:?}", instr)
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u8 {
        let byte = mmu.rb(self.reg.pc);
        let mut clocks = CLOCKS[byte as usize];

        let instr = match byte == 0xCB {
            false => Instruction::decode(byte),
            true => {
                self.reg.pc += 1;
                let cb_byte = mmu.rb(self.reg.pc);
                clocks = CB_CLOCKS[cb_byte as usize];
                Instruction::decode_cb(cb_byte)
            }
        };

        self.execute(mmu, instr);

        // DEBUGGING
        if mmu.rb(0xFF50) != 0 {
            println!("{:#X}: {:?}\nSTATE AFTER EXECUTION:", byte, instr);
            println!(
                "PC: {:#X}, AF: {:#X}, BC: {:#X}, DE: {:#X}, HL: {:#X}, SP: {:#X}",
                self.reg.pc, self.reg.af(), self.reg.bc(), self.reg.de(), self.reg.hl(), self.reg.sp
            );
            println!(
                "Z: {}, N: {}, H: {}, C: {}\n",
                self.reg.get_flag(Flag::Z), self.reg.get_flag(Flag::N),
                self.reg.get_flag(Flag::H), self.reg.get_flag(Flag::C)
            );
        }
        if self.reg.pc == 0x294 {
            panic!("STOP");
        }
        clocks
    }

    pub fn get_imm16(&self, mmu: &MMU) -> u16 {
        let lo = mmu.rb(self.reg.pc + 1);
        let hi = mmu.rb(self.reg.pc + 2);
        ((hi as u16) << 8) | lo as u16
    }

    pub fn get_imm8(&self, mmu: &MMU) -> u8 {
        mmu.rb(self.reg.pc + 1)
    }

    pub fn push(&mut self, mmu: &mut MMU, value: u16) {
        mmu.wb(self.reg.sp - 1, (value >> 8) as u8);
        mmu.wb(self.reg.sp - 2, value as u8);
    }

    pub fn pop(&self, mmu: &MMU) -> u16 {
        let lo = mmu.rb(self.reg.sp);
        let hi = mmu.rb(self.reg.sp + 1);
        ((hi as u16) << 8) | lo as u16
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    LD(Target, Target),
    XOR(Target),
    INC(Target),
    DEC(Target),
    BIT(u8, Target),
    JR(JumpFlag),
    JP(JumpFlag),
    CALL(JumpFlag),
    RET(JumpFlag),
    PUSH(Target),
    POP(Target),
    RL(Target),
    RLA,
    CP(Target),
    SUB(Target),
    ADD(Target),
    NOP,
}

#[derive(Debug, Copy, Clone)]
enum Target {
    A, B, C, BC, D, E, DE, H, L, HL, HLI, HLD, SP, IMM8, IMM16, FFC, FFIMM8,
    AtHL // only used for INC (HL) and DEC (HL)
}

#[derive(Debug, Copy, Clone)]
enum JumpFlag {
    A, NZ, Z,
    AtHl,
}

impl Instruction {
    pub fn decode(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::NOP,
            0x04 => Instruction::INC(Target::B),
            0x05 => Instruction::DEC(Target::B),
            0x06 => Instruction::LD(Target::B, Target::IMM8),
            0x0C => Instruction::INC(Target::C),
            0x0D => Instruction::DEC(Target::C),
            0x0E => Instruction::LD(Target::C, Target::IMM8),
            0x11 => Instruction::LD(Target::DE, Target::IMM16),
            0x13 => Instruction::INC(Target::DE),
            0x15 => Instruction::DEC(Target::D),
            0x16 => Instruction::LD(Target::D, Target::IMM8),
            0x17 => Instruction::RLA,
            0x18 => Instruction::JR(JumpFlag::A),
            0x1A => Instruction::LD(Target::A, Target::DE),
            0x1D => Instruction::DEC(Target::E),
            0x1E => Instruction::LD(Target::E, Target::IMM8),
            0x20 => Instruction::JR(JumpFlag::NZ),
            0x21 => Instruction::LD(Target::HL, Target::IMM16),
            0x22 => Instruction::LD(Target::HLI, Target::A),
            0x23 => Instruction::INC(Target::HL),
            0x24 => Instruction::INC(Target::H),
            0x28 => Instruction::JR(JumpFlag::Z),
            0x2E => Instruction::LD(Target::L, Target::IMM8),
            0x31 => Instruction::LD(Target::SP, Target::IMM16),
            0x32 => Instruction::LD(Target::HLD, Target::A),
            0x3D => Instruction::DEC(Target::A),
            0x3E => Instruction::LD(Target::A, Target::IMM8),
            0x4F => Instruction::LD(Target::C, Target::A),
            0x57 => Instruction::LD(Target::D, Target::A),
            0x67 => Instruction::LD(Target::H, Target::A),
            0x77 => Instruction::LD(Target::HL, Target::A),
            0x78 => Instruction::LD(Target::A, Target::B),
            0x7B => Instruction::LD(Target::A, Target::E),
            0x7C => Instruction::LD(Target::A, Target::H),
            0x7D => Instruction::LD(Target::A, Target::L),
            0x86 => Instruction::ADD(Target::HL),
            0x90 => Instruction::SUB(Target::B),
            0xAF => Instruction::XOR(Target::A),
            0xBE => Instruction::CP(Target::HL),
            0xC1 => Instruction::POP(Target::BC),
            0xC3 => Instruction::JP(JumpFlag::A),
            0xC9 => Instruction::RET(JumpFlag::A),
            0xC5 => Instruction::PUSH(Target::BC),
            0xCD => Instruction::CALL(JumpFlag::A),
            0xE0 => Instruction::LD(Target::FFIMM8, Target::A),
            0xE2 => Instruction::LD(Target::FFC, Target::A),
            0xEA => Instruction::LD(Target::IMM16, Target::A),
            0xF0 => Instruction::LD(Target::A, Target::FFIMM8),
            0xFE => Instruction::CP(Target::IMM8),
            _ => panic!("Unrecognized opcode: {:#X}", opcode)
        }
    }

    pub fn decode_cb(opcode: u8) -> Instruction {
        match opcode {
            0x11 => Instruction::RL(Target::C),
            0x7C => Instruction::BIT(7, Target::H),
            _ => panic!("Unrecognized prefixed opcode: {:#X}", opcode)
        }
    }
}

#[cfg(test)]
mod test {
    use super::CPU;
    use super::Registers;
    use super::MMU;
    use super::Instruction;
    use super::Target;
    use super::JumpFlag;
    use super::Flag;

    #[test]
    fn rla() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x95;
        cpu.reg.set_flag(Flag::C, true);
        cpu.execute(&mut mmu, Instruction::RLA);
        assert_eq!(cpu.reg.a, 0x2B);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
    }

    #[test]
    fn add() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.set_hl(0x3000);
        cpu.reg.a = 0x3A;
        mmu.wb(cpu.reg.hl(), 0xC6);
        cpu.reg.set_flag(Flag::H, false);
        cpu.execute(&mut mmu, Instruction::ADD(Target::HL));
        assert_eq!(cpu.reg.a, 0x0);
        assert_eq!(cpu.reg.get_flag(Flag::Z), true);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
        assert_eq!(cpu.reg.get_flag(Flag::H), true);
    }
}