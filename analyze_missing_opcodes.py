#!/usr/bin/env python3
"""
Analyze missing opcodes in the 6809 CPU emulator
Compares implemented opcodes vs the complete 6809 instruction set
"""

# Complete 6809 instruction set (from official documentation)
COMPLETE_6809_OPCODES = {
    # Page 0 (0x00-0xFF)
    0x00: "NEG direct", 0x01: "OIM direct", 0x02: "AIM direct", 0x03: "COM direct",
    0x04: "LSR direct", 0x05: "EIM direct", 0x06: "ROR direct", 0x07: "ASR direct", 
    0x08: "ASL direct", 0x09: "ROL direct", 0x0A: "DEC direct", 0x0B: "TIM direct",
    0x0C: "INC direct", 0x0D: "TST direct", 0x0E: "JMP direct", 0x0F: "CLR direct",
    
    0x10: "Page1", 0x11: "Page2", 0x12: "NOP", 0x13: "SYNC",
    0x14: "HCF/HALT", 0x15: "HCF/HALT", 0x16: "LBRA", 0x17: "LBSR",
    0x18: "DAA", 0x19: "ORCC", 0x1A: "ANDCC", 0x1B: "SEX",
    0x1C: "EXG", 0x1D: "TFR", 0x1E: "BR", 0x1F: "LEA",
    
    0x20: "BRA", 0x21: "BRN", 0x22: "BHI", 0x23: "BLS",
    0x24: "BCC/BHS", 0x25: "BCS/BLO", 0x26: "BNE", 0x27: "BEQ",
    0x28: "BVC", 0x29: "BVS", 0x2A: "BPL", 0x2B: "BMI",
    0x2C: "BGE", 0x2D: "BLT", 0x2E: "BGT", 0x2F: "BLE",
    
    0x30: "LEAX", 0x31: "LEAY", 0x32: "LEAS", 0x33: "LEAU",
    0x34: "PSHS", 0x35: "PULS", 0x36: "PSHU", 0x37: "PULU",
    0x38: "ANDCC", 0x39: "RTS", 0x3A: "ABX", 0x3B: "RTI",
    0x3C: "CWAI", 0x3D: "MUL", 0x3E: "RESET", 0x3F: "SWI",
    
    # A register opcodes (0x40-0x5F)
    0x40: "NEGA", 0x43: "COMA", 0x44: "LSRA", 0x46: "RORA", 0x47: "ASRA",
    0x48: "ASLA/LSLA", 0x49: "ROLA", 0x4A: "DECA", 0x4C: "INCA", 0x4D: "TSTA",
    0x4F: "CLRA",
    
    0x50: "NEGB", 0x53: "COMB", 0x54: "LSRB", 0x56: "RORB", 0x57: "ASRB",
    0x58: "ASLB/LSLB", 0x59: "ROLB", 0x5A: "DECB", 0x5C: "INCB", 0x5D: "TSTB",
    0x5F: "CLRB",
    
    # Indexed opcodes (0x60-0x7F)
    0x60: "NEG indexed", 0x61: "OIM indexed", 0x62: "AIM indexed", 0x63: "COM indexed",
    0x64: "LSR indexed", 0x65: "EIM indexed", 0x66: "ROR indexed", 0x67: "ASR indexed",
    0x68: "ASL indexed", 0x69: "ROL indexed", 0x6A: "DEC indexed", 0x6B: "TIM indexed",
    0x6C: "INC indexed", 0x6D: "TST indexed", 0x6E: "JMP indexed", 0x6F: "CLR indexed",
    
    0x70: "NEG extended", 0x71: "OIM extended", 0x72: "AIM extended", 0x73: "COM extended",
    0x74: "LSR extended", 0x75: "EIM extended", 0x76: "ROR extended", 0x77: "ASR extended",
    0x78: "ASL extended", 0x79: "ROL extended", 0x7A: "DEC extended", 0x7B: "TIM extended",
    0x7C: "INC extended", 0x7D: "TST extended", 0x7E: "JMP extended", 0x7F: "CLR extended",
    
    # Load/Store opcodes (0x80-0xFF)
    0x80: "SUBA imm", 0x81: "CMPA imm", 0x82: "SBCA imm", 0x83: "SUBD imm",
    0x84: "ANDA imm", 0x85: "BITA imm", 0x86: "LDA imm", 0x87: "STA imm",
    0x88: "EORA imm", 0x89: "ADCA imm", 0x8A: "ORA imm", 0x8B: "ADDA imm",
    0x8C: "CMPX imm", 0x8D: "BSR", 0x8E: "LDX imm", 0x8F: "STX imm",
    
    0x90: "SUBA direct", 0x91: "CMPA direct", 0x92: "SBCA direct", 0x93: "SUBD direct",
    0x94: "ANDA direct", 0x95: "BITA direct", 0x96: "LDA direct", 0x97: "STA direct",
    0x98: "EORA direct", 0x99: "ADCA direct", 0x9A: "ORA direct", 0x9B: "ADDA direct",
    0x9C: "CMPX direct", 0x9D: "JSR direct", 0x9E: "LDX direct", 0x9F: "STX direct",
    
    0xA0: "SUBA indexed", 0xA1: "CMPA indexed", 0xA2: "SBCA indexed", 0xA3: "SUBD indexed",
    0xA4: "ANDA indexed", 0xA5: "BITA indexed", 0xA6: "LDA indexed", 0xA7: "STA indexed",
    0xA8: "EORA indexed", 0xA9: "ADCA indexed", 0xAA: "ORA indexed", 0xAB: "ADDA indexed",
    0xAC: "CMPX indexed", 0xAD: "JSR indexed", 0xAE: "LDX indexed", 0xAF: "STX indexed",
    
    0xB0: "SUBA extended", 0xB1: "CMPA extended", 0xB2: "SBCA extended", 0xB3: "SUBD extended",
    0xB4: "ANDA extended", 0xB5: "BITA extended", 0xB6: "LDA extended", 0xB7: "STA extended",
    0xB8: "EORA extended", 0xB9: "ADCA extended", 0xBA: "ORA extended", 0xBB: "ADDA extended",
    0xBC: "CMPX extended", 0xBD: "JSR extended", 0xBE: "LDX extended", 0xBF: "STX extended",
    
    0xC0: "SUBB imm", 0xC1: "CMPB imm", 0xC2: "SBCB imm", 0xC3: "ADDD imm",
    0xC4: "ANDB imm", 0xC5: "BITB imm", 0xC6: "LDB imm", 0xC7: "STB imm",
    0xC8: "EORB imm", 0xC9: "ADCB imm", 0xCA: "ORB imm", 0xCB: "ADDB imm",
    0xCC: "LDD imm", 0xCD: "STD imm", 0xCE: "LDU imm", 0xCF: "STU imm",
    
    0xD0: "SUBB direct", 0xD1: "CMPB direct", 0xD2: "SBCB direct", 0xD3: "ADDD direct",
    0xD4: "ANDB direct", 0xD5: "BITB direct", 0xD6: "LDB direct", 0xD7: "STB direct",
    0xD8: "EORB direct", 0xD9: "ADCB direct", 0xDA: "ORB direct", 0xDB: "ADDB direct",
    0xDC: "LDD direct", 0xDD: "STD direct", 0xDE: "LDU direct", 0xDF: "STU direct",
    
    0xE0: "SUBB indexed", 0xE1: "CMPB indexed", 0xE2: "SBCB indexed", 0xE3: "ADDD indexed",
    0xE4: "ANDB indexed", 0xE5: "BITB indexed", 0xE6: "LDB indexed", 0xE7: "STB indexed",
    0xE8: "EORB indexed", 0xE9: "ADCB indexed", 0xEA: "ORB indexed", 0xEB: "ADDB indexed",
    0xEC: "LDD indexed", 0xED: "STD indexed", 0xEE: "LDU indexed", 0xEF: "STU indexed",
    
    0xF0: "SUBB extended", 0xF1: "CMPB extended", 0xF2: "SBCB extended", 0xF3: "ADDD extended",
    0xF4: "ANDB extended", 0xF5: "BITB extended", 0xF6: "LDB extended", 0xF7: "STB extended",
    0xF8: "EORB extended", 0xF9: "ADCB extended", 0xFA: "ORB extended", 0xFB: "ADDB extended",
    0xFC: "LDD extended", 0xFD: "STD extended", 0xFE: "LDU extended", 0xFF: "STU extended",
}

