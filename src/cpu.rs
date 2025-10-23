use crate::memory::Memory;

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

    // Stack pointer and program counter
    pub sp: u16,
    pub pc: u16,

    // Holds whether the CPU is halted
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

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let addr = self.pc;
        let byte = memory.read(addr);
        self.pc.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let lo = self.fetch_byte(memory) as u16;
        let hi = self.fetch_byte(memory) as u16;
        (hi << 8) | lo
    }

    pub fn step(&mut self, memory: &mut Memory) -> u32 {}
    fn execute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {}
}
