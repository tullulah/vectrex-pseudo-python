# AUDITORÍA 1:1 VECTREXY vs emulator_v2
## Verificación de Cumplimiento de Reglas de Port

**Fecha**: 2025-09-26  
**Objetivo**: Verificar que cada opcode implementado en emulator_v2 sea port 1:1 desde Vectrexy C++  
**Regla Crítica**: NUNCA inventar implementación propia. TODO debe ser port línea-por-línea desde Vectrexy C++

---

## 📋 TODO LIST POR CATEGORÍAS

### ✅ COMPLETADO - ❌ FALTA VERIFICAR - ⚠️  DISCREPANCIA ENCONTRADA

### 1. MEMORY OPERATIONS (Direct/Indexed/Extended)
- [ ] **NEG** (0x00, 0x40, 0x50, 0x60, 0x70)
  - Verificar: lógica de negación, flags N/Z/V/C
  - Referencia: `Cpu.cpp` líneas por determinar
- [ ] **COM** (0x03, 0x43, 0x53, 0x63, 0x73)  
  - Verificar: complemento a 1, flags N/Z/V=0/C=1
- [ ] **LSR** (0x04, 0x44, 0x54, 0x64, 0x74)
  - Verificar: shift lógico derecha, bit 0 → Carry
- [ ] **ROR** (0x06, 0x46, 0x56, 0x66, 0x76)
  - Verificar: rotación con carry
- [ ] **ASR** (0x07, 0x47, 0x57, 0x67, 0x77)
  - Verificar: shift aritmético (preserva signo)
- [ ] **ASL/LSL** (0x08, 0x48, 0x58, 0x68, 0x78)
  - Verificar: shift izquierda, bit 7 → Carry
- [ ] **ROL** (0x09, 0x49, 0x59, 0x69, 0x79)
  - Verificar: rotación izquierda con carry
- [ ] **DEC** (0x0A, 0x4A, 0x5A, 0x6A, 0x7A)
  - Verificar: decremento, flags N/Z/V (NO afecta Carry)
- [ ] **INC** (0x0C, 0x4C, 0x5C, 0x6C, 0x7C)
  - Verificar: incremento, flags N/Z/V (NO afecta Carry)
- [ ] **TST** (0x0D, 0x4D, 0x5D, 0x6D, 0x7D)
  - Verificar: test valor, flags N/Z, V=0, C=0
- [ ] **CLR** (0x0F, 0x4F, 0x5F, 0x6F, 0x7F)
  - Verificar: clear a 0, flags N=0/Z=1/V=0/C=0

### 2. JUMP AND SUBROUTINE OPERATIONS
- [ ] **JMP** (0x0E, 0x6E, 0x7E)
  - Verificar: PC = dirección efectiva
  - Cycles: Direct=3, Indexed=3, Extended=4
- [ ] **JSR** (0x9D, 0xAD, 0xBD)
  - Verificar: push PC, PC = dirección efectiva
  - Cycles: Direct=7, Indexed=7, Extended=8
- [ ] **BSR** (0x8D)
  - Verificar: como JSR pero relative
  - Cycles: 7
- [ ] **RTS** (0x39)
  - Verificar: pull PC del stack
  - Cycles: 5

### 3. BRANCH OPERATIONS (Short)
- [ ] **BRA/BRN** (0x20, 0x21)
  - Verificar: branch always/never
- [ ] **Conditional Branches** (0x22-0x2F)
  - BHI/BLS, BCC/BCS, BNE/BEQ, BVC/BVS
  - BPL/BMI, BGE/BLT, BGT/BLE
  - Verificar: condiciones exactas de flags
  - Todos cycles: 3

### 4. LONG BRANCH OPERATIONS (Page 1)
- [ ] **LBRA** (0x16)
  - Verificar: 16-bit relative
  - Cycles: 5
- [ ] **LBSR** (0x17)
  - Verificar: long branch subroutine
  - Cycles: 9
- [ ] **Long Conditional** (0x1021-0x102F)
  - Verificar: mismas condiciones que short
  - Todos cycles: 5

### 5. LOAD EFFECTIVE ADDRESS
- [ ] **LEA** (0x30-0x33)
  - LEAX, LEAY, LEAS, LEAU
  - Verificar: cálculo dirección efectiva
  - Flags: solo Z afectado (LEAX/LEAY)
  - Cycles: 4 todos

### 6. STACK OPERATIONS
- [ ] **PSHS/PULS** (0x34, 0x35)
  - Verificar: orden push/pull correcto
  - Referencia crítica: orden según postbyte
- [ ] **PSHU/PULU** (0x36, 0x37)
  - Verificar: stack U, orden correcto
- [ ] **RTI** (0x3B)
  - Verificar: pull CC, luego A/B o todo según stack

### 7. ARITHMETIC OPERATIONS - REGISTER A
- [ ] **SUBA** (0x80, 0x90, 0xA0, 0xB0)
- [ ] **ADDA** (0x8B, 0x9B, 0xAB, 0xBB)  
- [ ] **ADCA** (0x89, 0x99, 0xA9, 0xB9)
- [ ] **SBCA** (0x82, 0x92, 0xA2, 0xB2)
  - Verificar: flags H/N/Z/V/C exactos
  - Half-carry solo en 8-bit adds

### 8. ARITHMETIC OPERATIONS - REGISTER B  
- [ ] **SUBB** (0xC0, 0xD0, 0xE0, 0xF0)
- [ ] **ADDB** (0xCB, 0xDB, 0xEB, 0xFB)
- [ ] **ADCB** (0xC9, 0xD9, 0xE9, 0xF9)
- [ ] **SBCB** (0xC2, 0xD2, 0xE2, 0xF2)
  - Verificar: misma lógica que A

