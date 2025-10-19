#!/usr/bin/env python3
"""Verify correct address for VPy_LINE:10 by manually parsing ASM"""

import pathlib
import re

asm_path = pathlib.Path(r'examples\test_debug_simple.asm')
asm = asm_path.read_text()

addr = 0
org = 0
marker_line = None

print("=" * 80)
print("MANUAL ASM ADDRESS VERIFICATION")
print("=" * 80)

for i, line in enumerate(asm.split('\n'), 1):
    stripped = line.strip()
    
    # Empty lines
    if not stripped:
        continue
    
    # ORG directive
    if stripped.startswith('ORG'):
        org = int(stripped.split('$')[1].split()[0], 16)
        addr = org
        print(f"Line {i:3d}: ORG set to 0x{org:04X}")
        continue
    
    # VPy_LINE marker
    if stripped.startswith('; VPy_LINE:'):
        marker_line = stripped.split(':')[1].strip()
        print(f"Line {i:3d}: >>> MARKER VPy_LINE:{marker_line} seen at address 0x{addr:04X}")
        continue
    
    # Other comments (skip)
    if stripped.startswith(';'):
        continue
    
    # INCLUDE (skip)
    if stripped.startswith('INCLUDE'):
        continue
    
    # FCC (string)
    if stripped.startswith('FCC'):
        if '"' in stripped:
            start = stripped.find('"')
            end = stripped.rfind('"')
            if end > start:
                length = end - start - 1
                print(f"Line {i:3d}: FCC (string) +{length} bytes -> 0x{addr:04X}")
                addr += length
        continue
    
    # FCB (byte)
    if stripped.startswith('FCB'):
        print(f"Line {i:3d}: FCB (byte) +1 byte -> 0x{addr:04X}")
        addr += 1
        continue
    
    # FDB (word)
    if stripped.startswith('FDB'):
        print(f"Line {i:3d}: FDB (word) +2 bytes -> 0x{addr:04X}")
        addr += 2
        continue
    
    # Labels (no size)
    if ':' in stripped and not any(stripped.startswith(x) for x in ['LDD', 'LDA', 'LDB', 'STD', 'STA', 'STB', 'JSR', 'JMP', 'BRA', 'TFR', 'CLR', 'LDX']):
        label = stripped.split(':')[0]
        print(f"Line {i:3d}: Label '{label}' at 0x{addr:04X}")
        continue
    
    # Instructions
    if any(stripped.startswith(x) for x in ['LDD', 'LDA', 'LDB', 'STD', 'STA', 'STB', 'JSR', 'JMP', 'BRA', 'TFR', 'CLRA', 'CLRB', 'LDX']):
        # Estimate size
        if stripped.startswith('LDD #'):
            size = 3  # Immediate 16-bit
        elif stripped.startswith(('CLRA', 'CLRB')):
            size = 1  # Inherent
        elif stripped.startswith('LDA #'):
            size = 2  # Immediate 8-bit
        elif stripped.startswith(('STA ', 'STD ', 'STB ')):
            size = 3  # Extended addressing
        elif stripped.startswith(('JSR ', 'JMP ', 'LDX #')):
            size = 3
        elif stripped.startswith('TFR '):
            size = 2  # Register-to-register
        elif stripped.startswith('BRA '):
            size = 2  # Relative branch
        else:
            size = 3  # Default
        
        instr = stripped.split(';')[0].strip()  # Remove inline comments
        
        if marker_line:
            print(f"Line {i:3d}: {instr:30s} @ 0x{addr:04X} (size={size})")
            print(f"         >>> RECORDING VPy_LINE:{marker_line} at instruction address 0x{addr:04X} <<<")
            marker_line = None
        else:
            print(f"Line {i:3d}: {instr:30s} @ 0x{addr:04X} (size={size})")
        
        addr += size

print("\n" + "=" * 80)
print("VERIFICATION COMPLETE")
print("=" * 80)