# Page 1 opcodes (0x10xx)
PAGE1_OPCODES = {
    0x20: "LBRA", 0x21: "LBRN", 0x22: "LBHI", 0x23: "LBLS",
    0x24: "LBCC/LBHS", 0x25: "LBCS/LBLO", 0x26: "LBNE", 0x27: "LBEQ",
    0x28: "LBVC", 0x29: "LBVS", 0x2A: "LBPL", 0x2B: "LBMI",
    0x2C: "LBGE", 0x2D: "LBLT", 0x2E: "LBGT", 0x2F: "LBLE",
    
    0x3F: "SWI2",
    
    0x83: "CMPD imm", 0x8C: "CMPY imm", 0x8E: "LDY imm", 0x8F: "STY imm",
    0x93: "CMPD direct", 0x9C: "CMPY direct", 0x9E: "LDY direct", 0x9F: "STY direct",
    0xA3: "CMPD indexed", 0xAC: "CMPY indexed", 0xAE: "LDY indexed", 0xAF: "STY indexed",
    0xB3: "CMPD extended", 0xBC: "CMPY extended", 0xBE: "LDY extended", 0xBF: "STY extended",
    
    0xCE: "LDS imm", 0xCF: "STS imm",
    0xDE: "LDS direct", 0xDF: "STS direct",
    0xEE: "LDS indexed", 0xEF: "STS indexed",
    0xFE: "LDS extended", 0xFF: "STS extended",
}

# Page 2 opcodes (0x11xx)
PAGE2_OPCODES = {
    0x3F: "SWI3",
    
    0x83: "CMPU imm", 0x8C: "CMPS imm",
    0x93: "CMPU direct", 0x9C: "CMPS direct",
    0xA3: "CMPU indexed", 0xAC: "CMPS indexed",
    0xB3: "CMPU extended", 0xBC: "CMPS extended",
}

