#!/usr/bin/env python3
"""
Analyze ROM snapshot downloaded from emulator.
Usage: python3 analyze_rom_snapshot.py rom_snapshot_bankN_and_31.bin

This will:
1. Display file size (should be 32KB = 32768 bytes)
2. Show first bank (16KB at offset 0x0000):
   - First 32 bytes (ROM header verification)
   - First 512 bytes (GCE header + start of code)
3. Show bank #31 (16KB at offset 0x4000):
   - First 32 bytes (should contain CUSTOM_RESET code)
   - Entries at critical addresses like 0x0000 of this bank
"""

import sys
import os

def analyze_snapshot(filename):
    if not os.path.exists(filename):
        print(f"❌ File not found: {filename}")
        sys.exit(1)
    
    size = os.path.getsize(filename)
    print(f"\n📦 ROM Snapshot Analysis: {filename}")
    print(f"   Size: {size} bytes ({size // 1024}KB)")
    
    if size != 32768:
        print(f"   ⚠️  Expected 32KB (32768 bytes), got {size}")
    else:
        print(f"   ✅ Correct size (32KB)")
    
    with open(filename, 'rb') as f:
        data = f.read()
    
    # Bank 0 analysis
    print("\n" + "="*60)
    print("BANK 0 (Current Bank) - Offset 0x0000-0x3FFF")
    print("="*60)
    
    bank0 = data[0x0000:0x4000]
    print(f"\n📍 First 32 bytes (hex):")
    for i in range(0, 32, 16):
        hex_str = ' '.join(f'{b:02X}' for b in bank0[i:i+16])
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in bank0[i:i+16])
        print(f"   {i:04X}: {hex_str:<48} | {ascii_str}")
    
    # Check for GCE header
    if bank0[0:4] == b'\x00\x00\x00\x00':
        print("\n✅ Starts with null bytes (GCE-style header)")
    else:
        print(f"\n⚠️  Header bytes: {bank0[0:4].hex()}")
        # Try to detect what's there
        if bank0[0:1] == b'\x0C' or bank0[0:1] == b'\x00':
            print("   Looks like CODE (BRA or data)")
        else:
            print(f"   First byte: 0x{bank0[0]:02X}")
    
    # Bank #31 analysis
    print("\n" + "="*60)
    print("BANK #31 (Fixed Bank) - Offset 0x4000-0x7FFF")
    print("="*60)
    
    bank31 = data[0x4000:0x8000]
    print(f"\n📍 First 32 bytes (hex):")
    for i in range(0, 32, 16):
        hex_str = ' '.join(f'{b:02X}' for b in bank31[i:i+16])
        ascii_str = ''.join(chr(b) if 32 <= b < 127 else '.' for b in bank31[i:i+16])
        print(f"   {i:04X}: {hex_str:<48} | {ascii_str}")
    
    # Disassemble first few instructions
    print(f"\n📍 First instructions (disassembled):")
    
    # Common M6809 opcodes
    opcodes = {
        0x86: 'LDA #',
        0x87: 'STA #',  # invalid but often used
        0xB6: 'LDA direct',
        0xB7: 'STA direct',
        0x39: 'RTS',
        0x3F: 'SWI',
        0x10: '(2-byte prefix)',
        0x11: '(2-byte prefix)',
        0x1F: 'TFR',
        0x1E: 'EXG',
        0x0C: 'INC direct',
        0x0D: 'DEC direct',
        0x4F: 'CLRA',
        0x5F: 'CLRB',
    }
    
    print(f"   Address Code Op  Operand")
    for i in range(min(16, len(bank31))):
        byte = bank31[i]
        op_desc = opcodes.get(byte, f'0x{byte:02X}')
        print(f"   0x{i:04X}:    {byte:02X}   {op_desc}")
    
    # Check for expected CUSTOM_RESET pattern
    print(f"\n📍 Checking for CUSTOM_RESET pattern:")
    expected_pattern = [0x86, 0x00]  # LDA #0
    if bank31[0:2] == bytes(expected_pattern):
        print(f"   ✅ Found 'LDA #0' at offset 0x0000 (CUSTOM_RESET start)")
    else:
        print(f"   ⚠️  Expected LDA #0 (86 00), found: {bank31[0]:02X} {bank31[1]:02X}")
    
    # Size analysis
    print("\n" + "="*60)
    print("📊 Summary")
    print("="*60)
    
    # Count non-zero bytes
    bank0_nonzero = sum(1 for b in bank0 if b != 0)
    bank31_nonzero = sum(1 for b in bank31 if b != 0)
    
    print(f"\nBank 0: {bank0_nonzero:5} / {len(bank0)} bytes non-zero")
    print(f"Bank 31: {bank31_nonzero:5} / {len(bank31)} bytes non-zero")
    
    print(f"\n✅ Analysis complete!")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 analyze_rom_snapshot.py <snapshot_file.bin>")
        sys.exit(1)
    
    analyze_snapshot(sys.argv[1])
