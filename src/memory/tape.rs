use std::fs;

pub struct Tape {
    data: Vec<u8>,
    position: usize,
    loading: bool,
}

impl Tape {
    pub fn load(filename: &str) -> Result<Self, String> {
        let data = fs::read(filename).map_err(|e| format!("Failed to read tape file: {}", e))?;
        println!("Loaded tape file: {} ({} bytes)", filename, data.len());
        Ok(Self {
            data,
            position: 0,
            loading: false,
        })
    }

    pub fn get_next_byte(&mut self) -> Option<u8> {
        if self.position < self.data.len() {
            let byte = self.data[self.position];
            self.position += 1;
            Some(byte)
        } else {
            // We're at the end of the tape
            None
        }
    }

    pub fn rewind(&mut self) {
        self.position = 0;
    }

    pub fn is_finished(&self) -> bool {
        self.position >= self.data.len()
    }

    pub fn start_loading(&mut self) {
        self.loading = true;
        self.rewind();
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }
}
