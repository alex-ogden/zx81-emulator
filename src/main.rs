use std::env;
use std::process;
use std::thread::sleep;
use std::time::Duration;

use zx81_emulator::{Emulator, memory::load_rom};

fn main() {
    let args: Vec<String> = env::args().collect();

    // We need a minimum of 2 args
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_file> [--debug]", args[0]);
        process::exit(1);
    }

    // Check if debug is enabled
    let debug_enabled: bool = args.contains(&"--debug".to_string());

    // Remove --debug from args if it did exist
    let args: Vec<String> = args.into_iter().filter(|arg| arg != "--debug").collect();

    // Load ROM file from args[1]
    let rom = match load_rom(&args[1]) {
        Ok(data) => {
            println!("Loaded ROM: {} ({} bytes)", args[1], data.len());
            data
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let mut emulator = match Emulator::new(rom, debug_enabled) {
        Ok(emu) => emu,
        Err(e) => {
            eprintln!("Failed to create emulator: {}", e);
            process::exit(1);
        }
    };

    // Simple test runner - execute until HALT
    println!("Starting emulation...\n");

    let mut total_cycles = 0u64;
    let mut instruction_count = 0;
    let mut display_initialised = false;

    while emulator.is_window_open() {
        for _ in 0..1000 {
            // Print CPU state before execution
            //print_cpu_state(&emulator, instruction_count);

            // Execute one instruction
            let cycles = emulator.step();
            total_cycles += cycles as u64;
            instruction_count += 1;

            if emulator.is_halted() {
                break;
            }
        }

        if instruction_count > 50000 && !display_initialised {
            println!("Display initialisation should be complete");
            display_initialised = true;
        }

        if display_initialised {
            emulator.render_display().unwrap_or_else(|e| {
                eprintln!("Display error: {}", e);
            });
        } else {
            emulator.update_display();
        }

        std::thread::sleep(std::time::Duration::from_millis(16)); // 60 FPS
    }
}

fn print_cpu_state(emulator: &Emulator, instruction_num: usize) {
    let cpu = emulator.cpu();

    println!(
        "Instruction #{:04} | PC: 0x{:04X} | A: 0x{:02X} | BC: 0x{:04X} | DE: 0x{:04X} | HL: 0x{:04X} | SP: 0x{:04X} | F: {:08b}",
        instruction_num,
        cpu.pc,
        cpu.a,
        cpu.bc(),
        cpu.de(),
        cpu.hl(),
        cpu.sp,
        cpu.f,
    );
}
