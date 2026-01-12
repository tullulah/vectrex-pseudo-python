#!/bin/bash
# Compile ASM multibank test
# Requires: lwasm

set -e

WORK_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$WORK_DIR"

echo "=== ASM Multibank Test ==="
echo "Step 1: Assemble Bank 0..."
lwasm -I../../ide/frontend/public/include -o bank0.bin bank0.asm

echo "Step 2: Assemble Bank 31..."
lwasm -I../../ide/frontend/public/include -o bank31.bin bank31.asm

echo "Step 3: Create 512KB multibank ROM..."

# Create 512KB ROM (32 banks × 16KB)
python3 << 'PYTHON_EOF'
import os

work_dir = os.path.dirname(os.path.abspath(__file__))
os.chdir(work_dir)

# Create empty 512KB ROM
rom = bytearray(512 * 1024)

# Read Bank 0 binary (skip lwasm header: first 4 bytes)
with open('bank0.bin', 'rb') as f:
    f.seek(4)  # Skip lwasm header
    bank0_data = f.read()
print(f"Bank 0 size: {len(bank0_data)} bytes (stripped header)")
rom[0:len(bank0_data)] = bank0_data

# Read Bank 31 binary (skip lwasm header)
with open('bank31.bin', 'rb') as f:
    f.seek(4)  # Skip lwasm header
    bank31_data = f.read()
print(f"Bank 31 size: {len(bank31_data)} bytes (stripped header)")
# Bank 31 offset: 31 × 16KB = 0x7C000
rom[0x7C000:0x7C000+len(bank31_data)] = bank31_data

# Copy header from Bank 0 to position 0 (first 256 bytes)
# Bank 0 header should already be at rom[0:256]

# Write final ROM
with open('multibank.bin', 'wb') as f:
    f.write(rom)

print(f"Multibank ROM created: multibank.bin ({len(rom)} bytes)")
print(f"Bank 0 at $0000, Bank 31 at $7C000")
PYTHON_EOF

echo ""
echo "=== Compilation Complete ==="
echo "Output: multibank.bin (512 KB)"
echo ""
echo "Ready to test in emulator!"
