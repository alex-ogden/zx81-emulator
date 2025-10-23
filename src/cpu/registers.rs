use super::Cpu;

// Register and flag helper methods
impl Cpu {
    // == Flag-setting helper functions == //
    fn set_flag_c(&mut self, val: bool) {
        if val {
            self.f |= 0x01;
        } else {
            self.f &= !0x01;
        }
    }
    fn set_flag_n(&mut self, val: bool) {
        if val {
            self.f |= 0x02;
        } else {
            self.f &= !0x02;
        }
    }
    fn set_flag_pv(&mut self, val: bool) {
        if val {
            self.f |= 0x04;
        } else {
            self.f &= !0x04;
        }
    }
    fn set_flag_y(&mut self, val: bool) {
        if val {
            self.f |= 0x08;
        } else {
            self.f &= !0x08;
        }
    }
    fn set_flag_h(&mut self, val: bool) {
        if val {
            self.f |= 0x10;
        } else {
            self.f &= !0x10;
        }
    }
    fn set_flag_x(&mut self, val: bool) {
        if val {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }
    fn set_flag_z(&mut self, val: bool) {
        if val {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }
    fn set_flag_s(&mut self, val: bool) {
        if val {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
        }
    }
    // == Flag-getting helper functions == //
    fn get_flag_c(&self) -> bool {
        (self.f & 0x01) != 0
    }
    fn get_flag_n(&self) -> bool {
        (self.f & 0x02) != 0
    }
    fn get_flag_pv(&self) -> bool {
        (self.f & 0x04) != 0
    }
    fn get_flag_y(&self) -> bool {
        (self.f & 0x08) != 0
    }
    fn get_flag_h(&self) -> bool {
        (self.f & 0x10) != 0
    }
    fn get_flag_x(&self) -> bool {
        (self.f & 0x20) != 0
    }
    fn get_flag_z(&self) -> bool {
        (self.f & 0x40) != 0
    }
    fn get_flag_s(&self) -> bool {
        (self.f & 0x80) != 0
    }
}
