#!/usr/bin/env python3
"""
Compara las tablas de opcodes entre Vectrexy (C++) y nuestra implementación Rust
para encontrar opcodes que están en Vectrexy pero faltan en Rust.
"""

import re
import sys

# Tabla extraída de Vectrexy CpuOpCodes.h (Page 0 - 256 opcodes completa)
VECTREXY_PAGE0 = {
    0x00: ("NEG", "Direct", 6, 2),
    0x01: ("Illegal", "Illegal", 1, 1),
    0x02: ("Illegal", "Illegal", 1, 1),
    0x03: ("COM", "Direct", 6, 2),
    0x04: ("LSR", "Direct", 6, 2),
    0x05: ("Illegal", "Illegal", 1, 1),
    0x06: ("ROR", "Direct", 6, 2),
    0x07: ("ASR", "Direct", 6, 2),
    0x08: ("LSL/ASL", "Direct", 6, 2),
    0x09: ("ROL", "Direct", 6, 2),
    0x0A: ("DEC", "Direct", 6, 2),
    0x0B: ("Illegal", "Illegal", 1, 1),
    0x0C: ("INC", "Direct", 6, 2),
    0x0D: ("TST", "Direct", 6, 2),
    0x0E: ("JMP", "Direct", 3, 2),
    0x0F: ("CLR", "Direct", 6, 2),
    0x10: ("PAGE1+", "Variant", 1, 1),
    0x11: ("PAGE2+", "Variant", 1, 1),
    0x12: ("NOP", "Inherent", 2, 1),
    0x13: ("SYNC", "Inherent", 2, 1),
    0x14: ("Illegal", "Illegal", 1, 1),
    0x15: ("Illegal", "Illegal", 1, 1),
    0x16: ("LBRA", "Relative", 5, 3),
    0x17: ("LBSR", "Relative", 9, 3),
    0x18: ("Illegal", "Illegal", 1, 1),
    0x19: ("DAA", "Inherent", 2, 1),
    0x1A: ("ORCC", "Immediate", 3, 2),
    0x1B: ("Illegal", "Illegal", 1, 1),
    0x1C: ("ANDCC", "Immediate", 3, 2),
    0x1D: ("SEX", "Inherent", 2, 1),
    0x1E: ("EXG", "Inherent", 8, 2),
    0x1F: ("TFR", "Inherent", 6, 2),
    # 0x20-0x2F: Branches
    0x20: ("BRA", "Relative", 3, 2),
    0x21: ("BRN", "Relative", 3, 2),
    0x22: ("BHI", "Relative", 3, 2),
    0x23: ("BLS", "Relative", 3, 2),
    0x24: ("BHS/BCC", "Relative", 3, 2),
    0x25: ("BLO/BCS", "Relative", 3, 2),
    0x26: ("BNE", "Relative", 3, 2),
    0x27: ("BEQ", "Relative", 3, 2),
    0x28: ("BVC", "Relative", 3, 2),
    0x29: ("BVS", "Relative", 3, 2),
    0x2A: ("BPL", "Relative", 3, 2),
    0x2B: ("BMI", "Relative", 3, 2),
    0x2C: ("BGE", "Relative", 3, 2),
    0x2D: ("BLT", "Relative", 3, 2),
    0x2E: ("BGT", "Relative", 3, 2),
    0x2F: ("BLE", "Relative", 3, 2),
    # 0x30-0x37: LEA and Stack
    0x30: ("LEAX", "Indexed", 4, 2),
    0x31: ("LEAY", "Indexed", 4, 2),
    0x32: ("LEAS", "Indexed", 4, 2),
    0x33: ("LEAU", "Indexed", 4, 2),
    0x34: ("PSHS", "Immediate", 5, 2),
    0x35: ("PULS", "Immediate", 5, 2),
    0x36: ("PSHU", "Immediate", 5, 2),
    0x37: ("PULU", "Immediate", 5, 2),
    0x38: ("Illegal", "Illegal", 1, 1),
    0x39: ("RTS", "Inherent", 5, 1),
    0x3A: ("ABX", "Inherent", 3, 1),  # ← El que faltaba!
    0x3B: ("RTI", "Inherent", 0, 1),
    0x3C: ("CWAI", "Immediate", 20, 2),
    0x3D: ("MUL", "Inherent", 11, 1),
    0x3E: ("RESET*", "Inherent", 0, 1),
    0x3F: ("SWI", "Inherent", 19, 1),
    # 0x40-0x4F: A register operations
    0x40: ("NEGA", "Inherent", 2, 1),
    0x41: ("Illegal", "Illegal", 1, 1),
    0x42: ("Illegal", "Illegal", 1, 1),
    0x43: ("COMA", "Inherent", 2, 1),
    0x44: ("LSRA", "Inherent", 2, 1),
    0x45: ("Illegal", "Illegal", 1, 1),
    0x46: ("RORA", "Inherent", 2, 1),
    0x47: ("ASRA", "Inherent", 2, 1),
    0x48: ("LSLA/ASLA", "Inherent", 2, 1),
    0x49: ("ROLA", "Inherent", 2, 1),
    0x4A: ("DECA", "Inherent", 2, 1),
    0x4B: ("Illegal", "Illegal", 1, 1),
    0x4C: ("INCA", "Inherent", 2, 1),
    0x4D: ("TSTA", "Inherent", 2, 1),
    0x4E: ("Illegal", "Illegal", 1, 1),
    0x4F: ("CLRA", "Inherent", 2, 1),
    # 0x50-0x5F: B register operations
    0x50: ("NEGB", "Inherent", 2, 1),
    0x51: ("Illegal", "Illegal", 1, 1),
    0x52: ("Illegal", "Illegal", 1, 1),
    0x53: ("COMB", "Inherent", 2, 1),
    0x54: ("LSRB", "Inherent", 2, 1),
    0x55: ("Illegal", "Illegal", 1, 1),
    0x56: ("RORB", "Inherent", 2, 1),
    0x57: ("ASRB", "Inherent", 2, 1),
    0x58: ("LSLB/ASLB", "Inherent", 2, 1),
    0x59: ("ROLB", "Inherent", 2, 1),
    0x5A: ("DECB", "Inherent", 2, 1),
    0x5B: ("Illegal", "Illegal", 1, 1),
    0x5C: ("INCB", "Inherent", 2, 1),
    0x5D: ("TSTB", "Inherent", 2, 1),
    0x5E: ("Illegal", "Illegal", 1, 1),
    0x5F: ("CLRB", "Inherent", 2, 1),
}

