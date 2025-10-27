//! # ZX81 Emulator Library
//!
//! A cycle-accurate emulator for the Sinclair ZX81 home computer, implementing
//! the Z80 CPU and the unique hardware characteristics of this 1981 machine.
//!
//! ## Overview
//!
//! The ZX81 was notable for its minimalist design:
//! - Z80 CPU @ 3.25MHz
//! - 8KB ROM containing BASIC interpreter
//! - 1KB base RAM (expandable to 16KB)
//! - Character-based display (32×24)
//! - CPU-driven video generation (SLOW mode)
//!
//! ## Current Implementation Status
//!
//! ### Z80 CPU Core
//! - **204 opcodes implemented** (80% of main instruction set)
//! - Accurate flag handling including undocumented X/Y flags
//! - Cycle-accurate timing
//! - Shadow register support (AF', BC', DE', HL')
//! - All common instructions for BASIC ROM execution
//!
//! ### Implemented Instruction Groups
//! - ✅ Data movement (LD variants, PUSH, POP, EX)
//! - ✅ 8-bit arithmetic (ADD, ADC, SUB, SBC, INC, DEC)
//! - ✅ 16-bit arithmetic (ADD HL, INC rr, DEC rr)
//! - ✅ Logical operations (AND, OR, XOR, CP)
//! - ✅ Control flow (JP, JR, CALL, RET, DJNZ, RST)
//! - ✅ Exchange instructions (EX DE,HL, EXX, etc.)
//! - ✅ I/O operations (IN, OUT)
//! - ✅ Special operations (DI, EI, SCF, CCF, NOP, HALT)
//!
//! ### Not Yet Implemented
//! - CB-prefixed (bit operations, rotates, shifts)
//! - ED-prefixed (block operations, 16-bit I/O)
//! - DD/FD-prefixed (IX/IY index registers)
//!
//! ## Quick Example
//!
//! ```no_run
//! use zx81_emulator::{Emulator, memory::load_rom};
//!
//! // Load a ZX81 ROM
//! let rom = load_rom("zx81.rom").expect("Failed to load ROM");
//!
//! // Create emulator instance
//! let mut emulator = Emulator::new(rom);
//!
//! // Execute instructions until HALT
//! while !emulator.is_halted() {
//!     emulator.step();
//! }
//! ```
//!
//! ## Architecture
//!
//! The emulator is organized into several subsystems:
//!
//! - [`cpu`] - Z80 CPU core with instruction execution
//! - [`memory`] - Memory management (ROM/RAM)
//! - [`video`] - Display generation and character rendering (stub)
//! - [`io`] - Input/output (keyboard, tape) (stub)
//! - [`platform`] - Platform-specific backends (display, audio) (stub)
//! - [`emulator`] - Main emulator coordination
//!
//! ## Testing
//!
//! The emulator includes 29 comprehensive test ROMs covering all implemented
//! instructions. Run tests with:
//!
//! ```bash
//! ./run_tests.sh
//! ```
//!
//! All tests verify correct instruction execution, flag updates, and cycle timing.

pub mod cpu;
pub mod emulator;
pub mod io;
pub mod memory;
pub mod platform;
pub mod video;

pub use emulator::Emulator;
