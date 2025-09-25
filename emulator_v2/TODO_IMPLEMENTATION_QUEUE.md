# TODO LIST - Emulator v2 Opcode Implementation Priority Queue

## IMMEDIATE PRIORITY (High Impact BIOS Operations) 🔥

### 1. JMP (Jump) - CRÍTICO ⭐⭐⭐⭐⭐
- **Opcodes**: 0x0E (direct), 0x6E (indexed), 0x7E (extended)  
- **Cycles**: 3, 3, 4 respectivamente
- **C++ Reference**: `vectrexy_backup/libs/emulator/src/Cpu.cpp` → `void OpJMP() { PC = EA; }`
- **Importancia**: BIOS hace saltos incondicionales constantemente
- **Estado**: ❌ NO IMPLEMENTADO
- **Feature Branch**: `feature/jmp-opcode-implementation`

### 2. MUL (Multiply) - ALTO IMPACTO ⭐⭐⭐⭐
- **Opcode**: 0x3D (inherent)
- **Cycles**: 11
- **C++ Reference**: `A:B = A * B; CC.Zero = (A:B == 0); CC.Carry = B[7];`
- **Importancia**: Usado para cálculos vectoriales en la BIOS
- **Estado**: ❌ NO IMPLEMENTADO

### 3. SWI (Software Interrupt) - CRÍTICO DEBUG ⭐⭐⭐⭐
- **Opcodes**: 0x3F (SWI), 0x103F (SWI2), 0x113F (SWI3)
- **Cycles**: 19, 20, 20
- **C++ Reference**: Push registers, set interrupt mask, jump to vector
- **Importancia**: BIOS usa para manejo de errores y debug
- **Estado**: ❌ NO IMPLEMENTADO

### 4. RTI (Return from Interrupt) - CRÍTICO ⭐⭐⭐⭐
- **Opcode**: 0x3B (inherent)
- **Cycles**: 6 o 15 (depende del stack frame)
- **C++ Reference**: Pop registers from stack, restore PC y CC
- **Importancia**: Complementa SWI para manejo correcto de interrupts
- **Estado**: ❌ NO IMPLEMENTADO

## PRIORITY LEVEL 2 (Core Missing Instructions) 🚀

### 5. SYNC - Importante para timing ⭐⭐⭐
- **Opcode**: 0x13 (inherent)
- **Cycles**: 2 (+ wait for interrupt)
- **Importancia**: BIOS usa para sincronización con interrupts
- **Estado**: ❌ NO IMPLEMENTADO

### 6. CWAI (Clear CC and Wait) ⭐⭐⭐
- **Opcode**: 0x3C (immediate) 
- **Cycles**: 20 (+ wait for interrupt)
- **C++ Reference**: Clear specified CC bits, push registers, wait for interrupt
- **Estado**: ❌ NO IMPLEMENTADO

### 7. SEX (Sign Extend B to A) ⭐⭐⭐
- **Opcode**: 0x1D (inherent)
- **Cycles**: 2
- **C++ Reference**: `A = B[7] ? 0xFF : 0x00; CC.N = A[7]; CC.Z = (D == 0);`
- **Importancia**: Conversión signed 8→16 bit muy común
- **Estado**: ❌ NO IMPLEMENTADO

### 8. Long Branch Instructions ⭐⭐⭐
- **Opcodes**: 0x1020-0x102F (LBRA, LBSR, LBHI, LBCC, etc.)
- **Cycles**: 5-6 cada uno
- **Importancia**: Saltos largos para code > 128 bytes de distancia
- **Estado**: Algunos implementados, verificar completitud

### 9. DAA (Decimal Adjust A) ⭐⭐⭐
- **Opcode**: 0x19 (inherent)
- **Cycles**: 2
- **C++ Reference**: Adjust A for BCD arithmetic after ADD/ADC
- **Estado**: ❌ NO IMPLEMENTADO

### 10. ORCC/ANDCC (Condition Code Operations) ⭐⭐⭐
- **Opcodes**: 0x1A (ORCC), 0x1C (ANDCC)
- **Cycles**: 3 cada uno
- **Importancia**: Manipulación directa de flags del procesador
- **Estado**: ❌ NO IMPLEMENTADO

## PRIORITY LEVEL 3 (Memory Operations) 💾

### 11. CLR (Clear Memory) - Faltantes ⭐⭐
- **Opcodes**: 0x0F (direct), 0x6F (indexed), 0x7F (extended)
- **Estado**: Solo CLR A,B implementados, faltan memory modes

### 12. TST (Test Memory) - Faltantes ⭐⭐  
- **Opcodes**: 0x0D (direct), 0x6D (indexed), 0x7D (extended)
- **Estado**: Solo TST A,B implementados, faltan memory modes

### 13. NEG (Negate Memory) - Faltantes ⭐⭐
- **Opcodes**: 0x00 (direct), 0x60 (indexed), 0x70 (extended)
- **Estado**: Solo NEG A,B implementados, faltan memory modes

### 14. COM (Complement Memory) - Faltantes ⭐⭐
- **Opcodes**: 0x03 (direct), 0x63 (indexed), 0x73 (extended)
- **Estado**: Solo COM A,B implementados, faltan memory modes

### 15. INC/DEC Memory - Faltantes ⭐⭐
- **INC**: 0x0C (direct), 0x6C (indexed), 0x7C (extended)
- **DEC**: 0x0A (direct), 0x6A (indexed), 0x7A (extended)
- **Estado**: Solo registro A,B implementados, faltan memory modes

## PRIORITY LEVEL 4 (Extended Arithmetic) 🧮

