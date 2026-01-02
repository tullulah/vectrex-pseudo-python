# Vectrex Memory Map - VPy Compiler

Este documento describe todas las direcciones de memoria usadas por el compilador VPy y su relación con el hardware Vectrex real.

## 1. Vectrex Hardware Memory Map (Oficial)

Según la documentación oficial del Vectrex (VECTREX.INC):

```
$C800-$C87F  BIOS RAM (128 bytes)
             Variables del sistema usadas por la BIOS
             
$C880-$CBEA  USER RAM (874 bytes disponibles)
             ✅ Área segura para programas de usuario
             
$CBEA-$CFFF  BIOS Stack y vectores de interrupción
             Stack default en $CBEA
             
$D000-$D7FF  6522 VIA (shadowed 128 times)
             Hardware I/O (PSG, joystick, etc.)
             
$E000-$FFFF  System ROM (8KB BIOS)
             Código de la BIOS
```

## 2. Variables BIOS Reales (Usadas por VPy)

El compilador VPy usa estas direcciones BIOS REALES del hardware Vectrex:

### 2.1 Joystick (Joy_Analog BIOS)
```asm
$C81A  Vec_Joy_Resltn  ; Resolution (usado en init)
$C81B  Vec_Joy_1_X     ; ✅ Joystick 1 X axis (0-255)
$C81C  Vec_Joy_1_Y     ; ✅ Joystick 1 Y axis (0-255)
$C81F  Vec_Joy_Mux_1_X ; X axis enable (usado en init)
$C820  Vec_Joy_Mux_1_Y ; Y axis enable (usado en init)
$C821  Vec_Joy_Mux_2_X ; Joystick 2 disable (usado en init)
$C822  Vec_Joy_Mux_2_Y ; Joystick 2 disable (usado en init)
$C823  Analog mode flag ; Cleared in init (Joy_Analog hace DEC)
```

**IMPORTANTE**: Estas son las ÚNICAS direcciones válidas para joystick. NO inventar direcciones custom.

### 2.2 Buttons (Read_Btns BIOS)
```asm
$C80F  Vec_Btn_State   ; Button state (written by Read_Btns)
```

## 3. Variables VPy Compiler

El compilador VPy usa el sistema **`RamLayout`** para asignar variables automáticamente, garantizando CERO colisiones:

### 3.1 RamLayout Automático (Sistema Perfecto ✅)
```rust
// TODAS las variables se asignan en este orden automáticamente:
let mut ram = RamLayout::new(0xC880);

// 1. Runtime temporaries
ram.allocate("RESULT", 2, "Main result temporary");
ram.allocate("TMPPTR", 2, "Pointer temp");
// ...

// 2. PSG Music (si hay assets de música)
ram.allocate("PSG_MUSIC_PTR", 2, "...");
// ...

// 3. SFX (si hay assets de SFX)
ram.allocate("SFX_PTR", 2, "...");
// ...

// 4. PRINT_NUMBER buffer
ram.allocate("NUM_STR", 6, "...");

// 5. Vector list variables
ram.allocate("VL_PTR", 2, "Current position in vector list");
ram.allocate("VL_Y", 1, "Y position");
ram.allocate("VL_X", 1, "X position");
ram.allocate("VL_SCALE", 1, "Scale factor");

// 6. Drawing helpers (DRAW_VECTOR, DRAW_LINE, DRAW_CIRCLE)
ram.allocate("DRAW_VEC_X", 1, "...");
ram.allocate("MIRROR_X", 1, "...");
// ...

// 7. User variables (ÚLTIMO - usa espacio restante)
ram.allocate("VAR_PLAYER_X", 2, "...");
ram.allocate("VAR_ENEMIES_DATA", 6, "Array 3 elements");
// ...
```

### 3.2 Resultado en ASM
```asm
; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 34 bytes (ejemplo programa pequeño)
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TEMP_YX              EQU $C880+$02   ; Temporary y,x storage (2 bytes)
VL_PTR               EQU $C880+$0C   ; Current position in vector list (2 bytes)
DRAW_VEC_X           EQU $C880+$11   ; DRAW_VECTOR X offset (1 bytes)
VAR_PLAYER_X         EQU $C880+$22   ; User variable (2 bytes)
...
```

**Ventajas del Sistema RamLayout**:
- ✅ **Cero colisiones**: Imposible que dos variables usen la misma dirección
- ✅ **Compacto**: No hay huecos ni espacio desperdiciado
- ✅ **Automático**: No hay que calcular offsets manualmente
- ✅ **Flexible**: Agregar/quitar variables no rompe nada
- ✅ **Seguro**: Garantiza no exceder USER RAM limit ($CBEA)

## 4. ERRORES HISTÓRICOS CORREGIDOS

### 4.1 Custom Joystick Addresses (CORREGIDO 2025-12-30)
❌ **ERROR (antiguo)**:
```asm
$CF00  Joy_1_X  ; DIRECCIÓN INVENTADA - no existe en hardware
$CF01  Joy_1_Y  ; DIRECCIÓN INVENTADA - no existe en hardware
```

