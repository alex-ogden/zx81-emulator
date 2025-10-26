use super::Cpu;
use crate::memory::Memory;

// Register and flag helper methods
impl Cpu {
    // == Flag-setting helper functions == //
    pub fn set_flag_c(&mut self, val: bool) {
        if val {
            self.f |= 0x01;
        } else {
            self.f &= !0x01;
        }
    }
    pub fn set_flag_n(&mut self, val: bool) {
        if val {
            self.f |= 0x02;
        } else {
            self.f &= !0x02;
        }
    }
    pub fn set_flag_pv(&mut self, val: bool) {
        if val {
            self.f |= 0x04;
        } else {
            self.f &= !0x04;
        }
    }
    pub fn set_flag_y(&mut self, val: bool) {
        if val {
            self.f |= 0x08;
        } else {
            self.f &= !0x08;
        }
    }
    pub fn set_flag_h(&mut self, val: bool) {
        if val {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
    }
    pub fn set_flag_x(&mut self, val: bool) {
        if val {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }
    pub fn set_flag_z(&mut self, val: bool) {
        if val {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }
    pub fn set_flag_s(&mut self, val: bool) {
        if val {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
    }
    // == Flag-getting helper functions == //
    pub fn get_flag_c(&self) -> bool {
        (self.f & 0x01) != 0
    }
    pub fn get_flag_n(&self) -> bool {
        (self.f & 0x02) != 0
    }
    pub fn get_flag_pv(&self) -> bool {
        (self.f & 0x04) != 0
    }
    pub fn get_flag_y(&self) -> bool {
        (self.f & 0x08) != 0
    }
    pub fn get_flag_h(&self) -> bool {
        (self.f & 0x10) != 0
    }
    pub fn get_flag_x(&self) -> bool {
        (self.f & 0x20) != 0
    }
    pub fn get_flag_z(&self) -> bool {
        (self.f & 0x40) != 0
    }
    pub fn get_flag_s(&self) -> bool {
        (self.f & 0x80) != 0
    }

    // == Flag pair helper functions == //
    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = val as u8;
    }
    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    // == Read & Write to registers == //
    pub fn read_reg(&self, reg_code: u8, memory: &Memory) -> u8 {
        match reg_code {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => memory.read(self.hl()), // Memory - special case
            7 => self.a,
            _ => unreachable!(),
        }
    }

    pub fn write_reg(&mut self, reg_code: u8, val: u8, memory: &mut Memory) {
        match reg_code {
            0 => self.b = val,
            1 => self.c = val,
            2 => self.d = val,
            3 => self.e = val,
            4 => self.h = val,
            5 => self.l = val,
            6 => memory.write(self.hl(), val), // Memory - special case
            7 => self.a = val,
            _ => unreachable!(),
        }
    }

    // == POP and PUSH helper functions == //
    pub fn push(&mut self, val: u16, memory: &mut Memory) {
        self.sp = self.sp.wrapping_sub(2);
        memory.write_word(self.sp, val);
    }

    pub fn pop(&mut self, memory: &Memory) -> u16 {
        let lo = memory.read(self.sp) as u16;
        let hi = memory.read(self.sp.wrapping_add(1)) as u16;
        self.sp = self.sp.wrapping_add(2);
        (hi << 8) | lo
    }
}
