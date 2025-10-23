mod ram;
mod rom;

pub use rom::load_rom;

pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0x400], // 1K RAM (base model)
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                let offset = (addr - 0x4000) as usize;
                if offset < self.ram.len() {
                    self.ram[offset]
                } else {
                    0xFF // Return default out of bounds error
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {} // ROM is not writable
            0x4000..=0x7FFF => {
                let offset = (addr - 0x4000) as usize;
                if offset < self.ram.len() {
                    self.ram[offset] = val;
                }
            }
            _ => {} // Ignore writes to out-of-bounds memory addresses
        }
    }
}
