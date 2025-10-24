#!/bin/bash
# Test runner for ZX81 emulator ROM tests

echo "======================================"
echo "ZX81 Emulator Test Suite"
echo "======================================"
echo ""

echo "Building emulator..."
cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)" || true
echo ""

if [ ! -f "target/release/zx81-emulator" ]; then
    echo "❌ Build failed!"
    exit 1
fi

total_tests=$(ls test_roms/*.rom 2>/dev/null | wc -l | tr -d ' ')
if [ "$total_tests" -eq 0 ]; then
    echo "❌ No test ROMs found in test_roms/"
    echo "Run: python3 test_roms/make_test_rom.py"
    exit 1
fi

echo "Running $total_tests test(s)..."
echo ""

passed=0
failed=0

# Run each test ROM
for rom in test_roms/*.rom; do
    test_name=$(basename "$rom" .rom)
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Test: $test_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    # Run the emulator and capture output
    output=$(./target/release/zx81-emulator "$rom" 2>&1)
    exit_code=$?

    # Check if it halted successfully
    if echo "$output" | grep -q "=== HALTED ==="; then
        echo "✅ PASS"
        ((passed++))
    else
        echo "❌ FAIL (did not halt properly)"
        ((failed++))
    fi

    # Show key output lines
    echo "$output" | grep -E "(Loaded|HALTED|Total|Unknown opcode)"
    echo ""
done

# Summary
echo "======================================"
echo "Test Summary"
echo "======================================"
echo "Total:  $total_tests"
echo "Passed: $passed"
echo "Failed: $failed"
echo ""

if [ "$failed" -eq 0 ]; then
    echo "✅ All tests passed!"
    exit 0
else
    echo "❌ Some tests failed"
    exit 1
fi
