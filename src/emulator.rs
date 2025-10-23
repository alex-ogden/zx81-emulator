use crate::cpu::Cpu;
use crate::io::IoController;
use crate::memory::Memory;
use crate::video::Video;

// Main emulator state machine
pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    video: Video,
    io: IoController,
    cycles: u64,
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(rom),
            video: Video::new(),
            io: IoController::new(),
            cycles: 0,
        }
    }

    pub fn step(&mut self) {
        let cycles = self.cpu.step(&mut self.memory);
        self.cycles += cycles as u64;
        // TODO: Handle video timing
        // TODO: Handle interrupts
    }

    pub fn run_frame(&mut self) {
        // TODO: Run emulation for one frame (1/50th second)
        // TODO: Generate display
    }
}
