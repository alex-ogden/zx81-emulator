use super::Cpu;
use crate::memory::Memory;

// DD-prefixed opcodes (IX register operations)
impl Cpu {
    pub(super) fn execute_dd_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0xE1 => self.pop_ix(memory),
            0xE9 => self.jp_ix(),
            _ => {
                eprintln!(
                    "Unknown DD opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.pc - 2
                );
                4
            }
        }
    }

    fn pop_ix(&mut self, memory: &Memory) -> u8 {
        self.ix = self.pop(memory);
        14
    }

    fn jp_ix(&mut self) -> u8 {
        self.pc = self.ix;
        8
    }
}
