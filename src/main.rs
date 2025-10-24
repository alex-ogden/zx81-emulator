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

    let mut emulator = Emulator::new(rom);

    // Simple test runner - execute until HALT
    println!("Starting emulation...\n");

    let mut total_cycles = 0u64;
    let max_instructions = 1000; // Safety limit

    for instruction_count in 0..max_instructions {
        // Print CPU state before execution
        print_cpu_state(&emulator, instruction_count);

        // Execute one instruction
        let cycles = emulator.step();
        total_cycles += cycles as u64;

        // Check if halted
        if emulator.is_halted() {
            println!("\n=== HALTED ===");
            println!("Total instructions executed: {}", instruction_count + 1);
            println!("Total cycles: {}", total_cycles);
            print_cpu_state(&emulator, instruction_count + 1);
            break;
        }
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
