use crate::mmu::MMU;
use crate::cartridge::Cartridge;
use crate::cpu::{ CPU, Instruction, JumpFlag, Target };
use crate::registers::{ Registers, Flag };
use crate::interrupts::IntReq;
use crate::timer::Timer;
use std::rc::Rc;
use std::cell::RefCell;

const ROM: [u8; 32768] = [0; 32768];

pub fn cartridge() -> Cartridge {
    let mut cartridge = match Cartridge::new(ROM.to_vec()) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c,
    };
    return cartridge;
}

#[test]
fn ld() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());

    // LD r r
    cpu.reg.a = 0x20;
    cpu.reg.b = 0x10;
    assert_eq!(cpu.reg.b, 0x10);
    cpu.execute(&mut mmu, Instruction::LD(Target::B, Target::A));
    assert_eq!(cpu.reg.b, 0x20);
    cpu.reg.e = 0x6D;
    cpu.reg.h = 0x0;
    assert_eq!(cpu.reg.h, 0x0);
    cpu.execute(&mut mmu, Instruction::LD(Target::H, Target::E));
    assert_eq!(cpu.reg.h, 0x6D);

    // LD r (HL)
    cpu.reg.a = 0x33;
    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0x1F);
    assert_eq!(cpu.reg.a, 0x33);
    cpu.execute(&mut mmu, Instruction::LD(Target::A, Target::HL));
    assert_eq!(cpu.reg.a, 0x1F);

    // LD (HL) r
    cpu.reg.l = 0x7D;
    mmu.wb(cpu.reg.hl(), 0x17);
    assert_eq!(mmu.rb(cpu.reg.hl()), 0x17);
    cpu.execute(&mut mmu, Instruction::LD(Target::HL, Target::L));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0x7D);

    // LD A (C)
    cpu.reg.a = 0x52;
    cpu.reg.c = 0x6C;
    mmu.wb(0xFF00 | (cpu.reg.c as u16), 0x0F);
    assert_eq!(cpu.reg.a, 0x52);
    cpu.execute(&mut mmu, Instruction::LD(Target::A, Target::FFC));
    assert_eq!(cpu.reg.a, 0x0F);

    // LD (C) A
    cpu.reg.a = 0x52;
    cpu.reg.c = 0x40;
    mmu.wb(0xFF00 | (cpu.reg.c as u16), 0x81);
    assert_eq!(mmu.rb(0xFF00 | (cpu.reg.c as u16)), 0x81);
    cpu.execute(&mut mmu, Instruction::LD(Target::FFC, Target::A));
    assert_eq!(mmu.rb(0xFF00 | (cpu.reg.c as u16)), 0x52);
}

