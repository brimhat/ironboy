use crate::mmu::MMU;
use crate::timer::Timer;
use crate::registers::{Registers, Flag};
use std::rc::Rc;
use std::cell::RefCell;

pub const CLOCKS: [u8; 256] = [
    1,3,2,2,1,1,2,1,5,2,2,2,1,1,2,1,
    0,3,2,2,1,1,2,1,3,2,2,2,1,1,2,1,
    2,3,2,2,1,1,2,1,2,2,2,2,1,1,2,1,
    2,3,2,2,3,3,3,1,2,2,2,2,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    2,2,2,2,2,2,0,2,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    1,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,
    2,3,3,4,3,4,2,4,2,4,3,0,3,6,2,4,
    2,3,3,0,3,4,2,4,2,4,3,0,3,0,2,4,
    3,3,2,0,0,4,2,4,4,1,4,0,0,0,2,4,
    3,3,2,1,0,4,2,4,3,2,4,1,0,0,2,4,
];

pub const CB_CLOCKS: [u8; 256] = [
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,3,2,2,2,2,2,2,2,3,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
    2,2,2,2,2,2,4,2,2,2,2,2,2,2,4,2,
];

pub struct CPU {
    pub reg: Registers,
    ime: bool,
    halt: bool,
    pub(crate) timer: Rc<RefCell<Timer>>,
    last_instr: Instruction,
}

impl CPU {
    pub fn new(timer: Rc<RefCell<Timer>>) -> CPU {
        CPU {
            reg: Registers::new(),
            ime: false,
            halt: false,
            timer,
            last_instr: Instruction::NULL,
        }
    }

