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
  - **250+ opcodes implemented** across all prefix groups
  - Accurate flag handling (including undocumented X/Y flags)
  - Cycle-accurate timing
  - Shadow register support (AF', BC', DE', HL')
  - Index registers (IX, IY)
  - Interrupt registers (I, R)
  - Can successfully run real ZX81 ROM

- ✅ **Memory System**
  - 8KB ROM support
  - 1KB base RAM (expandable to 16KB)
  - Correct address mapping (ROM: 0x0000-0x1FFF, RAM: 0x4000-0x7FFF)
  - Bounds checking for memory operations
  - 16-bit word read/write operations

- ✅ **Instruction Set** (90%+ complete)
  - **Data movement**: LD (all variants), PUSH, POP, EX, LDI, LDIR, LDDR
  - **8-bit arithmetic**: ADD, ADC, SUB, SBC, INC, DEC, CP
  - **16-bit arithmetic**: ADD HL,rr, INC rr, DEC rr, SBC HL,rr
  - **Logical operations**: AND, OR, XOR, CP (register and immediate)
  - **Rotate/shift**: RLA, RRA, RLCA, RRCA, RL, RR, RLC, RRC, SRL
  - **Bit operations**: BIT, SET, RES (all CB-prefixed)
  - **Control flow**: JP, JR, CALL, RET (all conditional variants), DJNZ, RST
  - **Exchange**: EX DE,HL, EX AF,AF', EXX, EX (SP),HL
  - **Index operations**: IX/IY addressing (DD/FD-prefixed)
  - **Block operations**: LDI, LDIR, CPIR, LDDR
  - **Special**: NOP, HALT, DI, EI, SCF, CCF, IM, IN, OUT, LD I/R,A, LD A,I/R, CPL

- ✅ **Video System**
  - Window creation via minifb
  - Display file rendering infrastructure
  - Character set decoding from ROM
  - Scale factor support (2x default)

- ✅ **Development Tools**
  - ROM loading from file
  - Test ROM generator (Python script)
  - **29 comprehensive test ROMs**
  - Automated test suite (all passing)
  - CPU state inspector

### 🚧 In Progress

- Video display output (infrastructure complete, debugging rendering)
- Keyboard input (8×5 matrix)
- Interrupt system (NMI for display timing)
- On-screen debugger (optional --debug flag support)

### 📋 Planned

- Complete remaining Z80 opcodes
- Tape loading (.p and .81 files)
- SLOW mode display generation
- Sound (tape interface audio)
- Debugger with breakpoints and step-through
- Save states
- Configurable memory expansion (16KB/64KB)

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

# Run with optional debug panel
cargo run --release --debug path/to/your/rom.rom
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

### Data Movement (70+ opcodes)
- `LD r, r'` - Register to register (63 opcodes)
- `LD r, n` - Load immediate into register (8 opcodes)
- `LD rr, nn` - Load 16-bit immediate (4 opcodes)
- `LD (HL), r` / `LD r, (HL)` - Memory operations via HL
- `LD (BC), A` / `LD A, (BC)` - Indirect via BC
- `LD (DE), A` / `LD A, (DE)` - Indirect via DE
- `LD (nn), A` / `LD A, (nn)` - Direct memory addressing
- `LD (nn), HL` / `LD HL, (nn)` - 16-bit memory operations
- `PUSH rr` / `POP rr` - Stack operations (8 opcodes)

### Arithmetic (50+ opcodes)
- `ADD A, r` / `ADD A, n` - 8-bit addition (16 opcodes)
- `ADC A, r` / `ADC A, n` - Add with carry (16 opcodes)
- `SUB r` / `SUB n` - 8-bit subtraction (16 opcodes)
- `SBC A, r` / `SBC A, n` - Subtract with carry (16 opcodes)
- `INC r` / `DEC r` - 8-bit increment/decrement (14 opcodes)
- `INC rr` / `DEC rr` - 16-bit increment/decrement (8 opcodes)
- `INC (HL)` / `DEC (HL)` - Memory arithmetic
- `ADD HL, rr` - 16-bit addition (4 opcodes)

### Logical Operations (40+ opcodes)
- `AND r` / `AND n` - Logical AND (16 opcodes)
- `OR r` / `OR n` - Logical OR (16 opcodes)
- `XOR r` / `XOR n` - Logical XOR (16 opcodes)
- `CP r` / `CP n` - Compare (16 opcodes)