def analyze_opcodes():
    print("=== 6809 CPU OPCODE IMPLEMENTATION ANALYSIS ===\n")
    
    # Read current implementation from cpu6809.rs
    try:
        with open("C:/Users/DanielFerrerGuerrero/source/repos/pseudo-python/emulator_v2/src/core/cpu6809.rs", "r") as f:
            cpu_content = f.read()
    except FileNotFoundError:
        print("Could not find cpu6809.rs file")
        return
        
    # Find implemented opcodes by looking for case statements
    import re
    
    # Page 0 opcodes
    page0_matches = re.findall(r'0x([0-9A-Fa-f]{2}) => \{', cpu_content)
    implemented_page0 = set(int(m, 16) for m in page0_matches)
    
    print("PAGE 0 OPCODES (0x00-0xFF):")
    print("============================")
    
    missing_page0 = []
    for opcode in range(0x00, 0x100):
        if opcode in [0x10, 0x11]:  # Skip page markers
            continue
        if opcode in implemented_page0:
            status = "✅ IMPLEMENTED"
        else:
            status = "❌ MISSING"
            missing_page0.append(opcode)
            
        opcode_name = COMPLETE_6809_OPCODES.get(opcode, "UNKNOWN")
        print(f"  0x{opcode:02X}: {opcode_name:<20} {status}")
    
    print(f"\nPage 0 Summary: {len(implemented_page0)}/254 implemented, {len(missing_page0)} missing")
    
    # Page 1 opcodes  
    print(f"\nPAGE 1 OPCODES (0x10xx):")
    print("=========================")
    
    # Find page 1 implementations
    page1_section = re.search(r'1 => \{.*?// Page 1 instructions.*?\}', cpu_content, re.DOTALL)
    if page1_section:
        page1_matches = re.findall(r'0x([0-9A-Fa-f]{2}) => \{', page1_section.group(0))
        implemented_page1 = set(int(m, 16) for m in page1_matches)
    else:
        implemented_page1 = set()
    
    missing_page1 = []
    for opcode, name in PAGE1_OPCODES.items():
        if opcode in implemented_page1:
            status = "✅ IMPLEMENTED"
        else:
            status = "❌ MISSING"
            missing_page1.append(opcode)
        print(f"  0x10{opcode:02X}: {name:<20} {status}")
    
    print(f"\nPage 1 Summary: {len(implemented_page1)}/{len(PAGE1_OPCODES)} implemented, {len(missing_page1)} missing")
    
    # Page 2 opcodes
    print(f"\nPAGE 2 OPCODES (0x11xx):")
    print("=========================")
    
    page2_section = re.search(r'2 => \{.*?// Page 2 instructions.*?\}', cpu_content, re.DOTALL)
    if page2_section:
        page2_matches = re.findall(r'0x([0-9A-Fa-f]{2}) => \{', page2_section.group(0))
        implemented_page2 = set(int(m, 16) for m in page2_matches)
    else:
        implemented_page2 = set()
    
    missing_page2 = []
    for opcode, name in PAGE2_OPCODES.items():
        if opcode in implemented_page2:
            status = "✅ IMPLEMENTED"
        else:
            status = "❌ MISSING"
            missing_page2.append(opcode)
        print(f"  0x11{opcode:02X}: {name:<20} {status}")
    
    print(f"\nPage 2 Summary: {len(implemented_page2)}/{len(PAGE2_OPCODES)} implemented, {len(missing_page2)} missing")
    
    # Overall summary
    total_implemented = len(implemented_page0) + len(implemented_page1) + len(implemented_page2)
    total_opcodes = 254 + len(PAGE1_OPCODES) + len(PAGE2_OPCODES)  # 254 because we exclude 0x10, 0x11
    
    print(f"\n=== OVERALL SUMMARY ===")
    print(f"Total Implemented: {total_implemented}/{total_opcodes} opcodes")
    print(f"Implementation Rate: {total_implemented/total_opcodes*100:.1f}%")
    
    print(f"\n=== HIGH PRIORITY MISSING OPCODES ===")
    high_priority = [
        (0x34, "PSHS - Push System Stack"),
        (0x35, "PULS - Pull System Stack"), 
        (0x36, "PSHU - Push User Stack"),
        (0x37, "PULU - Pull User Stack"),
        (0x3C, "CWAI - Clear and Wait for Interrupt"),
        (0x3D, "MUL - Multiply"),
        (0x3E, "RESET - Reset"),
        (0x3F, "SWI - Software Interrupt"),
        (0x12, "NOP - No Operation"),
        (0x13, "SYNC - Synchronize with Interrupt"),
        (0x19, "DAA - Decimal Adjust A"),
    ]
    
    for opcode, desc in high_priority:
        if opcode in missing_page0:
            print(f"  ❌ 0x{opcode:02X}: {desc}")
        elif opcode in implemented_page0:
            print(f"  ✅ 0x{opcode:02X}: {desc}")

if __name__ == "__main__":
    analyze_opcodes()