### 16. ABX (Add B to X) ⭐⭐
- **Opcode**: 0x3A (inherent)
- **Cycles**: 3
- **C++ Reference**: `X = X + B; // No flags affected`
- **Importancia**: Offset calculations para arrays
- **Estado**: ❌ NO IMPLEMENTADO

### 17. ADCA/ADCB (Add with Carry) ⭐⭐
- **Opcodes**: 0x89,0x99,0xA9,0xB9 (ADCA), 0xC9,0xD9,0xE9,0xF9 (ADCB)
- **Estado**: ❌ NO IMPLEMENTADO (muy importante para multi-byte arithmetic)

### 18. SBCA/SBCB (Subtract with Carry) ⭐⭐
- **Opcodes**: 0x82,0x92,0xA2,0xB2 (SBCA), 0xC2,0xD2,0xE2,0xF2 (SBCB)
- **Estado**: ❌ NO IMPLEMENTADO

### 19. ADDD/SUBD (16-bit Arithmetic) ⭐⭐
- **ADDD**: 0xC3,0xD3,0xE3,0xF3
- **SUBD**: 0x83,0x93,0xA3,0xB3  
- **Estado**: ❌ NO IMPLEMENTADO (crítico for 16-bit operations)

## PRIORITY LEVEL 5 (Stack & System) 📚

### 20. Complete Stack Operations ⭐⭐
- Verificar todos los PSHS/PULS/PSHU/PULU masks están completos
- Validar orden correcto de push/pop (PCR, U/S, Y, X, DP, B, A, CC)

### 21. Complete Transfer/Exchange ⭐⭐
- Verificar que EXG/TFR manejan todos los registros válidos
- Test con combinaciones inválidas (deben comportarse como en C++)

## PRIORITY LEVEL 6 (Advanced Features) 🔬

### 22. Interrupt Handling Complete ⭐
- IRQ/FIRQ vectors y handling
- NMI (Non-Maskable Interrupt)
- Reset sequence completa

### 23. Memory Protection ⭐
- Verificar que no se puede escribir en ROM (BIOS área)
- Validar unmapped memory access handling

### 24. Timing Accuracy ⭐
- Validar todos los cycle counts vs 6809_cycles_nominal.json
- Implement extra cycles para page boundary crossings en indexed

## IMPLEMENTATION METHODOLOGY (Para cada opcode)

### Workflow Establecido:
1. **Create Feature Branch**: `git checkout -b feature/{opcode-name}-implementation`
2. **Study C++ Reference**: Leer el código exacto en `vectrexy_backup/libs/emulator/src/Cpu.cpp`
3. **Add Switch Cases**: Agregar casos 0xXX en cpu6809.rs
4. **Implement Method**: Crear `op_{name}()` con comentario `// C++ Original:`
5. **Create Tests**: Mínimo 4 tests en `tests/test_{opcode}_implementation.rs`
6. **Validate Cycles**: Confirmar timing vs JSON specification
7. **Test Memory**: Usar RAM area 0xC800-0xCFFF para evitar bus panics
8. **Commit & Merge**: Document, commit, merge to main, cleanup branch

### Test Template (usar para cada opcode):
```rust
#[test] fn test_{opcode}_basic_functionality()
#[test] fn test_{opcode}_flag_effects()  
#[test] fn test_{opcode}_boundary_conditions()
#[test] fn test_{opcode}_vs_cpp_reference_compliance()
```

## PROGRESS TRACKING

### ✅ COMPLETED:
- RTS (0x39) - Return from Subroutine ✅
- All basic LD instructions (LDA, LDB, LDX, LDD, LDU) ✅  
- All basic ST instructions (STA, STB, STX, STD, STU) ✅
- All CMP instructions (CMPA, CMPB, CMPX, CMPY, CMPD, CMPS, CMPU) ✅
- All branch instructions (BEQ, BNE, BCC, BCS, etc.) ✅
- Basic arithmetic (ADD, SUB, AND, OR, EOR immediate/direct/extended) ✅
- Single register operations (INC/DEC/CLR/TST/NEG/COM A,B) ✅
- Shift/rotate register operations (LSR, ASL, ASR, ROL, ROR A,B) ✅
- Stack operations (PSHS, PULS, PSHU, PULU) ✅
- Jump to subroutine (JSR all modes, BSR, LBSR) ✅
- Transfer/Exchange (TFR, EXG) ✅
- LEA instructions (LEAX, LEAY, LEAS, LEAU) ✅
- NOP (0x12) ✅

### 🔧 IN PROGRESS:
- Analysis complete, ready for JMP implementation

### ❌ CRITICAL MISSING:
- JMP (Jump) - **START HERE** 🎯
- MUL (Multiply)
- SWI/RTI (Interrupt handling)
- Memory variants of CLR, TST, NEG, COM, INC, DEC
- SEX (Sign extend)
- ABX (Add B to X)
- Multi-byte arithmetic (ADDD, SUBD, ADCA, SBCA)

---

## ESTIMATED COMPLETION TIME:
- **JMP Implementation**: ~2-3 hours (high priority)
- **MUL Implementation**: ~3-4 hours (complex arithmetic)
- **SWI/RTI System**: ~4-6 hours (interrupt handling complex)
- **Memory Operations**: ~6-8 hours (15+ opcodes)
- **Remaining Arithmetic**: ~4-6 hours (8+ opcodes)

**TOTAL REMAINING**: ~20-27 hours for complete 6809 implementation

---
**Next Action**: Implement JMP (0x0E, 0x6E, 0x7E) following established workflow
**Current Status**: Ready to begin feature/jmp-opcode-implementation branch