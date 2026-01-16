#!/usr/bin/env python3
# Analyze what the ASM file says vs what está en el ROM

asm_content = open('examples/test_multibank_pdb/src/main.asm', 'r').read()

# Find where the strings are defined
print("=== ASM STRUCTURE ===\n")

# Count lines to find approximate positions
lines = asm_content.split('\n')
for i, line in enumerate(lines):
    if 'ORG' in line or 'PRINT_TEXT' in line or 'LOOP_BODY' in line or 'MAIN_LOOP' in line or 'FCC' in line:
        print(f"Line {i+1}: {line[:80]}")

print("\n=== ANALYSIS ===")
print("The ASM has:")
print("1. START at ORG $0000")
print("2. MAIN code")
print("3. LOOP_BODY (has JSR VECTREX_PRINT_TEXT)")
print("4. String data (PRINT_TEXT_STR_68624562, PRINT_TEXT_STR_82781042)")
print("5. ORG $4000 - switches to bank 31")
print("6. VECTREX_PRINT_TEXT helper")
print("7. MUL16, DIV16 helpers")
print()
print("The problem:")
print("- Strings are defined in BANK 0 section (before ORG $4000)")
print("- But when LDX #PRINT_TEXT_STR_68624562 is assembled, it assumes")
print("  the string is in BANK 31 (where VECTREX_PRINT_TEXT is)")
print()
print("This is a LINKER symbol resolution issue:")
print("- Bank 0 assembler doesn't know where strings will end up")
print("- Bank 31 assembler doesn't know about Bank 0 strings")
print("- The two-pass assembly needs to resolve this correctly")
