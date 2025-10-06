# Corrección Tabla de Opcodes 1:1 con Vectrexy
**Fecha**: 2025-10-06  
**Problema Original**: ABX faltaba en tabla de opcodes  
**Investigación**: Auditoría completa de tabla Page0 vs Vectrexy

---

## 🔍 Problema Detectado

Durante la implementación de ABX (0x3A) se descubrió que:
- ✅ La **implementación** de ABX existía en `cpu6809.rs`
- ❌ La **entrada en tabla** faltaba en `cpu_op_codes.rs`
- ⚠️  Potencial problema sistemático: **invenciones** en lugar de port 1:1

### Patrón de Error
```
Vectrexy (referencia) → Tabla completa 256 entradas (array-based)
Rust (nuestra impl)   → Tabla match-based (requiere entry manual)
                      → RIESGO: Gaps o datos incorrectos
```

---

## ✅ Solución Aplicada

### 1. Script de Comparación Automática
Creado `compare_opcode_tables.py`:
- Extrae opcodes de Vectrexy CpuOpCodes.h
- Parsea Rust cpu_op_codes.rs (solo Page0)
- Compara cycles, size, addressing mode
- Reporta faltantes y diferencias

### 2. Correcciones Realizadas (7 total)

#### **A) Opcode Faltante (1)**

| Opcode | Nombre | Vectrexy | Rust Antes | Acción |
|--------|--------|----------|------------|--------|
| 0x3E   | RESET* | ✅ Present | ❌ Missing | **AGREGADO** |

**Nota**: RESET* es hardware reset (no ejecutable normalmente), cycles=0

---

#### **B) Correcciones de Metadata (6)**

| Opcode | Nombre | Campo      | Vectrexy     | Rust Antes    | ✅ Corregido |
|--------|--------|------------|--------------|---------------|--------------|
| 0x10   | PAGE1  | cycles     | 1            | 0             | ✅ 1         |
| 0x10   | PAGE1  | size       | 1            | 0             | ✅ 1         |
| 0x11   | PAGE2  | cycles     | 1            | 0             | ✅ 1         |
| 0x11   | PAGE2  | size       | 1            | 0             | ✅ 1         |
| 0x13   | SYNC   | cycles     | 2            | 4             | ✅ 2         |
| 0x1E   | EXG    | addr_mode  | **Inherent** | ~~Immediate~~ | ✅ Inherent  |
| 0x1F   | TFR    | addr_mode  | **Inherent** | ~~Immediate~~ | ✅ Inherent  |
| 0x3B   | RTI    | cycles     | 0 (variable) | 15 (fixed)    | ✅ 0         |

---

## 📊 Resultado Final

```bash
python compare_opcode_tables.py
```

**Output**:
```
✅ Opcodes FALTANTES en Rust (están en Vectrexy):
   ✅ ¡Todos los opcodes de Vectrexy (0x00-0x5F) están en Rust!

⚠️  Opcodes con DIFERENCIAS (están en ambos pero con datos distintos):
   ✅ ¡Todos los opcodes coinciden perfectamente!

================================================================================
✅ CONCLUSIÓN: La tabla Rust está completa para el rango verificado
```

---

## 🎯 Impacto

### Correcciones Críticas

**1. EXG/TFR (0x1E, 0x1F) - Addressing Mode**
```rust
// ANTES (INCORRECTO)
addr_mode: AddressingMode::Immediate  // ❌ Inventado

// AHORA (1:1 Vectrexy)
addr_mode: AddressingMode::Inherent   // ✅ Correcto
```
- **Por qué importa**: EXG/TFR son inherent (el post-byte NO es un operando immediate)
- **Documentación Vectrexy**: `{ 0x1E, "EXG", AddressingMode::Inherent, 8, 2, ... }`

**2. SYNC (0x13) - Cycles**
```rust
// ANTES (INVENTADO según "MC6809 Programming Manual")
cycles: 4  // ❌ Basado en interpretación de manual

// AHORA (1:1 Vectrexy)
cycles: 2  // ✅ Valor real de implementación de referencia
```
- **Por qué importa**: Timing crítico para emulación precisa

**3. RTI (0x3B) - Variable Timing**
```rust
// ANTES (FIJO)
cycles: 15  // ❌ Asumiendo siempre E=1

// AHORA (VARIABLE)
cycles: 0   // ✅ Indica timing variable (6 o 15 según E flag)
```
- **Por qué importa**: RTI puede ser 6 cycles (FIRQ) o 15 cycles (IRQ)
- **Documentación**: "6 cycles if E=0, 15 if E=1"

