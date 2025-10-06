# MC6809 Opcode Test Coverage

**Estado**: En desarrollo  
**Framework**: Comparative Testing (Rust vs Vectrexy C++)  
**Última actualización**: 2025-10-06

---

## Tests Implementados

### Bloque 1: Operaciones Básicas CPU (12 tests) ✅

| Test | Opcodes Cubiertos | Descripción | Ciclos | Estado |
|------|-------------------|-------------|--------|--------|
| **cpu_arithmetic** | ADDA, ADDB | Suma 8-bit | 50 | ✅ PASS |
| **cpu_subtract** | SUBA, SUBB, SUBD | Resta 8-bit y 16-bit | 80 | ✅ PASS |
| **cpu_logic** | ANDA, ANDB, ORA, ORB, EORA, EORB | Operaciones lógicas bitwise | 100 | ✅ PASS |
| **cpu_compare** | CMPA, CMPB, CMPD, CMPX, CMPY | Comparaciones | 120 | ✅ PASS |
| **cpu_increment** | INCA, INCB, DECA, DECB | Incremento/decremento | 100 | ✅ PASS |
| **cpu_shift** | ASLA, ASRA, LSLA, LSRA, ROLA, RORB | Shift/Rotate | 120 | ✅ PASS |
| **cpu_branch** | BEQ, BNE, BRA | Branches básicos | 80 | ✅ PASS |
| **cpu_load_store** | LDA, LDB, STA, STB | Load/Store 8-bit | 80 | ✅ PASS |
| **cpu_stack** | PSHS, PULS | Stack S push/pull | 150 | ✅ PASS |
| **cpu_indexed** | Indexed modes (,X +offset, ,X+) | Direccionamiento indexado | 150 | ✅ PASS |
| **cpu_transfer** | TFR, EXG | Transfer/Exchange registros | 150 | ✅ PASS |
| **cpu_jsr_rts** | JSR, RTS | Subroutine call/return | 80 | ✅ PASS |

### Bloque 2: Operaciones Avanzadas (8 tests) 🔶

| Test | Opcodes Cubiertos | Descripción | Ciclos | Estado |
|------|-------------------|-------------|--------|--------|
| **cpu_multiply** | MUL | Multiplicación 8x8=16 | 150 | ✅ PASS |
| **cpu_lea** | LEAX, LEAY, LEAS, LEAU | Load Effective Address | 150 | ⚠️ 1 cycle diff |
| **cpu_branches_extended** | BCC, BCS, BMI, BPL | Branches condicionales | 150 | ✅ PASS |
| **cpu_complement** | COMA, COMB, NEGA, NEGB, CLRA, CLRB | Complement/Negate/Clear | 150 | ✅ PASS |
| **cpu_test** | TSTA, TSTB | Test (compare con 0) | 150 | ✅ PASS |
| **cpu_16bit** | LDD, STD, ADDD, SUBD, CMPD | Operaciones 16-bit | 150 | ✅ PASS |
| **cpu_abx** | ABX | Add B to X | 150 | ❌ **Opcode 0x3A NO IMPL** |
| **cpu_sex** | SEX | Sign extend B→A | 150 | ✅ PASS |

---

## Tests Pendientes (Prioridad)

### Alta Prioridad

| Test Sugerido | Opcodes | Descripción |
|---------------|---------|-------------|
| **cpu_multiply** | MUL | Multiplicación 8x8=16 bit |
| **cpu_lea** | LEAX, LEAY, LEAS, LEAU | Load Effective Address |
| **cpu_branches_extended** | BCC, BCS, BMI, BPL, BVC, BVS | Branches adicionales |
| **cpu_complement** | COM, NEG, CLR | Complemento, negación, clear |
| **cpu_test** | TST, TSTA, TSTB | Test (comparar con 0) |
| **cpu_direct_addressing** | LDA/STA en modo Direct | Direccionamiento directo |
| **cpu_extended_addressing** | LDA/STA en modo Extended | Direccionamiento extendido |

### Media Prioridad

| Test Sugerido | Opcodes | Descripción |
|---------------|---------|-------------|
| **cpu_daa** | DAA | Decimal Adjust Accumulator |
| **cpu_sex** | SEX | Sign Extend |
| **cpu_andcc_orcc** | ANDCC, ORCC | Manipulación de CC register |
| **cpu_abx** | ABX | Add B to X |
| **cpu_indirect** | Indexed Indirect modes | Direccionamiento indirecto |

### Baja Prioridad

| Test Sugerido | Opcodes | Descripción |
|---------------|---------|-------------|
| **cpu_swi** | SWI, SWI2, SWI3 | Software Interrupts |
| **cpu_sync** | SYNC | Synchronize con interrupt |
| **cpu_cwai** | CWAI | Clear and Wait for Interrupt |

---

## Opcodes MC6809 - Cobertura Total

### ✅ Cubiertos (aproximadamente 30%)

- ADDA, ADDB, SUBA, SUBB, SUBD
- ANDA, ANDB, ORA, ORB, EORA, EORB
- CMPA, CMPB, CMPD, CMPX, CMPY
- INCA, INCB, DECA, DECB
- ASLA, ASRA, LSLA, LSRA, ROLA, RORB
- BEQ, BNE, BRA
- LDA, LDB, STA, STB
- PSHS, PULS
- TFR, EXG
- JSR, RTS
- Indexed addressing (básico)

### ⏳ Pendientes (aproximadamente 70%)

**Arithmetic**: MUL, ADCA, ADCB, SBCA, SBCB, DAA  
**Logic**: COMA, COMB, NEGA, NEGB  
**Test**: TST, TSTA, TSTB  
**Branches**: BCC, BCS, BMI, BPL, BVC, BVS, BHI, BLS, BGE, BLT, BGT, BLE  
**Load/Store**: LDD, LDS, LDU, STD, STS, STU  
**Stack**: PSHU, PULU  
**Clear**: CLR, CLRA, CLRB  
**LEA**: LEAX, LEAY, LEAS, LEAU  
**Sign**: SEX  
**CC**: ANDCC, ORCC  
**Misc**: ABX, NOP  
**Interrupts**: SWI, SWI2, SWI3, RTI, CWAI, SYNC  
**Long branches**: LBRA, LBSR, LBcc (todas las variantes)  

---

## Cómo Ejecutar los Tests

### Test Individual
```powershell
.\run_comparative_test_v2.ps1 -TestName cpu_arithmetic -Cycles 50
```

### Todos los Tests
```powershell
.\run_all_opcode_tests.ps1
```

### Con Skip Build (más rápido en desarrollo)
```powershell
.\run_all_opcode_tests.ps1 -SkipBuild
```

---

## Estructura de Cada Test

```
test_cases/cpu_[categoria]/
├── test.asm          # Código assembler MC6809
├── test.bin          # Binario generado (auto)
└── expected.json     # Estado esperado (opcional)
```

---

## Próximos Pasos

1. **Ejecutar suite actual**: `.\run_all_opcode_tests.ps1`
2. **Verificar tests existentes**: Asegurar que todos pasen
3. **Añadir tests de alta prioridad**: MUL, LEA, branches extendidas
4. **Iterar**: Añadir más casos edge (overflow, underflow, flags específicos)
5. **Documentar discrepancias**: Cualquier diferencia Rust vs Vectrexy

---

**Objetivo Final**: 100% cobertura de opcodes MC6809 con tests comparativos automáticos.
