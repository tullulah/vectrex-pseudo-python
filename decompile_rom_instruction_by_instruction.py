#!/usr/bin/env python3

# Load the snapshot
data = open('examples/test_multibank_pdb/rom_snapshot_bank0_and_31.bin', 'rb').read()
bank0 = data[0:0x4000]

print("=== DECOMPILING AROUND FIRST JSR (0x006A-0x0076) ===\n")

# Looking at: 8E 00 A1 BF CF E4 BD 40 00
# Let's decode this:
# 8E 00 A1 = LDX #$00A1
# BF CF E4 = STX $CFE4
# BD 40 00 = JSR $4000

print("Raw bytes at 0x006A-0x007C:")
for i in range(0x006A, 0x007C):
    b = bank0[i]
    print(f"  0x{i:04X}: 0x{b:02X}", end="")
    # Try to decode M6809
    if i == 0x006A:
        print(" (8E)  LDX immediate")
    elif i == 0x006B:
        print(" (high byte)")
    elif i == 0x006C:
        print(" (low byte = 0xA1)")
    elif i == 0x006D:
        print(" (BF)  STX extended")
    elif i == 0x006E:
        print(" (high byte of address)")
    elif i == 0x006F:
        print(" (low byte  of address = 0xE4)")
    elif i == 0x0070:
        print(" (BD)  JSR extended")
    elif i == 0x0071:
        print(" (high byte = 0x40)")
    elif i == 0x0072:
        print(" (low byte = 0x00)")
    else:
        print()

print("\n=== INTERPRETATION ===")
print("LDX #$00A1       Load X with 0x00A1 (address of HELLO string)")
print("STX $CFE4        Store X at $CFE4 (VAR_ARG2 slot)")
print("JSR $4000        Jump to VECTREX_PRINT_TEXT in bank31")
print()
print("So VAR_ARG2 gets 0x00A1 which is correct (points to 'H' of HELLO)")
print()

# Now what's at 0x00A1?
print(f"=== STRING MEMORY ===")
print(f"0x00A0-0x00A8: {' '.join(f'{b:02X}' for b in bank0[0x00A0:0x00A9])}")
print(f"0x00A0 value: 0x{bank0[0x00A0]:02X} = {chr(bank0[0x00A0]) if 32 <= bank0[0x00A0] < 127 else '?'} (RTS opcode)")
print(f"0x00A1 value: 0x{bank0[0x00A1]:02X} = {chr(bank0[0x00A1]) if 32 <= bank0[0x00A1] < 127 else '?'} (H)")
print(f"0x00A2 value: 0x{bank0[0x00A2]:02X} = {chr(bank0[0x00A2]) if 32 <= bank0[0x00A2] < 127 else '?'} (E)")
print()

# Ah wait, let's see what VAR_ARG2 points to again
print("So when PRINT_TEXT runs:")
print("  VAR_ARG2 = 0x00A1")
print("  PRINT_TEXT loads U = VAR_ARG2 = 0x00A1")
print("  U points to 0x00A1 which is 'H'")
print("  Print_Str_d reads: H E L L O 80")
print("  Should print: HELLO")
print()

# But we're only getting two dashes (--), not HELLO
print("BUT WE ONLY GET: -- on screen")
print()
print("Possible causes:")
print("1. PRINT_TEXT is being called but Print_Str_d isn't working")
print("2. The string pointer is wrong (but we just verified it's 0x00A1)")
print("3. The coordinates are wrong (causing dashes instead of letters)")
print("4. The BIOS Print_Str_d is interpreting coordinates differently")
print("5. Bank31 doesn't actually contain VECTREX_PRINT_TEXT properly")