### 9. ARITHMETIC OPERATIONS - 16-BIT
- [ ] **SUBD** (0x83, 0x93, 0xA3, 0xB3)
- [ ] **ADDD** (0xC3, 0xD3, 0xE3, 0xF3)
  - Verificar: aritmética 16-bit, flags N/Z/V/C
  - NO half-carry en 16-bit

### 10. LOGICAL OPERATIONS
- [ ] **ANDA/ANDB** (0x84/0xC4, 0x94/0xD4, 0xA4/0xE4, 0xB4/0xF4)
- [ ] **ORA/ORB** (0x8A/0xCA, 0x9A/0xDA, 0xAA/0xEA, 0xBA/0xFA)  
- [ ] **EORA/EORB** (0x88/0xC8, 0x98/0xD8, 0xA8/0xE8, 0xB8/0xF8)
  - Verificar: flags N/Z, V=0, C no afectado

### 11. BIT TEST OPERATIONS
- [ ] **BITA/BITB** (0x85/0xC5, 0x95/0xD5, 0xA5/0xE5, 0xB5/0xF5)
  - Verificar: AND lógico SIN guardar resultado
  - Flags: N/Z, V=0, C no afectado

### 12. COMPARISON OPERATIONS
- [ ] **CMPA/CMPB** (0x81/0xC1, 0x91/0xD1, 0xA1/0xE1, 0xB1/0xF1)
- [ ] **CMPX** (0x8C, 0x9C, 0xAC, 0xBC)
- [ ] **CMPD** (Page 1: 0x1083, 0x1093, 0x10A3, 0x10B3)
- [ ] **CMPY** (Page 1: 0x108C, 0x109C, 0x10AC, 0x10BC)
- [ ] **CMPU** (Page 2: 0x1183, 0x1193, 0x11A3, 0x11B3)
- [ ] **CMPS** (Page 2: 0x118C, 0x119C, 0x11AC, 0x11BC)
  - Verificar: substracción SIN guardar resultado
  - Flags exactos para cada tamaño

### 13. LOAD/STORE OPERATIONS
- [ ] **LDA/STA** (0x86/0x97, 0x96/0x97, 0xA6/0xA7, 0xB6/0xB7)
- [ ] **LDB/STB** (0xC6/0xD7, 0xD6/0xD7, 0xE6/0xE7, 0xF6/0xF7)
- [ ] **LDD/STD** (0xCC/0xDD, 0xDC/0xDD, 0xEC/0xED, 0xFC/0xFD)
- [ ] **LDX/STX** (0x8E/0x9F, 0x9E/0x9F, 0xAE/0xAF, 0xBE/0xBF)
- [ ] **LDU/STU** (0xCE/0xDF, 0xDE/0xDF, 0xEE/0xEF, 0xFE/0xFF)
- [ ] **LDY/STY** (Page 1: 0x108E/0x109F, etc.)
- [ ] **LDS/STS** (Page 1: 0x10CE/0x10DF, etc.)
  - ⚠️  **CRÍTICO**: Verificar 0xEF = STS, 0xFF = STU

### 14. SYSTEM OPERATIONS
- [ ] **NOP** (0x12)
- [ ] **SYNC** (0x13)  
- [ ] **DAA** (0x19)
- [ ] **SEX** (0x1D)
- [ ] **ABX** (0x3A)
- [ ] **MUL** (0x3D)
  - Verificar: D = A * B (unsigned)
- [ ] **CWAI** (0x3C)
- [ ] **SWI/SWI2/SWI3** (0x3F, 0x103F, 0x113F)

### 15. REGISTER TRANSFER
- [ ] **TFR** (0x1F)
- [ ] **EXG** (0x1E)
  - Verificar: tabla de registros
  - Size handling correcto

### 16. CONDITION CODE OPERATIONS  
- [ ] **ORCC** (0x1A)
- [ ] **ANDCC** (0x1C)
  - Verificar: manipulación CC register

---

## 🎯 METODOLOGÍA DE AUDITORÍA

### Paso 1: Localizar Implementación Vectrexy
Para cada opcode:
1. Buscar en `vectrexy_backup/libs/emulator/src/Cpu.cpp`
2. Encontrar función `OpXXX` correspondiente
3. Extraer lógica exacta línea por línea

### Paso 2: Localizar Implementación emulator_v2  
1. Buscar en `emulator_v2/src/core/cpu6809.rs`
2. Encontrar match case `0xXX =>`
3. Comparar implementación

### Paso 3: Verificación Específica
Para cada opcode verificar:
- ✅ **Lógica**: Operación matemática idéntica
- ✅ **Flags**: Cálculo N/Z/V/C/H exacto  
- ✅ **Cycles**: Número idéntico
- ✅ **Addressing**: EA calculation correcta
- ✅ **Side effects**: Todos los efectos

### Paso 4: Documentar Discrepancias
- **❌ ERROR**: Implementación incorrecta
- **⚠️  WARNING**: Posible diferencia menor
- **📝 NOTE**: Comentario o aclaración

---

## 🚨 CASOS CRÍTICOS IDENTIFICADOS

### A. STU vs STS Confusion (YA CORREGIDO)
- ✅ 0xEF: Corregido de STU → STS
- ✅ 0xFF: Mantenido como STU (correcto)

### B. Verificar Próximamente
- **Stack Order**: PSHS/PULS/PSHU/PULU
- **Flag Calculation**: Especialmente overflow
- **Indexed Addressing**: EA calculation
- **16-bit Operations**: SUBD/ADDD/CMPD/etc.

---

## 📊 PROGRESO AUDITORÍA

**Total Opcodes**: 271  
**Auditados**: 0  
**Confirmados 1:1**: 0  
**Discrepancias**: 0  

---

*Próximo paso: Iniciar auditoría sistemática por categorías*