#[test]
fn rla() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x95;
    cpu.reg.set_flag(Flag::C, true);
    cpu.execute(&mut mmu, Instruction::RLA);
    assert_eq!(cpu.reg.a, 0x2B);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn rlca() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x85;
    cpu.execute(&mut mmu, Instruction::RLCA);
    assert_eq!(cpu.reg.a, 0x0B);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn rrca() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x3B;
    cpu.execute(&mut mmu, Instruction::RRCA);
    assert_eq!(cpu.reg.a, 0x9D);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn rra() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x81;
    cpu.reg.set_flag(Flag::C, false);
    cpu.execute(&mut mmu, Instruction::RRA);
    assert_eq!(cpu.reg.a, 0x40);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn rl() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.l = 0x80;
    cpu.execute(&mut mmu, Instruction::RL(Target::L));
    assert_eq!(cpu.reg.l, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.c = 0x11;
    cpu.execute(&mut mmu, Instruction::RL(Target::C));
    assert_eq!(cpu.reg.c, 0x23);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn rlc() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x85;
    cpu.execute(&mut mmu, Instruction::RLC(Target::A));
    assert_eq!(cpu.reg.a, 0x0B);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.e = 0x0;
    cpu.execute(&mut mmu, Instruction::RLC(Target::E));
    assert_eq!(cpu.reg.e, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn rr() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.l = 0x01;
    cpu.execute(&mut mmu, Instruction::RR(Target::L));
    assert_eq!(cpu.reg.l, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.c = 0x8A;
    cpu.execute(&mut mmu, Instruction::RR(Target::C));
    assert_eq!(cpu.reg.c, 0xC5);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn rrc() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.c = 0x01;
    cpu.execute(&mut mmu, Instruction::RRC(Target::C));
    assert_eq!(cpu.reg.c, 0x80);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.e = 0x0;
    cpu.execute(&mut mmu, Instruction::RRC(Target::E));
    assert_eq!(cpu.reg.e, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
}

#[test]
fn sla() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.d = 0x80;
    cpu.execute(&mut mmu, Instruction::SLA(Target::D));
    assert_eq!(cpu.reg.a, 0x00);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0xFF);
    cpu.execute(&mut mmu, Instruction::SLA(Target::HL));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0xFE);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn sra() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x8A;
    cpu.execute(&mut mmu, Instruction::SRA(Target::A));
    assert_eq!(cpu.reg.a, 0xC5);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);

    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0x01);
    cpu.execute(&mut mmu, Instruction::SRA(Target::HL));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0x00);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn srl() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x01;
    cpu.execute(&mut mmu, Instruction::SRL(Target::A));
    assert_eq!(cpu.reg.a, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0xFF);
    cpu.execute(&mut mmu, Instruction::SRL(Target::HL));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0x7F);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn and() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x5A;
    cpu.reg.b = 0x3F;
    cpu.reg.c = 0x38;
    cpu.reg.d = 0x0;
    cpu.execute(&mut mmu, Instruction::AND(Target::B));
    assert_eq!(cpu.reg.a, 0x1A);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);

    cpu.reg.a = 0x5A;
    cpu.execute(&mut mmu, Instruction::AND(Target::C));
    assert_eq!(cpu.reg.a, 0x18);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);

    cpu.reg.a = 0x5A;
    cpu.execute(&mut mmu, Instruction::AND(Target::D));
    assert_eq!(cpu.reg.a, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);
}

#[test]
fn or() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x5A;
    cpu.reg.b = 0x03;
    cpu.reg.c = 0x0F;
    cpu.execute(&mut mmu, Instruction::OR(Target::A));
    assert_eq!(cpu.reg.a, 0x5A);

    cpu.reg.a = 0x5A;
    cpu.execute(&mut mmu, Instruction::OR(Target::B));
    assert_eq!(cpu.reg.a, 0x5B);

    cpu.reg.a = 0x5A;
    cpu.execute(&mut mmu, Instruction::OR(Target::C));
    assert_eq!(cpu.reg.a, 0x5F);
}

#[test]
fn xor() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0xFF;
    cpu.reg.b = 0x0F;
    cpu.reg.c = 0x8A;
    cpu.execute(&mut mmu, Instruction::XOR(Target::A));
    assert_eq!(cpu.reg.a, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);

    cpu.reg.a = 0xFF;
    cpu.execute(&mut mmu, Instruction::XOR(Target::B));
    assert_eq!(cpu.reg.a, 0xF0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);

    cpu.reg.a = 0xFF;
    cpu.execute(&mut mmu, Instruction::XOR(Target::C));
    assert_eq!(cpu.reg.a, 0x75);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
}

#[test]
fn inc() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0xFF;
    cpu.execute(&mut mmu, Instruction::INC(Target::A));
    assert_eq!(cpu.reg.a, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);

    cpu.reg.set_hl(0x235F);
    cpu.execute(&mut mmu, Instruction::INC(Target::HL));
    assert_eq!(cpu.reg.hl(), 0x2360);
}

#[test]
fn dec() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x01;
    cpu.execute(&mut mmu, Instruction::DEC(Target::A));
    assert_eq!(cpu.reg.a, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);

    cpu.reg.set_de(0x235F);
    cpu.execute(&mut mmu, Instruction::DEC(Target::DE));
    assert_eq!(cpu.reg.de(), 0x235E);
}

