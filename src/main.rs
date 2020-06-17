mod mmu;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use crate::mmu::MMU;

fn main() {
    let mut file = File::open("ROMS/DMG_ROM.bin").unwrap();
    let mut boot = Vec::<u8>::new();
    file.read_to_end(&mut boot);

    let mut mmu = MMU::new();
    mmu.read_boot(boot);

    println!("{:#X}", mmu.boot[1]);
}
