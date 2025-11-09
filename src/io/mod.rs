use crate::tape::Tape;

pub struct IoController {
    keyboard_state: [[bool; 5]; 8],
}

impl IoController {
    pub fn new() -> Self {
        Self {
            keyboard_state: [[false; 5]; 8],
        }
    }

    pub fn read_port(&self, port: u8, addr_high: u8, tape: &Option<Tape>) -> u8 {
        if port == 0xFE {
            let mut result = 0xBF;

            let row = match addr_high {
                0xFE => 0,
                0xFD => 1,
                0xFB => 2,
                0xF7 => 3,
                0xEF => 4,
                0xDF => 5,
                0xBF => 6,
                0x7F => 7,
                _ => return 0xBF,
            };

            for col in 0..5 {
                if self.keyboard_state[row][col] {
                    result &= !(1 << col);
                }
            }

            // EAR bit 6 from tape
            let ear = if let Some(t) = tape {
                if t.is_playing() {
                    if t.get_level() { 0 } else { 1 }
                } else {
                    1 // Floating low when no tape playing 
                }
            } else {
                1 // No tape loaded, floating low 
            };

            result = (result & 0xBF) | (ear << 6); // Set bit 5 if high 
            result
        } else {
            0xBF
        }
    }

    pub fn update_keys(&mut self, keys: &[minifb::Key]) {
        self.keyboard_state = [[false; 5]; 8];

        for key in keys {
            if let Some((row, col)) = Self::map_key(*key) {
                self.keyboard_state[row][col] = true;
            }
        }
    }

    fn map_key(key: minifb::Key) -> Option<(usize, usize)> {
        match key {
            minifb::Key::LeftShift | minifb::Key::RightShift => Some((0, 0)),
            minifb::Key::Z => Some((0, 1)),
            minifb::Key::X => Some((0, 2)),
            minifb::Key::C => Some((0, 3)),
            minifb::Key::V => Some((0, 4)),

            minifb::Key::A => Some((1, 0)),
            minifb::Key::S => Some((1, 1)),
            minifb::Key::D => Some((1, 2)),
            minifb::Key::F => Some((1, 3)),
            minifb::Key::G => Some((1, 4)),

            minifb::Key::Q => Some((2, 0)),
            minifb::Key::W => Some((2, 1)),
            minifb::Key::E => Some((2, 2)),
            minifb::Key::R => Some((2, 3)),
            minifb::Key::T => Some((2, 4)),

            minifb::Key::Key1 => Some((3, 0)),
            minifb::Key::Key2 => Some((3, 1)),
            minifb::Key::Key3 => Some((3, 2)),
            minifb::Key::Key4 => Some((3, 3)),
            minifb::Key::Key5 => Some((3, 4)),

            minifb::Key::Key0 => Some((4, 0)),
            minifb::Key::Key9 => Some((4, 1)),
            minifb::Key::Key8 => Some((4, 2)),
            minifb::Key::Key7 => Some((4, 3)),
            minifb::Key::Key6 => Some((4, 4)),

            minifb::Key::P => Some((5, 0)),
            minifb::Key::O => Some((5, 1)),
            minifb::Key::I => Some((5, 2)),
            minifb::Key::U => Some((5, 3)),
            minifb::Key::Y => Some((5, 4)),

            minifb::Key::Enter => Some((6, 0)),
            minifb::Key::L => Some((6, 1)),
            minifb::Key::K => Some((6, 2)),
            minifb::Key::J => Some((6, 3)),
            minifb::Key::H => Some((6, 4)),

            minifb::Key::Space => Some((7, 0)),
            minifb::Key::Period => Some((7, 1)),
            minifb::Key::M => Some((7, 2)),
            minifb::Key::N => Some((7, 3)),
            minifb::Key::B => Some((7, 4)),

            _ => None,
        }
    }

    pub fn write_port(&mut self, _port: u8, _value: u8) {}
}
