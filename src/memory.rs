pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0x2000], // 8K RAM
        }
    }
}