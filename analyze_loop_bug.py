#!/usr/bin/env python3
"""
Script para generar un test específico del loop problemático
"""

def generate_test_sequence():
    """Generar la secuencia exacta que causa el problema"""
    
    print("=== SECUENCIA PROBLEMÁTICA ===")
    print("F548: 6F 8B    ; CLR 11,X")
    print("F54A: 83 00 01 ; SUBD #$0001")
    print("F54D: 2A F9    ; BPL $F548")
    print()
    
    print("Estado inicial esperado:")
    print("X = C800, B = 7A")
    print()
    
    print("Loop iteration 1:")
    print("1. CLR 11,X  -> clear memory at C800+11=C80B, X should remain C800")
    print("2. SUBD #1   -> D = D-1, so B becomes 79, X should remain C800")  
    print("3. BPL       -> branch back to F548 if positive")
    print()
    
    print("Expected: X stays C800 throughout")
    print("Actual (Rust): X increments C800->C801->C802...")
    print("Actual (JSVecx): X stays C800")
    print()
    
    print("HYPOTHESIS: CLR indexed is incorrectly incrementing X")
    print("POSTBYTE: 8B = 10001011")
    print("  Bit 7 = 1: indexed mode")
    print("  Bits 6-5 = 00: X register")
    print("  Bits 4-0 = 01011 = 11: 5-bit signed offset")
    print("Should calculate: X + 11 = C800 + 0B = C80B")
    print("Should NOT modify X register")

if __name__ == "__main__":
    generate_test_sequence()