### Control Flow (40+ opcodes)
- `NOP` - No operation
- `HALT` - Stop execution
- `JP nn` - Unconditional jump
- `JP cc, nn` - Conditional jumps (8 opcodes)
- `JR e` - Relative jump
- `JR cc, e` - Conditional relative jumps (4 opcodes)
- `DJNZ e` - Decrement B and jump if not zero
- `CALL nn` - Unconditional call
- `CALL cc, nn` - Conditional calls (8 opcodes)
- `RET` - Unconditional return
- `RET cc` - Conditional returns (8 opcodes)
- `RST n` - Restart to fixed address (8 opcodes)

### Exchange (4 opcodes)
- `EX DE, HL` - Exchange DE and HL
- `EX AF, AF'` - Exchange with shadow AF
- `EXX` - Exchange BC, DE, HL with shadows
- `EX (SP), HL` - Exchange HL with top of stack

### Special Operations
- `DI` / `EI` - Disable/Enable interrupts
- `SCF` / `CCF` - Set/Complement carry flag
- `IN A, (n)` / `OUT (n), A` - I/O operations

### Status
**Total: 250+ Z80 opcodes implemented** (90%+ of instruction set including prefix groups)

### Recently Implemented
- ✅ CB-prefixed instructions (bit operations, rotates, shifts)
- ✅ ED-prefixed instructions (block operations, 16-bit I/O, LD I/R)
- ✅ DD-prefixed instructions (IX index register operations)
- ✅ FD-prefixed instructions (IY index register operations)
- ✅ All rotate/shift variants (RLA, RRA, RLCA, RRCA, RL, RR, RLC, RRC, SRL)
- ✅ Block transfer operations (LDI, LDIR, CPIR, LDDR)
- ✅ Bit manipulation (BIT, SET, RES)

### Still To Implement
- Some ED-prefixed I/O block operations
- DAA (Decimal Adjust Accumulator)
- Some undocumented/rarely-used opcodes
- RETI, RETN (return from interrupt)
- Remaining block operations (CPI, CPD, CPDR, INI, IND, OUTI, OUTD variants)

## 🧪 Testing

The project includes a comprehensive test suite with **29 test ROMs** covering all implemented instructions:

### Test ROMs
Pre-built test programs that verify instruction correctness:

**Basic Operations:**
- `01_nop_halt.rom` - Basic execution (NOP, HALT)
- `02_load_immediate.rom` - LD r, n instructions
- `03_increment.rom` - INC operations with flags
- `04_decrement.rom` - DEC operations
- `05_16bit_ops.rom` - 16-bit INC/DEC

**Data Movement:**
- `08_ld_r_r.rom` - Register-to-register transfers
- `09_ld_memory.rom` - Memory operations via (HL)
- `19_ld_bc_de_indirect.rom` - Indirect addressing via BC/DE
- `20_ld_nn_direct.rom` - Direct memory addressing
- `21_ld_hl_memory.rom` - 16-bit memory operations

**Arithmetic:**
- `10_add.rom` - ADD A,r operations
- `11_sub.rom` - SUB operations
- `12_adc.rom` - ADC (add with carry)
- `13_sbc.rom` - SBC (subtract with carry)
- `27_immediate_arithmetic.rom` - Immediate arithmetic (ADD A,n, SUB n, etc.)
- `28_add_hl_16bit.rom` - 16-bit ADD HL operations

**Logical Operations:**
- `14_and.rom` - AND operations
- `15_or.rom` - OR operations
- `16_xor.rom` - XOR operations
- `17_cp.rom` - Compare operations
- `18_logical_memory.rom` - Logical ops with memory

**Control Flow:**
- `06_jump.rom` - Jump instructions (JP, JR)
- `07_djnz_loop.rom` - DJNZ loop instruction
- `24_call_ret.rom` - CALL and RET
- `25_conditional_call.rom` - Conditional CALL instructions
- `26_rst.rom` - RST restart instructions

**Stack & Exchange:**
- `23_push_pop.rom` - Stack operations
- `29_exchange_instructions.rom` - Exchange instructions (EX DE,HL, EXX, etc.)

**Special:**
- `22_di_ei.rom` - Interrupt control (DI/EI)

### Test Results
All 29 tests pass successfully, verifying correct implementation of:
- Instruction execution
- Flag updates (including undocumented X/Y flags)
- Cycle timing
- Memory operations
- Stack operations

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
- *Some* instruction function handlers have been implemented with AI after initial learning phase (repeated/almost identical functions for prefixed groups, for example)
---

**Status:** 🚧 In Development (90%+ CPU complete, video system in progress) | **Version:** 0.1.0 | **Last Updated:** October 2025
