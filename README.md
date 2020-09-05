# ironboy

ironboy is a Game Boy emulator written in Rust.

ironboy started as an educational project to solidify
my knowledge of computer architecture and processors.
Through the solidifying of those topics as well as the learning of
painful lessons in the importance of smart software architecture 
and design decisions, ironboy has turned into a bit of
a hobby project. **This project will be put on a small hiatus**
while I learn more about software architecture, but I will
eventually return to make ironboy a more fully-featured emulator.

**Note:**
 * This project is still a WIP
 * ironboy requires a boot rom
 * ironboy can currently on play DMG ROMs
 * This project has only been tested on linux

**Buttons:**

| Game Boy | Keyboard |
|----------|----------|
| A        | S        |
| B        | A        |
| Start    | Enter    |
| Select   | R. Shift |
| DPad     | Arrows   |

**how to run:**

        $ mv path_to_boot /ironboy/roms/DMG_ROM.bin
        $ cargo build --release
        $ cargo run --release -- path_to_rom

## Things I will do in the future:
 * Add CGB support
 * Implement the APU

## Things I may do in the future:
 * make ironboy cycle accurate
 * hook up imgui
 * implement serialization
 * QoL addition like a 5x speed button

## References
 * [Game Boy Dev Wiki](https://gbdev.gg8.se/wiki/articles/Main_Page)
 * [The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI)
 * [Game Boy Emulation in JavaScript](https://imrannazar.com/gameBoy-Emulation-in-JavaScript)
 * [Game Boy Opcode Table](https://izik1.github.io/gbops)
 * Gekkio's ["Game Boy: Complete Technical Reference"](https://github.com/Gekkio/gb-ctr)
 * [BGB](https://bgb.bircd.org)
