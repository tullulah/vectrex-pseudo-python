# Dynamic Memory Optimization - Work In Progress

**Rama**: feature/dynamic-memory-optimization  
**Commit actual**: 0a88e202  
**Estado**: 40% completado

## ✅ Completado

1. **RAM Layout** (mod.rs):
   - ✅ Eliminados: LEVEL_BG_BUFFER, LEVEL_GP_BUFFER, LEVEL_FG_BUFFER (640 bytes)
   - ✅ Añadido: LEVEL_DYNAMIC_BUFFER (72 bytes)
   - ✅ Añadido: LEVEL_DYNAMIC_COUNT (1 byte)
   - ✅ Ahorro: 568 bytes (88.75%)

2. **LOAD_LEVEL_RUNTIME** (emission.rs lines 1480-1600):
   - ✅ Escanea todos los objetos en ROM
   - ✅ Detecta flag physicsEnabled (bit 0 en offset +14)
   - ✅ Solo copia estado dinámico (6 bytes) a RAM
   - ✅ Inicializa velocidades desde ROM
   - ✅ Maneja overflow (max 12 objetos dinámicos)

3. **Subroutine LLR_SCAN_LAYER**:
   - ✅ Recorre objetos de una capa
   - ✅ Lee flags en offset +14
   - ✅ Extrae rom_index, velocity_x, velocity_y
   - ✅ Escribe en LEVEL_DYNAMIC_BUFFER

## 🔄 En Progreso

### SHOW_LEVEL_RUNTIME (emission.rs lines 1630-1800)
**Estado**: Código antiguo NO compatible

**Problema**: 
- Actual: Renderiza desde RAM buffers (LEVEL_BG_BUFFER, etc.)
- Necesario: Fase 1 (ROM estáticos) + Fase 2 (dinámicos)

**Arquitectura Nueva**:
```asm
SHOW_LEVEL_RUNTIME:
    PSHS D,X,Y,U
    JSR $F1AA  ; DP_to_D0
    
    ; === Phase 1: Draw Static Objects (ROM-only) ===
    JSR SLR_DRAW_STATIC  ; New subroutine
    
    ; === Phase 2: Draw Dynamic Objects (ROM + RAM state) ===
    JSR SLR_DRAW_DYNAMIC ; New subroutine
    
    JSR $F1AF  ; DP_to_C8
    PULS D,X,Y,U,PC

; Subroutine: Draw all static objects directly from ROM
SLR_DRAW_STATIC:
    ; For each layer (BG, GP, FG):
    ;   For each object in ROM:
    ;     Read flags at offset +14
    ;     If (flags & 0x01) == 0: (static)
    ;       Read position from ROM (offset +2-5)
    ;       Read vector_ptr from ROM (offset +22-23)
    ;       Call SLR_DRAW_ONE_OBJECT
    RTS

; Subroutine: Draw dynamic objects (ROM base + RAM velocity)
SLR_DRAW_DYNAMIC:
    LDB LEVEL_DYNAMIC_COUNT
    BEQ SLR_DYN_DONE
    LDU #LEVEL_DYNAMIC_BUFFER
SLR_DYN_LOOP:
    ; Read rom_index from U+0
    LDA ,U
    ; Calculate ROM address: rom_index * 24 bytes
    ; Find layer and offset
    ; Read ROM object data (position, vector_ptr)
    ; Read velocity from U+1-4 (not used for rendering)
    ; Call SLR_DRAW_ONE_OBJECT
    LEAU 6,U
    DECB
    BNE SLR_DYN_LOOP
SLR_DYN_DONE:
    RTS
```

**Helpers Nuevos**:
- `SLR_CALC_ROM_PTR` - rom_index → ROM pointer
- `SLR_DRAW_ONE_OBJECT` - Draw single object from X (ROM ptr)

### UPDATE_LEVEL_RUNTIME (emission.rs lines 1800-2000)
**Estado**: Código antiguo NO compatible

**Problema**:
- Actual: Opera sobre RAM buffers completos
- Necesario: Opera solo sobre dinámicos

