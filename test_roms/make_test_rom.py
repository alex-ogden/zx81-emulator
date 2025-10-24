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

    with open(filename, 'wb') as f:
        f.write(bytes(rom))

    print(f"Created {filename} ({len(program_bytes)} bytes program, {rom_size} bytes total)")


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
        0x3E, 0x42,  # LD A, 0x42
        0x06, 0x12,  # LD B, 0x12
        0x0E, 0x34,  # LD C, 0x34
        0x16, 0x56,  # LD D, 0x56
        0x1E, 0x78,  # LD E, 0x78
        0x26, 0x9A,  # LD H, 0x9A
        0x2E, 0xBC,  # LD L, 0xBC
        0x76,        # HALT
    ]
    make_rom(program, "test_roms/02_load_immediate.rom")


# Test 3: Increment operations
def test_increment():
    program = [
        0x3E, 0x00,  # LD A, 0x00
        0x3C,        # INC A        ; A = 0x01
        0x3C,        # INC A        ; A = 0x02
        0x3C,        # INC A        ; A = 0x03

        0x06, 0xFE,  # LD B, 0xFE
        0x04,        # INC B        ; B = 0xFF
        0x04,        # INC B        ; B = 0x00 (should set Z flag)

        0x76,        # HALT
    ]
    make_rom(program, "test_roms/03_increment.rom")


# Test 4: Decrement operations
def test_decrement():
    program = [
        0x3E, 0x05,  # LD A, 0x05
        0x3D,        # DEC A        ; A = 0x04
        0x3D,        # DEC A        ; A = 0x03
        0x3D,        # DEC A        ; A = 0x02
        0x3D,        # DEC A        ; A = 0x01
        0x3D,        # DEC A        ; A = 0x00 (Z flag set)

        0x76,        # HALT
    ]
    make_rom(program, "test_roms/04_decrement.rom")


# Test 5: 16-bit operations
def test_16bit_ops():
    program = [
        0x01, 0x00, 0x10,  # LD BC, 0x1000
        0x03,              # INC BC  ; BC = 0x1001
        0x03,              # INC BC  ; BC = 0x1002
        0x0B,              # DEC BC  ; BC = 0x1001

        0x11, 0xFF, 0xFF,  # LD DE, 0xFFFF
        0x13,              # INC DE  ; DE = 0x0000 (wraps)

        0x76,              # HALT
    ]
    make_rom(program, "test_roms/05_16bit_ops.rom")


# Test 6: Unconditional jump
def test_jump():
    program = [
        0x3E, 0x01,        # LD A, 0x01      ; addr 0x0000
        0xC3, 0x07, 0x00,  # JP 0x0007       ; addr 0x0002
        0x3E, 0xFF,        # LD A, 0xFF      ; addr 0x0005 (skipped!)
        0x3E, 0x42,        # LD A, 0x42      ; addr 0x0007 (jump target)
        0x76,              # HALT            ; addr 0x0009
    ]
    make_rom(program, "test_roms/06_jump.rom")


# Test 7: DJNZ loop
def test_djnz_loop():
    program = [
        0x06, 0x05,        # LD B, 5         ; Loop counter
        0x3E, 0x00,        # LD A, 0         ; addr 0x0002
        0x3C,              # INC A           ; addr 0x0004 (loop start)
        0x10, 0xFD,        # DJNZ -3         ; addr 0x0005, jumps to 0x0004
                           #                 ; After loop: A = 5, B = 0
        0x76,              # HALT
    ]
    make_rom(program, "test_roms/07_djnz_loop.rom")


def test_ld_r_r():
    """Test register-to-register copies"""
    program = [
        0x3E, 0x42,  # LD A, 0x42
        0x47,        # LD B, A        ; B = 0x42
        0x48,        # LD C, B        ; C = 0x42
        0x51,        # LD D, C        ; D = 0x42
        0x5A,        # LD E, D        ; E = 0x42
        0x63,        # LD H, E        ; H = 0x42
        0x6C,        # LD L, H        ; L = 0x42
        0x7D,        # LD A, L        ; A = 0x42 (round trip!)
        0x76,        # HALT
    ]
    make_rom(program, "test_roms/08_ld_r_r.rom")


def test_ld_memory():
    """Test memory read/write via (HL)"""
    program = [
        0x21, 0x00, 0x40,  # LD HL, 0x4000 (RAM address)
        0x3E, 0x99,        # LD A, 0x99
        0x77,              # LD (HL), A     ; Write 0x99 to RAM[0x4000]
        0x3E, 0x00,        # LD A, 0x00     ; Clear A
        0x7E,              # LD A, (HL)     ; Read back from RAM[0x4000]
        0x76,              # HALT           ; A should be 0x99
    ]
    make_rom(program, "test_roms/09_ld_memory.rom")


if __name__ == "__main__":
    print("Generating Z80 test ROMs...\n")

    test_nop_halt()
    test_load_immediate()
    test_increment()
    test_decrement()
    test_16bit_ops()
    test_jump()
    test_djnz_loop()
    test_ld_r_r()
    test_ld_memory()

    print("\nAll test ROMs generated successfully!")
    print("Each ROM is 8KB (0x2000 bytes) as required by the ZX81 emulator.")
