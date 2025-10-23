use std::env;
use std::process;

use crate::cpu::Cpu;
use crate::memory::Memory;

fn main() {
    let args: Vec<String> = env::args().collect();

    // We need a minimum of 2 args
    if args.len() < 2 {
        eprintln!("Too few arguments! Expected {}, got {}", 2, args.len());
        process::exit(1);
    }

    let cpu = Cpu::new();
    let memory = Memory::new(); // pass rom file 
}
