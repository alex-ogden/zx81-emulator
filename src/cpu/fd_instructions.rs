use super::Cpu;
use crate::memory::Memory;

impl Cpu {
    pub(super) fn execute_fd_instruction(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x21 => self.ld_iy_nn(memory),
            0x36 => self.ld_iy_d_n(memory),
            0x35 => self.dec_iy_d(memory),
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => self.ld_r_iy_d(opcode, memory),
            0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x77 => self.ld_iy_d_r(opcode, memory),
            0xAE => self.xor_iy_d(memory),
            0xBE => self.cp_iy_d(memory),
            0xCB => {
                let d = self.fetch_byte(memory) as i8;
                let sub_opcode = self.fetch_byte(memory);
                self.execute_fd_cb_instruction(sub_opcode, d, memory)
            }
            _ => {
                eprintln!("Unknown FD opcode: 0x{:02X} at PC: 0x{:04X}", opcode, self.pc - 2);
                4
            }
        }
    }

    fn ld_iy_nn(&mut self, memory: &mut Memory) -> u8 {
        let val = self.fetch_word(memory);
        self.iy = val;
        14
    }

    fn ld_iy_d_n(&mut self, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let n = self.fetch_byte(memory);
        let addr = self.iy.wrapping_add(d as u16);
        memory.write(addr, n);
        19
    }

    fn dec_iy_d(&mut self, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
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

        23
    }

    fn ld_r_iy_d(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);

        let reg = (opcode >> 3) & 0x07;
        match reg {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            7 => self.a = val,
            _ => unreachable!(),
        }

        19
    }

    fn ld_iy_d_r(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);

        let reg = opcode & 0x07;
        let val = match reg {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            7 => self.a,
            _ => unreachable!(),
        };

        memory.write(addr, val);
        19
    }

    fn xor_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        self.a ^= val;

        self.set_flag_s((self.a & 0x80) != 0);
        self.set_flag_z(self.a == 0);
        self.set_flag_h(false);
        self.set_flag_pv(self.a.count_ones() % 2 == 0);
        self.set_flag_n(false);
        self.set_flag_c(false);
        self.set_flag_x((self.a & 0x20) != 0);
        self.set_flag_y((self.a & 0x08) != 0);

        19
    }

    fn cp_iy_d(&mut self, memory: &Memory) -> u8 {
        let d = self.fetch_byte(memory) as i8;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = self.a.wrapping_sub(val);

        self.set_flag_c(val > self.a);
        self.set_flag_n(true);
        self.set_flag_pv(((self.a ^ val) & (self.a ^ result) & 0x80) != 0);
        self.set_flag_h((self.a & 0x0F) < (val & 0x0F));
        self.set_flag_z(self.a == val);
        self.set_flag_s((result & 0x80) != 0);
        self.set_flag_x((result & 0x20) != 0);
        self.set_flag_y((result & 0x08) != 0);

        19
    }

    fn execute_fd_cb_instruction(&mut self, opcode: u8, d: i8, memory: &mut Memory) -> u8 {
        match opcode {
            0x40..=0x7F => self.bit_n_iy_d(opcode, d, memory),
            0x80..=0xBF => self.res_n_iy_d(opcode, d, memory),
            0xC0..=0xFF => self.set_n_iy_d(opcode, d, memory),
            _ => {
                eprintln!("Unknown FD CB opcode: 0x{:02X} at PC: 0x{:04X}", opcode, self.pc - 4);
                23
            }
        }
    }

    fn bit_n_iy_d(&mut self, opcode: u8, d: i8, memory: &Memory) -> u8 {
        let bit = (opcode >> 3) & 0x07;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = val & (1 << bit);

        self.set_flag_z(result == 0);
        self.set_flag_n(false);
        self.set_flag_h(true);
        self.set_flag_s((bit == 7) && (result != 0));
        self.set_flag_pv(result == 0);

        20
    }

    fn res_n_iy_d(&mut self, opcode: u8, d: i8, memory: &mut Memory) -> u8 {
        let bit = (opcode >> 3) & 0x07;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = val & !(1 << bit);
        memory.write(addr, result);

        23
    }

    fn set_n_iy_d(&mut self, opcode: u8, d: i8, memory: &mut Memory) -> u8 {
        let bit = (opcode >> 3) & 0x07;
        let addr = self.iy.wrapping_add(d as u16);
        let val = memory.read(addr);
        let result = val | (1 << bit);
        memory.write(addr, result);

        23
    }
}
