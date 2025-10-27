use super::Cpu;
use crate::memory::Memory;

// ED-prefixed opcodes (extended instructions)
impl Cpu {
    pub(super) fn execute_ed_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x4F => self.ld_r_a(),
            _ => {
                eprintln!(
                    "Unknown ED opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.pc - 2
                );
                4
            }
        }
    }

    fn ld_r_a(&mut self) -> u8 {
        self.r = self.a;
        9
    }
}
