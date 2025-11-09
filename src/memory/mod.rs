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
            ram: vec![0; 0x4000], // 16K RAM
        }
    }

    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    pub fn load_program(&mut self, data: &[u8]) -> Result<(), String> {
        if data.is_empty() {
            return Err("Empty .p file".to_string());
        }

        // Preserve 0x4000–0x4011
        let mut saved = [0u8; 18];
        for i in 0..18 {
            saved[i] = self.read(0x4000 + i as u16);
        }

        // Load from 0x4009
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x4009 + i as u16;
            if addr >= 0x8000 {
                break;
            }
            self.write(addr, byte);
        }

        // Restore sysvars
        for i in 0..18 {
            self.write(0x4000 + i as u16, saved[i]);
        }

        // Set STKEND
        let end = 0x4009 + data.len() as u16;
        self.write_word(0x401F, end);

        Ok(())
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
            _ => 0xFF,
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read(addr) as u16;
        let hi = self.read(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {} // ROM is not writable
            0x4000..=0x7FFF => {
                // RAM expandable up to 16KB
                let offset = (addr - 0x4000) as usize;
                if offset < self.ram.len() {
                    self.ram[offset] = val;
                }
            }
            _ => {} // Ignore writes to out-of-bounds memory addresses
        }
    }

    pub fn write_word(&mut self, addr: u16, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = val as u8;

        self.write(addr, lo);
        self.write(addr.wrapping_add(1), hi);
    }
}
