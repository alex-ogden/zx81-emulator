# 🎮 ZX81 Emulator

A cycle-accurate Sinclair ZX81 emulator written in Rust, implementing the legendary Z80 CPU and the quirky hardware of this 1981 home computer.

[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-red.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

## 📖 About the ZX81

The Sinclair ZX81 was a revolutionary home computer released in 1981, bringing computing to the masses with its £49.95 price tag. It featured:

- **Z80 CPU** @ 3.25 MHz
- **8KB ROM** containing BASIC interpreter
- **1KB RAM** (expandable to 16KB/64KB)
- **Character-based display** (32×24 text)
- **Two speed modes**: FAST (full CPU speed) and SLOW (display generation)
- **Cassette tape** for program storage

## ✨ Features

### Currently Implemented

- ✅ **Z80 CPU Core**
  - ~76 opcodes implemented
  - Accurate flag handling (including undocumented X/Y flags)
  - Cycle-accurate timing

- ✅ **Memory System**
  - 8KB ROM support
  - 1KB base RAM (expandable)
  - Correct address mapping (ROM: 0x0000-0x1FFF, RAM: 0x4000+)
  - Bounds checking for memory operations

- ✅ **Instruction Set** (Partial)
  - Data movement: LD, PUSH, POP
  - Arithmetic: INC, DEC (8-bit and 16-bit)
  - Control flow: JP, JR, DJNZ, CALL, RET
  - Register operations: Full register-to-register transfers
  - Memory operations: Indirect addressing via (HL)

- ✅ **Development Tools**
  - ROM loading from file
  - Test ROM generator (Python script)
  - Automated test suite
  - CPU state inspector

### 🚧 In Progress

- Video system (character rendering, display file parsing)
- Keyboard input (8×5 matrix)
- Full Z80 instruction set
- Interrupt system (NMI for display timing)

### 📋 Planned

- Tape loading (.p and .81 files)
- SLOW mode display generation
- Sound (tape interface audio)
- Debugger with breakpoints
- Save states

## 🚀 Quick Start

### Prerequisites

- Rust (install from [rustup.rs](https://rustup.rs))
- A ZX81 ROM file (8KB, usually named `zx81.rom`)

### Building

```bash
# Clone the repository
git clone https://github.com/alex-ogden/zx81-emulator.git
cd zx81-emulator

# Build in release mode
cargo build --release

# Run with a ROM file
cargo run --release path/to/your/rom.rom
```

### Running Tests

```bash
# Generate test ROMs
python3 test_roms/make_test_rom.py

# Run all tests
./run_tests.sh

# Or run individual tests
cargo run --release test_roms/01_nop_halt.rom
```

## 📚 Project Structure

```
zx81-emulator/
├── src/
│   ├── cpu/              # Z80 CPU implementation
│   │   ├── mod.rs        # Core CPU state and execution loop
│   │   ├── instructions.rs   # Standard instruction implementations
│   │   ├── cb_instructions.rs  # CB-prefixed opcodes (bit operations)
│   │   ├── ed_instructions.rs  # ED-prefixed opcodes (extended)
│   │   ├── dd_instructions.rs  # DD-prefixed opcodes (IX operations)
│   │   ├── fd_instructions.rs  # FD-prefixed opcodes (IY operations)
│   │   └── registers.rs  # Register helpers and flag operations
│   ├── memory/           # Memory management
│   │   ├── mod.rs        # Memory bus and address decoding
│   │   ├── rom.rs        # ROM loading
│   │   └── ram.rs        # RAM expansion handling
│   ├── video/            # Display system
│   │   ├── mod.rs        # Video subsystem
│   │   ├── display_file.rs   # D_FILE parser
│   │   ├── character_set.rs  # Character rendering
│   │   └── renderer.rs   # Pixel generation
│   ├── io/               # Input/Output
│   │   ├── mod.rs        # I/O port handling
│   │   ├── keyboard.rs   # Keyboard matrix
│   │   └── tape.rs       # Tape interface
│   ├── platform/         # Platform abstraction
│   │   ├── mod.rs        # Platform layer
│   │   ├── minifb.rs     # Display backend
│   │   └── audio.rs      # Audio output
│   ├── emulator.rs       # Main emulator coordination
│   ├── lib.rs            # Library root
│   └── main.rs           # CLI entry point
├── test_roms/            # Test ROM generation
│   └── make_test_rom.py  # Python script to create test ROMs
├── tests/                # Integration tests
└── run_tests.sh          # Test runner script
```

## 🎯 Implemented Instructions

### Data Movement
- `LD r, r'` - Register to register (63 opcodes)
- `LD r, n` - Load immediate into register (8 opcodes)
- `LD rr, nn` - Load 16-bit immediate (4 opcodes)
- `LD (HL), r` / `LD r, (HL)` - Memory operations via HL

### Arithmetic & Logic
- `INC r` / `DEC r` - 8-bit increment/decrement (14 opcodes)
- `INC rr` / `DEC rr` - 16-bit increment/decrement (8 opcodes)
- `INC (HL)` / `DEC (HL)` - Memory increment/decrement

### Control Flow
- `NOP` - No operation
- `HALT` - Stop execution
- `JP nn` - Unconditional jump
- `DJNZ d` - Decrement B and jump if not zero

### Status
**Total:** ~76 Z80 opcodes implemented

For a complete list, see the [instruction implementation guide](docs/INSTRUCTIONS.md).

## 🧪 Testing

The project includes a comprehensive test suite with multiple levels:

### Test ROMs
Pre-built test programs that verify instruction correctness:
- `01_nop_halt.rom` - Basic execution
- `02_load_immediate.rom` - LD r, n instructions
- `03_increment.rom` - INC operations with flags
- `04_decrement.rom` - DEC operations
- `05_16bit_ops.rom` - 16-bit arithmetic
- `06_jump.rom` - Jump instructions
- `07_djnz_loop.rom` - Loop instruction
- `08_ld_r_r.rom` - Register transfers
- `09_ld_memory.rom` - Memory operations

### Creating Your Own Tests

```python
# Edit test_roms/make_test_rom.py
def test_my_program():
    program = [
        0x3E, 0x42,  # LD A, 0x42
        0x3C,        # INC A
        0x76,        # HALT
    ]
    make_rom(program, "test_roms/my_test.rom")

# Then regenerate
python3 test_roms/make_test_rom.py
```

## 🔧 Development

### Building Documentation

```bash
# Generate and open rustdoc documentation
cargo doc --open
```

### Code Style

This project follows standard Rust conventions:
- Run `cargo fmt` before committing
- Run `cargo clippy` for linting
- Maintain test coverage for new instructions

### Adding New Instructions

1. Implement the instruction in `src/cpu/instructions.rs`
2. Add the opcode to the match statement in `execute_instruction()`
3. Create a test ROM in `test_roms/make_test_rom.py`
4. Run the test suite to verify

Example:
```rust
fn add_a_r(&mut self, opcode: u8) -> u8 {
    let src_code = opcode & 0x07;
    let value = self.read_reg(src_code, memory);

    // Perform addition with flag updates
    let result = self.a.wrapping_add(value);

    // Update flags
    self.set_flag_z(result == 0);
    self.set_flag_n(false);
    // ... etc

    self.a = result;
    4  // Cycles
}
```

## 📊 Performance

The emulator aims for cycle-accurate emulation:
- Each instruction returns the correct T-state count
- Memory operations account for timing differences
- Future: Frame-accurate display timing (50Hz PAL)

Current performance: ~100,000+ instructions per second (debug mode on modern hardware)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The ZX81 community for preservation efforts and documentation
- [Z80 CPU User Manual](http://www.zilog.com/docs/z80/um0080.pdf) - Official instruction reference
- Sean Young's [Z80 documentation](https://worldofspectrum.org/faq/reference/z80reference.htm)
- The Rust community for excellent tooling and libraries

## 📚 Resources

- [ZX81 Technical Information](http://www.user.dccnet.com/wrigter/index_files/zx81tech.htm)
- [ZX81 ROM Disassembly](http://www.user.dccnet.com/wrigter/index_files/zx81rom.htm)
- [Z80 Instruction Set](https://clrhome.org/table/)
- [ZX81 Emulation Guide](http://www.z80.info/z80info.htm)

## 🤖 AI usage

- AI has been used in this project to generate and update documentation (including this README you're reading now!)
- AI has been used as a research tool for learning about the ZX81, summarising large data sheets, and keeping track of where I'm up to with implementation
- AI has not been used to implement core functionality.
---

**Status:** 🚧 In Development | **Version:** 0.1.0 | **Last Updated:** October 2025
