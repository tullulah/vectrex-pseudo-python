#!/bin/bash
# Compile ASM multibank test with BOTH assemblers
# Requires: lwasm AND native vectrexc assembler

set -e

WORK_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$WORK_DIR"

echo "======================================"
echo "  ASM Multibank Test - DUAL COMPILE"
echo "======================================"
echo ""

# ========================================
# TEST 1: LWASM Compilation
# ========================================
echo ">>> TEST 1: LWASM Compilation"
echo "Step 1a: Assemble Bank 0 with lwasm..."
lwasm -I../../ide/frontend/public/include -o bank0_lwasm.bin bank0.asm

echo "Step 1b: Assemble Bank 31 with lwasm..."
lwasm -I../../ide/frontend/public/include -o bank31_lwasm.bin bank31.asm

echo "Step 1c: Link with Python (lwasm binaries)..."
python3 << 'PYTHON_LWASM'
import os

# Create empty 512KB ROM
rom = bytearray(512 * 1024)

# Read Bank 0 binary (skip lwasm header: first 4 bytes)
with open('bank0_lwasm.bin', 'rb') as f:
    f.seek(4)  # Skip lwasm header
    bank0_data = f.read()
print(f"  Bank 0 size: {len(bank0_data)} bytes")
rom[0:len(bank0_data)] = bank0_data

# Read Bank 31 binary (skip lwasm header)
with open('bank31_lwasm.bin', 'rb') as f:
    f.seek(4)  # Skip lwasm header
    bank31_data = f.read()
print(f"  Bank 31 size: {len(bank31_data)} bytes")
rom[0x7C000:0x7C000+len(bank31_data)] = bank31_data

# Write final ROM
with open('multibank_lwasm.bin', 'wb') as f:
    f.write(rom)

print(f"✓ LWASM ROM: multibank_lwasm.bin ({len(rom)} bytes)")
PYTHON_LWASM

echo ""

# ========================================
# TEST 2: Native Assembler Compilation
# ========================================
echo ">>> TEST 2: Native Assembler Compilation"
echo "Step 2a: Assemble Bank 0 with native assembler..."
cargo run --quiet --release --bin vectrexc -- build bank0.asm --bin

echo "Step 2b: Assemble Bank 31 with native assembler..."
cargo run --quiet --release --bin vectrexc -- build bank31.asm --bin

echo "Step 2c: Link with Python (native binaries)..."
python3 << 'PYTHON_NATIVE'
import os

# Create empty 512KB ROM
rom = bytearray(512 * 1024)

# Read Bank 0 binary (native assembler produces raw binary)
with open('bank0.bin', 'rb') as f:
    bank0_data = f.read()
print(f"  Bank 0 size: {len(bank0_data)} bytes")
rom[0:len(bank0_data)] = bank0_data

# Read Bank 31 binary
with open('bank31.bin', 'rb') as f:
    bank31_data = f.read()
print(f"  Bank 31 size: {len(bank31_data)} bytes")
rom[0x7C000:0x7C000+len(bank31_data)] = bank31_data

# Write final ROM
with open('multibank_native.bin', 'wb') as f:
    f.write(rom)

print(f"✓ Native ROM: multibank_native.bin ({len(rom)} bytes)")
PYTHON_NATIVE

echo ""

# ========================================
# COMPARISON
# ========================================
echo "======================================"
echo "  COMPARISON"
echo "======================================"
echo ""
echo "File sizes:"
ls -lh multibank_lwasm.bin multibank_native.bin | awk '{print "  " $9 ": " $5}'
echo ""

echo "Bank 0 - First 64 bytes comparison:"
echo "  LWASM:"
xxd -l 64 -s 0 multibank_lwasm.bin | head -4
echo "  Native:"
xxd -l 64 -s 0 multibank_native.bin | head -4
echo ""

echo "Bank 31 - First 64 bytes comparison:"
echo "  LWASM:"
xxd -l 64 -s 0x7C000 multibank_lwasm.bin | head -4
echo "  Native:"
xxd -l 64 -s 0x7C000 multibank_native.bin | head -4
echo ""

echo "Binary diff check:"
if cmp -s multibank_lwasm.bin multibank_native.bin; then
    echo "  ✅ BINARIES ARE IDENTICAL!"
else
    echo "  ⚠️  BINARIES DIFFER - checking details..."
    cmp -l multibank_lwasm.bin multibank_native.bin | head -20 | awk '{printf "    Offset 0x%08X: lwasm=0x%02X native=0x%02X\n", $1-1, $2, $3}'
fi

echo ""
echo "=== Compilation Complete ==="
echo "Ready to test in emulator!"
echo "  - multibank_lwasm.bin (lwasm)"
echo "  - multibank_native.bin (native)"
