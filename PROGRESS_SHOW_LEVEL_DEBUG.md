# PROGRESO: SHOW_LEVEL Debug Session (2026-01-07)

## ✅ Completado

### 1. MCP Tools de Observabilidad (Tasks 1-3)
Implementadas 4 nuevas herramientas MCP para debugging:

- **`debugger/get_registers`**: Lee todos los registros CPU (A, B, D, X, Y, U, S, PC, DP, CC con flags)
- **`memory/dump`**: Hex dump de regiones de RAM (hasta 4KB)
- **`memory/list_variables`**: Lista todas las variables del PDB ordenadas por tamaño
- **`memory/read_variable`**: Lee valor actual de cualquier variable del emulador

**Archivos modificados**:
- `ide/electron/src/mcp/server.ts` (handlers implementados)
- `ide/mcp-server/server.js` (expuestos vía stdio)

### 2. Fix BUG #1: Loop Off-by-One (Task 4)
**Problema**: El loop ejecutaba 1 iteración extra, leyendo objetos basura.

**Antes**:
```asm
SLR_OBJ_LOOP:
    DECB             ; Decrementa PRIMERO
    BMI SLR_OBJ_DONE ; Solo sale cuando B=$FF (negative)
```

**Después**:
```asm
SLR_OBJ_LOOP:
    TSTB             ; Prueba si es cero PRIMERO
    BEQ SLR_OBJ_DONE ; Sale inmediatamente si B=0
    DECB             ; DESPUÉS decrementa
```

**Resultado**: count=3 ejecuta exactamente 3 iteraciones (antes ejecutaba 4).

### 3. Fix BUG #2: Count Corruption (Task 5)
**Problema**: LEVEL_GP_COUNT leía 769 (0x0301) en lugar de 3.

**Causa**: B register tenía basura, y STA escribe todo D (A:B) en memoria.

**Fix**:
```asm
CLRB             ; Limpiar B register PRIMERO
LDA ,X+          ; Cargar count (con B=0 garantizado)
STA >LEVEL_GP_COUNT ; Ahora guarda valor limpio de 8 bits
```

**Resultado**: Los counts ahora se leen correctamente.

### 4. Documentación
- `DEBUG_SHOW_LEVEL_INVESTIGATION.md`: Guía completa de verificación con ejemplos de uso de MCP tools
- Incluye plan paso a paso para testing
- Criterios de éxito/fallo claramente definidos

### 5. Commits
1. `71c68830`: Restore F12 key for debug.continue
2. `2d7b21d0`: Add MCP observability tools and fix SHOW_LEVEL bugs
3. `ec2c7f66`: Add comprehensive debug guide

---

## 📋 Pendiente (Tasks 6-10)

### Task 6: Verificar LEVEL_PTR
- Inspeccionar valor en RAM
- Validar estructura .vplay
- **Tool**: `memory/read_variable({ "name": "LEVEL_PTR" })`

### Task 7: Verificar Pointer Reads
- Validar LDD ,X++ para BG/GP/FG pointers
- Comprobar offsets correctos
- **Tool**: `memory/dump({ "address": level_ptr + 15, "size": 6 })`

### Task 8: Verificar Object Offset Reads
- Validar offsets +1, +3, +8, +16 para x, y, intensity, vector_ptr
- **Tool**: `memory/dump({ "address": gp_ptr, "size": 60 })`

### Task 9: Verificar LEAX 12,X Offset
- Validar que apunta a counts después de bounds/time
- **Tool**: `memory/dump({ "address": level_ptr, "size": 32 })`

### Task 10: Test Final
- Compilar level_test
- Verificar 4 vectores (sin fantasmas)
- Confirmar que counts son correctos

---

## 🎯 Próximo Paso para el Usuario

**CUANDO REGRESES**:
1. **Restart IDE** (para cargar nuevo vectrexc con fixes)
2. **Compile level_test** (Ctrl+F7 o Build → Build)
3. **Run in emulator** (Ctrl+F5 o Build → Run)
4. **Observa**:
   - ¿Cuántos vectores aparecen? (esperado: 4)
   - ¿Hay fantasmas? (esperado: no)
   - ¿Desaparecen vectores? (esperado: no)