**Arquitectura Nueva**:
```asm
UPDATE_LEVEL_RUNTIME:
    PSHS D,X,Y,U
    
    ; === Update only dynamic objects ===
    LDB LEVEL_DYNAMIC_COUNT
    BEQ ULR_DONE
    LDU #LEVEL_DYNAMIC_BUFFER
    LDX LEVEL_PTR  ; World bounds
    
ULR_LOOP:
    PSHS B
    
    ; Read rom_index from U+0
    LDA ,U+
    
    ; Calculate ROM offset: A * 24
    JSR ULR_GET_ROM_PTR  ; X = ROM object pointer
    
    ; Read position from ROM (offset +2-5)
    LDD 2,X  ; X position
    STD TMPVAR_X
    LDD 4,X  ; Y position
    STD TMPVAR_Y
    
    ; Read velocity from dynamic buffer (U points to vel_x after rom_index)
    LDD ,U++  ; velocity_x
    STD TMPVAR_VX
    LDD ,U++  ; velocity_y
    STD TMPVAR_VY
    
    ; Apply physics: pos += vel
    LDD TMPVAR_X
    ADDD TMPVAR_VX
    STD 2,X  ; Write back to ROM position (WAIT - ROM is read-only!)
    
    ; ... collision checks ...
    
    ; Write back velocity to dynamic buffer
    LEAU -4,U  ; Back to velocity fields
    LDD TMPVAR_VX
    STD ,U++
    LDD TMPVAR_VY
    STD ,U++
    
    LEAU 1,U  ; Skip active_flags
    
    PULS B
    DECB
    BNE ULR_LOOP
    
ULR_DONE:
    PULS D,X,Y,U,PC
```

**PROBLEMA CRÍTICO**: 
- ROM es read-only!
- No podemos modificar posición en ROM
- **Solución**: Ampliar dynamic buffer a 10 bytes:
  - +0: rom_index (1 byte)
  - +1-2: position_x (2 bytes) ← NEW
  - +3-4: position_y (2 bytes) ← NEW
  - +5-6: velocity_x (2 bytes)
  - +7-8: velocity_y (2 bytes)
  - +9: active_flags (1 byte)
  - **Total**: 12 objetos × 10 bytes = 120 bytes (vs 72 original)

## 📊 Métricas Actualizadas

**Ahorro REAL después de ajuste**:
```
Antes:  640 bytes (3 buffers separados)
Después: 120 bytes (12 dinámicos × 10 bytes)
Ahorro:  81.25% 🎉

level_test: 69.2% → ~12-15% RAM (estimado)
```

## 🔧 Próximos Pasos

### 1. Ajustar RAM Layout (mod.rs)
```rust
// Change:
ram.allocate("LEVEL_DYNAMIC_BUFFER", 72, "...");

// To:
ram.allocate("LEVEL_DYNAMIC_BUFFER", 120, "Dynamic objects state (12 * 10 bytes)");

// Comment:
// Dynamic object state (10 bytes per object):
//   +0: rom_index (1 byte)
//   +1-2: position_x (2 bytes) - mutable copy from ROM
//   +3-4: position_y (2 bytes) - mutable copy from ROM
//   +5-6: velocity_x (2 bytes)
//   +7-8: velocity_y (2 bytes)
//   +9: active_flags (1 byte)
```

### 2. Actualizar LLR_SCAN_LAYER (emission.rs)
- Copy position_x, position_y from ROM (offsets +2-5)
- Adjust U pointer increments from 6 to 10

### 3. Implementar SHOW_LEVEL_RUNTIME
- New: `SLR_DRAW_STATIC` - iterate ROM, skip if (flags & 0x01)
- New: `SLR_DRAW_DYNAMIC` - iterate LEVEL_DYNAMIC_BUFFER
- New: `SLR_CALC_ROM_PTR` - rom_index → ROM address
- Reuse: `SLR_DRAW_ONE_OBJECT` - existing rendering logic

### 4. Implementar UPDATE_LEVEL_RUNTIME
- Iterate LEVEL_DYNAMIC_BUFFER only
- Read/write position from/to dynamic buffer (NOT ROM)
- Apply physics to mutable copy
- Collision detection uses dynamic positions

### 5. Testing
- Compile level_test: verify RAM usage <15%
- Run in emulator: verify static + dynamic render correctly
- Physics test: verify dynamic objects move, static don't

## 🐛 Debugging Checklist

- [ ] LEVEL_BG_BUFFER undefined → Update SHOW_LEVEL/UPDATE_LEVEL
- [ ] Dynamic objects not visible → Check SLR_DRAW_DYNAMIC
- [ ] Static objects missing → Check SLR_DRAW_STATIC flag logic
- [ ] Physics broken → Verify position copy in/out of dynamic buffer
- [ ] Overflow crash → Check LEVEL_DYNAMIC_COUNT < 12

## 📝 Notas de Diseño

**Por qué no podemos actualizar ROM**:
- ROM es read-only en Vectrex (cartridge)
- Escribir a ROM se ignora o crash
- Solución: Mantener copia mutable en RAM

**Trade-off**: 120 bytes vs 640 bytes
- Ahorro: 81% (vs 88% si posición en ROM)
- Necesario para objetos dinámicos funcionales

**Backward Compatibility**:
- Si physicsEnabled no existe → assume false (estático)
- Objetos sin flag → renderizados desde ROM (0 RAM)
- Objetos con flag → copiados a RAM (10 bytes)

---
**Última actualización**: 2026-01-09 23:45
**Próxima sesión**: Implementar SHOW_LEVEL y UPDATE_LEVEL completos
