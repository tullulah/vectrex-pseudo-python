# Dynamic Memory Optimization - Design Document

**Fecha**: 2026-01-09  
**Rama**: feature/dynamic-memory-optimization  
**Problema**: level_test usa 69.2% RAM (709B/1024B) mostrando solo 3-4 objetos

## 📊 Análisis del Problema

### Uso Actual de RAM
```
LEVEL_BG_BUFFER:  160 bytes (8 objetos × 20 bytes)  - Siempre reservado
LEVEL_GP_BUFFER:  320 bytes (16 objetos × 20 bytes) - Siempre reservado  
LEVEL_FG_BUFFER:  160 bytes (8 objetos × 20 bytes)  - Siempre reservado
────────────────────────────────────────────────────
Total:            640 bytes (62.5% de 1024B RAM)
```

**Desperdicio**:
- Se copian TODOS los objetos de ROM a RAM (incluso estáticos)
- Se reservan buffers completos aunque no se usen
- Objetos estáticos (paredes, plataformas) NO necesitan RAM

### Estructura Objeto Actual (24 bytes)
```
+0:  type (1 byte)
+1:  sprite_id (1 byte)  
+2:  x (2 bytes)
+4:  y (2 bytes)
+6:  width (2 bytes)
+8:  height (2 bytes)
+10: velocity_x (2 bytes)
+12: velocity_y (2 bytes)
+14: flags (2 bytes)
+16: intensity (1 byte)
+17: scale (1 byte)
+18: rotation (1 byte)
+19: collision_size (1 byte)
+20: spawn_delay (2 bytes)
+22: vector_ptr (2 bytes - pointer to ROM)
```

## 🎯 Solución: ROM-First + Dynamic State

### Concepto
1. **Objetos estáticos** → Permanecen en ROM (0 bytes RAM)
2. **Objetos dinámicos** → ROM (base) + RAM (estado) = 6 bytes RAM
3. **Buffer único** → 12 objetos dinámicos × 6 bytes = **72 bytes RAM**

### Reducción de Memoria
```
Antes:  640 bytes (3 buffers separados)
Después: 72 bytes (1 buffer dinámico)
Ahorro:  88.75% 🎉

level_test: 69.2% → ~7-10% RAM
```

## 📐 Nueva Arquitectura

### RAM Layout
```
; Dynamic objects buffer (ONLY mutable state)
LEVEL_DYNAMIC_COUNT  EQU 1 byte   ; Number of active dynamic objects (max 12)
LEVEL_DYNAMIC_BUFFER EQU 72 bytes ; 12 objects × 6 bytes

; Dynamic object state (6 bytes per object):
+0: rom_index (1 byte)      ; Index to ROM object array
+1: velocity_x_hi (1 byte)  ; High byte of velocity_x  
+2: velocity_x_lo (1 byte)  ; Low byte of velocity_x
+3: velocity_y_hi (1 byte)  ; High byte of velocity_y
+4: velocity_y_lo (1 byte)  ; Low byte of velocity_y
+5: active_flags (1 byte)   ; Runtime flags (active, visible, colliding, etc.)

Total RAM: 73 bytes (vs 640 bytes)
```

### ROM Layout (sin cambios)
```
; Level header (12 bytes)
+0:  xMin (2 bytes)
+2:  xMax (2 bytes)  
+4:  yMin (2 bytes)
+6:  yMax (2 bytes)
+8:  timeLimit (2 bytes)
+10: targetScore (2 bytes)

; Object counts (3 bytes)
+12: bgCount (1 byte)
+13: gameplayCount (1 byte)
+14: fgCount (1 byte)

; Layer pointers (6 bytes)
+15: bgObjectsPtr (2 bytes)
+17: gameplayObjectsPtr (2 bytes)
+19: fgObjectsPtr (2 bytes)

; Object arrays (24 bytes per object)
; Each object includes "dynamic" flag at offset +14 (flags field bit 0)
```

## 🔧 Implementación

