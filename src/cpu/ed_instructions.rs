use super::Cpu;
use crate::memory::Memory;

// ED-prefixed opcodes (extended instructions)
impl Cpu {
    pub(super) fn execute_ed_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x4F => self.ld_r_a(),
            0x47 => self.ld_i_a(),
            0x5F => self.ld_a_r(),
            0x56 => self.im_1(),
            0x78 => self.in_a_c(),

            // Consolidated patterns:
            0x4B | 0x5B | 0x7B => self.ld_rr_nn_indirect(opcode, memory),
            0x43 | 0x53 | 0x63 | 0x73 => self.ld_nn_indirect_rr(opcode, memory),
            0x42 | 0x52 | 0x62 | 0x72 => self.sbc_hl_rr(opcode),

            0xA0 => self.ldi(memory),
            0xB0 => self.ldir(memory),
            0xB1 => self.cpir(memory),
            0xB8 => self.lddr(memory),
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

    fn ldi(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));

        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_h(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.bc() != 0);

        16
    }

    fn ldir(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_de(self.de().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_h(false);
        self.set_flag_n(false);

        if self.bc() != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(false);
        16
    }

    fn cpir(&mut self, memory: &Memory) -> u8 {
        let val = memory.read(self.hl());
        let result = self.a.wrapping_sub(val);

        self.set_flag_z(self.a == val);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_n(true);

        self.set_hl(self.hl().wrapping_add(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_pv(self.bc() != 0);

        // Check if we need to repeat
        if self.bc() != 0 && self.a != val {
            self.pc = self.pc.wrapping_sub(2); // Go back 2 bytes (ED + opcode)
            return 21;
        }

        16
    }
    fn lddr(&mut self, memory: &mut Memory) -> u8 {
        let byte = memory.read(self.hl());
        memory.write(self.de(), byte);

        self.set_hl(self.hl().wrapping_sub(1));
        self.set_de(self.de().wrapping_sub(1));
        self.set_bc(self.bc().wrapping_sub(1));

        self.set_flag_h(false);
        self.set_flag_n(false);

        if self.bc() != 0 {
            self.pc = self.pc.wrapping_sub(2);
            self.set_flag_pv(true);
            return 21;
        }

        self.set_flag_pv(false);
        16
    }

    fn ld_rr_nn_indirect(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);
        let val = memory.read_word(addr);

        match (opcode >> 4) & 0x03 {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.sp = val,
            _ => unreachable!(),
        }

        20
    }
    fn ld_nn_indirect_rr(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let val = match (opcode >> 4) & 0x03 {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        };

        memory.write_word(addr, val);
        20
    }
    fn sbc_hl_rr(&mut self, opcode: u8) -> u8 {
        let hl = self.hl();

        let operand = match (opcode >> 4) & 0x03 {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.sp,
            _ => unreachable!(),
        };

        let carry = if self.get_flag_c() { 1u16 } else { 0u16 };
        let result = hl.wrapping_sub(operand).wrapping_sub(carry);

        // Calculate flags
        let full_sub = (hl as u32)
            .wrapping_sub(operand as u32)
            .wrapping_sub(carry as u32);

        self.set_hl(result);
        self.set_flag_c(full_sub > 0xFFFF);
        self.set_flag_n(true);
        self.set_flag_z(result == 0);
        self.set_flag_s((result & 0x8000) != 0);
        self.set_flag_h(((hl & 0x0FFF) as i32 - (operand & 0x0FFF) as i32 - carry as i32) < 0);
        self.set_flag_pv(((hl ^ operand) & (hl ^ result) & 0x8000) != 0);
        self.set_flag_x(((result >> 8) & 0x20) != 0);
        self.set_flag_y(((result >> 8) & 0x08) != 0);

        15
    }

    fn ld_r_a(&mut self) -> u8 {
        self.r = self.a;
        9
    }

    fn ld_i_a(&mut self) -> u8 {
        self.i = self.a;
        9
    }

    fn ld_a_r(&mut self) -> u8 {
        self.a = self.r;
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.iff2);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);
        9
    }

    fn in_a_c(&mut self) -> u8 {
        self.a = 0xFF;
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);
        12
    }

    fn im_1(&mut self) -> u8 {
        self.interrupt_mode = 1;
        8
    }
}
