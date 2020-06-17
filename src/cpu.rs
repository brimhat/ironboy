use crate::mmu::MMU;
use crate::registers::Registers;
use crate::registers::Flag;

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
                    (Target::HL, Target::IMM16) => {
                        self.reg.set_hl(self.get_imm16(mmu));
                        self.reg.pc += 3;
                    }
                    (Target::HLD, Target::A) => {
                        mmu.wb(self.reg.hld(), self.reg.a);
                        self.reg.pc += 1;
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
            Instruction::JR(f) => {
                match f {
                    JumpFlag::NZ => {
                        let pc = self.reg.pc + 2;
                        if !self.reg.get_flag(Flag::Z) {
                            let n = self.get_imm8(mmu) as i8;
                            self.reg.pc = pc.wrapping_add(n as u16);
                        } else {
                            self.reg.pc = pc;
                        }
                    }
                }
            }
            _ => panic!("Unrecognized instr: {:?}", instr)
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) {
        let byte = mmu.rb(self.reg.pc);

        let instr = match byte == 0xCB {
            false => Instruction::decode(byte),
            true => {
                self.reg.pc += 1;
                let cb_byte = mmu.rb(self.reg.pc);
                Instruction::decode_cb(cb_byte)
            }
        };

        self.execute(mmu, instr);

        // DEBUGGING
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

    pub fn get_imm16(&self, mmu: &MMU) -> u16 {
        let lo = mmu.rb(self.reg.pc + 1);
        let hi = mmu.rb(self.reg.pc + 2);
        ((hi as u16) << 8) | lo as u16
    }

    pub fn get_imm8(&self, mmu: &MMU) -> u8 {
        mmu.rb(self.reg.pc + 1)
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    LD(Target, Target),
    XOR(Target),
    BIT(u8, Target),
    JR(JumpFlag)
}

#[derive(Debug, Copy, Clone)]
enum Target {
    A, F, AF, B, C, BC, D, E, DE, H, L, HL, HLI, HLD, SP, IMM8, IMM16
}

#[derive(Debug, Copy, Clone)]
enum JumpFlag {
    NZ
}

impl Instruction {
    pub fn decode(opcode: u8) -> Instruction {
        match opcode {
            0x20 => Instruction::JR(JumpFlag::NZ),
            0x21 => Instruction::LD(Target::HL, Target::IMM16),
            0x31 => Instruction::LD(Target::SP, Target::IMM16),
            0x32 => Instruction::LD(Target::HLD, Target::A),
            0xAF => Instruction::XOR(Target::A),
            _ => panic!("Unrecognized opcode: {:#X}", opcode)
        }
    }

    pub fn decode_cb(opcode: u8) -> Instruction {
        match opcode {
            0x7C => Instruction::BIT(7, Target::H),
            _ => panic!("Unrecognized prefixed opcode: {:#X}", opcode)
        }
    }
}