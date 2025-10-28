use minifb::Error;

use crate::cpu::Cpu;
use crate::io::IoController;
use crate::memory::Memory;
use crate::video::Video;

pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    video: Video,
    io: IoController,
    cycles: u64,
}

impl Emulator {
    pub fn new(rom: Vec<u8>, debug_enabled: bool) -> Result<Self, minifb::Error> {
        Ok(Self {
            cpu: Cpu::new(),
            memory: Memory::new(rom),
            video: Video::new(debug_enabled)?,
            io: IoController::new(),
            cycles: 0,
        })
    }

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

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted
    }

    pub fn total_cycles(&self) -> u64 {
        self.cycles
    }

    pub fn update_display(&mut self) -> Result<(), minifb::Error> {
        self.video.update()
    }

    pub fn is_window_open(&self) -> bool {
        self.video.is_open()
    }

    pub fn render_display(&mut self) -> Result<(), minifb::Error> {
        self.video.render(&self.memory, self.memory.rom());
        self.video.update()
    }
}
