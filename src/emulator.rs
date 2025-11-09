use crate::cpu::Cpu;
use crate::io::IoController;
use crate::memory::Memory;
use crate::tape::Tape;
use crate::video::Video;

pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    video: Video,
    io: IoController,
    cycles: u64,
    pub tape: Option<Tape>,
}

impl Emulator {
    pub fn new(rom: Vec<u8>, debug_enabled: bool, rev_video: bool) -> Result<Self, minifb::Error> {
        Ok(Self {
            cpu: Cpu::new(),
            memory: Memory::new(rom),
            video: Video::new(debug_enabled, rev_video)?,
            io: IoController::new(),
            cycles: 0,
            tape: None,
        })
    }

    pub fn load_tape(&mut self, tape: Tape) {
        self.tape = Some(tape);
    }

    pub fn step(&mut self) -> u8 {
        let tape_ref = &self.tape;
        let cycles = self.cpu.step(&mut self.memory, &mut self.io, tape_ref);
        if let Some(t) = &mut self.tape {
            t.advance(cycles as u64);
        }
        self.cycles += cycles as u64;
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

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn cpu_mut(&mut self) -> &mut Cpu {
        &mut self.cpu
    }

    pub fn video(&self) -> &Video {
        &self.video
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
        self.video
            .render(&self.memory, self.memory.rom(), &self.cpu);
        self.video.update()
    }

    pub fn update_keyboard(&mut self) {
        let keys = self.video.get_keys();
        self.io.update_keys(&keys);
    }
}
