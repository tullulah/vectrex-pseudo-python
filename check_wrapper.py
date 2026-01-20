#!/usr/bin/env python3
data = open('examples/test_incremental/src/main.bin', 'rb').read()
bank_31_start = 31 * 0x4000

print("Bank 31 wrapper analysis:")
print("\nFirst 30 bytes (hex):")
for i in range(0, 30, 10):
    chunk = data[bank_31_start+i:bank_31_start+i+10]
    print(f"  +{i:02d}: {chunk.hex()}")

# Bank 31 START should have MUSIC_BANK_TABLE (1 byte per asset)
print("\nLookup tables:")
print(f"  MUSIC_BANK_TABLE[0] (offset +3): {data[bank_31_start+3]:02X}")
print(f"  MUSIC_ADDR_TABLE starts around offset +32")

# Find PLAY_MUSIC_BANKED code (should start around +64 or so)
print("\nSearching for PLAY_MUSIC_BANKED wrapper...")
for offset in range(0, 500):
    if data[bank_31_start+offset:bank_31_start+offset+4] == bytes([0x1F, 0x13, 0xB6, 0xCF]):
        print(f"  Found at offset +{offset} (${ 0x4000+offset:04X}): TFR X,U + LDA CURRENT_ROM_BANK")
        # Show next 30 bytes
        wrapper_code = data[bank_31_start+offset:bank_31_start+offset+30]
        print(f"  Code: {wrapper_code.hex()}")
        break
