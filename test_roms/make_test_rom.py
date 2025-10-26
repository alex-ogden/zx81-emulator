#!/usr/bin/env python3
"""
Simple Z80 test ROM generator for ZX81 emulator testing.
Creates 8KB ROM files padded with 0xFF (empty ROM pattern).
"""


def make_rom(program_bytes, filename, rom_size=0x2000):
    """
    Create a ROM file from a list of bytes.

    Args:
        program_bytes: List of integers (0-255) representing the program
        filename: Output filename
        rom_size: Total ROM size in bytes (default 8KB for ZX81)
    """
    # Pad to full ROM size with 0xFF (common for empty ROM)
    rom = program_bytes + [0xFF] * (rom_size - len(program_bytes))

    if len(rom) != rom_size:
        raise ValueError(f"ROM size mismatch: expected {rom_size}, got {len(rom)}")

    with open(filename, "wb") as f:
        f.write(bytes(rom))

    print(
        f"Created {filename} ({len(program_bytes)} bytes program, {rom_size} bytes total)"
    )


# Test 1: Simple NOP and HALT
def test_nop_halt():
    program = [
        0x00,  # NOP
        0x00,  # NOP
        0x00,  # NOP
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/01_nop_halt.rom")


# Test 2: Load immediate values
def test_load_immediate():
    program = [
        0x3E,
        0x42,  # LD A, 0x42
        0x06,
        0x12,  # LD B, 0x12
        0x0E,
        0x34,  # LD C, 0x34
        0x16,
        0x56,  # LD D, 0x56
        0x1E,
        0x78,  # LD E, 0x78
        0x26,
        0x9A,  # LD H, 0x9A
        0x2E,
        0xBC,  # LD L, 0xBC
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/02_load_immediate.rom")


# Test 3: Increment operations
def test_increment():
    program = [
        0x3E,
        0x00,  # LD A, 0x00
        0x3C,  # INC A        ; A = 0x01
        0x3C,  # INC A        ; A = 0x02
        0x3C,  # INC A        ; A = 0x03
        0x06,
        0xFE,  # LD B, 0xFE
        0x04,  # INC B        ; B = 0xFF
        0x04,  # INC B        ; B = 0x00 (should set Z flag)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/03_increment.rom")


# Test 4: Decrement operations
def test_decrement():
    program = [
        0x3E,
        0x05,  # LD A, 0x05
        0x3D,  # DEC A        ; A = 0x04
        0x3D,  # DEC A        ; A = 0x03
        0x3D,  # DEC A        ; A = 0x02
        0x3D,  # DEC A        ; A = 0x01
        0x3D,  # DEC A        ; A = 0x00 (Z flag set)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/04_decrement.rom")


# Test 5: 16-bit operations
def test_16bit_ops():
    program = [
        0x01,
        0x00,
        0x10,  # LD BC, 0x1000
        0x03,  # INC BC  ; BC = 0x1001
        0x03,  # INC BC  ; BC = 0x1002
        0x0B,  # DEC BC  ; BC = 0x1001
        0x11,
        0xFF,
        0xFF,  # LD DE, 0xFFFF
        0x13,  # INC DE  ; DE = 0x0000 (wraps)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/05_16bit_ops.rom")


# Test 6: Unconditional jump
def test_jump():
    program = [
        0x3E,
        0x01,  # LD A, 0x01      ; addr 0x0000
        0xC3,
        0x07,
        0x00,  # JP 0x0007       ; addr 0x0002
        0x3E,
        0xFF,  # LD A, 0xFF      ; addr 0x0005 (skipped!)
        0x3E,
        0x42,  # LD A, 0x42      ; addr 0x0007 (jump target)
        0x76,  # HALT            ; addr 0x0009
    ]
    make_rom(program, "test_roms/06_jump.rom")


# Test 7: DJNZ loop
def test_djnz_loop():
    program = [
        0x06,
        0x05,  # LD B, 5         ; Loop counter
        0x3E,
        0x00,  # LD A, 0         ; addr 0x0002
        0x3C,  # INC A           ; addr 0x0004 (loop start)
        0x10,
        0xFD,  # DJNZ -3         ; addr 0x0005, jumps to 0x0004
        #                 ; After loop: A = 5, B = 0
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/07_djnz_loop.rom")


def test_ld_r_r():
    """Test register-to-register copies"""
    program = [
        0x3E,
        0x42,  # LD A, 0x42
        0x47,  # LD B, A        ; B = 0x42
        0x48,  # LD C, B        ; C = 0x42
        0x51,  # LD D, C        ; D = 0x42
        0x5A,  # LD E, D        ; E = 0x42
        0x63,  # LD H, E        ; H = 0x42
        0x6C,  # LD L, H        ; L = 0x42
        0x7D,  # LD A, L        ; A = 0x42 (round trip!)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/08_ld_r_r.rom")


def test_ld_memory():
    """Test memory read/write via (HL)"""
    program = [
        0x21,
        0x00,
        0x40,  # LD HL, 0x4000 (RAM address)
        0x3E,
        0x99,  # LD A, 0x99
        0x77,  # LD (HL), A     ; Write 0x99 to RAM[0x4000]
        0x3E,
        0x00,  # LD A, 0x00     ; Clear A
        0x7E,  # LD A, (HL)     ; Read back from RAM[0x4000]
        0x76,  # HALT           ; A should be 0x99
    ]
    make_rom(program, "test_roms/09_ld_memory.rom")


def test_add():
    """Test ADD A, r operations"""
    program = [
        # Basic addition
        0x3E,
        0x10,  # LD A, 0x10
        0x06,
        0x05,  # LD B, 0x05
        0x80,  # ADD A, B        ; A = 0x15, no carry
        # Addition with carry
        0x3E,
        0xFF,  # LD A, 0xFF
        0x0E,
        0x02,  # LD C, 0x02
        0x81,  # ADD A, C        ; A = 0x01, C flag set
        # Addition resulting in zero
        0x3E,
        0x00,  # LD A, 0x00
        0x16,
        0x00,  # LD D, 0x00
        0x82,  # ADD A, D        ; A = 0x00, Z flag set
        # Overflow test (positive + positive = negative)
        0x3E,
        0x7F,  # LD A, 0x7F (127)
        0x1E,
        0x01,  # LD E, 0x01
        0x83,  # ADD A, E        ; A = 0x80 (-128), P/V flag set
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/10_add.rom")


def test_sub():
    """Test SUB A, r operations"""
    program = [
        # Basic subtraction
        0x3E,
        0x10,  # LD A, 0x10
        0x06,
        0x05,  # LD B, 0x05
        0x90,  # SUB B           ; A = 0x0B, no borrow
        # Subtraction with borrow
        0x3E,
        0x05,  # LD A, 0x05
        0x0E,
        0x10,  # LD C, 0x10
        0x91,  # SUB C           ; A = 0xF5, C flag set (borrow)
        # Subtraction resulting in zero
        0x3E,
        0x42,  # LD A, 0x42
        0x16,
        0x42,  # LD D, 0x42
        0x92,  # SUB D           ; A = 0x00, Z flag set
        # Overflow test (negative - positive = positive)
        0x3E,
        0x80,  # LD A, 0x80 (-128)
        0x1E,
        0x01,  # LD E, 0x01
        0x93,  # SUB E           ; A = 0x7F (127), P/V flag set
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/11_sub.rom")


def test_adc():
    """Test ADC A, r (Add with Carry) operations"""
    program = [
        # ADC without initial carry
        0x3E,
        0x10,  # LD A, 0x10
        0x06,
        0x05,  # LD B, 0x05
        0x37,  # SCF             ; Set carry flag
        0x3F,  # CCF             ; Clear carry flag
        0x88,  # ADC A, B        ; A = 0x15, carry was 0
        # ADC with carry set
        0x3E,
        0x10,  # LD A, 0x10
        0x0E,
        0x05,  # LD C, 0x05
        0x37,  # SCF             ; Set carry flag
        0x89,  # ADC A, C        ; A = 0x16 (0x10 + 0x05 + 1)
        # ADC causing carry
        0x3E,
        0xFF,  # LD A, 0xFF
        0x16,
        0x01,  # LD D, 0x01
        0x37,  # SCF             ; Set carry flag
        0x8A,  # ADC A, D        ; A = 0x01, C flag set (0xFF + 0x01 + 1 = 0x101)
        # Chain multiple ADC (like multi-byte addition)
        0x3E,
        0x00,  # LD A, 0x00
        0x1E,
        0xFF,  # LD E, 0xFF
        0x83,  # ADD A, E        ; A = 0xFF, C = 0
        0x3E,
        0x00,  # LD A, 0x00
        0x26,
        0x01,  # LD H, 0x01
        0x84,  # ADD A, H        ; A = 0x01, C = 0 (for next byte)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/12_adc.rom")


def test_sbc():
    """Test SBC A, r (Subtract with Carry) operations"""
    program = [
        # SBC without initial carry
        0x3E,
        0x10,  # LD A, 0x10
        0x06,
        0x05,  # LD B, 0x05
        0x37,  # SCF             ; Set carry flag
        0x3F,  # CCF             ; Clear carry flag
        0x98,  # SBC A, B        ; A = 0x0B (0x10 - 0x05 - 0)
        # SBC with carry set
        0x3E,
        0x10,  # LD A, 0x10
        0x0E,
        0x05,  # LD C, 0x05
        0x37,  # SCF             ; Set carry flag
        0x99,  # SBC A, C        ; A = 0x0A (0x10 - 0x05 - 1)
        # SBC causing borrow
        0x3E,
        0x00,  # LD A, 0x00
        0x16,
        0x01,  # LD D, 0x01
        0x37,  # SCF             ; Set carry flag
        0x9A,  # SBC A, D        ; A = 0xFE (0x00 - 0x01 - 1), C flag set
        # SBC resulting in zero
        0x3E,
        0x06,  # LD A, 0x06
        0x1E,
        0x05,  # LD E, 0x05
        0x37,  # SCF             ; Set carry flag
        0x9B,  # SBC A, E        ; A = 0x00 (0x06 - 0x05 - 1), Z flag set
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/13_sbc.rom")


def test_and():
    """Test AND A, r operations"""
    program = [
        # Basic AND
        0x3E,
        0xFF,  # LD A, 0xFF
        0x06,
        0x0F,  # LD B, 0x0F
        0xA0,  # AND B           ; A = 0x0F
        # AND resulting in zero
        0x3E,
        0xF0,  # LD A, 0xF0
        0x0E,
        0x0F,  # LD C, 0x0F
        0xA1,  # AND C           ; A = 0x00, Z flag set
        # AND with all bits set
        0x3E,
        0xFF,  # LD A, 0xFF
        0x16,
        0xFF,  # LD D, 0xFF
        0xA2,  # AND D           ; A = 0xFF, S flag set
        # AND setting sign flag
        0x3E,
        0xFF,  # LD A, 0xFF
        0x1E,
        0x80,  # LD E, 0x80
        0xA3,  # AND E           ; A = 0x80, S flag set
        # AND with self (common pattern)
        0x3E,
        0x42,  # LD A, 0x42
        0xA7,  # AND A           ; A = 0x42 (tests flags)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/14_and.rom")


def test_or():
    """Test OR A, r operations"""
    program = [
        # Basic OR
        0x3E,
        0xF0,  # LD A, 0xF0
        0x06,
        0x0F,  # LD B, 0x0F
        0xB0,  # OR B            ; A = 0xFF
        # OR resulting in zero
        0x3E,
        0x00,  # LD A, 0x00
        0x0E,
        0x00,  # LD C, 0x00
        0xB1,  # OR C            ; A = 0x00, Z flag set
        # OR with self to test flags
        0x3E,
        0x80,  # LD A, 0x80
        0xB7,  # OR A            ; A = 0x80, S flag set
        # OR clearing zero flag
        0x3E,
        0x00,  # LD A, 0x00
        0x16,
        0x01,  # LD D, 0x01
        0xB2,  # OR D            ; A = 0x01
        # OR with pattern
        0x3E,
        0x55,  # LD A, 0x55 (01010101)
        0x1E,
        0xAA,  # LD E, 0xAA (10101010)
        0xB3,  # OR E            ; A = 0xFF
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/15_or.rom")


def test_xor():
    """Test XOR A, r operations"""
    program = [
        # Basic XOR
        0x3E,
        0xFF,  # LD A, 0xFF
        0x06,
        0x0F,  # LD B, 0x0F
        0xA8,  # XOR B           ; A = 0xF0
        # XOR with self (common way to zero A)
        0x3E,
        0x42,  # LD A, 0x42
        0xAF,  # XOR A           ; A = 0x00, Z flag set
        # XOR resulting in negative
        0x3E,
        0x7F,  # LD A, 0x7F
        0x0E,
        0xFF,  # LD C, 0xFF
        0xA9,  # XOR C           ; A = 0x80, S flag set
        # XOR pattern test
        0x3E,
        0x55,  # LD A, 0x55 (01010101)
        0x16,
        0xAA,  # LD D, 0xAA (10101010)
        0xAA,  # XOR D           ; A = 0xFF
        # XOR same value twice (should get original back)
        0x3E,
        0x42,  # LD A, 0x42
        0x1E,
        0x99,  # LD E, 0x99
        0xAB,  # XOR E           ; A = 0xDB
        0xAB,  # XOR E           ; A = 0x42 (back to original)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/16_xor.rom")


def test_cp():
    """Test CP (compare) operations"""
    program = [
        # Compare equal values (Z flag set)
        0x3E,
        0x42,  # LD A, 0x42
        0x06,
        0x42,  # LD B, 0x42
        0xB8,  # CP B            ; Z flag set, A unchanged
        # Compare A > operand (no flags)
        0x3E,
        0x10,  # LD A, 0x10
        0x0E,
        0x05,  # LD C, 0x05
        0xB9,  # CP C            ; A = 0x10 (unchanged)
        # Compare A < operand (C flag set - borrow)
        0x3E,
        0x05,  # LD A, 0x05
        0x16,
        0x10,  # LD D, 0x10
        0xBA,  # CP D            ; C flag set, A = 0x05 (unchanged)
        # Compare with zero
        0x3E,
        0x00,  # LD A, 0x00
        0x1E,
        0x00,  # LD E, 0x00
        0xBB,  # CP E            ; Z flag set
        # Compare resulting in negative (sign flag)
        0x3E,
        0x00,  # LD A, 0x00
        0x26,
        0x01,  # LD H, 0x01
        0xBC,  # CP H            ; S flag set (result is 0xFF)
        # Compare with self (always equal)
        0x3E,
        0x99,  # LD A, 0x99
        0xBF,  # CP A            ; Z flag set, A = 0x99
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/17_cp.rom")


def test_logical_memory():
    """Test logical operations with (HL) - memory access"""
    program = [
        # Setup: Write test value to memory
        0x21,
        0x00,
        0x40,  # LD HL, 0x4000
        0x3E,
        0x0F,  # LD A, 0x0F
        0x77,  # LD (HL), A      ; RAM[0x4000] = 0x0F
        # AND (HL)
        0x3E,
        0xFF,  # LD A, 0xFF
        0xA6,  # AND (HL)        ; A = 0x0F
        # OR (HL)
        0x3E,
        0xF0,  # LD A, 0xF0
        0xB6,  # OR (HL)         ; A = 0xFF
        # XOR (HL)
        0x3E,
        0xFF,  # LD A, 0xFF
        0xAE,  # XOR (HL)        ; A = 0xF0
        # CP (HL)
        0x3E,
        0x0F,  # LD A, 0x0F
        0xBE,  # CP (HL)         ; Z flag set (equal)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/18_logical_memory.rom")


def test_ld_indirect_bc_de():
    """Test LD (BC), A and LD (DE), A - store A via register pairs"""
    program = [
        # Setup memory addresses
        0x01,
        0x00,
        0x40,  # LD BC, 0x4000
        0x11,
        0x10,
        0x40,  # LD DE, 0x4010
        # Store via BC
        0x3E,
        0x42,  # LD A, 0x42
        0x02,  # LD (BC), A      ; RAM[0x4000] = 0x42
        # Store via DE
        0x3E,
        0x99,  # LD A, 0x99
        0x12,  # LD (DE), A      ; RAM[0x4010] = 0x99
        # Load back via BC
        0x3E,
        0x00,  # LD A, 0x00      ; Clear A
        0x0A,  # LD A, (BC)      ; A = 0x42
        # Load back via DE
        0x3E,
        0x00,  # LD A, 0x00      ; Clear A
        0x1A,  # LD A, (DE)      ; A = 0x99
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/19_ld_bc_de_indirect.rom")


def test_ld_nn_direct():
    """Test LD (nn), A and LD A, (nn) - direct memory addressing"""
    program = [
        # Store A to absolute address
        0x3E,
        0x55,  # LD A, 0x55
        0x32,
        0x00,
        0x40,  # LD (0x4000), A  ; RAM[0x4000] = 0x55
        # Store different value
        0x3E,
        0xAA,  # LD A, 0xAA
        0x32,
        0x20,
        0x40,  # LD (0x4020), A  ; RAM[0x4020] = 0xAA
        # Load back from first address
        0x3E,
        0x00,  # LD A, 0x00
        0x3A,
        0x00,
        0x40,  # LD A, (0x4000)  ; A = 0x55
        # Load back from second address
        0x3E,
        0x00,  # LD A, 0x00
        0x3A,
        0x20,
        0x40,  # LD A, (0x4020)  ; A = 0xAA
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/20_ld_nn_direct.rom")


def test_ld_hl_memory():
    """Test LD (nn), HL and LD HL, (nn) - 16-bit memory transfers"""
    program = [
        # Store HL to memory
        0x21,
        0x34,
        0x12,  # LD HL, 0x1234
        0x22,
        0x00,
        0x40,  # LD (0x4000), HL
        # RAM[0x4000] = 0x34 (low byte)
        # RAM[0x4001] = 0x12 (high byte)
        # Change HL
        0x21,
        0x00,
        0x00,  # LD HL, 0x0000
        # Load HL back from memory
        0x2A,
        0x00,
        0x40,  # LD HL, (0x4000)  ; HL = 0x1234
        # Store different value
        0x21,
        0xCD,
        0xAB,  # LD HL, 0xABCD
        0x22,
        0x10,
        0x40,  # LD (0x4010), HL
        # Verify by loading back
        0x21,
        0x00,
        0x00,  # LD HL, 0x0000
        0x2A,
        0x10,
        0x40,  # LD HL, (0x4010)  ; HL = 0xABCD
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/21_ld_hl_memory.rom")


def test_di_ei():
    """Test DI and EI - interrupt control"""
    program = [
        # Enable interrupts
        0xFB,  # EI               ; IFF1=1, IFF2=1
        # Some operations
        0x3E,
        0x42,  # LD A, 0x42
        0x04,  # INC B
        # Disable interrupts
        0xF3,  # DI               ; IFF1=0, IFF2=0
        # More operations
        0x3E,
        0x99,  # LD A, 0x99
        0x05,  # DEC B
        # Re-enable
        0xFB,  # EI               ; IFF1=1, IFF2=1
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/22_di_ei.rom")


def test_push_pop():
    """Test PUSH and POP operations"""
    program = [
        # Setup stack pointer
        0x31,
        0xFF,
        0x43,  # LD SP, 0x43FF
        # Load test values
        0x01,
        0x34,
        0x12,  # LD BC, 0x1234
        0x11,
        0x78,
        0x56,  # LD DE, 0x5678
        0x21,
        0xBC,
        0x9A,  # LD HL, 0x9ABC
        0x3E,
        0x42,  # LD A, 0x42
        0x06,
        0x55,  # LD B, 0x55 (updates BC)
        # Push all
        0xC5,  # PUSH BC
        0xD5,  # PUSH DE
        0xE5,  # PUSH HL
        # Corrupt registers
        0x01,
        0x00,
        0x00,  # LD BC, 0x0000
        0x11,
        0x00,
        0x00,  # LD DE, 0x0000
        0x21,
        0x00,
        0x00,  # LD HL, 0x0000
        # Pop back
        0xE1,  # POP HL          ; HL = 0x9ABC
        0xD1,  # POP DE          ; DE = 0x5678
        0xC1,  # POP BC          ; BC = 0x5534 (B was changed)
        0x76,  # HALT
    ]
    make_rom(program, "test_roms/23_push_pop.rom")


def test_call_ret():
    """Test CALL and RET - subroutine calls"""
    program = [
        # Setup stack
        0x31,
        0xFF,
        0x43,  # LD SP, 0x43FF    ; addr 0x0000
        # Main code
        0x3E,
        0x01,  # LD A, 0x01       ; addr 0x0003
        0xCD,
        0x0C,
        0x00,  # CALL 0x000C      ; addr 0x0005 (call subroutine)
        0x3E,
        0x03,  # LD A, 0x03       ; addr 0x0008 (after return)
        0x76,  # HALT             ; addr 0x000A
        0x00,  # NOP (padding)    ; addr 0x000B
        # Subroutine at 0x000C
        0x3E,
        0x02,  # LD A, 0x02       ; addr 0x000C
        0xC9,  # RET              ; addr 0x000E
    ]
    make_rom(program, "test_roms/24_call_ret.rom")

def test_conditional_calls():
    """Test conditional CALL instructions"""
    program = [
        0x31, 0xFF, 0x43,        # LD SP, 0x43FF
        0x3E, 0x01,              # LD A, 0x01
        0xCC, 0x20, 0x00,        # CALL Z, 0x0020   (should NOT call)
        0xC4, 0x20, 0x00,        # CALL NZ, 0x0020  (should call)
        0x3E, 0x00,              # LD A, 0x00
        0xB7,                    # OR A             (sets Z)
        0xCC, 0x25, 0x00,        # CALL Z, 0x0025   (should call)
        0x76,                    # HALT
        # Padding up to 0x0020 (we're currently at 0x0012)
    ] + [0x00] * (0x0020 - 0x0012) + [
        # Subroutine 1 at 0x0020
        0x04, 0xC9,              # INC B; RET
        # Padding up to 0x0025
    ] + [0x00] * (0x0025 - 0x0022) + [
        # Subroutine 2 at 0x0025
        0x0C, 0xC9,              # INC C; RET
    ]

    make_rom(program, "test_roms/25_conditional_call.rom")

if __name__ == "__main__":
    print("Generating Z80 CPU test ROMs...\n")

    test_nop_halt()
    test_load_immediate()
    test_increment()
    test_decrement()
    test_16bit_ops()
    test_jump()
    test_djnz_loop()
    test_ld_r_r()
    test_ld_memory()
    test_add()
    test_adc()
    test_sub()
    test_sbc()
    test_and()
    test_or()
    test_xor()
    test_cp()
    test_logical_memory()
    test_ld_indirect_bc_de()
    test_ld_nn_direct()
    test_ld_hl_memory()
    test_di_ei()
    test_push_pop()
    test_call_ret()
    test_conditional_calls()

    print("\nAll test ROMs generated successfully!")
    print("Each ROM is 8KB (0x2000 bytes) as required by the ZX81 emulator.")