#[test]
fn add() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.set_hl(0x8000);
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
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
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
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.set_hl(0x8A23);
    cpu.reg.set_de(0x0605);
    cpu.execute(&mut mmu, Instruction::ADDHL(Target::DE));
    assert_eq!(cpu.reg.hl(), 0x9028);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);

    cpu.reg.set_hl(0x8A23);
    cpu.reg.set_bc(0x8A23);
    cpu.execute(&mut mmu, Instruction::ADDHL(Target::BC));
    assert_eq!(cpu.reg.hl(), 0x1446);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn sub() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
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

    cpu.reg.a = 0x3E;
    cpu.execute(&mut mmu, Instruction::SUB(Target::C));
    assert_eq!(cpu.reg.a, 0x2F);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);

    cpu.reg.a = 0x3E;
    cpu.execute(&mut mmu, Instruction::SUB(Target::D));
    assert_eq!(cpu.reg.a, 0xFE);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
}

#[test]
fn sbc() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x3B;
    cpu.reg.b = 0x2A;
    cpu.reg.c = 0x4F;
    cpu.reg.d = 0x3A;
    cpu.reg.set_flag(Flag::C, true);
    cpu.execute(&mut mmu, Instruction::SBC(Target::B));
    assert_eq!(cpu.reg.a, 0x10);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);

    cpu.reg.a = 0x3B;
    cpu.execute(&mut mmu, Instruction::SBC(Target::C));
    assert_eq!(cpu.reg.a, 0xEC);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);

    cpu.reg.a = 0x3B;
    cpu.execute(&mut mmu, Instruction::SBC(Target::D));
    assert_eq!(cpu.reg.a, 0x0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
}

#[test]
fn cp() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x3C;
    cpu.reg.b = 0x2F;
    cpu.reg.c = 0x3C;
    cpu.reg.d = 0x40;
    cpu.execute(&mut mmu, Instruction::CP(Target::B));
    assert_eq!(cpu.reg.a, 0x3C);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);

    cpu.execute(&mut mmu, Instruction::CP(Target::C));
    assert_eq!(cpu.reg.a, 0x3C);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);

    cpu.execute(&mut mmu, Instruction::CP(Target::D));
    assert_eq!(cpu.reg.a, 0x3C);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn swap() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x00;
    cpu.execute(&mut mmu, Instruction::SWAP(Target::A));
    assert_eq!(cpu.reg.a, 0x00);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);

    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0x0F);
    cpu.execute(&mut mmu, Instruction::SWAP(Target::HL));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0xF0);
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);
}

#[test]
fn daa() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
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

#[test]
fn bit() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x80;
    cpu.execute(&mut mmu, Instruction::BIT(7, Target::A));
    assert_eq!(cpu.reg.get_flag(Flag::Z), false);

    cpu.reg.l = 0xEF;
    cpu.execute(&mut mmu, Instruction::BIT(4, Target::L));
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);
}

#[test]
fn res() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x80;
    cpu.execute(&mut mmu, Instruction::RES(7, Target::A));
    assert_eq!(cpu.reg.a, 0x0);

    cpu.reg.l = 0x3B;
    cpu.execute(&mut mmu, Instruction::RES(1, Target::L));
    assert_eq!(cpu.reg.l, 0x39);

    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0xFF);
    cpu.execute(&mut mmu, Instruction::RES(3, Target::HL));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0xF7);
}

#[test]
fn set() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x80;
    cpu.execute(&mut mmu, Instruction::SET(2, Target::A));
    assert_eq!(cpu.reg.a, 0x84);

    cpu.reg.l = 0x3B;
    cpu.execute(&mut mmu, Instruction::SET(7, Target::L));
    assert_eq!(cpu.reg.l, 0xBB);

    cpu.reg.set_hl(0x8000);
    mmu.wb(cpu.reg.hl(), 0x00);
    cpu.execute(&mut mmu, Instruction::SET(2, Target::HL));
    assert_eq!(mmu.rb(cpu.reg.hl()), 0x04);
}

#[test]
fn cpl() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.a = 0x35;
    cpu.execute(&mut mmu, Instruction::CPL);
    assert_eq!(cpu.reg.a, 0xCA);
    assert_eq!(cpu.reg.get_flag(Flag::N), true);
    assert_eq!(cpu.reg.get_flag(Flag::H), true);
}

