use super::Cpu;
use crate::memory::Memory;

// Further implementation of Cpu with opcode functions
impl Cpu {
    pub(super) fn execute_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            // HALT and NOP
            0x00 => self.nop(),
            0x76 => self.halt(),
            // LD r, n pattern opcodes
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => self.ld_r_n(opcode, memory),
            // INC r pattern opcode
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C => self.inc_r(opcode),
            // Jump
            0xC3 => self.jp_nn(memory),
            _ => {
                eprintln!(
                    "Unknown opcode: 0x{:02X} at PC: 0x{:04X}",
                    opcode,
                    self.pc - 1
                );
                4
            }
        }
    }
    fn nop(&mut self) -> u8 {
        // No operation - 4 cycles
        4
    }
    fn halt(&mut self) -> u8 {
        self.is_halted = true;
        4
    }
    fn ld_r_n(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = self.fetch_byte(memory);
        match (opcode >> 3) & 0x07 {
            0b111 => self.a = val,
            0b000 => self.b = val,
            0b001 => self.c = val,
            0b010 => self.d = val,
            0b011 => self.e = val,
            0b100 => self.h = val,
            0b101 => self.l = val,
            _ => unreachable!(),
        }
        7
    }
    fn inc_r(&mut self, opcode: u8) -> u8 {
        // Logic is the same on all INC operations
        let reg = match opcode {
            0x04 => &mut self.b,
            0x0C => &mut self.c,
            0x14 => &mut self.d,
            0x1C => &mut self.e,
            0x24 => &mut self.h,
            0x2C => &mut self.l,
            0x3C => &mut self.a,
            _ => panic!("Invalid INC r opcode: 0x{:02X}", opcode),
        };

        let old_val = *reg;
        *reg = old_val.wrapping_add(1);
        let new_val = *reg;

        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x0F);
        self.set_flag_pv(old_val == 0x7F);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        4
    }
    fn jp_nn(&mut self, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);
        self.pc = addr;
        10
    }
}
