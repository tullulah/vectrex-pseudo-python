#!/usr/bin/env python3
"""
Verify multibank PDB addresses against actual .bin file content.
"""
import json
import sys
from pathlib import Path

def verify_pdb():
    pdb_path = Path('examples/test_multibank_pdb/src/main.pdb')
    bin_path = Path('examples/test_multibank_pdb/src/main.bin')
    
    if not pdb_path.exists():
        print(f"❌ PDB not found: {pdb_path}")
        return
    if not bin_path.exists():
        print(f"❌ BIN not found: {bin_path}")
        return
    
    # Load PDB and BIN
    with open(pdb_path) as f:
        pdb = json.load(f)
    
    with open(bin_path, 'rb') as f:
        bin_data = f.read()
    
    print(f"📊 Binary size: {len(bin_data)} bytes ({len(bin_data)//1024}KB)")
    print(f"🔍 Checking {len(pdb.get('symbols', {}))} symbols...")
    print()
    
    symbols = pdb.get('symbols', {})
    invalid_addrs = []
    valid_addrs = []
    
    # Bank layout for multibank
    # Bank #0-#30: $0000-$3FFF in cartridge window (in binary: bank_id * 0x4000)
    # Bank #31: $4000-$7FFF (fixed, in binary: 31 * 0x4000 = 0x7C000)
    
    print("Symbol Address Analysis:")
    print("=" * 80)
    
    for name, addr_str in sorted(symbols.items()):
        try:
            addr = int(addr_str, 16)
            
            # Determine which bank this should be in
            if addr < 0x4000:
                # Should be in banks 0-30 (switchable window)
                expected_bank = 0  # Could be any bank 0-30
                expected_binary_base = None  # Multiple possibilities
                status = "⚠️  AMBIGUOUS"
            elif 0x4000 <= addr < 0x8000:
                # Should be in bank 31 (fixed window at 0x4000)
                expected_bank = 31
                expected_binary_base = 31 * 0x4000
                status = "ℹ️  BANK31"
            else:
                status = "❌ OUT_OF_RANGE"
                addr_str_display = f"0x{addr:04X}"
                print(f"  {name:30} {addr_str_display:10} -> {status}")
                invalid_addrs.append((name, addr, "out of range"))
                continue
            
            # For bank #31, verify it matches actual binary
            if expected_bank == 31:
                binary_offset = expected_binary_base + (addr - 0x4000)
                if binary_offset < len(bin_data):
                    byte_val = bin_data[binary_offset]
                    addr_str_display = f"0x{addr:04X}"
                    print(f"  {name:30} {addr_str_display:10} -> bin[0x{binary_offset:06X}]=0x{byte_val:02X} ✓")
                    valid_addrs.append((name, addr, binary_offset))
                else:
                    addr_str_display = f"0x{addr:04X}"
                    print(f"  {name:30} {addr_str_display:10} -> OFFSET 0x{binary_offset:06X} OUTSIDE BIN ❌")
                    invalid_addrs.append((name, addr, "offset outside binary"))
            else:
                # Ambiguous - could be multiple banks
                addr_str_display = f"0x{addr:04X}"
                print(f"  {name:30} {addr_str_display:10} -> {status} (could be any bank 0-30)")
        except ValueError:
            print(f"  {name:30} {addr_str:10} -> ❌ INVALID HEX")
            invalid_addrs.append((name, addr_str, "invalid hex"))
    
    print()
    print("=" * 80)
    print(f"✓ Valid: {len(valid_addrs)}")
    print(f"❌ Invalid: {len(invalid_addrs)}")
    
    if invalid_addrs:
        print()
        print("Invalid addresses:")
        for name, addr, reason in invalid_addrs:
            if isinstance(addr, int):
                print(f"  - {name:30} 0x{addr:04X} ({reason})")
            else:
                print(f"  - {name:30} {addr} ({reason})")
    
    # Analyze bank structure
    print()
    print("=" * 80)
    print("Bank Structure Analysis:")
    print(f"  Bank #0-#30 (switchable): 0x0000-0x3FFF each (16KB)")
    print(f"  Bank #31 (fixed):         0x4000-0x7FFF (16KB)")
    print(f"  Total multibank size:     512KB (32 banks)")
    print(f"  Binary file:              {len(bin_data)} bytes")
    print()
    
    # Check what symbols are actually in bank #31
    bank31_symbols = [(name, addr) for name, addr in symbols.items() 
                      if 0x4000 <= int(addr, 16) < 0x8000]
    print(f"Symbols in Bank #31 range (0x4000-0x7FFF): {len(bank31_symbols)}")
    if bank31_symbols:
        print("  Examples:")
        for name, addr in sorted(bank31_symbols)[:5]:
            print(f"    - {name:30} {addr}")
    
    # The real problem: offsets in banks are relative to bank base (0x0000),
    # but when extracted during assembly, they're NOT being offset by bank*0x4000
    print()
    print("🔧 Potential Issue:")
    print("  Symbols extracted from bank_NN.asm are relative to that bank's ORG (0x0000)")
    print("  But they're being used as absolute addresses in the 512KB ROM")
    print("  Bank #31 code should have addresses 0x4000+, but may only have offsets 0x0000+")
    print()
    print("💡 Solution: Add bank offset to symbol addresses during extraction")
    print("  Bank #NN offset = NN * 0x4000 (except Bank #31 which is fixed at 0x4000)")

if __name__ == '__main__':
    verify_pdb()
