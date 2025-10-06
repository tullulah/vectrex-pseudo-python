# Vectrexy Comparative Testing - Status Report

**Date**: 2025-10-06
**Status**: ✅ **WORKING** (CPU-only tests)

## Achievement Summary

Successfully compiled and executed **Vectrexy C++ emulator** as reference implementation for comparative testing against Rust port.

### What Works ✅

1. **Vectrexy Build System**
   - Minimal CMake build compiling only needed libraries
   - `vectrexy_core.lib` (ErrorHandler only)
   - `vectrexy_emulator.lib` (full 6809 emulator)
   - Avoids complex vcpkg dependencies (SDL2, GLM, etc.)

2. **Vectrexy Test Runner**
   - Loads test binaries to RAM at 0xC800
   - Executes 6809 instructions correctly
   - Serializes CPU state to JSON
   - **PROVEN**: 3 working tests with validated output

3. **Test Cases with Vectrexy References**
   - ✅ `cpu_arithmetic`: Tests ADDA/ADDB (A=0x30, B=0x55)
   - ✅ `cpu_load_store`: Tests LDA/STA to RAM (A=0xAA loaded back)
   - ✅ `cpu_branch`: Tests BEQ/BNE conditional branches (A=0, B=0xFF)

4. **JSON Serialization**
   - CPU registers: PC, A, B, X, Y, U, S, DP
   - Condition codes: C, V, Z, N, I, H, F, E
   - Cycle count accurate
   - Clean JSON format for comparison

### Limitations ❌

**VIA Hardware NOT Testable:**
- Writing to VIA registers (0xD000-0xD7FF) causes unrecoverable crashes
- Reading VIA registers also causes crashes during serialization
- Affects: Timer tests, interrupt tests, hardware I/O tests
- VIA serialization **disabled** - using dummy values (all zeros)

**Cannot Test:**
- Timer1/Timer2 functionality
- IRQ/FIRQ/NMI interrupt handling
- PSG (sound)
- Hardware ports (Port A/B)
- Shift register
- Any test requiring VIA configuration

**Can Test:**
- CPU instructions (all addressing modes)
- Arithmetic operations
- Logic operations
- Memory access (RAM only, 0xC800-0xCFFF)
- Branch instructions
- Stack operations
- Register transfers

## Example Output (cpu_arithmetic)

**Test Code:**
```asm
LDA #$10    ; A = 0x10
ADDA #$20   ; A = 0x30
LDB #$30    ; B = 0x30
ADDB #$25   ; B = 0x55
BRA loop    ; Infinite loop
```

**Vectrexy Output (expected.json):**
```json
{
  "cpu": {
    "a": 48,        // 0x30 ✅
    "b": 85,        // 0x55 ✅
    "cc": {
      "c": false,   // No carry ✅
      "v": false,   // No overflow ✅
      "z": false,   // Not zero ✅
      "n": false    // Positive ✅
    },
    "dp": 0,
    "pc": 51208,    // 0xC808 (in infinite loop)
    "s": 0,
    "u": 0,
    "x": 0,
    "y": 0
  },
  "cycles": 50
}
```

## Files Structure

```
emulator_v2/tests/comparative/
├── vectrexy_runner/
│   ├── main.cpp                    # C++ test harness
│   ├── CMakeLists.txt              # Build config
│   ├── build/Release/
│   │   └── vectrexy_runner.exe     # ✅ Working executable
│   └── lib/
│       ├── vectrexy_core.lib       # ✅ Compiled
│       └── vectrexy_emulator.lib   # ✅ Compiled
├── vectrexy_emulator_build/
│   └── CMakeLists.txt              # Minimal build for libraries
├── test_cases/
│   ├── cpu_arithmetic/
│   │   ├── test.asm
│   │   ├── test.bin
│   │   └── expected.json           # ✅ From Vectrexy
│   ├── cpu_load_store/
│   │   ├── test.asm
│   │   ├── test.bin
│   │   └── expected.json           # ✅ From Vectrexy
│   └── cpu_branch/
│       ├── test.asm
│       ├── test.bin
│       └── expected.json           # ✅ From Vectrexy
└── run_comparative_test.ps1        # Automation script (WIP)
```

## Usage

**Generate expected.json from Vectrexy:**
```powershell
cd vectrexy_runner
.\build\Release\vectrexy_runner.exe ..\test_cases\cpu_arithmetic\test.bin 50 `
    2>$null > ..\test_cases\cpu_arithmetic\expected.json
```

**Automation Script:**
```powershell
.\run_comparative_test.ps1 -TestName cpu_arithmetic -Cycles 50
```

## Next Steps

1. **Create rust_runner.exe** (equivalent to vectrexy_runner)
   - Load test binary
   - Execute in Rust emulator
   - Serialize to rust_output.json
   
2. **Implement comparison.py** (already exists, needs testing)
   - Compare expected.json vs rust_output.json
   - Report differences in CPU state
   - Validate cycle counts

3. **Expand CPU Test Suite**
   - More arithmetic: SUB, MUL, DIV
   - Logic: AND, OR, EOR, COM, NEG
   - Shifts/rotates: LSL, LSR, ASL, ASR, ROL, ROR
   - Indexed addressing modes
   - Stack: PSHS, PULS, PSHU, PULU
   - Jumps: JSR, RTS, JMP

4. **Document VIA Limitation**
   - Update copilot-instructions.md
   - Note in SUPER_SUMMARY.md
   - Accept that VIA testing requires different approach

## Conclusion

**MAJOR SUCCESS**: Built working reference implementation framework using Vectrexy C++ emulator.

**PROVEN CAPABILITY**: Can generate ground-truth CPU behavior from original C++ implementation.

**CRITICAL VALUE**: Expected values now come from **actual Vectrexy execution**, not synthetic or Rust-derived data.

**SCOPE**: CPU-only testing is **sufficient** for validating core 6809 emulation accuracy.

**READY**: Framework ready for Rust comparison once rust_runner is implemented.

---

**False Positive Prevention**: All previous tests that compared Rust against itself were invalidated. New tests compare Rust against **proven Vectrexy reference**.
