use std::fs;
use std::process;

const CLOCK_HZ: u64 = 3_250_000;
const LEADER_SEC: u64 = 5;
const LEADER_T: u64 = LEADER_SEC * CLOCK_HZ;
const PULSE_HIGH_T: u64 = 488;
const PULSE_LOW_T: u64 = 487;
const SILENCE_BIT_T: u64 = 4225;
const PAUSE_END_T: u64 = CLOCK_HZ;

pub struct Tape {
    pub data: Vec<u8>, // Raw .p bytes
    pub pulses: Vec<(bool, u64)>,
    pub playing: bool,
    pub current_index: usize,
    pub remaining: u64,
    pub level: bool,
}

impl Tape {
    pub fn new(path: &str) -> Self {
        let data = match load_p_file(path) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("ERROR: Error loading tape: {}", e);
                process::exit(1);
            }
        };

        let pulses = Self::generate_pulses(&data);
        println!(
            "Tape: {} bytes, {} pulses, (leader={} cycles)",
            data.len(),
            pulses.len(),
            LEADER_T
        );

        Self {
            data,
            pulses,
            playing: false,
            current_index: 0,
            remaining: 0,
            level: false,
        }
    }

    fn generate_pulses(data: &Vec<u8>) -> Vec<(bool, u64)> {
        let mut pulses = Vec::new();
        pulses.push((false, LEADER_T)); // Long intro silence, low 

        // Add sync pulses after leader
        pulses.push((true, PULSE_HIGH_T));
        pulses.push((false, PULSE_LOW_T * 2));

        // Add dummy name: Single inverse character (0xA6) to signal end-of-name
        let dummy_name: Vec<u8> = vec![0xA6];

        // First, process dummy name bytes
        for &byte in dummy_name.iter() {
            for bit_pos in (0..8).rev() {
                // MSB-first
                let bit = ((byte >> bit_pos) & 1) != 0;
                let num_pulses = if bit { 9 } else { 4 };
                for _ in 0..num_pulses {
                    pulses.push((true, PULSE_HIGH_T));
                    pulses.push((false, PULSE_LOW_T));
                }
                // Inter-bit silence
                pulses.push((false, SILENCE_BIT_T));
            }
        }

        for &byte in data.iter() {
            for bit_pos in (0..8).rev() {
                // MSB first format
                let bit = ((byte >> bit_pos) & 1) != 0;
                let num_pulses = if bit { 9 } else { 4 };
                for _ in 0..num_pulses {
                    pulses.push((true, PULSE_HIGH_T));
                    pulses.push((false, PULSE_LOW_T));
                }

                // Add silence between bits
                pulses.push((false, SILENCE_BIT_T));
            }
        }

        // End pause
        pulses.push((false, PAUSE_END_T));
        pulses
    }

    pub fn start_playing(&mut self) {
        if !self.pulses.is_empty() {
            self.playing = true;
            self.current_index = 0;
            self.remaining = self.pulses[0].1;
            self.level = self.pulses[0].0;
            println!("Tape playing started!");
        }
    }

    pub fn advance(&mut self, cycles: u64) {
        if !self.playing {
            return;
        }
        let mut remaining_cycles = cycles;
        while remaining_cycles > 0 {
            if self.remaining > remaining_cycles {
                self.remaining -= remaining_cycles;
                break;
            }
            remaining_cycles -= self.remaining;
            self.current_index += 1;

            if self.current_index >= self.pulses.len() {
                self.playing = false;
                self.level = false; // Default to low 
                println!("Tape ended");
                break;
            }
            self.level = self.pulses[self.current_index].0;
            self.remaining = self.pulses[self.current_index].1;
            // Debug for visibility
            if self.current_index % 100 == 0 {
                println!(
                    "Tape progress: index={}, level={}, remaining={}",
                    self.current_index, self.level, self.remaining
                );
            }
        }
    }

    pub fn get_level(&self) -> bool {
        self.level
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }
}

fn load_p_file(path: &str) -> Result<Vec<u8>, String> {
    println!("INFO: Loading tape: {}", path);
    let tape_data = fs::read(path).map_err(|e| format!("Failed to read tape file: {}", e))?;

    let has_header = tape_data.len() > 128 && tape_data[0] == 0x00;

    let program_data: Vec<u8> = if has_header {
        println!("INFO: Detected .p file with header");
        tape_data[128..].to_vec()
    } else {
        println!("INFO: Detected .p file without header");
        tape_data.clone()
    };

    println!("INFO: P file size: {}", tape_data.len());
    println!("INFO: Program data size: {}", program_data.len());

    Ok(tape_data)
}
