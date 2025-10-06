#!/usr/bin/env python3
"""
Compare instruction traces from Rust and Vectrexy runners
Finds the first point of divergence
"""

import sys
import re

def parse_trace_line(line):
    """
    Parse trace line format:
    [TRACE] Instr #123: PC=0xF000 Opcode=0x8E
    Returns: (instr_num, pc, opcode)
    """
    match = re.search(r'Instr #(\d+): PC=0x([0-9A-Fa-f]+) Opcode=0x([0-9A-Fa-f]+)', line)
    if match:
        return (int(match.group(1)), int(match.group(2), 16), int(match.group(3), 16))
    return None

def main():
    if len(sys.argv) != 3:
        print("Usage: python compare_traces.py <rust_trace.log> <vectrexy_trace.log>")
        sys.exit(1)
    
    rust_file = sys.argv[1]
    vectrexy_file = sys.argv[2]
    
    print("Loading Rust trace...")
    with open(rust_file, 'r') as f:
        rust_lines = [line.strip() for line in f if '[TRACE]' in line]
    
    print("Loading Vectrexy trace...")
    with open(vectrexy_file, 'r') as f:
        vectrexy_lines = [line.strip() for line in f if '[TRACE]' in line]
    
    print(f"\nRust instructions: {len(rust_lines)}")
    print(f"Vectrexy instructions: {len(vectrexy_lines)}")
    print(f"\nComparing first {min(len(rust_lines), len(vectrexy_lines))} instructions...\n")
    
    divergence_found = False
    last_match_idx = -1
    
    for i in range(min(len(rust_lines), len(vectrexy_lines))):
        rust_data = parse_trace_line(rust_lines[i])
        vectrexy_data = parse_trace_line(vectrexy_lines[i])
        
        if not rust_data or not vectrexy_data:
            print(f"ERROR: Could not parse line {i+1}")
            print(f"  Rust: {rust_lines[i]}")
            print(f"  Vectrexy: {vectrexy_lines[i]}")
            break
        
        rust_num, rust_pc, rust_opcode = rust_data
        vec_num, vec_pc, vec_opcode = vectrexy_data
        
        if rust_pc != vec_pc or rust_opcode != vec_opcode:
            print(f"🔴 DIVERGENCE FOUND at instruction #{i+1}:")
            print(f"")
            print(f"  Rust:     Instr #{rust_num}: PC=0x{rust_pc:04X} Opcode=0x{rust_opcode:02X}")
            print(f"  Vectrexy: Instr #{vec_num}: PC=0x{vec_pc:04X} Opcode=0x{vec_opcode:02X}")
            print(f"")
            
            if rust_pc != vec_pc:
                print(f"  ❌ PC mismatch: Rust=0x{rust_pc:04X} vs Vectrexy=0x{vec_pc:04X} (Δ={rust_pc - vec_pc})")
            if rust_opcode != vec_opcode:
                print(f"  ❌ Opcode mismatch: Rust=0x{rust_opcode:02X} vs Vectrexy=0x{vec_opcode:02X}")
            
            # Show context (previous 5 instructions)
            print(f"\n  📜 Context (last 5 matching instructions):")
            for j in range(max(0, i-5), i):
                rust_ctx = parse_trace_line(rust_lines[j])
                vec_ctx = parse_trace_line(vectrexy_lines[j])
                if rust_ctx and vec_ctx:
                    r_num, r_pc, r_op = rust_ctx
                    v_num, v_pc, v_op = vec_ctx
                    match = "✅" if (r_pc == v_pc and r_op == v_op) else "❌"
                    print(f"    {match} #{j+1}: Rust PC=0x{r_pc:04X} Op=0x{r_op:02X} | Vectrexy PC=0x{v_pc:04X} Op=0x{v_op:02X}")
            
            divergence_found = True
            last_match_idx = i - 1
            break
        else:
            last_match_idx = i
            if (i + 1) % 10000 == 0:
                print(f"  ✅ Still matching at instruction #{i+1}: PC=0x{rust_pc:04X} Opcode=0x{rust_opcode:02X}")
    
    if not divergence_found:
        print(f"✅ NO DIVERGENCE FOUND in first {min(len(rust_lines), len(vectrexy_lines))} instructions")
        print(f"   Last matched instruction: #{last_match_idx + 1}")
        
        if len(rust_lines) != len(vectrexy_lines):
            print(f"\n⚠️  WARNING: Different instruction counts:")
            print(f"   Rust has {len(rust_lines)} instructions")
            print(f"   Vectrexy has {len(vectrexy_lines)} instructions")
            print(f"   Difference: {abs(len(rust_lines) - len(vectrexy_lines))} instructions")
    else:
        print(f"\n📊 SUMMARY:")
        print(f"   Instructions matched: {last_match_idx + 1}")
        print(f"   Divergence at instruction: #{last_match_idx + 2}")
        print(f"   Remaining Rust instructions: {len(rust_lines) - (last_match_idx + 2)}")
        print(f"   Remaining Vectrexy instructions: {len(vectrexy_lines) - (last_match_idx + 2)}")

if __name__ == "__main__":
    main()
