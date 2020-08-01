use crate::timer::Timer;
use crate::mmu::MMU;
use crate::cartridge::Cartridge;
use crate::interrupts::IntReq;
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::Borrow;

const ROM: [u8; 32768] = [0; 32768];

pub fn cartridge() -> Cartridge {
    let mut cartridge = match Cartridge::new(ROM.to_vec()) {
        Err(e) => panic!("Error loading cartridge: {:#?}", e),
        Ok(c) => c,
    };
    return cartridge;
}

#[test]
fn div() {
    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());

    for _ in 0..63 {
        timer.borrow_mut().tick();
    }

    assert_eq!(mmu.rb(0xFF04), 0);
    timer.borrow_mut().tick();
    assert_eq!(mmu.rb(0xFF04), 1);

    for _ in 0..64 {
        timer.borrow_mut().tick();
    }

    assert_eq!(mmu.rb(0xFF04), 2);

    for _ in 0..128 {
        timer.borrow_mut().tick();
    }

    assert_eq!(mmu.rb(0xFF04), 4);
}

#[test]
fn write_div() {
    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());

    timer.borrow_mut().tick_n(64);

    assert_eq!(mmu.rb(0xFF04), 1);
    mmu.wb(0xFF04, 0x7F);
    assert_eq!(mmu.rb(0xFF04), 0);
    assert_eq!(timer.borrow_mut().counter, 0);
    assert_eq!(timer.borrow_mut().div, 0);
}

#[test]
fn tima() {
    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());

    // enable timer, increment every 256 M-cycles
    mmu.wb(0xFF07, 0b0100);
    mmu.wb(0xFF05, 0);
    for _ in 0..255 {
        timer.borrow_mut().tick();
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.borrow_mut().tick();
    assert_eq!(mmu.rb(0xFF05), 1);

    // enable timer, increment every 4 M-cycles
    mmu.wb(0xFF07, 0b0101);
    mmu.wb(0xFF05, 0);
    for _ in 0..3 {
        timer.borrow_mut().tick();
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.borrow_mut().tick();
    assert_eq!(mmu.rb(0xFF05), 1);

    // enable timer, increment every 16 M-cycles
    mmu.wb(0xFF07, 0b0110);
    mmu.wb(0xFF05, 0);
    for _ in 0..15 {
        timer.borrow_mut().tick();
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.borrow_mut().tick();
    assert_eq!(mmu.rb(0xFF05), 1);

    // enable timer, increment every 64 M-cycles
    mmu.wb(0xFF07, 0b0111);
    mmu.wb(0xFF05, 0);
    for _ in 0..63 {
        timer.borrow_mut().tick();
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.borrow_mut().tick();
    assert_eq!(mmu.rb(0xFF05), 1);
}

#[test]
fn overflow() {
    let intr = Rc::new(RefCell::new(IntReq::new()));
    let timer = Rc::new(RefCell::new(Timer::new(intr.clone())));
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge, timer.clone());
    mmu.wb(0xFF06, 0x20);
    assert_eq!(timer.borrow_mut().tma, 0x20);
    assert_eq!(mmu.timer.borrow_mut().tma, 0x20);

    // enable timer, increment every 4 M-cycles
    mmu.wb(0xFF07, 0b0101);
    mmu.wb(0xFF05, 0);
    for _ in 0..1023 {
        timer.borrow_mut().tick();
        assert_eq!(mmu.rb(0xFF0F), 0b0000);
    }
    assert_eq!(mmu.rb(0xFF05), 255);
    timer.borrow_mut().tick();
    assert_eq!(mmu.rb(0xFF0F), 0b0100);
    assert_eq!(timer.borrow_mut().intr.borrow_mut().flags, 0b0100);
    assert_eq!(mmu.timer.borrow_mut().intr.borrow_mut().flags, 0b0100);
    assert_eq!(mmu.rb(0xFF05), 0x20);
}

// #[test]
// fn falling_edge() {
    // when writing to DIV, if the current output is '1' and the timer is enabled,
    // as the new value after reseting DIV will be '0', the falling edge detector will
    // detect a falling edge and TIMA will increase.
// }

// #[test]
// fn write_tac() {
    // when writing to TAC, if the previously selected multiplexer input was '1' and
    // the new input is '0', TIMA will increase too. This doesn't happen when the timer is
    // disabled, but it also happens when disabling the timer.
// }
