//! Memory management subsystem
//!
//! Handles the ZX81's memory layout with ROM and RAM regions.
//!
//! ## Memory Map
//!
//! ```text
//! 0x0000 - 0x1FFF  (8KB)   ROM - BASIC interpreter and system routines
//! 0x2000 - 0x3FFF          Unused (mirror/expansion)
//! 0x4000 - 0x43FF  (1KB)   RAM - Base system RAM
//! 0x4400 - 0x7FFF          RAM - Expansion (if installed)
//! ```

mod ram;
mod rom;

pub use rom::load_rom;

/// ZX81 Memory system
///
/// Manages ROM and RAM with correct address decoding for the ZX81.
pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
}

impl Memory {
    /// Creates a new memory system with the provided ROM
    ///
    /// Initializes with 1KB base RAM at 0x4000.
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            rom,
            ram: vec![0; 0x400], // 1K RAM (base model)
        }
    }

    /// Reads a byte from memory
    ///
    /// Returns 0xFF for out-of-bounds RAM addresses.
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

    /// Writes a byte to memory
    ///
    /// ROM writes and out-of-bounds RAM writes are silently ignored.
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
