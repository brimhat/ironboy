use crate::timer::Timer;
use crate::mmu::MMU;
use crate::cartridge::Cartridge;

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
    let mut timer = Timer::new();
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge);
    let clocks: u8 = 4;

    for _ in 0..63 {
        timer.step(&mut mmu, clocks);
    }

    assert_eq!(mmu.rb(0xFF04), 0);
    timer.step(&mut mmu, clocks);
    assert_eq!(mmu.rb(0xFF04), 1);

    for _ in 0..64 {
        timer.step(&mut mmu, clocks);
    }

    assert_eq!(mmu.rb(0xFF04), 2);

    for _ in 0..128 {
        timer.step(&mut mmu, clocks);
    }

    assert_eq!(mmu.rb(0xFF04), 4);
}

#[test]
fn write_div() {
    let mut timer = Timer::new();
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge);
    let clocks: u8 = 4;

    for _ in 0..64 {
        timer.step(&mut mmu, clocks);
    }

    assert_eq!(mmu.rb(0xFF04), 1);
    mmu.wb(0xFF04, 0x7F);
    assert_eq!(mmu.rb(0xFF04), 0);
}

#[test]
fn tima() {
    let mut timer = Timer::new();
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge);
    let clocks: u8 = 4;

    // enable timer, increment every 256 M-cycles
    mmu.wb(0xFF07, 0b0100);
    mmu.wb(0xFF05, 0);
    for _ in 0..255 {
        timer.step(&mut mmu, clocks);
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.step(&mut mmu, clocks);
    assert_eq!(mmu.rb(0xFF05), 1);

    // enable timer, increment every 4 M-cycles
    mmu.wb(0xFF07, 0b0101);
    mmu.wb(0xFF05, 0);
    for _ in 0..3 {
        timer.step(&mut mmu, clocks);
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.step(&mut mmu, clocks);
    assert_eq!(mmu.rb(0xFF05), 1);

    // enable timer, increment every 16 M-cycles
    mmu.wb(0xFF07, 0b0110);
    mmu.wb(0xFF05, 0);
    for _ in 0..15 {
        timer.step(&mut mmu, clocks);
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.step(&mut mmu, clocks);
    assert_eq!(mmu.rb(0xFF05), 1);

    // enable timer, increment every 64 M-cycles
    mmu.wb(0xFF07, 0b0111);
    mmu.wb(0xFF05, 0);
    for _ in 0..63 {
        timer.step(&mut mmu, clocks);
        assert_eq!(mmu.rb(0xFF05), 0);
    }
    timer.step(&mut mmu, clocks);
    assert_eq!(mmu.rb(0xFF05), 1);
}

#[test]
fn overflow() {
    let mut timer = Timer::new();
    let mut cartridge = cartridge();
    let mut mmu = MMU::new(&mut cartridge);
    let clocks: u8 = 4;
    let tma = mmu.rb(0xFF06);

    // enable timer, increment every 4 M-cycles
    mmu.wb(0xFF07, 0b0101);
    mmu.wb(0xFF05, 0);
    for _ in 0..1023 {
        timer.step(&mut mmu, clocks);
        assert_eq!(mmu.rb(0xFF0F), 0b0000);
    }
    assert_eq!(mmu.rb(0xFF05), 255);
    timer.step(&mut mmu, clocks);
    assert_eq!(mmu.rb(0xFF0F), 0b0100);
    assert_eq!(mmu.rb(0xFF05), 0);
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
