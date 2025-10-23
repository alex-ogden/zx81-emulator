mod keyboard;
mod tape;

// I/O port handling for ZX81
// Main I/O port is 0xFE for keyboard and tape
pub struct IoController {
    // TODO: Add state for keyboard matrix and tape interface
}

impl IoController {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_port(&self, port: u8) -> u8 {
        // TODO: Implement I/O port reading
        0xFF
    }

    pub fn write_port(&mut self, port: u8, value: u8) {
        // TODO: Implement I/O port writing
    }
}