#[test]
fn ccf() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.set_flag(Flag::C, true);
    cpu.execute(&mut mmu, Instruction::CCF);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), false);

    cpu.reg.set_flag(Flag::C, false);
    cpu.execute(&mut mmu, Instruction::CCF);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn scf() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.set_flag(Flag::C, true);
    cpu.reg.set_flag(Flag::N, true);
    cpu.reg.set_flag(Flag::H, true);
    cpu.execute(&mut mmu, Instruction::SCF);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    cpu.reg.set_flag(Flag::C, false);
    cpu.reg.set_flag(Flag::N, false);
    cpu.reg.set_flag(Flag::H, false);
    cpu.execute(&mut mmu, Instruction::SCF);
    assert_eq!(cpu.reg.get_flag(Flag::N), false);
    assert_eq!(cpu.reg.get_flag(Flag::H), false);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);
}

#[test]
fn ei() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.pc = 0x8000;
    cpu.reg.sp = 0xC000;
    mmu.wb(0xFF0F, 4);
    mmu.wb(0xFFFF, 4);
    // the effect of EI is delayed by one instruction. this means that EI followed
    // immediately by DI does not allow interrupts between the EI and the DI
    mmu.wb(0x8000, 0xFB); // EI
    cpu.step(&mut mmu);
    mmu.wb(0x8001, 0xF3); // DI
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x8002);

    mmu.wb(0x8002, 0xFB); // EI
    cpu.step(&mut mmu);
    mmu.wb(0x8003, 0x00); // Delay by one instruction
    cpu.step(&mut mmu);
    mmu.wb(0x8004, 0x00); // Interrupt
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x0051);     // Expect PC == 0x51 because ROM[0x0050] == NOP
}

#[test]
fn jp() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.pc = 0x8000;
    mmu.wb(0x8000, 0xC3); // JP(A)
    mmu.wb(0x8001, 0xD7); // lo
    mmu.wb(0x8002, 0x90); // hi
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x90D7);

    mmu.wb(0x90D7, 0x04); // INC(B)
    cpu.step(&mut mmu);
    mmu.wb(0x90D8, 0x05); // DEC(B)
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.get_flag(Flag::Z), true);

    mmu.wb(0x90D9, 0xC2); // JP(NZ) *should fail*
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x90DC);

    mmu.wb(0x90DC, 0xCA); // JP(Z) *should pass*
    mmu.wb(0x90DD, 0x34);
    mmu.wb(0x90DE, 0xC5);
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0xC534);

    mmu.wb(0xC534, 0x37); // SCF
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.get_flag(Flag::C), true);

    mmu.wb(0xC535, 0xD2); // JP(NC) *should fail*
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0xC538);

    mmu.wb(0xC538, 0xDA); // JP(C) *should pass*
    mmu.wb(0xC539, 0xB5);
    mmu.wb(0xC53A, 0x83);
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x83B5);

    cpu.reg.set_hl(0xEFFF);
    mmu.wb(0x83B5, 0xE9); // JP(AtHL)
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0xEFFF);
}

#[test]
fn jr() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.pc = 0x8000;
    mmu.wb(0x8000, 0x18);
    mmu.wb(0x8001, 0x60);
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x8062);
}

#[test]
fn call() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.pc = 0x8000;
    cpu.reg.sp = 0xFFFE;
    mmu.wb(0x8000, 0xCD); // CALL(A)
    mmu.wb(0x8001, 0x34);
    mmu.wb(0x8002, 0x12);
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x1234);
    assert_eq!(mmu.rb(cpu.reg.sp), 0x03);
    assert_eq!(mmu.rb(cpu.reg.sp + 1), 0x80);
}

#[test]
fn ret() {
    let mut intr = Rc::new(RefCell::new(IntReq::new()));
    let mut timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cpu = CPU::new(timer.clone());
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    cpu.reg.pc = 0x8000;
    cpu.reg.sp = 0xFFFE;
    mmu.wb(0x8000, 0xCD); // CALL(A)
    mmu.wb(0x8001, 0x00);
    mmu.wb(0x8002, 0x90);
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x9000);

    mmu.wb(0x9000, 0xC9);
    cpu.step(&mut mmu);
    assert_eq!(cpu.reg.pc, 0x8003);
}
