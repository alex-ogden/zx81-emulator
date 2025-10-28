use crate::memory::Memory;

pub struct DisplayFile {
    pub characters: [[u8; 32]; 24], // 32 columns x 24 rows of character codes
}

impl DisplayFile {
    pub fn new() -> Self {
        Self {
            characters: vec![[0; 32]; 24],
        }
    }
    pub fn parse(&mut self, memory: &Memory) {
        // System D-FILE is at 0x400C -> 0x400D
        let d_file_start = memory.read_word(0x400C);
        let mut current_addr = d_file_start;

        for line in 0..24 {
            for char_pos in 0..32 {
                let char_code = memory.read(current_addr);
                self.characters[line][char_pos] = char_code;
                current_addr.wrapping_add(1);
            }
            // Skip the HALT instruction
            current_addr.wrapping_add(1);
        }
    }

    pub fn get_char(&self, row: usize, col: usize) -> u8 {
        self.characters[row][col] & 0x20
    }
    pub fn get_char_index(&self, row: usize, col: usize) -> usize {}
    pub fn is_inverse(&self, row: usize, col: usize) -> bool {
        self.characters[row][col] & 0x80 == 1
    }
}

