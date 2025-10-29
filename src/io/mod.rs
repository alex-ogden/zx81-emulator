// I/O port handling for ZX81
// Main I/O port is 0xFE for keyboard and tape
use std::collections::HashSet;

pub struct IoController {
    keyboard_state: [[bool; 10]; 4],    // 4 rows x 10 cols
    pressed_keys: HashSet<minifb::Key>, // For key input
}

impl IoController {
    pub fn new() -> Self {
        Self {
            keyboard_state: [[false; 10]; 4],
            pressed_keys: HashSet::new(),
        }
    }

    fn create_key_map() -> HashSet<minifb::Key, (usize, usize)> {
        let mut map = HashSet::new();

        // Row 0
        map.insert(minifb::Key::Key1, (0, 0));
        map.insert(minifb::Key::Key2, (0, 1));
        map.insert(minifb::Key::Key3, (0, 2));
        map.insert(minifb::Key::Key4, (0, 3));
        map.insert(minifb::Key::Key5, (0, 4));
        map.insert(minifb::Key::Key6, (0, 5));
        map.insert(minifb::Key::Key7, (0, 6));
        map.insert(minifb::Key::Key8, (0, 7));
        map.insert(minifb::Key::Key9, (0, 8));
        map.insert(minifb::Key::Key0, (0, 9));

        // Row 1
        map.insert(minifb::Key::Q, (1, 0));
        map.insert(minifb::Key::W, (1, 1));
        map.insert(minifb::Key::E, (1, 2));
        map.insert(minifb::Key::R, (1, 3));
        map.insert(minifb::Key::T, (1, 4));
        map.insert(minifb::Key::Y, (1, 5));
        map.insert(minifb::Key::U, (1, 6));
        map.insert(minifb::Key::I, (1, 7));
        map.insert(minifb::Key::O, (1, 8));
        map.insert(minifb::Key::P, (1, 9));

        // Row 2
        map.insert(minifb::Key::A, (2, 0));
        map.insert(minifb::Key::S, (2, 1));
        map.insert(minifb::Key::D, (2, 2));
        map.insert(minifb::Key::F, (2, 3));
        map.insert(minifb::Key::G, (2, 4));
        map.insert(minifb::Key::H, (2, 5));
        map.insert(minifb::Key::J, (2, 6));
        map.insert(minifb::Key::K, (2, 7));
        map.insert(minifb::Key::L, (2, 8));
        map.insert(minifb::Key::Enter, (2, 9));

        // Row 3
        map.insert(minifb::Key::LeftShift, (3, 0));
        map.insert(minifb::Key::RightShift, (3, 0)); // Both shift keys same
        map.insert(minifb::Key::Z, (3, 1));
        map.insert(minifb::Key::X, (3, 2));
        map.insert(minifb::Key::C, (3, 3));
        map.insert(minifb::Key::V, (3, 4));
        map.insert(minifb::Key::B, (3, 5));
        map.insert(minifb::Key::N, (3, 6));
        map.insert(minifb::Key::M, (3, 7));
        map.insert(minifb::Key::Period, (3, 8));
        map.insert(minifb::Key::Space, (3, 9));

        map
    }
}
