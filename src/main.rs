use std::env;
use std::process;

use zx81_emulator::{Emulator, memory::load_rom};

fn main() {
    let args: Vec<String> = env::args().collect();

    // We need a minimum of 2 args
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        process::exit(1);
    }

    // TODO: Load ROM file from args[1]
    let rom = match load_rom(&args[1]) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let mut emulator = Emulator::new(rom);

    // TODO: Implement main emulation loop
    // emulator.run_frame();
}