---

## 📝 Lecciones Aprendidas

### Regla 0.2 - VERIFICACIÓN 1:1 OBLIGATORIA

**NUNCA MÁS**:
- ❌ Inventar valores basados en "manual genérico"
- ❌ Asumir addressing modes sin verificar
- ❌ Copiar cycles de "otra fuente" que no sea Vectrexy
- ❌ Marcar opcodes como Immediate cuando son Inherent

**SIEMPRE**:
- ✅ Leer archivo Vectrexy C++ correspondiente LÍNEA POR LÍNEA
- ✅ Copiar valores EXACTOS (cycles, size, addr_mode)
- ✅ Documentar origen: `// C++ Original: { 0xXX, "NAME", Mode, cycles, size, "desc" }`
- ✅ Validar con script de comparación automática

---

## 🔧 Archivos Modificados

### `emulator_v2/src/core/cpu_op_codes.rs`

**Correcciones**:
1. 0x10 PAGE1: cycles 0→1, size 0→1
2. 0x11 PAGE2: cycles 0→1, size 0→1
3. 0x13 SYNC: cycles 4→2
4. 0x1E EXG: Immediate→Inherent
5. 0x1F TFR: Immediate→Inherent
6. 0x3B RTI: cycles 15→0 (variable)
7. 0x3E RESET*: AGREGADO (nuevo opcode)

**Total líneas modificadas**: ~40 líneas (7 bloques CpuOp)

---

## ✅ Validación

### Tests Ejecutados
```bash
cargo test --test test_opcodes test_abx --release
```
**Resultado**: ✅ 5/5 tests pasando

### Comparación Automática
```bash
python compare_opcode_tables.py
```
**Resultado**: ✅ 0 faltantes, 0 diferencias

### Compilación
```bash
cargo build --release
```
**Resultado**: ✅ Compilación exitosa (1 warning no relacionado)

---

## 🚀 Próximos Pasos Sugeridos

### Short Term
1. **Extender comparación a Page1 y Page2** (prefijos 0x10, 0x11)
2. **Validar opcodes 0x60-0xFF** (segunda mitad de Page0)
3. **Agregar CI check** que ejecute compare_opcode_tables.py en cada commit

### Medium Term
1. **Port completo de tabla desde Vectrexy** usando script automatizado
2. **Generar tabla const OPCODE_TABLE_PAGE0: [CpuOp; 256]** (array-based, imposible tener gaps)
3. **Documentar TODAS las diferencias intencionales** (si las hay)

### Long Term
1. **Sincronización automática** con Vectrexy en CI/CD
2. **Test de regresión** para cada opcode modificado
3. **Cobertura 100%** de todos los 256 + 38 + 9 = 303 opcodes

---

## 📚 Referencias

**Vectrexy Source of Truth**:
```
C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\vectrexy\libs\emulator\include\emulator\CpuOpCodes.h
```

**Documentación Original**:
- MC6809 Programming Manual (secundario - NO autoritativo)
- Vectrexy CpuOpCodes.h (primario - AUTORITATIVO)

**Script de Validación**:
```
emulator_v2/compare_opcode_tables.py
```

---

## 🎯 Conclusión

**PROBLEMA ORIGINAL**: ABX marcado como ILLEGAL pese a estar implementado  
**CAUSA RAÍZ**: Entrada faltante en tabla de opcodes  
**SOLUCIÓN**: Port 1:1 desde Vectrexy + validación automática  
**ESTADO**: ✅ **100% RESUELTO** para Page0 rango 0x00-0x5F  

**IMPACTO**: 
- ✅ ABX funcional
- ✅ 6 opcodes con metadata corregida
- ✅ 1 opcode faltante agregado (RESET*)
- ✅ Script de validación automática creado
- ✅ Cero diferencias con Vectrexy en rango verificado

**POLÍTICA FUTURA**: 
**NUNCA inventar valores. SIEMPRE verificar 1:1 con Vectrexy.**

---

**Firma**: Corrección realizada siguiendo Regla 0.2 (VERIFICACIÓN 1:1 OBLIGATORIA)  
**Validado**: Script automático + tests unitarios + compilación exitosa
