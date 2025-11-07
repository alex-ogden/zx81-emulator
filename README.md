# 🎮 ZX81 Emulator

A cycle-accurate Sinclair ZX81 emulator written in Rust, implementing the legendary Z80 CPU and the quirky hardware of this 1981 home computer.

[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-red.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

**Status:** 🚧 In Development (100% non-prefixed instructions implemented, video output implemented, tape loading and sound in progress)

### 🚧 In Progress

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

# Run with optional debug panel (still in progress)
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

### Still To Implement
- Some prefixed operations
- Some undocumented/rarely-used opcodes

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

## 📊 Performance

The emulator aims for cycle-accurate emulation:
- Each instruction returns the correct T-state count
- Memory operations account for timing differences
- Future: Frame-accurate display timing (50Hz PAL)

Current performance: ~100,000+ instructions per second (debug mode on modern hardware)

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤖 AI usage

- AI has been used in this project to generate and update documentation (including this README you're reading now!)
- AI has been used as a research tool for learning about the ZX81, summarising large data sheets, and keeping track of where I'm up to with implementation
- *Some* instruction function handlers have been implemented with AI after initial learning phase (repeated/almost identical functions for prefixed groups, for example)
---

