# ironboy

ironboy is a Game Boy emulator written in Rust.

**Warning:**
 * This project is still a WIP
 * ironboy requires a boot rom

**What I am currently working on:**

A more machine-accurate implementation of the timer.
Currently the PPU isn't synced properly and I
am getting a flickering image (though the boot rom works fine).
I suspect that this is due to my shoddy/hacky timer implementation:

![flickering image](data/flickering_image.gif)

## Things I may do in the future:
 * Implement the APU
 * Serialization
 * Write a debugger
 * Convert ironboy into a Game Boy Color
 * QoL additions like a 5x speed button

## Resources
 * [Game Boy Dev Wiki](https://gbdev.gg8.se/wiki/articles/Main_Page)
 * [The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI)
 * [Game Boy Emulation in JavaScript](https://imrannazar.com/gameBoy-Emulation-in-JavaScript)
 * [Game Boy Opcode Table](https://izik1.github.io/gbops)
 * Gekkio's ["Game Boy: Complete Technical Reference"](https://github.com/Gekkio/gb-ctr)
 * Game Boy Development Manual v1.1
