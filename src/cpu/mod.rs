

//! Z80 CPU implementation
//!
//! This module implements the Zilog Z80 CPU core used in the ZX81.
//! The Z80 is an 8-bit microprocessor with a 16-bit address bus.
//!
//! ## Features
//!
//! - Full register set (A, F, B, C, D, E, H, L, IX, IY, SP, PC)
//! - Shadow registers (AF', BC', DE', HL')
//! - 8 flags (S, Z, Y, H, X, P/V, N, C) including undocumented X and Y
//! - Cycle-accurate instruction timing
//! - Pattern-based instruction decoding

use crate::memory::Memory;

mod cb_instructions;
mod dd_instructions;
mod ed_instructions;
mod fd_instructions;
mod instructions;
mod registers;

/// Z80 CPU state
///
/// Represents the complete state of the Z80 microprocessor, including
/// all registers, flags, and execution state.
///
/// ## Registers
///
/// The Z80 has several register sets:
/// - **Main registers**: A (accumulator), F (flags), B, C, D, E, H, L
/// - **Shadow registers**: AF', BC', DE', HL' (for fast context switching)
/// - **Index registers**: IX, IY (16-bit addressing)
/// - **Special registers**: I (interrupt vector), R (memory refresh)
/// - **Pointers**: SP (stack pointer), PC (program counter)
///
/// ## Flags (in F register)
///
/// ```text
/// Bit 7: S  - Sign flag
/// Bit 6: Z  - Zero flag
/// Bit 5: X  - Undocumented (copy of bit 5 of result)
/// Bit 4: H  - Half-carry flag
/// Bit 3: Y  - Undocumented (copy of bit 3 of result)
/// Bit 2: P/V - Parity/Overflow flag
/// Bit 1: N  - Add/Subtract flag
/// Bit 0: C  - Carry flag
/// ```
pub struct Cpu {
    // Z80 CPU @ 3.25MHz
    // Registers
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    // Shadow registers
    pub af_shadow: u16,
    pub bc_shadow: u16,
    pub de_shadow: u16,
    pub hl_shadow: u16,
    // Index registers
    pub ix: u16,
    pub iy: u16,
    // Interrupt and memory registers
    pub i: u8,
    pub r: u8,
    // Interrupt flip-flops
    pub iff1: bool,
    pub iff2: bool,
    // Stack pointer and program counter
    pub sp: u16,
    pub pc: u16,
    // Holds whether the CPU is halted
    pub is_halted: bool,
}

impl Cpu {
    /// Creates a new CPU with initialized state
    ///
    /// The CPU starts with:
    /// - All registers set to 0
    /// - Stack pointer at 0xFFFF
    /// - Program counter at 0x0000
    /// - Not halted
    ///
    /// # Examples
    ///
    /// ```
    /// use zx81_emulator::cpu::Cpu;
    ///
    /// let cpu = Cpu::new();
    /// assert_eq!(cpu.pc, 0x0000);
    /// assert_eq!(cpu.sp, 0xFFFF);
    /// ```
    pub fn new() -> Self {
        Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            af_shadow: 0,
            bc_shadow: 0,
            de_shadow: 0,
            hl_shadow: 0,
            ix: 0,
            iy: 0,
            i: 0,
            r: 0,
            iff1: false,
            iff2: false,
            sp: 0xFFFF,
            pc: 0x0000,
            is_halted: false,
        }
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let addr = self.pc;
        let byte = memory.read(addr);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let lo = self.fetch_byte(memory) as u16;
        let hi = self.fetch_byte(memory) as u16;
        (hi << 8) | lo
    }

    /// Executes a single instruction
    ///
    /// Fetches the next instruction from memory at PC, executes it,
    /// and returns the number of T-cycles consumed.
    ///
    /// # Arguments
    ///
    /// * `memory` - Reference to the memory system
    ///
    /// # Returns
    ///
    /// Number of T-cycles (clock cycles) consumed by the instruction
    ///
    /// # Examples
    ///
    /// ```
    /// use zx81_emulator::{cpu::Cpu, memory::Memory};
    ///
    /// let mut cpu = Cpu::new();
    /// let mut memory = Memory::new(vec![0; 0x2000]);
    /// let cycles = cpu.step(&mut memory);
    /// ```
    pub fn step(&mut self, memory: &mut Memory) -> u8 {
        // Is the system halted?
        if self.is_halted {
            // For now just return 4 cycles
            return 4;
        }

        // Retrieve the opcode in the memory where our program counter currently is
        // PC is incremented in fetch_byte automatically
        let opcode = self.fetch_byte(memory);
        let cycles = self.execute(opcode, memory);
        cycles
    }
    fn execute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        // execute_instruction is found in instructions.rs
        self.execute_instruction(opcode, memory)
    }
}
