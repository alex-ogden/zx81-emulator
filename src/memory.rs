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

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.rom[addr as usize],
            0x2000..=0x3FFF => self.ram[(addr - 0x2000) as usize],
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {} // ROM is not writable
            0x2000..=0x3FFF => self.ram[(addr - 0x2000) as usize] = val,
            _ => {} // Ignore writes to out-of-bounds memory addresses
        }
    }
}

