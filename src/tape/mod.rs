use std::fs;
use std::process;

pub struct Tape {
    pub data: Vec<u8>,
}

impl Tape {
    pub fn new(path: &str) -> Self {
        let data = match load_p_file(path) {
            Ok(data) => {
                println!("INFO: Loaded tape: {}", path);
                data
            }
            Err(e) => {
                eprintln!("ERROR: Error loading tape: {}", e);
                process::exit(1);
            }
        };

        Self { data }
    }
}

fn load_p_file(path: &str) -> Result<Vec<u8>, String> {
    println!("INFO: Loading tape: {}", path);
    let tape_data = fs::read(path).map_err(|e| format!("Failed to read tape file: {}", e))?;

    // Do an initial sanity check on tape_data length (must be at least big enough for headers)
    if tape_data.len() < 17 {
        return Err(format!(
            "ERROR: Tape data length must be at least 17 bytes. Found: {}",
            tape_data.len()
        ));
    }

    /* Tape layout:
    -> Bytes:
        -> 0-1:     File type (0x00 is a program)
        -> 2-9:     Fileanme: (8 chars, space padded)
        -> 10-11:   Program length
        -> 12-13:   Autostart line number (0x8000 if none?)
        -> 14-15:   Program length again
        -> 16:      Checksum (XOR of all bytes so far) */

    // Verify program
    println!("INFO: Verifying program bytes");
    let program_bytes: u8 = tape_data[0];
    if program_bytes != 0x00 {
        return Err(format!(
            "ERROR: P file is not a program. First bytes: {:04X}",
            program_bytes
        ));
    }

    // Get the program length
    println!("INFO: Getting program length");
    let program_length: u16 = (tape_data[11] as u16) << 8 | tape_data[10] as u16;
    println!("INFO: Program length: {}", program_length);

    // Validate program length
    if tape_data.len() >= 17 + program_length as usize {
        println!("INFO: Program length validated");
    } else {
        println!(
            "WARNING: Program length does not match file actual length. Expected: {}, Actual: {}",
            (program_length + 17),
            tape_data.len()
        );
    }

    // Get and check the checksum
    println!("INFO: Computing and verifying checksum");
    let program_checksum: u8 = tape_data[16];
    // Iterate through the tape header data and apply XOR to each byte, storing the result in 'acc'
    let xor_result: u8 = tape_data[0..16].iter().fold(0u8, |acc, &b| acc ^ b);
    // Ensure checksum passes
    if program_checksum != xor_result {
        println!(
            "WARNING: P file checksum doesn't match! Checksum: {}, Actual: {}",
            program_checksum, xor_result
        );
    } else {
        println!("INFO: P file checksum matched!")
    }

    // Finally extract the program data and return to be loaded into memory
    let data_start = 17;
    let data_end = std::cmp::min(data_start + program_length as usize, tape_data.len());
    let program_data: Vec<u8> = tape_data[data_start..data_end].to_vec();
    Ok(program_data)
}
