//! ROM loading utilities

use std::fs;

/// Expected size of ZX81 ROM in bytes (8KB)
const ZX81_ROM_SIZE: usize = 0x2000;

/// Loads a ZX81 ROM file from disk
///
/// The ROM must be exactly 8KB (8192 bytes) in size. The ZX81 ROM contains:
/// - BASIC interpreter
/// - System routines
/// - Character set (at 0x0E00-0x0FFF)
///
/// # Arguments
///
/// * `rom_path` - Path to the ROM file
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - ROM data if successful
/// * `Err(String)` - Error message if loading fails
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be read
/// - The file is not exactly 8KB in size
///
/// # Examples
///
/// ```no_run
/// use zx81_emulator::memory::load_rom;
///
/// let rom = load_rom("zx81.rom").expect("Failed to load ROM");
/// assert_eq!(rom.len(), 8192);
/// ```
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