### Fase 1: Formato .vplay
Añadir campo `physicsEnabled` (ya existe) como indicador de objeto dinámico:
```json
{
  "type": "platform",
  "physicsEnabled": false,  // Static → ROM only
  "x": 0, "y": -50
}
```

### Fase 2: Compilador (vplay_compiler.rs)
- Detectar objetos con `physicsEnabled: true`
- Marcar con bit flag en ROM
- Asignar índices dinámicos (0-11)

### Fase 3: LOAD_LEVEL_RUNTIME
```asm
LOAD_LEVEL_RUNTIME:
    ; 1. Store ROM pointer
    STX LEVEL_PTR
    
    ; 2. Clear dynamic buffer
    CLR LEVEL_DYNAMIC_COUNT
    LDX #LEVEL_DYNAMIC_BUFFER
    JSR CLEAR_DYNAMIC_BUFFER  ; Fill with 0xFF
    
    ; 3. Scan ALL objects in ROM
    ; 4. For each object with (flags & 0x01):
    ;    - Copy rom_index to dynamic buffer
    ;    - Copy initial velocity
    ;    - Set active_flags = 0x01
    ;    - Increment LEVEL_DYNAMIC_COUNT
    
    RTS
```

### Fase 4: UPDATE_LEVEL_RUNTIME
```asm
UPDATE_LEVEL_RUNTIME:
    ; 1. Loop LEVEL_DYNAMIC_COUNT
    ; 2. For each dynamic object:
    ;    - Read rom_index
    ;    - Load ROM position (X,Y from ROM)
    ;    - Load velocity from dynamic buffer
    ;    - Update position: X += vel_x, Y += vel_y
    ;    - Check bounds/collisions
    ;    - Update velocity in dynamic buffer
    
    RTS
```

### Fase 5: SHOW_LEVEL_RUNTIME
```asm
SHOW_LEVEL_RUNTIME:
    ; Phase 1: Render static objects (ROM only)
    LDX LEVEL_PTR
    JSR SHOW_STATIC_OBJECTS  ; Read from ROM, no RAM lookup
    
    ; Phase 2: Render dynamic objects (ROM + RAM)
    LDB LEVEL_DYNAMIC_COUNT
    BEQ SHOW_DONE
    LDU #LEVEL_DYNAMIC_BUFFER
SHOW_DYN_LOOP:
    ; Read rom_index from U+0
    LDA ,U
    ; Calculate ROM offset: A * 24
    ; Load ROM object data (type, sprite, x, y, etc.)
    ; Load velocity from U+1-4 (override ROM velocity)
    ; Render object
    LEAU 6,U              ; Next dynamic object
    DECB
    BNE SHOW_DYN_LOOP
SHOW_DONE:
    RTS
```

## 📋 Checklist de Implementación

- [ ] Actualizar RAM layout en mod.rs (eliminar buffers grandes)
- [ ] Añadir LEVEL_DYNAMIC_BUFFER (72 bytes)
- [ ] Modificar LOAD_LEVEL_RUNTIME (escaneo selectivo)
- [ ] Modificar UPDATE_LEVEL_RUNTIME (solo dinámicos)
- [ ] Modificar SHOW_LEVEL_RUNTIME (ROM-first + dinámicos)
- [ ] Actualizar ejemplos .vplay con physicsEnabled
- [ ] Probar con level_test (objetivo: <10% RAM)
- [ ] Documentar en SUPER_SUMMARY.md

## ⚠️ Backward Compatibility

**Compatibilidad con .vplay existentes**:
- Si `physicsEnabled` no existe → assume `false` (estático)
- Si `physicsEnabled: true` → marcar como dinámico
- Objetos sin velocidad pero con `physicsEnabled: true` → tratados como dinámicos (permite activación futura)

## 🧪 Testing

**Test cases**:
1. Solo estáticos (0 dinámicos) → 0% overhead RAM
2. Mezcla (2 estáticos + 2 dinámicos) → ~1-2% RAM
3. Full dinámicos (12 objetos con física) → ~7% RAM
4. Overflow (>12 dinámicos) → Error en compilación

**Objetivo**: level_test 69.2% → <10% RAM
