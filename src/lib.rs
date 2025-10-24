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
//! - 1KB base RAM (expandable)
//! - Character-based display (32×24)
//! - CPU-driven video generation (SLOW mode)
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
//! - [`video`] - Display generation and character rendering
//! - [`io`] - Input/output (keyboard, tape)
//! - [`platform`] - Platform-specific backends (display, audio)
//! - [`emulator`] - Main emulator coordination

pub mod cpu;
pub mod memory;
pub mod video;
pub mod io;
pub mod platform;
pub mod emulator;

pub use emulator::Emulator;
