use crate::mmu::MMU;
use crate::registers::Registers;
use crate::registers::Flag;
use crate::instructions::{CLOCKS, CB_CLOCKS};

pub struct CPU {
    pub reg: Registers,
    pub ime: bool,
    pub halt: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: Registers::new(),
            ime: false,
            halt: false,
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
                    (Target::BC, Target::IMM16) => {
                        self.reg.set_bc(self.get_imm16(mmu));
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
                    (Target::IMM16, Target::SP) => {
                        let lo = (self.reg.sp & 0x00FF) as u8;
                        let hi = ((self.reg.sp & 0xFF00) >> 8) as u8;
                        let nn = self.get_imm16(mmu);
                        mmu.wb(nn, lo);
                        mmu.wb(nn + 1, hi);
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
                    (Target::A, Target::HLI) => {
                        self.reg.a = mmu.rb(self.reg.hli());
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::HLD) => {
                        self.reg.a = mmu.rb(self.reg.hld());
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::HL) => {
                        self.reg.a = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::HL) => {
                        self.reg.b = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::HL) => {
                        self.reg.c = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::HL) => {
                        self.reg.d = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::HL) => {
                        self.reg.e = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::HL) => {
                        self.reg.h = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::HL) => {
                        self.reg.l = mmu.rb(self.reg.hl());
                        self.reg.pc += 1;
                    },
                    (Target::BC, Target::A) => {
                        mmu.wb(self.reg.bc(), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::DE, Target::A) => {
                        mmu.wb(self.reg.de(), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::A) => {
                        mmu.wb(self.reg.hl(), self.reg.a);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::B) => {
                        mmu.wb(self.reg.hl(), self.reg.b);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::C) => {
                        mmu.wb(self.reg.hl(), self.reg.c);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::D) => {
                        mmu.wb(self.reg.hl(), self.reg.d);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::E) => {
                        mmu.wb(self.reg.hl(), self.reg.e);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::H) => {
                        mmu.wb(self.reg.hl(), self.reg.h);
                        self.reg.pc += 1;
                    },
                    (Target::HL, Target::L) => {
                        mmu.wb(self.reg.hl(), self.reg.l);
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::BC) => {
                        self.reg.a = mmu.rb(self.reg.bc());
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::DE) => {
                        self.reg.a = mmu.rb(self.reg.de());
                        self.reg.pc += 1;
                    },
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
                    (Target::H, Target::IMM8) => {
                        self.reg.h = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::L, Target::IMM8) => {
                        self.reg.l = self.get_imm8(mmu);
                        self.reg.pc += 2;
                    },
                    (Target::HL, Target::IMM8) => {
                        let imm8 = self.get_imm8(mmu);
                        mmu.wb(self.reg.hl(), imm8);
                        self.reg.pc += 2;
                    },
                    (Target::B, Target::A) => {
                        self.reg.b = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::B) => {
                        self.reg.b = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::C) => {
                        self.reg.b = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::D) => {
                        self.reg.b = self.reg.d;
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::E) => {
                        self.reg.b = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::H) => {
                        self.reg.b = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::B, Target::L) => {
                        self.reg.b = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::A) => {
                        self.reg.c = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::B) => {
                        self.reg.c = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::C) => {
                        self.reg.c = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::D) => {
                        self.reg.c = self.reg.d;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::E) => {
                        self.reg.c = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::H) => {
                        self.reg.c = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::C, Target::L) => {
                        self.reg.c = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::A) => {
                        self.reg.d = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::B) => {
                        self.reg.d = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::C) => {
                        self.reg.d = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::D) => {
                        self.reg.d = self.reg.d;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::E) => {
                        self.reg.d = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::H) => {
                        self.reg.d = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::D, Target::L) => {
                        self.reg.d = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::A) => {
                        self.reg.e = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::B) => {
                        self.reg.e = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::C) => {
                        self.reg.e = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::D) => {
                        self.reg.e = self.reg.d;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::E) => {
                        self.reg.e = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::H) => {
                        self.reg.e = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::E, Target::L) => {
                        self.reg.e = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::A) => {
                        self.reg.h = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::B) => {
                        self.reg.h = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::C) => {
                        self.reg.h = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::D) => {
                        self.reg.h = self.reg.d;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::E) => {
                        self.reg.h = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::H) => {
                        self.reg.h = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::H, Target::L) => {
                        self.reg.h = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::A) => {
                        self.reg.l = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::B) => {
                        self.reg.l = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::C) => {
                        self.reg.l = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::D) => {
                        self.reg.l = self.reg.d;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::E) => {
                        self.reg.l = self.reg.e;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::H) => {
                        self.reg.l = self.reg.h;
                        self.reg.pc += 1;
                    },
                    (Target::L, Target::L) => {
                        self.reg.l = self.reg.l;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::A) => {
                        self.reg.a = self.reg.a;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::B) => {
                        self.reg.a = self.reg.b;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::C) => {
                        self.reg.a = self.reg.c;
                        self.reg.pc += 1;
                    },
                    (Target::A, Target::D) => {
                        self.reg.a = self.reg.d;
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
                    (Target::A, Target::IMM16) => {
                        self.reg.a = mmu.rb(self.get_imm16(mmu));
                        self.reg.pc += 3;
                    }
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
                    Target::C => {
                        self.reg.a ^= self.reg.c;
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, false);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::OR(t) => {
                match t {
                    Target::B => {
                        self.reg.a |= self.reg.b;
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.set_flag(Flag::H, false);
                        self.reg.pc += 1;
                    },
                    Target::C => {
                        self.reg.a |= self.reg.c;
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.set_flag(Flag::H, false);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::AND(t) => {
                match t {
                    Target::IMM8 => {
                        self.reg.a &= self.get_imm8(mmu);
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, true);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.pc += 2;
                    },
                    Target::A => {
                        self.reg.a &= self.reg.a;
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, true);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.pc += 1;
                    },
                    Target::C => {
                        self.reg.a &= self.reg.c;
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, true);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::INC(t) => {
                match t {
                    Target::A => {
                        let a = self.reg.a.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) + 1 > 0x0F);
                        self.reg.a = a;
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
                    Target::C => {
                        let c = self.reg.c.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, c == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.c & 0x0F) + 1 > 0x0F);
                        self.reg.c = c;
                        self.reg.pc += 1;
                    },
                    Target::D => {
                        let d = self.reg.d.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, d == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.d & 0x0F) + 1 > 0x0F);
                        self.reg.d = d;
                        self.reg.pc += 1;
                    },
                    Target::E => {
                        let e = self.reg.e.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, e == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.e & 0x0F) + 1 > 0x0F);
                        self.reg.e = e;
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
                    Target::L => {
                        let l = self.reg.l.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, l == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.l & 0x0F) + 1 > 0x0F);
                        self.reg.l = l;
                        self.reg.pc += 1;
                    },
                    Target::BC => {
                        self.reg.set_bc(self.reg.bc().wrapping_add(1));
                        self.reg.pc += 1;
                    },
                    Target::DE => {
                        self.reg.set_de(self.reg.de().wrapping_add(1));
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        self.reg.set_hl(self.reg.hl().wrapping_add(1));
                        self.reg.pc += 1;
                    },
                    Target::SP => {
                        self.reg.sp = self.reg.sp.wrapping_add(1);
                        self.reg.pc += 1;
                    },
                    Target::AtHL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let new_hl = at_hl.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, new_hl == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (at_hl & 0x0F) + 1 > 0x0F);
                        mmu.wb(self.reg.hl(), new_hl);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::ADD(t) => {
                match t {
                    Target::A => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.a);
                        let hc = (self.reg.a & 0x0F) + (self.reg.a & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::B => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.b);
                        let hc = (self.reg.a & 0x0F) + (self.reg.b & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::C => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.c);
                        let hc = (self.reg.a & 0x0F) + (self.reg.c & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::D => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.d);
                        let hc = (self.reg.a & 0x0F) + (self.reg.d & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::E => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.e);
                        let hc = (self.reg.a & 0x0F) + (self.reg.e & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::H => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.h);
                        let hc = (self.reg.a & 0x0F) + (self.reg.h & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::L => {
                        let (v, c) = self.reg.a.overflowing_add(self.reg.l);
                        let hc = (self.reg.a & 0x0F) + (self.reg.l & 0x0F) > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
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
            },
            Instruction::ADC(t) => {
                match t {
                    Target::A => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.a);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.a & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::B => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.b);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.b & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::C => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.c);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.c & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::D => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.d);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.d & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::E => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.e);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.e & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::H => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.h);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.h & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::L => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let (_v, c1) = self.reg.a.overflowing_add(self.reg.l);
                        let (v, c2) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0xF) + (self.reg.l & 0xF) + cf > 0xF;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c1 || c2);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        let cf = self.reg.get_flag(Flag::C) as u8;
                        let at_hl = mmu.rb(self.reg.hl());
                        let (_v, c1) = self.reg.a.overflowing_add(at_hl);
                        let (v, c) = _v.overflowing_add(cf);
                        let hc = (self.reg.a & 0x0F) + (at_hl & 0x0F) + cf > 0x0F;
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::ADDHL(t) => {
                match t {
                    Target::BC => {
                        let bc = self.reg.bc();
                        let (v, c) = self.reg.hl().overflowing_add(bc);
                        let hc = (self.reg.hl() & 0x0FFF) + (bc & 0x0FFF) > 0x0FFF;
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.set_hl(v);
                        self.reg.pc += 1;
                    },
                    Target::DE => {
                        let de = self.reg.de();
                        let (v, c) = self.reg.hl().overflowing_add(de);
                        let hc = (self.reg.hl() & 0x0FFF) + (de & 0x0FFF) > 0x0FFF;
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.set_hl(v);
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        let hl = self.reg.hl();
                        let (v, c) = self.reg.hl().overflowing_add(hl);
                        let hc = (self.reg.hl() & 0x0FFF) + (hl & 0x0FFF) > 0x0FFF;
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.set_hl(v);
                        self.reg.pc += 1;
                    },
                    Target::SP => {
                        let sp = self.reg.sp;
                        let (v, c) = self.reg.hl().overflowing_add(sp);
                        let hc = (self.reg.hl() & 0x0FFF) + (sp & 0x0FFF) > 0x0FFF;
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.set_hl(v);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
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
                    Target::H => {
                        let h = self.reg.h.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, h == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.h & 0x0F) == 0);
                        self.reg.h = h;
                        self.reg.pc += 1;
                    },
                    Target::L => {
                        let l = self.reg.l.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, l == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.l & 0x0F) == 0);
                        self.reg.l = l;
                        self.reg.pc += 1;
                    },
                    Target::BC => {
                        self.reg.set_bc(self.reg.bc().wrapping_sub(1));
                        self.reg.pc += 1;
                    },
                    Target::DE => {
                        self.reg.set_de(self.reg.de().wrapping_sub(1));
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        self.reg.set_hl(self.reg.hl().wrapping_sub(1));
                        self.reg.pc += 1;
                    },
                    Target::SP => {
                        self.reg.sp = self.reg.sp.wrapping_sub(1);
                        self.reg.pc += 1;
                    },
                    Target::AtHL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let new_hl = at_hl.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, new_hl == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (at_hl & 0x0F) == 0);
                        mmu.wb(self.reg.hl(), new_hl);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::SUB(t) => {
                match t {
                    Target::A => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.a);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::B => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.b);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::C => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.c);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::D => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.d);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::E => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.e);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::H => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.h);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::L => {
                        let (v, c) = self.reg.a.overflowing_sub(self.reg.l);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let (v, c) = self.reg.a.overflowing_sub(at_hl);
                        self.reg.set_flag(Flag::Z, v == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) < (v & 0x0F));
                        self.reg.set_flag(Flag::C, c);
                        self.reg.a = v;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
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
            },
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
            Instruction::RES(i , t) => {
                match t {
                    Target::A => {
                        self.reg.a = self.reg.a & !(1 << i);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::JR(f) => {
                match f {
                    JumpFlag::NZ => {
                        let mut pc = self.reg.pc + 2;
                        if !self.reg.get_flag(Flag::Z) {
                            let n = self.get_imm8(mmu) as i8;
                            pc = pc.wrapping_add(n as u16);
                        }
                        self.reg.pc = pc;
                    },
                    JumpFlag::NC => {
                        let mut pc = self.reg.pc + 2;
                        if !self.reg.get_flag(Flag::C) {
                            let n = self.get_imm8(mmu) as i8;
                            pc = pc.wrapping_add(n as u16);
                        }
                        self.reg.pc = pc;
                    },
                    JumpFlag::Z => {
                        let mut pc = self.reg.pc + 2;
                        if self.reg.get_flag(Flag::Z) {
                            let n = self.get_imm8(mmu) as i8;
                            pc = pc.wrapping_add(n as u16);
                        }
                        self.reg.pc = pc;
                    },
                    JumpFlag::C => {
                        let mut pc = self.reg.pc + 2;
                        if self.reg.get_flag(Flag::C) {
                            let n = self.get_imm8(mmu) as i8;
                            pc = pc.wrapping_add(n as u16);
                        }
                        self.reg.pc = pc;
                    },
                    JumpFlag::A => {
                        let mut pc = self.reg.pc + 2;
                        let n = self.get_imm8(mmu) as i8;
                        self.reg.pc = pc.wrapping_add(n as u16);
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::JP(f) => {
                match f {
                    JumpFlag::AtHL => self.reg.pc = self.reg.hl(),
                    JumpFlag::A => {
                        let jump = self.get_imm16(mmu);
                        self.reg.pc = jump;
                    },
                    JumpFlag::Z => {
                        let mut pc = self.reg.pc + 3;
                        if self.reg.get_flag(Flag::Z) {
                            pc = self.get_imm16(mmu);
                        }
                        self.reg.pc = pc;
                    },
                    JumpFlag::C => {
                        let mut pc = self.reg.pc + 3;
                        if self.reg.get_flag(Flag::C) {
                            pc = self.get_imm16(mmu);
                        }
                        self.reg.pc = pc;
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
                    JumpFlag::NZ => {
                        self.reg.pc += 1;
                        if !self.reg.get_flag(Flag::Z) {
                            self.reg.pc = self.pop(mmu);
                            self.reg.sp += 2;
                        }
                    },
                    JumpFlag::Z => {
                        self.reg.pc += 1;
                        if self.reg.get_flag(Flag::Z) {
                            self.reg.pc = self.pop(mmu);
                            self.reg.sp += 2;
                        }
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            }
            Instruction::PUSH(t) => {
                match t {
                    Target::AF => {
                        self.push(mmu, self.reg.af());
                        self.reg.sp -= 2;
                        self.reg.pc += 1;
                    },
                    Target::BC => {
                        self.push(mmu, self.reg.bc());
                        self.reg.sp -= 2;
                        self.reg.pc += 1;
                    },
                    Target::DE => {
                        self.push(mmu, self.reg.de());
                        self.reg.sp -= 2;
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        self.push(mmu, self.reg.hl());
                        self.reg.sp -= 2;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::POP(t) => {
                match t {
                    Target::AF => {
                        self.reg.set_af(self.pop(mmu));
                        self.reg.sp += 2;
                        self.reg.pc += 1;
                    },
                    Target::BC => {
                        self.reg.set_bc(self.pop(mmu));
                        self.reg.sp += 2;
                        self.reg.pc += 1;
                    },
                    Target::DE => {
                        self.reg.set_de(self.pop(mmu));
                        self.reg.sp += 2;
                        self.reg.pc += 1;
                    },
                    Target::HL => {
                        self.reg.set_hl(self.pop(mmu));
                        self.reg.sp += 2;
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::RST(t) => {
                self.push(mmu, self.reg.pc + 1);
                self.reg.sp -= 2;
                self.reg.pc = t as u16;
            },
            Instruction::RLCA => {
                let c = (self.reg.a & 0x80) != 0;
                self.reg.a = (self.reg.a << 1); // | c as u8;
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.pc += 1;
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
            Instruction::RRCA => {
                let c = (self.reg.a & 0x01) != 0;
                self.reg.a = ((c as u8) << 7) | (self.reg.a >> 1);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.pc += 1;
            },
            Instruction::RRA => {
                let c = (self.reg.a & 0x01) != 0;
                let c_flag = self.reg.get_flag(Flag::C) as u8;
                self.reg.a = (c_flag << 7) | (self.reg.a >> 1);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.pc += 1;
            },
            Instruction::SWAP(t) => {
                match t {
                    Target::A => {
                        let hi = (self.reg.a & 0x0F) << 4;
                        let lo = (self.reg.a & 0xF0) >> 4;
                        self.reg.a = hi | lo;
                        self.reg.set_flag(Flag::Z, self.reg.a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, false);
                        self.reg.set_flag(Flag::C, false);
                        self.reg.pc += 1;
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::CPL => {
                self.reg.a = !self.reg.a;
                self.reg.set_flag(Flag::N, true);
                self.reg.set_flag(Flag::H, true);
                self.reg.pc += 1;
            },
            Instruction::SCF => {
                self.reg.set_flag(Flag::C, true);
                self.reg.pc += 1;
            },
            Instruction::CCF => {
                let c_flag = self.reg.get_flag(Flag::C);
                self.reg.set_flag(Flag::C, !c_flag);
                self.reg.pc += 1;
            }
            Instruction::DAA => {
                let c_flag = self.reg.get_flag(Flag::C);
                let h_flag = self.reg.get_flag(Flag::H);
                let n_flag = self.reg.get_flag(Flag::N);

                let mut carry = false;
                let mut adjust = 0;
                let a = self.reg.a;
                let hi = self.reg.a & 0xF0;
                let lo = self.reg.a & 0x0F;

                if !n_flag {
                    if h_flag || (a & 0x0F) > 0x09 {
                        adjust |= 0x06;
                    }
                    if c_flag || a > 0x99 {
                        carry = true;
                        adjust |= 0x60;
                    }
                } else {
                    if c_flag && h_flag {
                        carry = true;
                        adjust |= 0x9A;
                    } else if c_flag {
                        carry = true;
                        adjust |= 0xA0;
                    } else if h_flag {
                        adjust |= 0xFA;
                    }
                }

                self.reg.a = self.reg.a.wrapping_add(adjust);
                self.reg.set_flag(Flag::Z, self.reg.a == 0);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, carry);
                self.reg.pc += 1;
            },
            Instruction::DI => {
                self.ime = false;
                self.reg.pc += 1;
            },
            Instruction::EI => {
                self.ime = true;
                self.reg.pc += 1;
            },
            Instruction::HALT => {
                self.halt = true;
                self.reg.pc += 1;
            },
            Instruction::RETI => {
                self.ime = true;
                self.reg.pc = self.pop(mmu);
                self.reg.sp += 2;
            },
            Instruction::NOP => self.reg.pc += 1,
            Instruction::STOP => {},
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

        let old_pc = self.reg.pc;
        if self.interrupt_exists(mmu) {
            self.handle_interrupt(mmu);
        }

        if !self.halt && self.reg.pc == old_pc {
            self.execute(mmu, instr);

            // DEBUGGING
            if mmu.rb(0xFF50) != 0 {
                println!("{:#X}: {:?}\nSTATE AFTER EXECUTION:", byte, instr);
                println!(
                    "PC: {:#X}, AF: {:#X}, BC: {:#X}, DE: {:#X}, HL: {:#X}, SP: {:#X}",
                    self.reg.pc, self.reg.af(), self.reg.bc(),
                    self.reg.de(), self.reg.hl(), self.reg.sp,
                );
                println!(
                    "Z: {}, N: {}, H: {}, C: {}, IME: {}, HALT: {}\n",
                    self.reg.get_flag(Flag::Z), self.reg.get_flag(Flag::N),
                    self.reg.get_flag(Flag::H), self.reg.get_flag(Flag::C),
                    self.ime, self.halt
                );
            }
        }

        clocks
    }

    pub fn interrupt_exists(&self, mmu: &mut MMU) -> bool {
        let e_i = mmu.rb(0xFFFF);
        let i_f = mmu.rb(0xFF0F);
        let e_f = e_i & i_f;

        (self.halt || self.ime) && e_f != 0
    }

    pub fn handle_interrupt(&mut self, mmu: &mut MMU) {
        self.halt = false;
        if !self.ime {
            return;
        }
        self.ime = false;

        let e_i = mmu.rb(0xFFFF);
        let i_f = mmu.rb(0xFF0F);
        let e_f = e_i & i_f;
        let index = e_f.trailing_zeros();
        mmu.wb(0xFF0F, e_f & !(1 << index));

        self.push(mmu, self.reg.pc);
        self.reg.sp -= 2;
        self.reg.pc = match index {
            0 => 0x40, // VBlank
            1 => 0x48, // LCD Stat
            2 => 0x50, // Timer overflow
            3 => 0x58, // Serial link
            4 => 0x60, // Joypad press
            _ => panic!("unrecognized interrupt: {:#b}", e_f),
        };
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
    OR(Target),
    AND(Target),
    INC(Target),
    DEC(Target),
    BIT(u8, Target),
    RES(u8, Target),
    SWAP(Target),
    JR(JumpFlag),
    JP(JumpFlag),
    CALL(JumpFlag),
    RET(JumpFlag),
    PUSH(Target),
    POP(Target),
    RST(u8),
    RL(Target),
    RLA,
    RLCA,
    RRCA,
    RRA,
    CP(Target),
    SUB(Target),
    ADD(Target),
    ADC(Target),
    ADDHL(Target),
    CPL,
    SCF,
    CCF,
    NOP,
    STOP,
    HALT,
    DAA,
    DI,
    EI,
    RETI,
}

#[derive(Debug, Copy, Clone)]
enum Target {
    A, AF, B, C, BC, D, E, DE, H, L, HL, HLI, HLD, SP, IMM8, IMM16, FFC, FFIMM8,
    AtHL // only used for INC (HL) and DEC (HL)
}

#[derive(Debug, Copy, Clone)]
enum JumpFlag {
    A, NZ, Z, NC, C,
    AtHL,
}

impl Instruction {
    pub fn decode(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::NOP,
            0x01 => Instruction::LD(Target::BC, Target::IMM16),
            0x02 => Instruction::LD(Target::BC, Target::A),
            0x03 => Instruction::INC(Target::BC),
            0x04 => Instruction::INC(Target::B),
            0x05 => Instruction::DEC(Target::B),
            0x06 => Instruction::LD(Target::B, Target::IMM8),
            0x07 => Instruction::RLCA,
            0x08 => Instruction::LD(Target::IMM16, Target::SP),
            0x09 => Instruction::ADDHL(Target::BC),
            0x0A => Instruction::LD(Target::A, Target::BC),
            0x0B => Instruction::DEC(Target::BC),
            0x0C => Instruction::INC(Target::C),
            0x0D => Instruction::DEC(Target::C),
            0x0E => Instruction::LD(Target::C, Target::IMM8),
            0x0F => Instruction::RRCA,
            0x10 => Instruction::STOP,
            0x11 => Instruction::LD(Target::DE, Target::IMM16),
            0x12 => Instruction::LD(Target::DE, Target::A),
            0x13 => Instruction::INC(Target::DE),
            0x14 => Instruction::INC(Target::D),
            0x15 => Instruction::DEC(Target::D),
            0x16 => Instruction::LD(Target::D, Target::IMM8),
            0x17 => Instruction::RLA,
            0x18 => Instruction::JR(JumpFlag::A),
            0x19 => Instruction::ADDHL(Target::DE),
            0x1A => Instruction::LD(Target::A, Target::DE),
            0x1B => Instruction::DEC(Target::DE),
            0x1C => Instruction::INC(Target::E),
            0x1D => Instruction::DEC(Target::E),
            0x1E => Instruction::LD(Target::E, Target::IMM8),
            0x1F => Instruction::RRA,
            0x20 => Instruction::JR(JumpFlag::NZ),
            0x21 => Instruction::LD(Target::HL, Target::IMM16),
            0x22 => Instruction::LD(Target::HLI, Target::A),
            0x23 => Instruction::INC(Target::HL),
            0x24 => Instruction::INC(Target::H),
            0x25 => Instruction::DEC(Target::D),
            0x26 => Instruction::LD(Target::H, Target::IMM8),
            0x27 => Instruction::DAA,
            0x28 => Instruction::JR(JumpFlag::Z),
            0x29 => Instruction::ADDHL(Target::HL),
            0x2A => Instruction::LD(Target::A, Target::HLI),
            0x2B => Instruction::DEC(Target::HL),
            0x2C => Instruction::INC(Target::L),
            0x2D => Instruction::DEC(Target::L),
            0x2E => Instruction::LD(Target::L, Target::IMM8),
            0x2F => Instruction::CPL,
            0x30 => Instruction::JR(JumpFlag::NC),
            0x31 => Instruction::LD(Target::SP, Target::IMM16),
            0x32 => Instruction::LD(Target::HLD, Target::A),
            0x33 => Instruction::INC(Target::SP),
            0x34 => Instruction::INC(Target::AtHL),
            0x35 => Instruction::DEC(Target::AtHL),
            0x36 => Instruction::LD(Target::HL, Target::IMM8),
            0x37 => Instruction::SCF,
            0x38 => Instruction::JR(JumpFlag::C),
            0x39 => Instruction::ADDHL(Target::SP),
            0x3A => Instruction::LD(Target::A, Target::HLD),
            0x3B => Instruction::DEC(Target::SP),
            0x3C => Instruction::INC(Target::A),
            0x3D => Instruction::DEC(Target::A),
            0x3E => Instruction::LD(Target::A, Target::IMM8),
            0x3F => Instruction::CCF,
            0x40 => Instruction::LD(Target::B, Target::B),
            0x41 => Instruction::LD(Target::B, Target::C),
            0x42 => Instruction::LD(Target::B, Target::D),
            0x43 => Instruction::LD(Target::B, Target::E),
            0x44 => Instruction::LD(Target::B, Target::H),
            0x45 => Instruction::LD(Target::B, Target::L),
            0x46 => Instruction::LD(Target::B, Target::HL),
            0x47 => Instruction::LD(Target::B, Target::A),
            0x48 => Instruction::LD(Target::C, Target::B),
            0x49 => Instruction::LD(Target::C, Target::C),
            0x4A => Instruction::LD(Target::C, Target::D),
            0x4B => Instruction::LD(Target::C, Target::E),
            0x4C => Instruction::LD(Target::C, Target::H),
            0x4D => Instruction::LD(Target::C, Target::L),
            0x4E => Instruction::LD(Target::C, Target::HL),
            0x4F => Instruction::LD(Target::C, Target::A),
            0x50 => Instruction::LD(Target::D, Target::B),
            0x51 => Instruction::LD(Target::D, Target::C),
            0x52 => Instruction::LD(Target::D, Target::D),
            0x53 => Instruction::LD(Target::D, Target::E),
            0x54 => Instruction::LD(Target::D, Target::H),
            0x55 => Instruction::LD(Target::D, Target::L),
            0x56 => Instruction::LD(Target::D, Target::HL),
            0x57 => Instruction::LD(Target::D, Target::A),
            0x58 => Instruction::LD(Target::E, Target::B),
            0x59 => Instruction::LD(Target::E, Target::C),
            0x5A => Instruction::LD(Target::E, Target::D),
            0x5B => Instruction::LD(Target::E, Target::E),
            0x5C => Instruction::LD(Target::E, Target::H),
            0x5D => Instruction::LD(Target::E, Target::L),
            0x5E => Instruction::LD(Target::E, Target::HL),
            0x5F => Instruction::LD(Target::E, Target::A),
            0x60 => Instruction::LD(Target::H, Target::B),
            0x61 => Instruction::LD(Target::H, Target::C),
            0x62 => Instruction::LD(Target::H, Target::D),
            0x63 => Instruction::LD(Target::H, Target::E),
            0x64 => Instruction::LD(Target::H, Target::H),
            0x65 => Instruction::LD(Target::H, Target::L),
            0x66 => Instruction::LD(Target::H, Target::HL),
            0x67 => Instruction::LD(Target::H, Target::A),
            0x68 => Instruction::LD(Target::L, Target::B),
            0x69 => Instruction::LD(Target::L, Target::C),
            0x6A => Instruction::LD(Target::L, Target::D),
            0x6B => Instruction::LD(Target::L, Target::E),
            0x6C => Instruction::LD(Target::L, Target::H),
            0x6D => Instruction::LD(Target::L, Target::L),
            0x6E => Instruction::LD(Target::L, Target::HL),
            0x6F => Instruction::LD(Target::L, Target::A),
            0x70 => Instruction::LD(Target::HL, Target::B),
            0x71 => Instruction::LD(Target::HL, Target::C),
            0x72 => Instruction::LD(Target::HL, Target::D),
            0x73 => Instruction::LD(Target::HL, Target::E),
            0x74 => Instruction::LD(Target::HL, Target::H),
            0x75 => Instruction::LD(Target::HL, Target::L),
            0x76 => Instruction::HALT,
            0x77 => Instruction::LD(Target::HL, Target::A),
            0x78 => Instruction::LD(Target::A, Target::B),
            0x79 => Instruction::LD(Target::A, Target::C),
            0x7A => Instruction::LD(Target::A, Target::D),
            0x7B => Instruction::LD(Target::A, Target::E),
            0x7C => Instruction::LD(Target::A, Target::H),
            0x7D => Instruction::LD(Target::A, Target::L),
            0x7E => Instruction::LD(Target::A, Target::HL),
            0x7F => Instruction::LD(Target::A, Target::A),
            0x80 => Instruction::ADD(Target::B),
            0x81 => Instruction::ADD(Target::C),
            0x82 => Instruction::ADD(Target::D),
            0x83 => Instruction::ADD(Target::E),
            0x84 => Instruction::ADD(Target::H),
            0x85 => Instruction::ADD(Target::L),
            0x86 => Instruction::ADD(Target::HL),
            0x87 => Instruction::ADD(Target::A),
            0x88 => Instruction::ADC(Target::B),
            0x89 => Instruction::ADC(Target::C),
            0x8A => Instruction::ADC(Target::D),
            0x8B => Instruction::ADC(Target::E),
            0x8C => Instruction::ADC(Target::H),
            0x8D => Instruction::ADC(Target::L),
            0x8E => Instruction::ADC(Target::HL),
            0x8F => Instruction::ADC(Target::A),
            0x90 => Instruction::SUB(Target::B),
            0xA1 => Instruction::AND(Target::C),
            0xA7 => Instruction::AND(Target::A),
            0xA9 => Instruction::XOR(Target::C),
            0xAF => Instruction::XOR(Target::A),
            0xB0 => Instruction::OR(Target::B),
            0xB1 => Instruction::OR(Target::C),
            0xBE => Instruction::CP(Target::HL),
            0xC0 => Instruction::RET(JumpFlag::NZ),
            0xC1 => Instruction::POP(Target::BC),
            0xC3 => Instruction::JP(JumpFlag::A),
            0xC9 => Instruction::RET(JumpFlag::A),
            0xC5 => Instruction::PUSH(Target::BC),
            0xCA => Instruction::JP(JumpFlag::Z),
            0xC8 => Instruction::RET(JumpFlag::Z),
            0xCD => Instruction::CALL(JumpFlag::A),
            0xD1 => Instruction::POP(Target::DE),
            0xD5 => Instruction::PUSH(Target::DE),
            0xD9 => Instruction::RETI,
            0xE0 => Instruction::LD(Target::FFIMM8, Target::A),
            0xE1 => Instruction::POP(Target::HL),
            0xE2 => Instruction::LD(Target::FFC, Target::A),
            0xE5 => Instruction::PUSH(Target::HL),
            0xE6 => Instruction::AND(Target::IMM8),
            0xE9 => Instruction::JP(JumpFlag::AtHL),
            0xEA => Instruction::LD(Target::IMM16, Target::A),
            0xEF => Instruction::RST(0x28),
            0xF0 => Instruction::LD(Target::A, Target::FFIMM8),
            0xF1 => Instruction::POP(Target::AF),
            0xF3 => Instruction::DI,
            0xF5 => Instruction::PUSH(Target::AF),
            0xFA => Instruction::LD(Target::A, Target::IMM16),
            0xFB => Instruction::EI,
            0xFE => Instruction::CP(Target::IMM8),
            _ => panic!("Unrecognized opcode: {:#X}", opcode)
        }
    }

    pub fn decode_cb(opcode: u8) -> Instruction {
        match opcode {
            0x11 => Instruction::RL(Target::C),
            0x37 => Instruction::SWAP(Target::A),
            0x7C => Instruction::BIT(7, Target::H),
            0x87 => Instruction::RES(0, Target::A),
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
    fn rlca() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x85;
        cpu.execute(&mut mmu, Instruction::RLCA);
        assert_eq!(cpu.reg.a, 0x0A);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
    }

    #[test]
    fn rrca() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x3B;
        cpu.execute(&mut mmu, Instruction::RRCA);
        assert_eq!(cpu.reg.a, 0x9D);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
    }

    #[test]
    fn rra() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x81;
        cpu.reg.set_flag(Flag::C, false);
        cpu.execute(&mut mmu, Instruction::RRA);
        assert_eq!(cpu.reg.a, 0x40);
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

    #[test]
    fn adc() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0xE1;
        cpu.reg.b = 0x0F;
        cpu.reg.c = 0x3B;
        cpu.reg.d = 0x1E;
        cpu.reg.set_flag(Flag::C, true);
        cpu.execute(&mut mmu, Instruction::ADC(Target::B));
        assert_eq!(cpu.reg.a, 0xF1);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::H), true);

        cpu.reg.a = 0xE1;
        cpu.execute(&mut mmu, Instruction::ADC(Target::C));
        assert_eq!(cpu.reg.a, 0x1C);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
        assert_eq!(cpu.reg.get_flag(Flag::H), false);

        cpu.reg.a = 0xE1;
        cpu.execute(&mut mmu, Instruction::ADC(Target::D));
        assert_eq!(cpu.reg.a, 0x00);
        assert_eq!(cpu.reg.get_flag(Flag::Z), true);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
        assert_eq!(cpu.reg.get_flag(Flag::H), true);
    }

    #[test]
    fn add_hl() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.set_hl(0x8A23);
        cpu.reg.set_de(0x0605);
        cpu.execute(&mut mmu, Instruction::ADDHL(Target::DE));
        assert_eq!(cpu.reg.hl(), 0x9028);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::H), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);

        cpu.reg.set_hl(0x8A23);
        cpu.reg.set_de(0x8A23);
        cpu.execute(&mut mmu, Instruction::ADDHL(Target::DE));
        assert_eq!(cpu.reg.hl(), 0x1446);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::H), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
    }

    #[test]
    fn sub() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x3E;
        cpu.reg.b = 0x3E;
        cpu.reg.c = 0x0F;
        cpu.reg.d = 0x40;
        cpu.execute(&mut mmu, Instruction::SUB(Target::B));
        assert_eq!(cpu.reg.a, 0x0);
        assert_eq!(cpu.reg.get_flag(Flag::Z), true);
        assert_eq!(cpu.reg.get_flag(Flag::N), true);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
        assert_eq!(cpu.reg.get_flag(Flag::H), false);
    }

    #[test]
    fn cp() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.set_hl(0x3000);
        cpu.reg.a = 0x3A;
        mmu.wb(cpu.reg.hl(), 0x40);
        cpu.execute(&mut mmu, Instruction::CP(Target::HL));
        assert_eq!(cpu.reg.a, 0x3A);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::N), true);
        assert_eq!(cpu.reg.get_flag(Flag::H), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), true);
    }

    #[test]
    fn swap() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x0F;
        cpu.execute(&mut mmu, Instruction::SWAP(Target::A));
        assert_eq!(cpu.reg.a, 0xF0);
        assert_eq!(cpu.reg.get_flag(Flag::Z), false);
        assert_eq!(cpu.reg.get_flag(Flag::N), false);
        assert_eq!(cpu.reg.get_flag(Flag::H), false);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);
    }

    #[test]
    fn daa() {
        let mut cpu = CPU::new();
        let mut mmu = MMU::new();
        cpu.reg.a = 0x45;
        cpu.reg.b = 0x38;
        cpu.execute(&mut mmu, Instruction::ADD(Target::B));
        assert_eq!(cpu.reg.a, 0x7D);
        cpu.execute(&mut mmu, Instruction::DAA);
        assert_eq!(cpu.reg.a, 0x83);
        assert_eq!(cpu.reg.get_flag(Flag::C), false);

        cpu.execute(&mut mmu, Instruction::SUB(Target::B));
        assert_eq!(cpu.reg.a, 0x4B);
        cpu.execute(&mut mmu, Instruction::DAA);
        assert_eq!(cpu.reg.a, 0x45);
    }
}
