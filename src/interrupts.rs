
#[derive(Debug)]
pub enum IntFlag {
    VBlank = 0b0000_0001,
    Stat = 0b0000_0010,
    Timer = 0b0000_0100,
    Serial = 0b0000_1000,
    Joypad = 0b0001_0000,
}

#[derive(Debug)]
pub struct IntReq {
    pub flags: u8,
}

impl IntReq {
    pub fn new() -> IntReq {
        IntReq { flags: 0 }
    }

    pub fn set_flag(&mut self, f: IntFlag) {
        self.flags |= f as u8;
    }
}
