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
    pub fn new(rom: Vec<u8>, debug_enabled: bool, rev_video: bool) -> Result<Self, minifb::Error> {
        Ok(Self {
            cpu: Cpu::new(),
            memory: Memory::new(rom),
            video: Video::new(debug_enabled, rev_video)?,
            io: IoController::new(),
            cycles: 0,
        })
    }

    pub fn step(&mut self) -> u8 {
        let cycles = self.cpu.step(&mut self.memory, &mut self.io);
        self.cycles += cycles as u64;
        // TODO: Handle video timing
        // TODO: Handle interrupts
        cycles
    }

    pub fn dump_system_vars(&self) {
        println!("\n=== ZX81 System Variables ===");
        let d_file = self.memory.read_word(0x400C);
        let vars = self.memory.read_word(0x4010);

        println!("D_FILE (0x400C): 0x{:04X}", d_file);
        println!("VARS   (0x4010): 0x{:04X}", vars);

        // Check display file structure
        if d_file >= 0x4000 && d_file < 0x8000 {
            let mut newlines = 0;
            for i in 0..800 {
                if self.memory.read(d_file + i) == 0x76 {
                    newlines += 1;
                }
            }
            println!("Newlines in D_FILE: {}", newlines);
        }
        println!("=============================\n");
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
