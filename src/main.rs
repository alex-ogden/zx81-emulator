use std::env;
use std::process;

use zx81_emulator::{Emulator, memory::load_rom};

fn main() {
    let args: Vec<String> = env::args().collect();

    // We need a minimum of 2 args
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <rom_file> [--debug] [--video-debug] [--rev-video]",
            args[0]
        );
        process::exit(1);
    }

    // Check if debug is enabled
    let debug_enabled: bool = args.contains(&"--debug".to_string());
    let rev_video: bool = args.contains(&"--rev-video".to_string());

    if debug_enabled {
        println!("Debug mode enabled...");
    } else {
        println!("Debug mode disabled...");
    }

    if rev_video {
        println!("Video colour reversal enabled...");
    } else {
        println!("Video colour reversal disabled...");
    }

    // Remove --debug from args if it did exist
    let args: Vec<String> = args
        .into_iter()
        .filter(|arg| arg != "--debug" && arg != "--rev-video")
        .collect();

    // Load ROM file from args[1]
    let rom = match load_rom(&args[1]) {
        Ok(data) => {
            println!("Loaded ROM: {} ({} bytes)", args[1], data.len());

            // == ROM Sanity Checks == //

            if data.len() != 8192 {
                eprintln!("WARNING: ROM size is {} bytes, expected 8192", data.len());
            }

            if data.len() > 3 {
                println!(
                    "ROM Signature: {:02X} {:02X} {:02X}",
                    data[0], data[1], data[2]
                );
            }

            data
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let mut emulator = match Emulator::new(rom, debug_enabled, rev_video) {
        Ok(emu) => emu,
        Err(e) => {
            eprintln!("Failed to create emulator: {}", e);
            process::exit(1);
        }
    };

    println!("Starting emulation...\n");

    const CYCLES_PER_FRAME: u64 = 65000; // ~3.25MHz / 50Hz
    const INIT_FRAMES: u32 = 20; // Wait 20 frames (~400ms) before rendering

    let mut total_cycles = 0u64;
    let mut frame_count = 0u32;
    let mut _frames_since_init = 0u32;

    while emulator.is_window_open() {
        let target_cycles = total_cycles + CYCLES_PER_FRAME;
        let mut frame_instruction_count = 0;

        while total_cycles < target_cycles {
            let cycles = emulator.step();
            total_cycles += cycles as u64;
            frame_instruction_count += 1;

            if emulator.is_halted() {
                break;
            }

            // Safety check to prevent infinite loops
            if frame_instruction_count > 100000 {
                eprintln!("WARNING: Too many instructions in one frame!");
                break;
            }
        }

        frame_count += 1;

        // Wait for init period
        if frame_count < INIT_FRAMES {
            if frame_count % 5 == 0 {
                println!("Initialising... frame {}/{}", frame_count, INIT_FRAMES);
            }

            // Still update display to keep window responsive
            emulator.update_display().unwrap_or_else(|e| {
                eprintln!("Error updating display: {}", e);
            });
        } else {
            // Start rendering
            if frame_count == INIT_FRAMES {
                println!("Initialisation complete! Start display rendering");
                println!("Total cycles executed: {}", total_cycles);
            }

            _frames_since_init += 1;

            // Get keyboard input
            emulator.update_keyboard();
            // Render display
            emulator
                .render_display()
                .unwrap_or_else(|e| eprintln!("Display error: {}", e));
        }

        // Maintain ~50Hz refresh rate (ZX81 standard)
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    println!("\nEmulation stopped.");
    println!("Total frames: {}", frame_count);
    println!("Total cycles: {}", total_cycles);
}
