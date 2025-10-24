//! Main emulator coordination
//!
//! The [`Emulator`] struct ties together all subsystems (CPU, memory, video, I/O)
//! and coordinates their execution.

use crate::cpu::Cpu;
use crate::io::IoController;
use crate::memory::Memory;
use crate::video::Video;

/// Main emulator state
///
/// Coordinates the CPU, memory, video, and I/O subsystems to emulate
/// a complete ZX81 system.
///
/// # Examples
///
/// ```no_run
/// use zx81_emulator::{Emulator, memory::load_rom};
///
/// let rom = load_rom("zx81.rom").unwrap();
/// let mut emulator = Emulator::new(rom);
///
/// while !emulator.is_halted() {
///     emulator.step();
/// }
/// ```
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    video: Video,
    io: IoController,
    cycles: u64,
}

impl Emulator {
    /// Creates a new emulator instance
    ///
    /// # Arguments
    ///
    /// * `rom` - ZX81 ROM data (8KB)
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(rom),
            video: Video::new(),
            io: IoController::new(),
            cycles: 0,
        }
    }

    /// Executes a single CPU instruction
    ///
    /// # Returns
    ///
    /// Number of T-cycles consumed
    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.memory);
        self.cycles += cycles as u64;
        // TODO: Handle video timing
        // TODO: Handle interrupts
        cycles
    }

    pub fn run_frame(&mut self) {
        // TODO: Run emulation for one frame (1/50th second)
        // TODO: Generate display
    }

    /// Returns a reference to the CPU state
    ///
    /// Useful for inspecting registers and flags during testing.
    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    /// Checks if the CPU is halted
    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted
    }

    /// Returns total T-cycles executed
    pub fn total_cycles(&self) -> u64 {
        self.cycles
    }
}