    pub fn execute(&mut self, mmu: &mut MMU, instr: Instruction) {
        match instr {
            Instruction::LD(t1, t2) => {
                match (t1, t2) {
                    (Target::SP, Target::IMM16) => self.reg.sp = self.get_imm16(mmu),
                    (Target::BC, Target::IMM16) => {
                        let imm16 = self.get_imm16(mmu);
                        self.reg.set_bc(imm16)
                    },
                    (Target::DE, Target::IMM16) => {
                        let imm16 = self.get_imm16(mmu);
                        self.reg.set_de(imm16)
                    },
                    (Target::HL, Target::IMM16) => {
                        let imm16 = self.get_imm16(mmu);
                        self.reg.set_hl(imm16)
                    },
                    (Target::IMM16, Target::SP) => {
                        let lo = (self.reg.sp & 0x00FF) as u8;
                        let hi = ((self.reg.sp & 0xFF00) >> 8) as u8;
                        let nn = self.get_imm16(mmu);
                        mmu.wb(nn, lo);
                        mmu.wb(nn + 1, hi);
                    },
                    (Target::HLI, Target::A) => mmu.wb(self.reg.hli(), self.reg.a),
                    (Target::HLD, Target::A) => mmu.wb(self.reg.hld(), self.reg.a),
                    (Target::A, Target::HLI) => self.reg.a = mmu.rb(self.reg.hli()),
                    (Target::A, Target::HLD) => self.reg.a = mmu.rb(self.reg.hld()),
                    (Target::A, Target::HL) => self.reg.a = mmu.rb(self.reg.hl()),
                    (Target::B, Target::HL) => self.reg.b = mmu.rb(self.reg.hl()),
                    (Target::C, Target::HL) => self.reg.c = mmu.rb(self.reg.hl()),
                    (Target::D, Target::HL) => self.reg.d = mmu.rb(self.reg.hl()),
                    (Target::E, Target::HL) => self.reg.e = mmu.rb(self.reg.hl()),
                    (Target::H, Target::HL) => self.reg.h = mmu.rb(self.reg.hl()),
                    (Target::L, Target::HL) => self.reg.l = mmu.rb(self.reg.hl()),
                    (Target::BC, Target::A) => mmu.wb(self.reg.bc(), self.reg.a),
                    (Target::DE, Target::A) => mmu.wb(self.reg.de(), self.reg.a),
                    (Target::HL, Target::A) => mmu.wb(self.reg.hl(), self.reg.a),
                    (Target::HL, Target::B) => mmu.wb(self.reg.hl(), self.reg.b),
                    (Target::HL, Target::C) => mmu.wb(self.reg.hl(), self.reg.c),
                    (Target::HL, Target::D) => mmu.wb(self.reg.hl(), self.reg.d),
                    (Target::HL, Target::E) => mmu.wb(self.reg.hl(), self.reg.e),
                    (Target::HL, Target::H) => mmu.wb(self.reg.hl(), self.reg.h),
                    (Target::HL, Target::L) => mmu.wb(self.reg.hl(), self.reg.l),
                    (Target::A, Target::BC) => self.reg.a = mmu.rb(self.reg.bc()),
                    (Target::A, Target::DE) => self.reg.a = mmu.rb(self.reg.de()),
                    (Target::A, Target::IMM8) => self.reg.a = self.get_imm8(mmu),
                    (Target::B, Target::IMM8) => self.reg.b = self.get_imm8(mmu),
                    (Target::C, Target::IMM8) => self.reg.c = self.get_imm8(mmu),
                    (Target::D, Target::IMM8) => self.reg.d = self.get_imm8(mmu),
                    (Target::E, Target::IMM8) => self.reg.e = self.get_imm8(mmu),
                    (Target::H, Target::IMM8) => self.reg.h = self.get_imm8(mmu),
                    (Target::L, Target::IMM8) => self.reg.l = self.get_imm8(mmu),
                    (Target::HL, Target::IMM8) => mmu.wb(self.reg.hl(), self.get_imm8(mmu)),
                    (Target::B, Target::A) => self.reg.b = self.reg.a,
                    (Target::B, Target::B) => self.reg.b = self.reg.b,
                    (Target::B, Target::C) => self.reg.b = self.reg.c,
                    (Target::B, Target::D) => self.reg.b = self.reg.d,
                    (Target::B, Target::E) => self.reg.b = self.reg.e,
                    (Target::B, Target::H) => self.reg.b = self.reg.h,
                    (Target::B, Target::L) => self.reg.b = self.reg.l,
                    (Target::C, Target::A) => self.reg.c = self.reg.a,
                    (Target::C, Target::B) => self.reg.c = self.reg.b,
                    (Target::C, Target::C) => self.reg.c = self.reg.c,
                    (Target::C, Target::D) => self.reg.c = self.reg.d,
                    (Target::C, Target::E) => self.reg.c = self.reg.e,
                    (Target::C, Target::H) => self.reg.c = self.reg.h,
                    (Target::C, Target::L) => self.reg.c = self.reg.l,
                    (Target::D, Target::A) => self.reg.d = self.reg.a,
                    (Target::D, Target::B) => self.reg.d = self.reg.b,
                    (Target::D, Target::C) => self.reg.d = self.reg.c,
                    (Target::D, Target::D) => self.reg.d = self.reg.d,
                    (Target::D, Target::E) => self.reg.d = self.reg.e,
                    (Target::D, Target::H) => self.reg.d = self.reg.h,
                    (Target::D, Target::L) => self.reg.d = self.reg.l,
                    (Target::E, Target::A) => self.reg.e = self.reg.a,
                    (Target::E, Target::B) => self.reg.e = self.reg.b,
                    (Target::E, Target::C) => self.reg.e = self.reg.c,
                    (Target::E, Target::D) => self.reg.e = self.reg.d,
                    (Target::E, Target::E) => self.reg.e = self.reg.e,
                    (Target::E, Target::H) => self.reg.e = self.reg.h,
                    (Target::E, Target::L) => self.reg.e = self.reg.l,
                    (Target::H, Target::A) => self.reg.h = self.reg.a,
                    (Target::H, Target::B) => self.reg.h = self.reg.b,
                    (Target::H, Target::C) => self.reg.h = self.reg.c,
                    (Target::H, Target::D) => self.reg.h = self.reg.d,
                    (Target::H, Target::E) => self.reg.h = self.reg.e,
                    (Target::H, Target::H) => self.reg.h = self.reg.h,
                    (Target::H, Target::L) => self.reg.h = self.reg.l,
                    (Target::L, Target::A) => self.reg.l = self.reg.a,
                    (Target::L, Target::B) => self.reg.l = self.reg.b,
                    (Target::L, Target::C) => self.reg.l = self.reg.c,
                    (Target::L, Target::D) => self.reg.l = self.reg.d,
                    (Target::L, Target::E) => self.reg.l = self.reg.e,
                    (Target::L, Target::H) => self.reg.l = self.reg.h,
                    (Target::L, Target::L) => self.reg.l = self.reg.l,
                    (Target::A, Target::A) => self.reg.a = self.reg.a,
                    (Target::A, Target::B) => self.reg.a = self.reg.b,
                    (Target::A, Target::C) => self.reg.a = self.reg.c,
                    (Target::A, Target::D) => self.reg.a = self.reg.d,
                    (Target::A, Target::E) => self.reg.a = self.reg.e,
                    (Target::A, Target::H) => self.reg.a = self.reg.h,
                    (Target::A, Target::L) => self.reg.a = self.reg.l,
                    (Target::FFIMM8, Target::A) => {
                        let address: u16 = 0xFF00 | (self.get_imm8(mmu) as u16);
                        mmu.wb(address, self.reg.a);
                    },
                    (Target::FFC, Target::A) => {
                        let address: u16 = 0xFF00 | (self.reg.c as u16);
                        mmu.wb(address, self.reg.a);
                    },
                    (Target::A, Target::FFIMM8) => {
                        let v =  mmu.rb(0xFF00 | (self.get_imm8(mmu) as u16));
                        self.reg.a = v;
                    },
                    (Target::A, Target::FFC) => {
                        let v = mmu.rb(0xFF00 | self.reg.c as u16);
                        self.reg.a = v;
                    },
                    (Target::IMM16, Target::A) => mmu.wb(self.get_imm16(mmu), self.reg.a),
                    (Target::A, Target::IMM16) => self.reg.a = mmu.rb(self.get_imm16(mmu)),
                    (Target::HL, Target::SP) => {
                        let imm8 = self.get_imm8(mmu) as i8 as i16 as u16;
                        let v = self.reg.sp.wrapping_add(imm8);
                        let hc = (self.reg.sp & 0xF) + (imm8 & 0xF) > 0xF;
                        let c = (self.reg.sp & 0xFF) + (imm8 & 0xFF) > 0xFF;
                        self.reg.set_flag(Flag::Z, false);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::C, c);
                        self.reg.set_flag(Flag::H, hc);
                        self.reg.set_hl(v);
                    },
                    (Target::SP, Target::HL) => self.reg.sp = self.reg.hl(),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::INC(t) => {
                match t {
                    Target::A => {
                        let a = self.reg.a.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, a == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.a & 0x0F) == 0x0F);
                        self.reg.a = a;
                    },
                    Target::B => {
                        let b = self.reg.b.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, b == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.b & 0x0F) == 0x0F);
                        self.reg.b = b;
                    },
                    Target::C => {
                        let c = self.reg.c.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, c == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.c & 0x0F) == 0x0F);
                        self.reg.c = c;
                    },
                    Target::D => {
                        let d = self.reg.d.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, d == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.d & 0x0F) == 0x0F);
                        self.reg.d = d;
                    },
                    Target::E => {
                        let e = self.reg.e.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, e == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.e & 0x0F) == 0x0F);
                        self.reg.e = e;
                    },
                    Target::H => {
                        let h = self.reg.h.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, h == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.h & 0x0F) == 0x0F);
                        self.reg.h = h;
                    },
                    Target::L => {
                        let l = self.reg.l.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, l == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (self.reg.l & 0x0F) == 0x0F);
                        self.reg.l = l;
                    },
                    Target::BC => self.reg.set_bc(self.reg.bc().wrapping_add(1)),
                    Target::DE => self.reg.set_de(self.reg.de().wrapping_add(1)),
                    Target::HL => self.reg.set_hl(self.reg.hl().wrapping_add(1)),
                    Target::SP => self.reg.sp = self.reg.sp.wrapping_add(1),
                    Target::AtHL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let new_hl = at_hl.wrapping_add(1);
                        self.reg.set_flag(Flag::Z, new_hl == 0);
                        self.reg.set_flag(Flag::N, false);
                        self.reg.set_flag(Flag::H, (at_hl & 0x0F) == 0x0F);
                        mmu.wb(self.reg.hl(), new_hl);
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
                    },
                    Target::B => {
                        let b = self.reg.b.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, b == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.b & 0x0F) == 0);
                        self.reg.b = b;
                    },
                    Target::C => {
                        let c = self.reg.c.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, c == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.c & 0x0F) == 0);
                        self.reg.c = c;
                    },
                    Target::D => {
                        let d = self.reg.d.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, d == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.d & 0x0F) == 0);
                        self.reg.d = d;
                    },
                    Target::E => {
                        let e = self.reg.e.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, e == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.e & 0x0F) == 0);
                        self.reg.e = e;
                    },
                    Target::H => {
                        let h = self.reg.h.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, h == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.h & 0x0F) == 0);
                        self.reg.h = h;
                    },
                    Target::L => {
                        let l = self.reg.l.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, l == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (self.reg.l & 0x0F) == 0);
                        self.reg.l = l;
                    },
                    Target::BC => self.reg.set_bc(self.reg.bc().wrapping_sub(1)),
                    Target::DE => self.reg.set_de(self.reg.de().wrapping_sub(1)),
                    Target::HL => self.reg.set_hl(self.reg.hl().wrapping_sub(1)),
                    Target::SP => self.reg.sp = self.reg.sp.wrapping_sub(1),
                    Target::AtHL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let new_hl = at_hl.wrapping_sub(1);
                        self.reg.set_flag(Flag::Z, new_hl == 0);
                        self.reg.set_flag(Flag::N, true);
                        self.reg.set_flag(Flag::H, (at_hl & 0x0F) == 0);
                        mmu.wb(self.reg.hl(), new_hl);
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::XOR(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.a ^= operand;
                self.reg.set_flag(Flag::Z, self.reg.a == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, false);
            },
            Instruction::OR(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.a |= operand;
                self.reg.set_flag(Flag::Z, self.reg.a == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::C, false);
                self.reg.set_flag(Flag::H, false);
            },
            Instruction::AND(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.a &= operand;
                self.reg.set_flag(Flag::Z, self.reg.a == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, true);
                self.reg.set_flag(Flag::C, false);
            },
            Instruction::ADD(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let (v, c) = self.reg.a.overflowing_add(operand);
                let hc = (self.reg.a & 0x0F) + (operand & 0x0F) > 0x0F;
                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.set_flag(Flag::H, hc);
                self.reg.a = v;
            },
            Instruction::ADDSP => {
                let imm8 = self.get_imm8(mmu) as i8 as i16 as u16;
                let v = self.reg.sp.wrapping_add(imm8);
                let hc = (self.reg.sp & 0xF) + (imm8 & 0xF) > 0xF;
                let c = (self.reg.sp & 0xFF) + (imm8 & 0xFF) > 0xFF;
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.set_flag(Flag::H, hc);
                self.reg.sp = v;
            },
            Instruction::ADC(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let cf = self.reg.get_flag(Flag::C) as u8;
                let (_v, c1) = self.reg.a.overflowing_add(operand);
                let (v, c2) = _v.overflowing_add(cf);
                let hc = (self.reg.a & 0xF) + (operand & 0xF) + cf > 0xF;
                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::C, c1 || c2);
                self.reg.set_flag(Flag::H, hc);
                self.reg.a = v;
            },
            Instruction::ADDHL(t) => {
                let operand = match t {
                    Target::BC => self.reg.bc(),
                    Target::DE => self.reg.de(),
                    Target::HL => self.reg.hl(),
                    Target::SP => self.reg.sp,
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let (v, c) = self.reg.hl().overflowing_add(operand);
                let hc = (self.reg.hl() & 0x0FFF) + (operand & 0x0FFF) > 0x0FFF;
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::C, c);
                self.reg.set_flag(Flag::H, hc);
                self.reg.set_hl(v);
            },
            Instruction::SUB(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let (v, c) = self.reg.a.overflowing_sub(operand);
                let hc = (self.reg.a & 0x0F) < (operand & 0x0F);
                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, true);
                self.reg.set_flag(Flag::H, hc);
                self.reg.set_flag(Flag::C, c);
                self.reg.a = v;
            },
            Instruction::SBC(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let cf = self.reg.get_flag(Flag::C) as u8;
                let (_v, c1) = self.reg.a.overflowing_sub(operand);
                let (v, c2) = _v.overflowing_sub(cf);
                let hc = (self.reg.a & 0x0F) < (operand & 0x0F) + cf;
                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, true);
                self.reg.set_flag(Flag::H, hc);
                self.reg.set_flag(Flag::C, c1 || c2);
                self.reg.a = v;
            },
            Instruction::CP(t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    Target::IMM8 => self.get_imm8(mmu),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let (v, c) = self.reg.a.overflowing_sub(operand);
                let hc = (self.reg.a & 0x0F) < (operand & 0x0F);
                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, true);
                self.reg.set_flag(Flag::H, hc);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::BIT(i, t) => {
                let operand = match t {
                    Target::A  => self.reg.a,
                    Target::B  => self.reg.b,
                    Target::C  => self.reg.c,
                    Target::D  => self.reg.d,
                    Target::E  => self.reg.e,
                    Target::H  => self.reg.h,
                    Target::L  => self.reg.l,
                    Target::HL => mmu.rb(self.reg.hl()),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, operand & (1 << i) == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, true);
            },
            Instruction::RES(i , t) => {
                match t {
                    Target::A  => self.reg.a &= !(1 << i),
                    Target::B  => self.reg.b &= !(1 << i),
                    Target::C  => self.reg.c &= !(1 << i),
                    Target::D  => self.reg.d &= !(1 << i),
                    Target::E  => self.reg.e &= !(1 << i),
                    Target::H  => self.reg.h &= !(1 << i),
                    Target::L  => self.reg.l &= !(1 << i),
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        mmu.wb(self.reg.hl(), at_hl & !(1 << i));
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::SET(i, t) => {
                match t {
                    Target::A  => self.reg.a |= (1 << i),
                    Target::B  => self.reg.b |= (1 << i),
                    Target::C  => self.reg.c |= (1 << i),
                    Target::D  => self.reg.d |= (1 << i),
                    Target::E  => self.reg.e |= (1 << i),
                    Target::H  => self.reg.h |= (1 << i),
                    Target::L  => self.reg.l |= (1 << i),
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        mmu.wb(self.reg.hl(), at_hl | (1 << i));
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::JR(f) => {
                let should_jump = match f {
                    JumpFlag::NZ => !self.reg.get_flag(Flag::Z),
                    JumpFlag::NC => !self.reg.get_flag(Flag::C),
                    JumpFlag::Z => self.reg.get_flag(Flag::Z),
                    JumpFlag::C => self.reg.get_flag(Flag::C),
                    JumpFlag::A => true,
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let jump = self.get_imm8(mmu) as i8;
                if should_jump {
                    self.reg.pc = self.reg.pc.wrapping_add(jump as u16);
                }
            },
            Instruction::JP(f) => {
                let mut jump = self.get_imm16(mmu);
                let should_jump = match f {
                    JumpFlag::NZ => !self.reg.get_flag(Flag::Z),
                    JumpFlag::NC => !self.reg.get_flag(Flag::C),
                    JumpFlag::Z => self.reg.get_flag(Flag::Z),
                    JumpFlag::C => self.reg.get_flag(Flag::C),
                    JumpFlag::A => true,
                    JumpFlag::AtHL => {
                        jump = self.reg.hl();
                        true
                    },
                };

                if should_jump {
                    self.reg.pc = jump;
                }
            },
            Instruction::CALL(f) => {
                let should_jump = match f {
                    JumpFlag::NZ => !self.reg.get_flag(Flag::Z),
                    JumpFlag::NC => !self.reg.get_flag(Flag::C),
                    JumpFlag::Z => self.reg.get_flag(Flag::Z),
                    JumpFlag::C => self.reg.get_flag(Flag::C),
                    JumpFlag::A => true,
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                let jump = self.get_imm16(mmu);
                if should_jump {
                    self.push(mmu, self.reg.pc);
                    self.reg.pc = jump
                }
            },
            Instruction::RET(f) => {
                let should_jump = match f {
                    JumpFlag::NZ => !self.reg.get_flag(Flag::Z),
                    JumpFlag::NC => !self.reg.get_flag(Flag::C),
                    JumpFlag::Z => self.reg.get_flag(Flag::Z),
                    JumpFlag::C => self.reg.get_flag(Flag::C),
                    JumpFlag::A => true,
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                if should_jump {
                    self.reg.pc = self.pop(mmu);
                }
            },
            Instruction::PUSH(t) => {
                match t {
                    Target::AF => self.push(mmu, self.reg.af()),
                    Target::BC => self.push(mmu, self.reg.bc()),
                    Target::DE => self.push(mmu, self.reg.de()),
                    Target::HL => self.push(mmu, self.reg.hl()),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::POP(t) => {
                let pop = self.pop(mmu);
                match t {
                    Target::AF => self.reg.set_af(pop),
                    Target::BC => self.reg.set_bc(pop),
                    Target::DE => self.reg.set_de(pop),
                    Target::HL => self.reg.set_hl(pop),
                    _ => panic!("Unrecognized instr: {:?}", instr)
                }
            },
            Instruction::RST(t) => {
                self.push(mmu, self.reg.pc);
                self.reg.pc = t as u16;
            },
            Instruction::RL(t) => {
                let cf = self.reg.get_flag(Flag::C) as u8;
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x80) != 0;
                        self.reg.a = (self.reg.a << 1) + cf;
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x80) != 0;
                        self.reg.b = (self.reg.b << 1) + cf;
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x80) != 0;
                        self.reg.c = (self.reg.c << 1) + cf;
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x80) != 0;
                        self.reg.d = (self.reg.d << 1) + cf;
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x80) != 0;
                        self.reg.e = (self.reg.e << 1) + cf;
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x80) != 0;
                        self.reg.h = (self.reg.h << 1) + cf;
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x80) != 0;
                        self.reg.l = (self.reg.l << 1) + cf;
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x80) != 0;
                        let new_hl = (at_hl << 1) + cf;
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    }
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RLC(t) => {
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x80) != 0;
                        self.reg.a = (self.reg.a << 1) + c as u8;
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x80) != 0;
                        self.reg.b = (self.reg.b << 1) + c as u8;
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x80) != 0;
                        self.reg.c = (self.reg.c << 1) + c as u8;
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x80) != 0;
                        self.reg.d = (self.reg.d << 1) + c as u8;
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x80) != 0;
                        self.reg.e = (self.reg.e << 1) + c as u8;
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x80) != 0;
                        self.reg.h = (self.reg.h << 1) + c as u8;
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x80) != 0;
                        self.reg.l = (self.reg.l << 1) + c as u8;
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x80) != 0;
                        let new_hl = (at_hl << 1) + c as u8;
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RRC(t) => {
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x01) != 0;
                        self.reg.a = self.reg.a.rotate_right(1);
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x01) != 0;
                        self.reg.b = self.reg.b.rotate_right(1);
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x01) != 0;
                        self.reg.c = self.reg.c.rotate_right(1);
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x01) != 0;
                        self.reg.d = self.reg.d.rotate_right(1);
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x01) != 0;
                        self.reg.e = self.reg.e.rotate_right(1);
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x01) != 0;
                        self.reg.h = self.reg.h.rotate_right(1);
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x01) != 0;
                        self.reg.l = self.reg.l.rotate_right(1);
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x01) != 0;
                        let new_hl = at_hl.rotate_right(1);
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RR(t) => {
                let cf = self.reg.get_flag(Flag::C);
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x01) != 0;
                        self.reg.a = ((cf as u8) << 7) | (self.reg.a >> 1);
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x01) != 0;
                        self.reg.b = ((cf as u8) << 7) | (self.reg.b >> 1);
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x01) != 0;
                        self.reg.c = ((cf as u8) << 7) | (self.reg.c >> 1);
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x01) != 0;
                        self.reg.d = ((cf as u8) << 7) | (self.reg.d >> 1);
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x01) != 0;
                        self.reg.e = ((cf as u8) << 7) | (self.reg.e >> 1);
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x01) != 0;
                        self.reg.h = ((cf as u8) << 7) | (self.reg.h >> 1);
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x01) != 0;
                        self.reg.l = ((cf as u8) << 7) | (self.reg.l >> 1);
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x01) != 0;
                        let new_hl = ((cf as u8) << 7) | (at_hl >> 1);
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);;
            },
            Instruction::SLA(t) => {
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x80) != 0;
                        self.reg.a = self.reg.a << 1;
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x80) != 0;
                        self.reg.b = self.reg.b << 1;
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x80) != 0;
                        self.reg.c = self.reg.c << 1;
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x80) != 0;
                        self.reg.d = self.reg.d << 1;
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x80) != 0;
                        self.reg.e = self.reg.e << 1;
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x80) != 0;
                        self.reg.h = self.reg.h << 1;
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x80) != 0;
                        self.reg.l = self.reg.l << 1;
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x80) != 0;
                        let new_hl = (at_hl << 1);
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::SRA(t) => {
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x01) != 0;
                        self.reg.a = (self.reg.a & 0x80) | (self.reg.a >> 1);
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x01) != 0;
                        self.reg.b = (self.reg.b & 0x80) | (self.reg.b >> 1);
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x01) != 0;
                        self.reg.c = (self.reg.c & 0x80) | (self.reg.c >> 1);
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x01) != 0;
                        self.reg.d = (self.reg.d & 0x80) | (self.reg.d >> 1);
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x01) != 0;
                        self.reg.e = (self.reg.e & 0x80) | (self.reg.e >> 1);
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x01) != 0;
                        self.reg.h = (self.reg.h & 0x80) | (self.reg.h >> 1);
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x01) != 0;
                        self.reg.l = (self.reg.l & 0x80) | (self.reg.l >> 1);
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x01) != 0;
                        let new_hl = (at_hl & 0x80) | (at_hl >> 1);
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::SRL(t) => {
                let (v, c) = match t {
                    Target::A => {
                        let c = (self.reg.a & 0x01) != 0;
                        self.reg.a = self.reg.a >> 1;
                        (self.reg.a, c)
                    },
                    Target::B => {
                        let c = (self.reg.b & 0x01) != 0;
                        self.reg.b = self.reg.b >> 1;
                        (self.reg.b, c)
                    },
                    Target::C => {
                        let c = (self.reg.c & 0x01) != 0;
                        self.reg.c = self.reg.c >> 1;
                        (self.reg.c, c)
                    },
                    Target::D => {
                        let c = (self.reg.d & 0x01) != 0;
                        self.reg.d = self.reg.d >> 1;
                        (self.reg.d, c)
                    },
                    Target::E => {
                        let c = (self.reg.e & 0x01) != 0;
                        self.reg.e = self.reg.e >> 1;
                        (self.reg.e, c)
                    },
                    Target::H => {
                        let c = (self.reg.h & 0x01) != 0;
                        self.reg.h = self.reg.h >> 1;
                        (self.reg.h, c)
                    },
                    Target::L => {
                        let c = (self.reg.l & 0x01) != 0;
                        self.reg.l = self.reg.l >> 1;
                        (self.reg.l, c)
                    },
                    Target::HL => {
                        let at_hl = mmu.rb(self.reg.hl());
                        let c = (at_hl & 0x01) != 0;
                        let new_hl = (at_hl >> 1);
                        mmu.wb(self.reg.hl(), new_hl);
                        (new_hl, c)
                    },
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RLCA => {
                let c = (self.reg.a & 0x80) != 0;
                self.reg.a = (self.reg.a << 1) | c as u8;
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RLA => {
                let c = (self.reg.a & 0x80) != 0;
                self.reg.a = (self.reg.a << 1) | (self.reg.get_flag(Flag::C) as u8);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RRCA => {
                let c = (self.reg.a & 0x01) != 0;
                self.reg.a = ((c as u8) << 7) | (self.reg.a >> 1);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::RRA => {
                let c = (self.reg.a & 0x01) != 0;
                let c_flag = self.reg.get_flag(Flag::C) as u8;
                self.reg.a = (c_flag << 7) | (self.reg.a >> 1);
                self.reg.set_flag(Flag::Z, false);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, c);
            },
            Instruction::SWAP(t) => {
                let v = match t {
                    Target::A => {
                        let v = ((self.reg.a & 0x0F) << 4) | ((self.reg.a & 0xF0) >> 4);
                        self.reg.a = v;
                        v
                    },
                    Target::B => {
                        let v = ((self.reg.b & 0x0F) << 4) | ((self.reg.b & 0xF0) >> 4);
                        self.reg.b = v;
                        v
                    },
                    Target::C => {
                        let v = ((self.reg.c & 0x0F) << 4) | ((self.reg.c & 0xF0) >> 4);
                        self.reg.c = v;
                        v
                    },
                    Target::D => {
                        let v = ((self.reg.d & 0x0F) << 4) | ((self.reg.d & 0xF0) >> 4);
                        self.reg.d = v;
                        v
                    },
                    Target::E => {
                        let v = ((self.reg.e & 0x0F) << 4) | ((self.reg.e & 0xF0) >> 4);
                        self.reg.e = v;
                        v
                    },
                    Target::H => {
                        let v = ((self.reg.h & 0x0F) << 4) | ((self.reg.h & 0xF0) >> 4);
                        self.reg.h = v;
                        v
                    },
                    Target::L => {
                        let v = ((self.reg.l & 0x0F) << 4) | ((self.reg.l & 0xF0) >> 4);
                        self.reg.l = v;
                        v
                    },
                    Target::HL => {
                        let new_hl = mmu.rb(self.reg.hl());
                        let v = ((new_hl & 0x0F) << 4) | ((new_hl & 0xF0) >> 4);
                        mmu.wb(self.reg.hl(), v);
                        v
                    }
                    _ => panic!("Unrecognized instr: {:?}", instr)
                };

                self.reg.set_flag(Flag::Z, v == 0);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, false);
            },
            Instruction::CPL => {
                self.reg.a = !self.reg.a;
                self.reg.set_flag(Flag::N, true);
                self.reg.set_flag(Flag::H, true);
            },
            Instruction::SCF => {
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, true);
            },
            Instruction::CCF => {
                let c_flag = self.reg.get_flag(Flag::C);
                self.reg.set_flag(Flag::N, false);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, !c_flag);
            },
            Instruction::DAA => {
                let c_flag = self.reg.get_flag(Flag::C);
                let h_flag = self.reg.get_flag(Flag::H);
                let n_flag = self.reg.get_flag(Flag::N);

                let mut carry = false;
                let mut a = self.reg.a;

                if !n_flag {
                    if c_flag || a > 0x99 {
                        carry = true;
                        a = a.wrapping_add(0x60);
                    }
                    if h_flag || (a & 0x0F) > 0x09 {
                        a = a.wrapping_add(0x06);
                    }
                } else {
                    if c_flag {
                        carry = true;
                        a = a.wrapping_sub(0x60);
                    }
                    if h_flag {
                        a = a.wrapping_sub(0x06);
                    }
                }

                self.reg.a = a;
                self.reg.set_flag(Flag::Z, self.reg.a == 0);
                self.reg.set_flag(Flag::H, false);
                self.reg.set_flag(Flag::C, carry);
            },
            Instruction::DI => self.ime = false,
            Instruction::EI => self.ime = true,
            Instruction::HALT => self.halt = true,
            Instruction::RETI => {
                self.ime = true;
                self.reg.pc = self.pop(mmu);
            },
            Instruction::NOP => {},
            Instruction::STOP => panic!("STOP"),
            Instruction::NULL => panic!("Unused opcode"),
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u8 {
        let (instr, clocks) = if self.halt {
            (Instruction::HALT, 4)
        } else {
            self.fetch_instr(mmu)
        };

        self.timer.borrow_mut().tick_n(clocks);

        if self.interrupt_exists(mmu) {
            // effect of EI is delayed one instruction
            if self.last_instr == Instruction::EI {
                self.execute(mmu, instr);
            }
            self.handle_interrupt(mmu);
        } else if !self.halt {
            self.execute(mmu, instr);
        }

        self.last_instr = instr;
        clocks * 4
    }

    pub fn interrupt_exists(&self, mmu: &mut MMU) -> bool {
        let e_i = mmu.rb(0xFFFF);
        let i_f = mmu.rb(0xFF0F);
        let e_f = e_i & i_f;

        (self.halt || self.ime) && (e_f & 0b0001_1111) != 0
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
        mmu.wb(0xFF0F, i_f & !(1 << index));

        // because EI is delayed, we don't have to return to the
        // instruction that we would have skipped otherwise
        if self.last_instr == Instruction::EI {
            self.push(mmu, self.reg.pc);
        } else {
            self.push(mmu, self.reg.pc - 1);
        }
        self.reg.pc = match index {
            0 => 0x40, // VBlank
            1 => 0x48, // LCD Stat
            2 => 0x50, // Timer overflow
            3 => 0x58, // Serial link
            4 => 0x60, // Joypad press
            _ => panic!("unrecognized interrupt: {:#b}", e_f),
        };
    }

    pub fn fetch_instr(&mut self, mmu: &MMU) -> (Instruction, u8) {
        let byte = self.get_imm8(mmu);
        match byte {
            0xCB => {
                let cb_byte = self.get_imm8(mmu);
                let instr = Instruction::decode_cb(cb_byte);
                let clocks = CB_CLOCKS[cb_byte as usize];
                (instr, clocks)
            },
            _ => {
                let instr = Instruction::decode(byte);
                let clocks = CLOCKS[byte as usize];
                (instr, clocks)
            },
        }
    }

    pub fn get_imm16(&mut self, mmu: &MMU) -> u16 {
        let lo = mmu.rb(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        let hi = mmu.rb(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        ((hi as u16) << 8) | lo as u16
    }

    pub fn get_imm8(&mut self, mmu: &MMU) -> u8 {
        let imm8 = mmu.rb(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        imm8
    }

    pub fn push(&mut self, mmu: &mut MMU, value: u16) {
        self.reg.sp = self.reg.sp.wrapping_sub(1);
        mmu.wb(self.reg.sp, (value >> 8) as u8);
        self.reg.sp = self.reg.sp.wrapping_sub(1);
        mmu.wb(self.reg.sp, value as u8);
    }

    pub fn pop(&mut self, mmu: &MMU) -> u16 {
        let lo = mmu.rb(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);
        let hi = mmu.rb(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);
        ((hi as u16) << 8) | lo as u16
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    LD(Target, Target),
    XOR(Target),
    OR(Target),
    AND(Target),
    INC(Target),
    DEC(Target),
    BIT(u8, Target),
    RES(u8, Target),
    SET(u8, Target),
    SWAP(Target),
    JR(JumpFlag),
    JP(JumpFlag),
    CALL(JumpFlag),
    RET(JumpFlag),
    PUSH(Target),
    POP(Target),
    RST(u8),
    RL(Target),
    RLC(Target),
    RRC(Target),
    RR(Target),
    SLA(Target),
    SRA(Target),
    SRL(Target),
    RLA,
    RLCA,
    RRCA,
    RRA,
    CP(Target),
    SUB(Target),
    SBC(Target),
    ADD(Target),
    ADC(Target),
    ADDHL(Target),
    ADDSP,
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
    NULL, // 0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Target {
    A, AF, B, C, BC, D, E, DE, H, L, HL, HLI, HLD, SP, IMM8, IMM16, FFC, FFIMM8,
    AtHL // only used for INC (HL) and DEC (HL)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JumpFlag {
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
            0x25 => Instruction::DEC(Target::H),
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
            0x91 => Instruction::SUB(Target::C),
            0x92 => Instruction::SUB(Target::D),
            0x93 => Instruction::SUB(Target::E),
            0x94 => Instruction::SUB(Target::H),
            0x95 => Instruction::SUB(Target::L),
            0x96 => Instruction::SUB(Target::HL),
            0x97 => Instruction::SUB(Target::A),
            0x98 => Instruction::SBC(Target::B),
            0x99 => Instruction::SBC(Target::C),
            0x9A => Instruction::SBC(Target::D),
            0x9B => Instruction::SBC(Target::E),
            0x9C => Instruction::SBC(Target::H),
            0x9D => Instruction::SBC(Target::L),
            0x9E => Instruction::SBC(Target::HL),
            0x9F => Instruction::SBC(Target::A),
            0xA0 => Instruction::AND(Target::B),
            0xA1 => Instruction::AND(Target::C),
            0xA2 => Instruction::AND(Target::D),
            0xA3 => Instruction::AND(Target::E),
            0xA4 => Instruction::AND(Target::H),
            0xA5 => Instruction::AND(Target::L),
            0xA6 => Instruction::AND(Target::HL),
            0xA7 => Instruction::AND(Target::A),
            0xA8 => Instruction::XOR(Target::B),
            0xA9 => Instruction::XOR(Target::C),
            0xAA => Instruction::XOR(Target::D),
            0xAB => Instruction::XOR(Target::E),
            0xAC => Instruction::XOR(Target::H),
            0xAD => Instruction::XOR(Target::L),
            0xAE => Instruction::XOR(Target::HL),
            0xAF => Instruction::XOR(Target::A),
            0xB0 => Instruction::OR(Target::B),
            0xB1 => Instruction::OR(Target::C),
            0xB2 => Instruction::OR(Target::D),
            0xB3 => Instruction::OR(Target::E),
            0xB4 => Instruction::OR(Target::H),
            0xB5 => Instruction::OR(Target::L),
            0xB6 => Instruction::OR(Target::HL),
            0xB7 => Instruction::OR(Target::A),
            0xB8 => Instruction::CP(Target::B),
            0xB9 => Instruction::CP(Target::C),
            0xBA => Instruction::CP(Target::D),
            0xBB => Instruction::CP(Target::E),
            0xBC => Instruction::CP(Target::H),
            0xBD => Instruction::CP(Target::L),
            0xBE => Instruction::CP(Target::HL),
            0xBF => Instruction::CP(Target::A),
            0xC0 => Instruction::RET(JumpFlag::NZ),
            0xC1 => Instruction::POP(Target::BC),
            0xC2 => Instruction::JP(JumpFlag::NZ),
            0xC3 => Instruction::JP(JumpFlag::A),
            0xC4 => Instruction::CALL(JumpFlag::NZ),
            0xC5 => Instruction::PUSH(Target::BC),
            0xC6 => Instruction::ADD(Target::IMM8),
            0xC7 => Instruction::RST(0x00),
            0xC8 => Instruction::RET(JumpFlag::Z),
            0xC9 => Instruction::RET(JumpFlag::A),
            0xCA => Instruction::JP(JumpFlag::Z),
            0xCB => panic!("found cb in decode."),
            0xCC => Instruction::CALL(JumpFlag::Z),
            0xCD => Instruction::CALL(JumpFlag::A),
            0xCE => Instruction::ADC(Target::IMM8),
            0xCF => Instruction::RST(0x08),
            0xD0 => Instruction::RET(JumpFlag::NC),
            0xD1 => Instruction::POP(Target::DE),
            0xD2 => Instruction::JP(JumpFlag::NC),
            0xD3 => Instruction::NULL, // unused opcode
            0xD4 => Instruction::CALL(JumpFlag::NC),
            0xD5 => Instruction::PUSH(Target::DE),
            0xD6 => Instruction::SUB(Target::IMM8),
            0xD7 => Instruction::RST(0x10),
            0xD8 => Instruction::RET(JumpFlag::C),
            0xD9 => Instruction::RETI,
            0xDA => Instruction::JP(JumpFlag::C),
            0xDB => Instruction::NULL, // unused opcode
            0xDC => Instruction::CALL(JumpFlag::C),
            0xDD => Instruction::NULL, // unused opcode
            0xDE => Instruction::SBC(Target::IMM8),
            0xDF => Instruction::RST(0x18),
            0xE0 => Instruction::LD(Target::FFIMM8, Target::A),
            0xE1 => Instruction::POP(Target::HL),
            0xE2 => Instruction::LD(Target::FFC, Target::A),
            0xE3 => Instruction::NULL, // unused opcode
            0xE4 => Instruction::NULL, // unused opcode
            0xE5 => Instruction::PUSH(Target::HL),
            0xE6 => Instruction::AND(Target::IMM8),
            0xE7 => Instruction::RST(0x20),
            0xE8 => Instruction::ADDSP,
            0xE9 => Instruction::JP(JumpFlag::AtHL),
            0xEA => Instruction::LD(Target::IMM16, Target::A),
            0xEB => Instruction::NULL, // unused opcode
            0xEC => Instruction::NULL, // unused opcode
            0xED => Instruction::NULL, // unused opcode
            0xEE => Instruction::XOR(Target::IMM8),
            0xEF => Instruction::RST(0x28),
            0xF0 => Instruction::LD(Target::A, Target::FFIMM8),
            0xF1 => Instruction::POP(Target::AF),
            0xF2 => Instruction::LD(Target::A, Target::FFC),
            0xF3 => Instruction::DI,
            0xF4 => Instruction::NULL, // unused opcode
            0xF5 => Instruction::PUSH(Target::AF),
            0xF6 => Instruction::OR(Target::IMM8),
            0xF7 => Instruction::RST(0x30),
            0xF8 => Instruction::LD(Target::HL, Target::SP),
            0xF9 => Instruction::LD(Target::SP, Target::HL),
            0xFA => Instruction::LD(Target::A, Target::IMM16),
            0xFB => Instruction::EI,
            0xFC => Instruction::NULL, // unused opcode
            0xFD => Instruction::NULL, // unused opcode
            0xFE => Instruction::CP(Target::IMM8),
            0xFF => Instruction::RST(0x38),
        }
    }

    pub fn decode_cb(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::RLC(Target::B),
            0x01 => Instruction::RLC(Target::C),
            0x02 => Instruction::RLC(Target::D),
            0x03 => Instruction::RLC(Target::E),
            0x04 => Instruction::RLC(Target::H),
            0x05 => Instruction::RLC(Target::L),
            0x06 => Instruction::RLC(Target::HL),
            0x07 => Instruction::RLC(Target::A),
            0x08 => Instruction::RRC(Target::B),
            0x09 => Instruction::RRC(Target::C),
            0x0A => Instruction::RRC(Target::D),
            0x0B => Instruction::RRC(Target::E),
            0x0C => Instruction::RRC(Target::H),
            0x0D => Instruction::RRC(Target::L),
            0x0E => Instruction::RRC(Target::HL),
            0x0F => Instruction::RRC(Target::A),
            0x10 => Instruction::RL(Target::B),
            0x11 => Instruction::RL(Target::C),
            0x12 => Instruction::RL(Target::D),
            0x13 => Instruction::RL(Target::E),
            0x14 => Instruction::RL(Target::H),
            0x15 => Instruction::RL(Target::L),
            0x16 => Instruction::RL(Target::HL),
            0x17 => Instruction::RL(Target::A),
            0x18 => Instruction::RR(Target::B),
            0x19 => Instruction::RR(Target::C),
            0x1A => Instruction::RR(Target::D),
            0x1B => Instruction::RR(Target::E),
            0x1C => Instruction::RR(Target::H),
            0x1D => Instruction::RR(Target::L),
            0x1E => Instruction::RR(Target::HL),
            0x1F => Instruction::RR(Target::A),
            0x20 => Instruction::SLA(Target::B),
            0x21 => Instruction::SLA(Target::C),
            0x22 => Instruction::SLA(Target::D),
            0x23 => Instruction::SLA(Target::E),
            0x24 => Instruction::SLA(Target::H),
            0x25 => Instruction::SLA(Target::L),
            0x26 => Instruction::SLA(Target::HL),
            0x27 => Instruction::SLA(Target::A),
            0x28 => Instruction::SRA(Target::B),
            0x29 => Instruction::SRA(Target::C),
            0x2A => Instruction::SRA(Target::D),
            0x2B => Instruction::SRA(Target::E),
            0x2C => Instruction::SRA(Target::H),
            0x2D => Instruction::SRA(Target::L),
            0x2E => Instruction::SRA(Target::HL),
            0x2F => Instruction::SRA(Target::A),
            0x30 => Instruction::SWAP(Target::B),
            0x31 => Instruction::SWAP(Target::C),
            0x32 => Instruction::SWAP(Target::D),
            0x33 => Instruction::SWAP(Target::E),
            0x34 => Instruction::SWAP(Target::H),
            0x35 => Instruction::SWAP(Target::L),
            0x36 => Instruction::SWAP(Target::HL),
            0x37 => Instruction::SWAP(Target::A),
            0x38 => Instruction::SRL(Target::B),
            0x39 => Instruction::SRL(Target::C),
            0x3A => Instruction::SRL(Target::D),
            0x3B => Instruction::SRL(Target::E),
            0x3C => Instruction::SRL(Target::H),
            0x3D => Instruction::SRL(Target::L),
            0x3E => Instruction::SRL(Target::HL),
            0x3F => Instruction::SRL(Target::A),
            0x40 => Instruction::BIT(0, Target::B),
            0x41 => Instruction::BIT(0, Target::C),
            0x42 => Instruction::BIT(0, Target::D),
            0x43 => Instruction::BIT(0, Target::E),
            0x44 => Instruction::BIT(0, Target::H),
            0x45 => Instruction::BIT(0, Target::L),
            0x46 => Instruction::BIT(0, Target::HL),
            0x47 => Instruction::BIT(0, Target::A),
            0x48 => Instruction::BIT(1, Target::B),
            0x49 => Instruction::BIT(1, Target::C),
            0x4A => Instruction::BIT(1, Target::D),
            0x4B => Instruction::BIT(1, Target::E),
            0x4C => Instruction::BIT(1, Target::H),
            0x4D => Instruction::BIT(1, Target::L),
            0x4E => Instruction::BIT(1, Target::HL),
            0x4F => Instruction::BIT(1, Target::A),
            0x50 => Instruction::BIT(2, Target::B),
            0x51 => Instruction::BIT(2, Target::C),
            0x52 => Instruction::BIT(2, Target::D),
            0x53 => Instruction::BIT(2, Target::E),
            0x54 => Instruction::BIT(2, Target::H),
            0x55 => Instruction::BIT(2, Target::L),
            0x56 => Instruction::BIT(2, Target::HL),
            0x57 => Instruction::BIT(2, Target::A),
            0x58 => Instruction::BIT(3, Target::B),
            0x59 => Instruction::BIT(3, Target::C),
            0x5A => Instruction::BIT(3, Target::D),
            0x5B => Instruction::BIT(3, Target::E),
            0x5C => Instruction::BIT(3, Target::H),
            0x5D => Instruction::BIT(3, Target::L),
            0x5E => Instruction::BIT(3, Target::HL),
            0x5F => Instruction::BIT(3, Target::A),
            0x60 => Instruction::BIT(4, Target::B),
            0x61 => Instruction::BIT(4, Target::C),
            0x62 => Instruction::BIT(4, Target::D),
            0x63 => Instruction::BIT(4, Target::E),
            0x64 => Instruction::BIT(4, Target::H),
            0x65 => Instruction::BIT(4, Target::L),
            0x66 => Instruction::BIT(4, Target::HL),
            0x67 => Instruction::BIT(4, Target::A),
            0x68 => Instruction::BIT(5, Target::B),
            0x69 => Instruction::BIT(5, Target::C),
            0x6A => Instruction::BIT(5, Target::D),
            0x6B => Instruction::BIT(5, Target::E),
            0x6C => Instruction::BIT(5, Target::H),
            0x6D => Instruction::BIT(5, Target::L),
            0x6E => Instruction::BIT(5, Target::HL),
            0x6F => Instruction::BIT(5, Target::A),
            0x70 => Instruction::BIT(6, Target::B),
            0x71 => Instruction::BIT(6, Target::C),
            0x72 => Instruction::BIT(6, Target::D),
            0x73 => Instruction::BIT(6, Target::E),
            0x74 => Instruction::BIT(6, Target::H),
            0x75 => Instruction::BIT(6, Target::L),
            0x76 => Instruction::BIT(6, Target::HL),
            0x77 => Instruction::BIT(6, Target::A),
            0x78 => Instruction::BIT(7, Target::B),
            0x79 => Instruction::BIT(7, Target::C),
            0x7A => Instruction::BIT(7, Target::D),
            0x7B => Instruction::BIT(7, Target::E),
            0x7C => Instruction::BIT(7, Target::H),
            0x7D => Instruction::BIT(7, Target::L),
            0x7E => Instruction::BIT(7, Target::HL),
            0x7F => Instruction::BIT(7, Target::A),
            0x80 => Instruction::RES(0, Target::B),
            0x81 => Instruction::RES(0, Target::C),
            0x82 => Instruction::RES(0, Target::D),
            0x83 => Instruction::RES(0, Target::E),
            0x84 => Instruction::RES(0, Target::H),
            0x85 => Instruction::RES(0, Target::L),
            0x86 => Instruction::RES(0, Target::HL),
            0x87 => Instruction::RES(0, Target::A),
            0x88 => Instruction::RES(1, Target::B),
            0x89 => Instruction::RES(1, Target::C),
            0x8A => Instruction::RES(1, Target::D),
            0x8B => Instruction::RES(1, Target::E),
            0x8C => Instruction::RES(1, Target::H),
            0x8D => Instruction::RES(1, Target::L),
            0x8E => Instruction::RES(1, Target::HL),
            0x8F => Instruction::RES(1, Target::A),
            0x90 => Instruction::RES(2, Target::B),
            0x91 => Instruction::RES(2, Target::C),
            0x92 => Instruction::RES(2, Target::D),
            0x93 => Instruction::RES(2, Target::E),
            0x94 => Instruction::RES(2, Target::H),
            0x95 => Instruction::RES(2, Target::L),
            0x96 => Instruction::RES(2, Target::HL),
            0x97 => Instruction::RES(2, Target::A),
            0x98 => Instruction::RES(3, Target::B),
            0x99 => Instruction::RES(3, Target::C),
            0x9A => Instruction::RES(3, Target::D),
            0x9B => Instruction::RES(3, Target::E),
            0x9C => Instruction::RES(3, Target::H),
            0x9D => Instruction::RES(3, Target::L),
            0x9E => Instruction::RES(3, Target::HL),
            0x9F => Instruction::RES(3, Target::A),
            0xA0 => Instruction::RES(4, Target::B),
            0xA1 => Instruction::RES(4, Target::C),
            0xA2 => Instruction::RES(4, Target::D),
            0xA3 => Instruction::RES(4, Target::E),
            0xA4 => Instruction::RES(4, Target::H),
            0xA5 => Instruction::RES(4, Target::L),
            0xA6 => Instruction::RES(4, Target::HL),
            0xA7 => Instruction::RES(4, Target::A),
            0xA8 => Instruction::RES(5, Target::B),
            0xA9 => Instruction::RES(5, Target::C),
            0xAA => Instruction::RES(5, Target::D),
            0xAB => Instruction::RES(5, Target::E),
            0xAC => Instruction::RES(5, Target::H),
            0xAD => Instruction::RES(5, Target::L),
            0xAE => Instruction::RES(5, Target::HL),
            0xAF => Instruction::RES(5, Target::A),
            0xB0 => Instruction::RES(6, Target::B),
            0xB1 => Instruction::RES(6, Target::C),
            0xB2 => Instruction::RES(6, Target::D),
            0xB3 => Instruction::RES(6, Target::E),
            0xB4 => Instruction::RES(6, Target::H),
            0xB5 => Instruction::RES(6, Target::L),
            0xB6 => Instruction::RES(6, Target::HL),
            0xB7 => Instruction::RES(6, Target::A),
            0xB8 => Instruction::RES(7, Target::B),
            0xB9 => Instruction::RES(7, Target::C),
            0xBA => Instruction::RES(7, Target::D),
            0xBB => Instruction::RES(7, Target::E),
            0xBC => Instruction::RES(7, Target::H),
            0xBD => Instruction::RES(7, Target::L),
            0xBE => Instruction::RES(7, Target::HL),
            0xBF => Instruction::RES(7, Target::A),
            0xC0 => Instruction::SET(0, Target::B),
            0xC1 => Instruction::SET(0, Target::C),
            0xC2 => Instruction::SET(0, Target::D),
            0xC3 => Instruction::SET(0, Target::E),
            0xC4 => Instruction::SET(0, Target::H),
            0xC5 => Instruction::SET(0, Target::L),
            0xC6 => Instruction::SET(0, Target::HL),
            0xC7 => Instruction::SET(0, Target::A),
            0xC8 => Instruction::SET(1, Target::B),
            0xC9 => Instruction::SET(1, Target::C),
            0xCA => Instruction::SET(1, Target::D),
            0xCB => Instruction::SET(1, Target::E),
            0xCC => Instruction::SET(1, Target::H),
            0xCD => Instruction::SET(1, Target::L),
            0xCE => Instruction::SET(1, Target::HL),
            0xCF => Instruction::SET(1, Target::A),
            0xD0 => Instruction::SET(2, Target::B),
            0xD1 => Instruction::SET(2, Target::C),
            0xD2 => Instruction::SET(2, Target::D),
            0xD3 => Instruction::SET(2, Target::E),
            0xD4 => Instruction::SET(2, Target::H),
            0xD5 => Instruction::SET(2, Target::L),
            0xD6 => Instruction::SET(2, Target::HL),
            0xD7 => Instruction::SET(2, Target::A),
            0xD8 => Instruction::SET(3, Target::B),
            0xD9 => Instruction::SET(3, Target::C),
            0xDA => Instruction::SET(3, Target::D),
            0xDB => Instruction::SET(3, Target::E),
            0xDC => Instruction::SET(3, Target::H),
            0xDD => Instruction::SET(3, Target::L),
            0xDE => Instruction::SET(3, Target::HL),
            0xDF => Instruction::SET(3, Target::A),
            0xE0 => Instruction::SET(4, Target::B),
            0xE1 => Instruction::SET(4, Target::C),
            0xE2 => Instruction::SET(4, Target::D),
            0xE3 => Instruction::SET(4, Target::E),
            0xE4 => Instruction::SET(4, Target::H),
            0xE5 => Instruction::SET(4, Target::L),
            0xE6 => Instruction::SET(4, Target::HL),
            0xE7 => Instruction::SET(4, Target::A),
            0xE8 => Instruction::SET(5, Target::B),
            0xE9 => Instruction::SET(5, Target::C),
            0xEA => Instruction::SET(5, Target::D),
            0xEB => Instruction::SET(5, Target::E),
            0xEC => Instruction::SET(5, Target::H),
            0xED => Instruction::SET(5, Target::L),
            0xEE => Instruction::SET(5, Target::HL),
            0xEF => Instruction::SET(5, Target::A),
            0xF0 => Instruction::SET(6, Target::B),
            0xF1 => Instruction::SET(6, Target::C),
            0xF2 => Instruction::SET(6, Target::D),
            0xF3 => Instruction::SET(6, Target::E),
            0xF4 => Instruction::SET(6, Target::H),
            0xF5 => Instruction::SET(6, Target::L),
            0xF6 => Instruction::SET(6, Target::HL),
            0xF7 => Instruction::SET(6, Target::A),
            0xF8 => Instruction::SET(7, Target::B),
            0xF9 => Instruction::SET(7, Target::C),
            0xFA => Instruction::SET(7, Target::D),
            0xFB => Instruction::SET(7, Target::E),
            0xFC => Instruction::SET(7, Target::H),
            0xFD => Instruction::SET(7, Target::L),
            0xFE => Instruction::SET(7, Target::HL),
            0xFF => Instruction::SET(7, Target::A),
        }
    }
}
