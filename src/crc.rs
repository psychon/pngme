// See chapter 15 (Appendix: Sample CRC Code) of the PNG spec

fn crc_table(n: u8) -> u32 {
    let mut c = u32::from(n);
    for _ in 0..8 {
        if c & 1 != 0 {
            c = 0xedb8_8320 ^ (c >> 1);
        } else {
            c = c >> 1;
        }
    }
    c
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Crc(u32);

impl Crc {
    pub fn new() -> Self {
        Self(!0)
    }

    pub fn get(&self) -> u32 {
        !self.0
    }

    pub fn update_byte(&self, byte: u8) -> Self {
        let index = (self.0 ^ u32::from(byte)) & 0xff;
        Self(crc_table(index as u8) ^ (self.0 >> 8))
    }

    pub fn update(&self, data: &[u8]) -> Self {
        let mut crc = *self;
        for byte in data {
            crc = crc.update_byte(*byte);
        }
        crc
    }
}
