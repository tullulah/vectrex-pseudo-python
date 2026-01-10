#!/usr/bin/env python3
"""
Manual .vo file generator for testing the linker.
Creates simple object files with relocations.
"""

import struct
import json

def write_vo_file(filename, sections, exports, imports, relocations):
    """Write a Vectrex Object file (.vo)"""
    with open(filename, 'wb') as f:
        # Header
        f.write(b'VObj')  # Magic
        f.write(struct.pack('>H', 1))  # Version
        f.write(struct.pack('B', 0))  # Target: M6809
        f.write(struct.pack('B', 0))  # Flags
        
        # Source file name (length-prefixed string)
        source = filename.replace('.vo', '.asm')
        f.write(struct.pack('>H', len(source)))
        f.write(source.encode('utf-8'))
        
        # Sections
        f.write(struct.pack('>H', len(sections)))
        for section in sections:
            name, data, alignment = section
            # Section name
            f.write(struct.pack('>H', len(name)))
            f.write(name.encode('utf-8'))
            # Section type (0=Text, 1=Data, 2=Bss, 3=ReadOnly)
            f.write(struct.pack('B', 0))
            # Bank hint (None = 255)
            f.write(struct.pack('B', 255))
            # Alignment
            f.write(struct.pack('>H', alignment))
            # Data
            f.write(struct.pack('>I', len(data)))
            f.write(bytes(data))
        
        # Symbol table
        # Exports
        f.write(struct.pack('>H', len(exports)))
        for name, section_idx, offset in exports:
            f.write(struct.pack('>H', len(name)))
            f.write(name.encode('utf-8'))
            # Section (Some(idx))
            f.write(struct.pack('B', 1))  # Some
            f.write(struct.pack('>H', section_idx))
            # Offset
            f.write(struct.pack('>H', offset))
            # Scope (0=Local, 1=Global, 2=Weak)
            f.write(struct.pack('B', 1))  # Global
            # SymbolType (0=Function, 1=Variable, 2=Constant)
            f.write(struct.pack('B', 0))  # Function
        
        # Imports
        f.write(struct.pack('>H', len(imports)))
        for name in imports:
            f.write(struct.pack('>H', len(name)))
            f.write(name.encode('utf-8'))
            # Section (None)
            f.write(struct.pack('B', 0))  # None
            # Offset
            f.write(struct.pack('>H', 0))
            # Scope
            f.write(struct.pack('B', 1))  # Global
            # SymbolType
            f.write(struct.pack('B', 0))  # Function
        
        # Relocations
        f.write(struct.pack('>H', len(relocations)))
        for section_idx, offset, reloc_type, symbol, addend in relocations:
            f.write(struct.pack('>H', section_idx))
            f.write(struct.pack('>H', offset))
            # RelocationType (0=Absolute16, 1=Relative8, 2=Relative16, etc)
            f.write(struct.pack('B', reloc_type))
            # Symbol name
            f.write(struct.pack('>H', len(symbol)))
            f.write(symbol.encode('utf-8'))
            # Addend
            f.write(struct.pack('>i', addend))
        
        # Debug info (empty for now)
        f.write(struct.pack('>I', 0))  # line_map entries

# Create lib.vo - exports helper_draw_square
lib_code = [
    0xBD, 0xF2, 0xAB,  # JSR $F2AB (Intensity_a)
    0x86, 0xF6,        # LDA #-10
    0xC6, 0xF6,        # LDB #-10  
    0xBD, 0xF3, 0x12,  # JSR $F312 (Moveto_d)
    0x86, 0x00,        # LDA #0
    0xC6, 0x14,        # LDB #20
    0xBD, 0xF3, 0xDF,  # JSR $F3DF (Draw_Line_d)
    0x39,              # RTS
]

write_vo_file('lib.vo', 
    sections=[('.text.lib', lib_code, 1)],
    exports=[('helper_draw_square', 0, 0)],
    imports=[],
    relocations=[]
)

# Create main.vo - imports helper_draw_square
main_code = [
    0xBD, 0xF2, 0xAB,  # JSR $F2AB (Intensity_a)
    0x86, 0x00,        # LDA #0
    0xC6, 0xE2,        # LDB #-30
    0xBD, 0xF3, 0x12,  # JSR $F312 (Moveto_d)
    0x86, 0x00,        # LDA #0
    0xC6, 0x3C,        # LDB #60
    0xBD, 0xF3, 0xDF,  # JSR $F3DF (Draw_Line_d)
    # JSR helper_draw_square (will be relocated)
    0xBD, 0x00, 0x00,  # JSR $0000 (placeholder)
    0x39,              # RTS
]

write_vo_file('main.vo',
    sections=[('.text.main', main_code, 1)],
    exports=[('main_program', 0, 0)],
    imports=['helper_draw_square'],
    relocations=[
        (0, 22, 0, 'helper_draw_square', 0)  # Absolute16 at offset 22
    ]
)

print("✓ Created lib.vo (exports helper_draw_square)")
print("✓ Created main.vo (imports helper_draw_square)")
print("\nNow run:")
print("  cargo run --bin vectrexc -- link lib.vo main.vo -o linked.bin")
