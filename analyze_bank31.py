import pathlib

# Read binary
bin_path = pathlib.Path('examples/test_incremental/build/test_incremental.bin')
data = bin_path.read_bytes()

# Bank 31 starts at 31 * 0x4000 = 0x7C000
bank31_start = 31 * 0x4000

print(f"Binary size: {len(data)} bytes")
print(f"Bank 31 starts at: 0x{bank31_start:X}")
print("\nFirst 200 bytes of Bank 31:")
for i in range(0, 200, 16):
    offset = bank31_start + i
    hex_str = ' '.join(f'{data[offset+j]:02X}' for j in range(16) if offset+j < len(data))
    print(f"  {offset:06X}: {hex_str}")

print("\n\nSearching for TFR X,U pattern (1F 13)...")
found_tfr = False
for i in range(bank31_start, min(bank31_start + 0x4000, len(data) - 1)):
    if data[i] == 0x1F and data[i+1] == 0x13:
        found_tfr = True
        print(f"  Found TFR X,U at offset 0x{i:X} (Bank 31 offset +0x{i-bank31_start:X})")
        # Show context
        ctx_start = max(i-10, bank31_start)
        ctx_end = min(i+30, len(data))
        hex_ctx = ' '.join(f'{data[j]:02X}' for j in range(ctx_start, ctx_end))
        print(f"    Context: {hex_ctx}")
if not found_tfr:
    print("  NOT FOUND!")

print("\n\nSearching for PSHS X,A pattern (34 12)...")
found_pshs = False
for i in range(bank31_start, min(bank31_start + 0x4000, len(data) - 1)):
    if data[i] == 0x34 and data[i+1] == 0x12:
        found_pshs = True
        print(f"  Found PSHS X,A at offset 0x{i:X} (Bank 31 offset +0x{i-bank31_start:X})")
        # Show context
        ctx_start = max(i-10, bank31_start)
        ctx_end = min(i+30, len(data))
        hex_ctx = ' '.join(f'{data[j]:02X}' for j in range(ctx_start, ctx_end))
        print(f"    Context: {hex_ctx}")
if not found_pshs:
    print("  NOT FOUND!")