**Problema**: En Vectrex real, estas direcciones contienen basura → joystick aleatorio

✅ **CORRECTO (actual)**:
```asm
$C81B  Vec_Joy_1_X  ; BIOS address real (escrita por Joy_Analog $F1F5)
$C81C  Vec_Joy_1_Y  ; BIOS address real (escrita por Joy_Analog $F1F5)
```

### 4.2 Vector List Variables en Stack Zone (RESUELTO 2025-12-30)
❌ **ERROR (antiguo)**:
```asm
$CF80  VL_PTR    ; En zona de stack ($CBEA-$CFFF) - PELIGROSO
$CF82  VL_Y
$CF83  VL_X
$CF84  VL_SCALE
```

**Problema**: Stack crece hacia abajo desde $CBEA → puede sobreescribir estas variables

✅ **CORRECTO (actual - RamLayout automático)**:
- Todas las variables VL_*, DRAW_*, y VAR_* se asignan automáticamente
- El sistema RamLayout garantiza que NO hay overlaps
- Ejemplo: En programa pequeño (34 bytes total):
  - VL_PTR en $C880+$0C
  - DRAW_VEC_X en $C880+$11
  - Variables de usuario después de TODO lo demás
- Imposible colisionar con stack zone porque RamLayout controla límite superior

## 5. REGLAS DE ASIGNACIÓN DE MEMORIA

### 5.1 Sistema RamLayout (Automático y Seguro)
✅ **Proceso automático**:
1. Todas las variables se asignan en orden secuencial
2. No hay huecos ni overlaps
3. Total calculado automáticamente
4. Garantía de no exceder USER RAM limit

✅ **Rangos Seguros**:
- Base: `$C880` (inicio USER RAM)
- Variables asignadas secuencialmente hacia arriba
- Límite: `$CBEA` (inicio stack zone) - RamLayout verifica automáticamente

❌ **Prohibido**: Direcciones hardcoded fuera de sistema RamLayout
❌ **Prohibido**: `$C800-$C87F` (BIOS system variables)
❌ **Prohibido**: `$CBEA-$CFFF` (Stack zone)

### 5.2 Stack Considerations (Sin Cambios)
El stack Vectrex por defecto comienza en `$CBEA` y crece hacia abajo:
```
$CBEA  ← Stack pointer inicial
$CBE9
$CBE8
...    ← Stack crece hacia aquí
$C8XX  ← RamLayout termina aquí (depende del programa)
```

**Espacio disponible**: 
- USER RAM total: 874 bytes ($C880-$CBEA)
- Programa pequeño: ~34 bytes → 840 bytes libres
- Programa grande: ~200 bytes → 674 bytes libres
- Stack: ~200 bytes seguros (asumiendo uso normal)

## 6. Frontend Emulator Integration

El frontend (`EmulatorPanel.tsx`) debe escribir valores de joystick a las direcciones BIOS REALES:

```typescript
// ✅ CORRECTO: Escribir a BIOS addresses
vecx.write8(0xC81B, analogX); // Vec_Joy_1_X
vecx.write8(0xC81C, analogY); // Vec_Joy_1_Y

// ❌ INCORRECTO: NO inventar direcciones custom
// vecx.write8(0xCF00, analogX); // ¡Esta dirección no existe en hardware!
```

## 7. Verificación de Colisiones

**Sistema RamLayout garantiza CERO colisiones automáticamente**:

1. **Variables VPy**: Asignadas secuencialmente por RamLayout
2. **VL_* variables**: Integradas en RamLayout (no más hardcoded)
3. **Límite superior**: RamLayout puede verificar que `total_size() + stack_size < 874 bytes`

**NO es necesario verificar manualmente** - el sistema RamLayout previene colisiones por diseño.

### 7.1 Ejemplo Real (testsmallline.vpy)
```
Total RAM used: 34 bytes
Máximo teórico: 874 bytes
Espacio libre: 840 bytes (96% libre!)
Stack seguro: 200 bytes asumidos → 640 bytes disponibles para variables
```

**Si un programa necesita más de ~674 bytes de variables**:
- ✅ RamLayout detectará automáticamente el límite
- ✅ Considerar optimizaciones:
  - Usar const arrays (almacenados en ROM, no RAM)
  - Reusar variables entre funciones
  - Usar structs más eficientes

## 8. Debugging Memory Issues

Si experimentas corrupción de memoria:

1. **Verificar tamaño de variables**: ¿Estás usando muchas arrays grandes?
2. **Verificar stack overflow**: ¿Tu programa tiene recursión profunda o muchas llamadas anidadas?
3. **Memory Panel en IDE**: Usa la búsqueda para inspeccionar rangos de memoria
4. **Watch List**: Agrega variables críticas para monitorear cambios

---

Última actualización: 2026-01-02 - Sistema RamLayout implementado: eliminados TODOS los hardcoded addresses, asignación automática de variables garantiza cero colisiones