# Los demás opcodes 0x60-0xFF siguen el mismo patrón (Indexed y Extended)
# pero no los agrego todos para mantener el script conciso.
# Solo nos interesan los que podrían faltar en la tabla Rust.

def parse_rust_opcodes(rust_file_path):
    """Extrae opcodes definidos en cpu_op_codes.rs - SOLO Page0"""
    opcodes = {}
    
    with open(rust_file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Encontrar la función lookup_cpu_op_page0 específicamente
    page0_start = content.find('fn lookup_cpu_op_page0(op_code: u8) -> CpuOp')
    if page0_start == -1:
        print("⚠️  No se encontró lookup_cpu_op_page0")
        return opcodes
    
    # Buscar el final de la función (antes de lookup_cpu_op_page1)
    page0_end = content.find('fn lookup_cpu_op_page1(', page0_start)
    if page0_end == -1:
        page0_end = len(content)
    
    # Extraer solo el contenido de Page0
    page0_content = content[page0_start:page0_end]
    
    # Buscar bloques de match con opcodes
    # Patrón: 0xNN => CpuOp { ... }
    pattern = r'0x([0-9A-Fa-f]{2})\s*=>\s*CpuOp\s*\{[^}]+name:\s*"([^"]+)"[^}]+addr_mode:\s*AddressingMode::(\w+)[^}]+cycles:\s*(\d+)[^}]+size:\s*(\d+)'
    
    matches = re.finditer(pattern, page0_content, re.MULTILINE | re.DOTALL)
    
    for match in matches:
        opcode = int(match.group(1), 16)
        name = match.group(2)
        addr_mode = match.group(3)
        cycles = int(match.group(4))
        size = int(match.group(5))
        
        opcodes[opcode] = (name, addr_mode, cycles, size)
    
    return opcodes

def main():
    rust_file = r"C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\emulator_v2\src\core\cpu_op_codes.rs"
    
    print("🔍 Comparando tablas de opcodes Vectrexy (C++) vs Rust...")
    print("=" * 80)
    
    rust_opcodes = parse_rust_opcodes(rust_file)
    
    print(f"\n📊 Estadísticas:")
    print(f"   Vectrexy Page0: {len(VECTREXY_PAGE0)} opcodes definidos (muestra)")
    print(f"   Rust:           {len(rust_opcodes)} opcodes encontrados")
    
    # Buscar opcodes que están en Vectrexy pero NO en Rust (en el rango 0x00-0x5F)
    missing_in_rust = []
    
    for opcode, vectrexy_data in VECTREXY_PAGE0.items():
        name, addr_mode, cycles, size = vectrexy_data
        
        # Ignorar opcodes ilegales
        if name == "Illegal":
            continue
        
        # Verificar si está en Rust
        if opcode not in rust_opcodes:
            missing_in_rust.append((opcode, name, addr_mode, cycles, size))
    
    print(f"\n❌ Opcodes FALTANTES en Rust (están en Vectrexy):")
    print("=" * 80)
    
    if missing_in_rust:
        for opcode, name, addr_mode, cycles, size in sorted(missing_in_rust):
            print(f"   0x{opcode:02X}: {name:12s} | {addr_mode:10s} | {cycles:2d} cycles | {size} bytes")
        print(f"\n   Total faltantes: {len(missing_in_rust)}")
    else:
        print("   ✅ ¡Todos los opcodes de Vectrexy (0x00-0x5F) están en Rust!")
    
    # Buscar diferencias en los que SÍ están
    print(f"\n⚠️  Opcodes con DIFERENCIAS (están en ambos pero con datos distintos):")
    print("=" * 80)
    
    differences = []
    for opcode, vectrexy_data in VECTREXY_PAGE0.items():
        v_name, v_addr, v_cycles, v_size = vectrexy_data
        
        if v_name == "Illegal":
            continue
        
        if opcode in rust_opcodes:
            r_name, r_addr, r_cycles, r_size = rust_opcodes[opcode]
            
            diff = []
            if v_cycles != r_cycles:
                diff.append(f"cycles: {v_cycles} vs {r_cycles}")
            if v_size != r_size:
                diff.append(f"size: {v_size} vs {r_size}")
            if v_addr != r_addr:
                diff.append(f"addr_mode: {v_addr} vs {r_addr}")
            
            if diff:
                differences.append((opcode, v_name, r_name, ", ".join(diff)))
    
    if differences:
        for opcode, v_name, r_name, diff_str in sorted(differences):
            print(f"   0x{opcode:02X}: {v_name:12s} | {diff_str}")
        print(f"\n   Total con diferencias: {len(differences)}")
    else:
        print("   ✅ ¡Todos los opcodes coinciden perfectamente!")
    
    print("\n" + "=" * 80)
    if missing_in_rust:
        print(f"❌ CONCLUSIÓN: Faltan {len(missing_in_rust)} opcodes en la tabla Rust")
        return 1
    else:
        print("✅ CONCLUSIÓN: La tabla Rust está completa para el rango verificado")
        return 0

if __name__ == "__main__":
    sys.exit(main())
