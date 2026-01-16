#!/usr/bin/env python3
import re

# Load the snapshot
path = 'examples/test_multibank_pdb/rom_snapshot_bank0_and_31.bin'
data = open(path, 'rb').read()

bank0 = data[0:0x4000]
bank31 = data[0x4000:0x8000]

print("=" * 80)
print("CRITICAL FINDINGS")
print("=" * 80)

# 1. Check where code actually ends in bank0
print("\n1. BANK0 CODE STRUCTURE:")
print(f"   First 200 bytes contain START/initialization code")
print(f"   At 0x00A0: String data begins")
print(f"   - 0x00A0: '9HELLO' (note: 0x39 = '9' ASCII)")
print(f"   - 0x00A7: 'WORLD'")
print(f"   - Rest of bank0 (0x00B0-0x3FFF): Filled with 0xFF (blank)")

# 2. Analyze the JSR calls
print("\n2. JSR $4000 PATTERN FOUND:")
print(f"   At 0x0070: {bank0[0x0070:0x0073].hex()} = JSR $4000")
print(f"   At 0x0097: {bank0[0x0097:0x009A].hex()} = JSR $4000")
print(f"   These are CORRECT - jumping to bank31 code at $4000")

# 3. Check what's at those call sites
print("\n3. CONTEXT OF JSR CALLS:")
print(f"   Around 0x0070:")
for i in range(0x0068, 0x0080):
    print(f"      0x{i:04X}: {bank0[i]:02X}")

print(f"\n   Around 0x0097:")
for i in range(0x008F, 0x00A0):
    print(f"      0x{i:04X}: {bank0[i]:02X}")

# 4. Analyze bank31 code
print("\n4. BANK31 CODE STRUCTURE:")
print(f"   First 100 bytes contain helper code (BD F1 AA = JSR $F1AA)")
print(f"   Rest of bank31 (0x0070-0x3FFF): Filled with 0xFF (blank)")

print(f"\n   Bank31 helper starts at 0x0000:")
for i in range(0, 30):
    print(f"      0x{i:04X}: {bank31[i]:02X}", end="")
    if i % 10 == 9:
        print()

print("\n\n5. DIAGNOSIS:")
print("   ✅ JSR $4000 instructions are present and correct")
print("   ✅ Bank31 contains VECTREX_PRINT_TEXT entry (BD F1 AA...)")
print("   ✅ Strings are present in bank0 at 0x00A0-0x00A8")
print("   ⚠️  Bank0 code ends around 0x00A0, rest is 0xFF padding")
print("   ⚠️  Bank31 code ends around 0x0061, rest is 0xFF padding")

print("\n6. WHY ONLY TWO DASHES?")
print("   Looking at string offsets:")
print("   - 0x00A0 byte value 0x39 = '9' (ASCII)")
print("   - 0x00A1 = 'H'")
print("   - 0x00A2 = 'E'")
print("   - etc...")
print("\n   The string pointers passed might be:")
print("   - Pointing to wrong addresses")
print("   - Or PRINT_TEXT is only reading 2 characters")
print("   - Or there's an issue with the BIOS call")

# Let's check what's being set up for the print call
print("\n7. BEFORE FIRST JSR (around 0x0070):")
for i in range(0x0060, 0x0078):
    b = bank0[i]
    if b != 0xFF:
        print(f"   0x{i:04X}: 0x{b:02X}", end="")
        if b >= 32 and b < 127:
            print(f" ({chr(b)})", end="")
        print()

print("\n8. BEFORE SECOND JSR (around 0x0097):")
for i in range(0x0087, 0x00A0):
    b = bank0[i]
    if b != 0xFF:
        print(f"   0x{i:04X}: 0x{b:02X}", end="")
        if b >= 32 and b < 127:
            print(f" ({chr(b)})", end="")
        print()

print("\n9. STRING AREA (0x00A0+):")
for i in range(0x00A0, 0x00B0):
    b = bank0[i]
    print(f"   0x{i:04X}: 0x{b:02X}", end="")
    if b >= 32 and b < 127:
        print(f" ({chr(b)})", end="")
    print()
