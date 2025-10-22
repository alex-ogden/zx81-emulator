pub struct Cpu {
    // Z80 CPU @ 3.25MHz
    // Registers
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub halted: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0xFFFF,
            pc: 0x0000,
            halted: false,
        }
    }

    pub fn step(&mut self, mem: &mut Memory) -> u32 {}
}

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
