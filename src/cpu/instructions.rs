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
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => self.ld_r_n(opcode, memory),
            // LD rr, nn pattern opcodes
            0x01 | 0x11 | 0x21 | 0x31 => self.ld_rr_nn(opcode, memory),
            // LD r, r' pattern opcodes
            0x40..=0x7F => self.ld_r_r(opcode, memory),
            0x02 | 0x12 => self.ld_rr_indirect_a(opcode, memory),
            0x0A | 0x1A => self.ld_a_rr_indirect(opcode, memory),
            0x32 => self.ld_nn_indirect_a(memory),
            0x3A => self.ld_a_nn_indirect(memory),
            0x22 => self.ld_nn_indirect_hl(memory),
            0x2A => self.ld_hl_nn_indirect(memory),
            // INC r pattern opcode
            0x04 | 0x0C | 0x14 | 0x1C | 0x24 | 0x2C | 0x3C => self.inc_r(opcode),
            // INC rr pattern opcode
            0x03 | 0x13 | 0x23 | 0x33 => self.inc_rr(opcode),
            0x34 => self.inc_hl_indirect(memory),
            // DEC
            0x05 | 0x15 | 0x25 | 0x0D | 0x1D | 0x2D | 0x3D => self.dec_r(opcode),
            0x0B | 0x1B | 0x2B | 0x3B => self.dec_rr(opcode),
            0x35 => self.dec_hl_indirect(memory),
            0x10 => self.dec_jnz_d(memory),
            // !TODO: Add tests for ADD and SUB (and future ADC and SBC) opcodes
            // ADD
            0x80..=0x87 => self.add_a_r(opcode, memory),
            // SUB
            0x90..=0x97 => self.sub_a_r(opcode, memory),
            // ADC
            0x88..=0x8F => self.adc_a_r(opcode, memory),
            // SBC
            0x98..=0x9F => self.sbc_a_r(opcode, memory),
            // SCF and CCF
            0x37 => self.scf(),
            0x3F => self.ccf(),
            // JP
            0xC3 => self.jp_nn(memory),
            // Conditional Jumps
            0xC2 | 0xCA | 0xD2 | 0xDA | 0xE2 | 0xEA | 0xF2 | 0xFA => self.jp_cc_nn(opcode, memory),
            // Relative Jumps
            0x18 | 0x20 | 0x28 | 0x30 | 0x38 => self.jr_cc_e(opcode, memory),
            // CALL
            0xCD | 0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => {
                self.call_cc_nn(opcode, memory)
            }
            0xC9 | 0xC0 | 0xC8 | 0xD0 | 0xD8 | 0xE0 | 0xE8 | 0xF0 | 0xF8 => {
                self.ret_cc(opcode, memory)
            }
            // PUSH
            0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push_rr(opcode, memory),
            // POP
            0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop_rr(opcode, memory),
            // Logical AND
            0xA0..=0xA7 => self.and_a_r(opcode, memory),
            // Logical OR
            0xB0..=0xB7 => self.or_a_r(opcode, memory),
            // Logical XOR
            0xA8..=0xAF => self.xor_a_r(opcode, memory),
            // CP / Compare
            0xB8..=0xBF => self.cp_a_r(opcode, memory),
            0xF3 => self.di(),
            0xFB => self.ei(),
            0xD3 => self.out_n_a(memory),
            0xDB => self.in_a_n(memory),
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
    fn di(&mut self) -> u8 {
        self.iff1 = false;
        self.iff2 = false;
        4
    }
    fn ei(&mut self) -> u8 {
        self.iff1 = true;
        self.iff2 = true;
        4
    }
    fn out_n_a(&mut self, memory: &Memory) -> u8 {
        let port = self.fetch_byte(memory);
        // TODO: Implement actual I/O handling
        // For now, just ignore the write
        // println!("OUT (0x{:02X}), A (A=0x{:02X})", port, self.a);
        11
    }
    fn in_a_n(&mut self, memory: &Memory) -> u8 {
        let port = self.fetch_byte(memory);
        // TODO: Implement actual I/O handling
        // For now, just return 0xFF (common for unconnected ports)
        self.a = 0xFF;
        // println!("IN A, (0x{:02X}) -> 0xFF", port);
        11
    }
    fn ld_rr_indirect_a(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = match opcode {
            0x02 => self.bc(),
            0x12 => self.de(),
            _ => unreachable!(),
        };
        memory.write(addr, self.a);
        7
    }
    fn ld_a_rr_indirect(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = match opcode {
            0x0A => self.bc(),
            0x1A => self.de(),
            _ => unreachable!(),
        };
        self.a = memory.read(addr);
        7
    }
    fn ld_nn_indirect_a(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        memory.write(addr, self.a);
        13
    }
    fn ld_a_nn_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        self.a = memory.read(addr);
        13
    }
    fn ld_nn_indirect_hl(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        memory.write_word(addr, self.hl());
        16
    }
    fn ld_hl_nn_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);
        let val = memory.read_word(addr);
        self.set_hl(val);
        16
    }
    fn and_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        self.a &= src;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(true);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }

    fn or_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        self.a |= src;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }

    fn xor_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        self.a ^= src;

        self.set_flag_c(false);
        self.set_flag_n(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_h(false);
        self.set_flag_z(self.a == 0);
        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }

    fn cp_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let src = self.read_reg(src_code, memory);
        let result = self.a.wrapping_sub(src);

        self.set_flag_c(src > self.a);
        self.set_flag_n(true);
        self.set_flag_pv(((self.a ^ src) & (self.a ^ result) & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (src & 0x0F));
        self.set_flag_z(self.a == src);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }
    fn push_rr(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = match opcode {
            0xC5 => self.bc(),
            0xD5 => self.de(),
            0xE5 => self.hl(),
            0xF5 => self.af(),
            _ => unreachable!("Invalid PUSH rr opcode: 0x{:02X}", opcode),
        };

        self.push(val, memory);
        11
    }
    fn pop_rr(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let val = self.pop(memory);
        match opcode {
            0xC1 => self.set_bc(val),
            0xD1 => self.set_de(val),
            0xE1 => self.set_hl(val),
            0xF1 => self.set_af(val),
            _ => unreachable!("Invalid POP rr opcode: 0x{:02X}", opcode),
        }

        10
    }
    fn call_cc_nn(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let (condition, cycles_taken, cycles_not_taken) = match opcode {
            0xCD => (true, 17, 17),
            0xC4 => (!self.get_flag_z(), 17, 10),
            0xCC => (self.get_flag_z(), 17, 10),
            0xD4 => (!self.get_flag_c(), 17, 10),
            0xDC => (self.get_flag_c(), 17, 10),
            0xE4 => (!self.get_flag_pv(), 17, 10),
            0xEC => (self.get_flag_pv(), 17, 10),
            0xF4 => (!self.get_flag_s(), 17, 10),
            0xFC => (self.get_flag_s(), 17, 10),
            _ => unreachable!("Invalid CALL e opcode: 0x{:02X}", opcode),
        };

        if condition {
            self.push(self.pc, memory);
            self.pc = addr;
            cycles_taken
        } else {
            cycles_not_taken
        }
    }
    fn ret_cc(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let (condition, cycles_taken, cycles_not_taken) = match opcode {
            0xC9 => (true, 10, 10),
            0xC0 => (!self.get_flag_z(), 11, 5),
            0xC8 => (self.get_flag_z(), 11, 5),
            0xD0 => (!self.get_flag_c(), 11, 5),
            0xD8 => (self.get_flag_c(), 11, 5),
            0xE0 => (!self.get_flag_pv(), 11, 5),
            0xE8 => (self.get_flag_pv(), 11, 5),
            0xF0 => (!self.get_flag_s(), 11, 5),
            0xF8 => (self.get_flag_s(), 11, 5),
            _ => unreachable!("Invalid RET cc opcode: 0x{:02X}", opcode),
        };

        if condition {
            self.pc = self.pop(memory);
            cycles_taken
        } else {
            cycles_not_taken
        }
    }
    fn ld_r_n(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = self.fetch_byte(memory);
        let reg = (opcode >> 3) & 0x07;
        self.write_reg(reg, val, memory);

        if reg == 6 { 10 } else { 7 }
    }
    fn ld_rr_nn(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let val = self.fetch_word(memory);
        match (opcode >> 4) & 0x03 {
            0 => self.set_bc(val),
            1 => self.set_de(val),
            2 => self.set_hl(val),
            3 => self.sp = val,
            _ => panic!("Invalid LD rr nn opcode: 0x{:02X}", opcode),
        }
        10
    }
    fn ld_r_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let src_code = opcode & 0x07;
        let dest_code = (opcode >> 3) & 0x07;
        let val = self.read_reg(src_code, memory);
        self.write_reg(dest_code, val, memory);

        if src_code == 6 || dest_code == 6 {
            // Memory operations take 7 cycles
            7
        } else {
            // Reg -> Reg operations take 4 cycles
            4
        }
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
    fn inc_rr(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x03 => self.set_bc(self.bc().wrapping_add(1)),
            0x13 => self.set_de(self.de().wrapping_add(1)),
            0x23 => self.set_hl(self.hl().wrapping_add(1)),
            0x33 => self.sp = self.sp.wrapping_add(1),
            _ => unreachable!("Invalid INC rr opcode: 0x{:02X}", opcode),
        }
        6
    }
    fn inc_hl_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.hl();
        let old_val = memory.read(addr);
        let new_val = old_val.wrapping_add(1);
        memory.write(addr, new_val);

        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x0F);
        self.set_flag_pv(old_val == 0x7F);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        11
    }
    fn dec_r(&mut self, opcode: u8) -> u8 {
        let reg = match opcode {
            0x05 => &mut self.b,
            0x0D => &mut self.c,
            0x15 => &mut self.d,
            0x1D => &mut self.e,
            0x25 => &mut self.h,
            0x2D => &mut self.l,
            0x3D => &mut self.a,
            _ => panic!("Invalid DEC r opcode: 0x{:02X}", opcode),
        };

        let old_val = *reg;
        *reg = old_val.wrapping_sub(1);
        let new_val = *reg;

        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x00);
        self.set_flag_pv(old_val == 0x80);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        4
    }
    fn dec_rr(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x0B => self.set_bc(self.bc().wrapping_sub(1)),
            0x1B => self.set_de(self.de().wrapping_sub(1)),
            0x2B => self.set_hl(self.hl().wrapping_sub(1)),
            0x3B => self.sp = self.sp.wrapping_sub(1),
            _ => unreachable!("Invalid DEC rr opcode: 0x{:02X}", opcode),
        }
        6
    }
    fn dec_hl_indirect(&mut self, memory: &mut Memory) -> u8 {
        let addr = self.hl();
        let old_val = memory.read(addr);
        let new_val = old_val.wrapping_sub(1);
        memory.write(addr, new_val);

        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) == 0x00);
        self.set_flag_pv(old_val == 0x80);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        11
    }
    fn dec_jnz_d(&mut self, memory: &Memory) -> u8 {
        let offset = self.fetch_byte(memory) as i8;
        self.b = self.b.wrapping_sub(1);
        if self.b != 0 {
            self.pc = self.pc.wrapping_add(offset as i16 as u16);
            return 13;
        }
        8
    }
    fn jp_nn(&mut self, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);
        self.pc = addr;
        10
    }
    fn jp_cc_nn(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let addr = self.fetch_word(memory);

        let condition = match opcode {
            0xC2 => !self.get_flag_z(),
            0xCA => self.get_flag_z(),
            0xD2 => !self.get_flag_c(),
            0xDA => self.get_flag_c(),
            0xE2 => !self.get_flag_pv(),
            0xEA => self.get_flag_pv(),
            0xF2 => !self.get_flag_s(),
            0xFA => self.get_flag_s(),
            _ => unreachable!("Invalid JP cc opcode: 0x{:02X}", opcode),
        };

        // If selected condition is true, jump
        if condition {
            self.pc = addr;
        }
        10
    }
    fn jr_cc_e(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let offset = self.fetch_byte(memory) as i8;

        let (condition, cycles_taken, cycles_not_taken) = match opcode {
            0x18 => (true, 12, 12),
            0x20 => (!self.get_flag_z(), 12, 7),
            0x28 => (self.get_flag_z(), 12, 7),
            0x30 => (!self.get_flag_c(), 12, 7),
            0x38 => (self.get_flag_c(), 12, 7),
            _ => unreachable!("Invalid JR cc opcode: 0x{:02X}", opcode),
        };

        if condition {
            self.pc = self.pc.wrapping_add(offset as i16 as u16);
            cycles_taken
        } else {
            cycles_not_taken
        }
    }
    fn add_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let new_val = self.a.wrapping_add(val);
        self.a = new_val;

        self.set_flag_c((old_val as u16) + (val as u16) > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        // Reg->Reg takes 4 cycles
        // Memory->Reg takes 7 cycles
        if src_code == 6 { 7 } else { 4 }
    }
    fn adc_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let carry = if self.get_flag_c() { 1u8 } else { 0u8 };
        let new_val = self.a.wrapping_add(val).wrapping_add(carry);
        self.a = new_val;

        let full_add = (old_val as u16)
            .wrapping_add(val as u16)
            .wrapping_add(carry as u16);

        self.set_flag_c(full_add > 0xFF);
        self.set_flag_n(false);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) + (val & 0x0F) + carry > 0x0F);
        self.set_flag_pv(((old_val ^ new_val) & (val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }
    fn sub_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let new_val = old_val.wrapping_sub(val);
        self.a = new_val;

        self.set_flag_c(val > old_val);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F));
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }
    fn sbc_a_r(&mut self, opcode: u8, memory: &Memory) -> u8 {
        let src_code = opcode & 0x07;
        let old_val = self.a;
        let val = self.read_reg(src_code, memory);
        let carry = if self.get_flag_c() { 1u8 } else { 0u8 };
        let new_val = self.a.wrapping_sub(val).wrapping_sub(carry);
        self.a = new_val;

        let full_sub = (old_val as u16)
            .wrapping_sub(val as u16)
            .wrapping_sub(carry as u16);

        self.set_flag_c(full_sub > 0xFF);
        self.set_flag_n(true);
        self.set_flag_z(new_val == 0);
        self.set_flag_s((new_val & 0x80) != 0);
        self.set_flag_h((old_val & 0x0F) < (val & 0x0F) + carry);
        self.set_flag_pv(((old_val ^ val) & (old_val ^ new_val) & 0x80) != 0);
        self.set_flag_x((new_val & 0x20) != 0);
        self.set_flag_y((new_val & 0x08) != 0);

        if src_code == 6 { 7 } else { 4 }
    }
    fn scf(&mut self) -> u8 {
        self.set_flag_c(true);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);
        4
    }
    fn ccf(&mut self) -> u8 {
        let old_carry = self.get_flag_c();
        self.set_flag_h(old_carry);
        self.set_flag_c(!old_carry);
        self.set_flag_n(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);
        4
    }
}