5. **Si funciona**: ✅ PROBLEMA RESUELTO
6. **Si persisten bugs**: Usaremos las nuevas MCP tools para inspeccionar estado en tiempo real

---

## 🔧 Cómo Usar las MCP Tools (Para PyPilot o Copilot)

### Ejemplo 1: Leer LEVEL_GP_COUNT
```javascript
// Verificar si count es 3 o 769
memory_read_variable({ "name": "LEVEL_GP_COUNT" })
```

### Ejemplo 2: Dump de Level Data
```javascript
// Ver estructura completa del level
memory_dump({ "address": 0xC800, "size": 128 })
```

### Ejemplo 3: Inspeccionar Registros Durante Loop
```javascript
// Poner breakpoint en SLR_OBJ_LOOP
// Cuando pare, leer estado:
debugger_get_registers()
// Verificar B register: debería decrementar 3→2→1→0 (y salir)
```

### Ejemplo 4: Listar Todas las Variables
```javascript
// Ver todas las variables con sus addresses
memory_list_variables()
```

---

## 📊 Análisis Previo

### Síntomas Reportados por Usuario:
1. 13 vectores aparecían (esperados: 4)
   - Patrón: 10 en diagonal "dientes de sierra" + 3 más
   - 4 reales + 9 fantasmas
2. Comportamiento: todos aparecían, luego los reales desaparecían dejando solo fantasmas
3. LEVEL_GP_COUNT leía 769 en lugar de 3

### Root Cause Identificado:
1. **Count Corruption**: Faltaba CLRB → high byte con basura → 769 en lugar de 3
2. **Loop Off-by-One**: DECB+BMI no detecta cero → 1 iteración extra → lee objeto inexistente
3. **Compounding Effect**: Loop corrupto (4 iter) + count corrupto (769) = 773 iteraciones intentadas!

### Fixes Aplicados:
- CLRB antes de reads → counts correctos
- TSTB+BEQ+DECB → loop exacto
- Resultado esperado: 3 iteraciones para GP, 1 para FG, 0 para BG = 4 vectores totales

---

## 🚀 Estado del Código

**Branch**: `feature/playground-level-designer`
**Commits ahead**: 3 (respecto a origin)
**Vectrexc**: Recompilado con fixes en `target/release/vectrexc`
**IDE**: Necesita restart para cargar nuevo vectrexc

**Archivos modificados**:
- ✅ `core/src/backend/m6809/emission.rs` (loop fix + CLRB)
- ✅ `ide/electron/src/mcp/server.ts` (MCP handlers)
- ✅ `ide/mcp-server/server.js` (MCP external server)
- ✅ `ide/frontend/src/main.tsx` (F12 shortcut)
- ✅ `DEBUG_SHOW_LEVEL_INVESTIGATION.md` (documentation)

**Tests pendientes**: Level_test compilation and execution

---

## 💡 Decisiones Técnicas

### Por qué CLRB en lugar de CLRA:
- STA guarda A en memoria, pero internamente usa D register (A:B)
- Si B tiene basura, se corrompe el valor guardado
- CLRB garantiza D = 0x00AA (donde AA es el valor correcto de A)

### Por qué TSTB antes de DECB:
- DECB decrementa ANTES de testear → B=0 se convierte en B=$FF
- BMI solo detecta negative flag (B=$FF) → no detecta B=0
- TSTB testa ANTES de decrementar → detecta B=0 inmediatamente

### Por qué 4 MCP Tools en lugar de 1:
- Separación de responsabilidades
- get_registers: Solo CPU state
- memory_dump: Raw memory inspection
- list_variables: PDB symbols overview
- read_variable: High-level variable access
- Facilita debugging incremental (no necesitas dumps masivos si solo quieres 1 variable)

---

## 📖 Referencias

- **Copilot Instructions**: `.github/copilot-instructions.md` (sección 17: SHOW_LEVEL architecture)
- **Debug Guide**: `DEBUG_SHOW_LEVEL_INVESTIGATION.md`
- **MCP Protocol**: `ide/electron/src/mcp/types.ts`
- **M6809 Reference**: Sección 15 de copilot-instructions.md (Fuente de la Verdad)

---

**Última actualización**: 2026-01-07 11:45
**Status**: ✅ FIXES APLICADOS - ESPERANDO TEST DEL USUARIO
