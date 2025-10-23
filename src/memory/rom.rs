// ROM loading and management
use std::fs;

use super::Memory;

// TODO: Add functions to load ROM from file
// ZX81 ROM is 8KB (0x0000 - 0x1FFF)
// Contains BASIC interpreter and character set

const ZX81_ROM_SIZE: usize = 0x2000; // 8KB ROM maximum

pub fn load_rom(rom_path: &str) -> Result<Vec<u8>, String> {
    let rom_data = fs::read(rom_path).map_err(|e| format!("Failed to read ROM: {}", e))?;

    // Verify the size of the ROM
    if rom_data.len() != ZX81_ROM_SIZE {
        return Err(format!(
            "ROM is not expected size: Expected {}, got {}",
            ZX81_ROM_SIZE,
            rom_data.len()
        ));
    }

    Ok(rom_data)
}
