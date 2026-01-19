# Auditoría Completa de Migración (Core vs Buildtools)

Este documento detalla todas las discrepancias encontradas entre el compilador original (`Core`) y el nuevo sistema (`Buildtools`).

## 1. Builtins (Funciones Integradas)

Comparación línea por línea de las funciones disponibles.

| Función en Core | Estado en Buildtools | Notas |
|----------------|----------------------|-------|
| `PRINT_TEXT` | ✅ Implementada | Misma firma (x, y, text) |
| `PRINT_NUMBER` | ✅ Implementada | Misma firma (x, y, num) |
| `DRAW_LINE` | ✅ Implementada | Soporta `DRAW_LINE_WRAPPER` (segmentación) |
| `DRAW_LINE_FAST` | ❌ **FALTANTE** | No implementada |
| `DRAW_LINE_WRAPPER` | ✅ Implementada | Helper interno expuesto |
| `DRAW_RECT` | ✅ Implementada | Nueva implementación modular |
| `DRAW_CIRCLE` | ✅ Implementada | Nueva implementación modular |
| `DRAW_VL` | ⚠️ Parcial | Existe `DRAW_VECTOR_LIST` pero `VECTREX_DRAW_VL` eliminado |
| `DRAW_VECTOR` | ✅ Implementada | Sistema de assets completo (.vec) |
| `DRAW_VECTOR_EX` | ✅ Implementada | Sistema de assets con espejo |
| `PLAY_MUSIC` | ✅ Implementada | Sistema de assets (.vmus) |
| `PLAY_MUSIC1` | ❌ **FALTANTE** | Versión antigua eliminada |
| `PLAY_SFX` | ✅ Implementada | Sistema de assets (.vsfx) |
| `WAIT_RECAL` | ✅ Implementada | Inyección automática (ahora `AUDIO_UPDATE` inyectado también) |
| `SET_INTENSITY` | ✅ Implementada | |
| `SET_ORIGIN` | ❌ **FALTANTE** | (VECTREX_SET_ORIGIN) |
| `RESET0REF` | ✅ Implementada | |
| `J1_X`, `J1_Y` | ✅ Implementada | Versión extendida con J2 |
| `UPDATE_BUTTONS` | ✅ Implementada | |
| `J1_BUTTON_1..4` | ✅ Implementada | |
| `MUL_A`, `DIV_A` | ❌ **ELIMINADAS** | Core las exponía como builtins. Buildtools usa sintaxis de expresión `*`, `/` |
| `MOD_A` | ❌ **ELIMINADA** | Buildtools usa `%` |
| `SIN`, `COS`, `TAN` | ✅ Implementada | Tablas LUT generadas condicionalmente |
| `ABS`, `MIN`, `MAX` | ✅ Implementada | |
| `CLAMP` | ✅ Implementada | |
| `SQRT`, `POW`, `ATAN2` | ✅ Implementada | Nuevas funciones matemáticas |
| `RAND`, `RAND_RANGE` | ✅ Implementada | Generador RNG implementado |
| `DEBUG_PRINT` | ✅ Implementada | |
| `POKE`, `PEEK` | ✅ Implementada | |
| `VECTREX_POKE/PEEK` | ❌ **RENOMBRADAS** | Ahora son `POKE` / `PEEK` |

## 2. Variables de RAM (Memoria del Sistema)

Buildtools ha cambiado radicalmente la gestión de memoria para las operaciones matemáticas, pasando de variables estáticas a uso de Pila (Stack), lo cual es una mejora técnica (permite recursividad y ahorra RAM estática).

| Variable en Core | Estado en Buildtools | Razón del Cambio |
|------------------|----------------------|-------------------|
| `RESULT` | ✅ Mantenida | Registro temporal principal |
| `TMPPTR` | ✅ Mantenida | Puntero temporal |
| `MUL_A` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `MUL_B` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `MUL_RES` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `MUL_TMP` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `MUL_CNT` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `DIV_A` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `DIV_B` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `DIV_Q` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `DIV_R` | ❌ **ELIMINADA** | Reemplazado por Stack (PSHS) |
| `VAR_ARG0..4` | ✅ Mantenida | Paso de argumentos a helpers |
| `DRAW_LINE_ARGS` | ✅ Mantenida | Buffer específico para DRAW_LINE |
| `VLINE_*` | ✅ Mantenida | Variables para segmentación de líneas |
| `DRAW_VEC_*` | ✅ Nueva | Variables para DRAW_VECTOR (posición, espejo) |
| `RAND_SEED` | ✅ Nueva | Semilla para RNG |

> **Nota Técnica**: La eliminación de `MUL_A`/`DIV_A` ahorra 18 bytes de RAM crítica en la página cero y permite llamar funciones matemáticas dentro de otras sin corromper los operandos estáticos.

## 3. Helpers (Funciones de Soporte)

| Helper | Implementación Core | Implementación Buildtools | Diferencia Crítica |
|--------|---------------------|---------------------------|-------------------|
| `MUL16` | Estática (RAM vars) | Stack (Reentrante) | Buildtools es más seguro |
| `DIV16` | Estática (RAM vars) | Stack (Reentrante) | Buildtools es más seguro |
| `PRINT_TEXT` | Wrapper BIOS | Wrapper con Reset_Pen | Buildtools añade seguridad (Reset_Pen) |
| `DRAW_LINE_WRAPPER` | Segmentación | Segmentación | Misma lógica |

## 4. Funcionalidades Faltantes Detectadas

1. **Validación de Aridad (Arity Check)**
   - Core: Validaba número de argumentos en tiempo de compilación.
   - Buildtools: Tabla `BUILTIN_ARITIES` existe pero **no se estaba usando**. (Corregido en commit reciente, pendiente de verificación).

2. **Builtins de Acceso Directo a BIOS**
   - Core permitía `VECTREX_POKE`/`VECTREX_PEEK`.
   - Buildtools requiere usar `POKE`/`PEEK` (wrappers seguros).

3. **Optimizaciones Específicas**
   - `DRAW_LINE_FAST` (versión sin segmentación/checks) no ha sido portada.

## 5. Constantes

Confirmado: **No existían constantes predefinidas** en `Core` (como `SCREEN_WIDTH`). El usuario debía definirlas. Buildtools mantiene este comportamiento (sin constantes mágicas).

---
**Conclusión**: La "pérdida" de variables `MUL_A`/`DIV_A` es intencional y beneficiosa. La falta de validación de aridad es un bug real (ya identificado). Las funcionalidades de assets (`DRAW_VECTOR`) son superiores en Buildtools.

## 6. Caso de Estudio Crítico: DRAW_VECTOR (Broken in Buildtools)

Se ha realizado una comparativa técnica detallada entre la implementación de referencia (Core) y la nueva (Buildtools) para `DRAW_VECTOR`.

### Diagnóstico: ❌ FALLO CRÍTICO EN LOGICA DE LLAMADA
La **Helper Function** (`Draw_Sync_List_At_With_Mirrors`) es idéntica y correcta.
La **Caller Function** (`emit_draw_vector`) está **ROTA** en Buildtools. Falla al iterar sobre los paths del asset porque carece de acceso al Registro de Assets del Proyecto.

### 1. Tabla Comparativa: Lógica del Llamador (`emit_draw_vector`)

| Característica | Lógica Core (`core/.../builtins.rs`) | Lógica Buildtools (`buildtools/.../builtins.rs`) | Estado |
| :--- | :--- | :--- | :--- |
| **Iteración de Paths** | **Itera 0..path_count**<br>`for path_idx in 0..path_count` | ❌ **Hardcoded a Primer Path**<br>`LDX #{symbol}_PATH0` | **ROTO** |
| **Conciencia de Assets** | ✅ Verifica `opts.assets`<br>Carga .vec para contar paths | ❌ **Emisión Ciega**<br>Sin acceso a `opts`, asume 1 path | **ROTO** |
| **Acceso a Contexto** | Recibe `opts: &CodegenOptions` | Recibe solo `args` y `out`<br>`fn(args, out)` | **Fallo de Diseño** |
| **Setup DP** | Safe Wrapper (`$F1AA` / `$F1AF`) | Safe Wrapper (`$F1AA` / `$F1AF`) | ✅ IGUAL |
| **Setup Coordenadas**| Guarda en `TMPPTR` luego `DRAW_VEC_X/Y` | Guarda en `TMPPTR` luego `DRAW_VEC_X/Y` | ✅ IGUAL |
| **Reset Espejo** | Explicito `CLR MIRROR_X/Y` | Explicito `CLR MIRROR_X/Y` | ✅ IGUAL |

### 2. Tabla Comparativa: Lógica del Helper (`Draw_Sync_List_At_With_Mirrors`)

| Operación | Core (`emission.rs`) | Buildtools (`drawing.rs`) | Estado |
| :--- | :--- | :--- | :--- |
| **Identidad** | Fuente Original | Copy-Paste del Original | ✅ IGUAL |
| **Setup** | Verifica override `DRAW_VEC_INTENSITY` | Verifica override `DRAW_VEC_INTENSITY` | ✅ IGUAL |
| **Posicionamiento** | Suma offsets `DRAW_VEC_X/Y` | Suma offsets `DRAW_VEC_X/Y` | ✅ IGUAL |
| **Espejo** | `TST MIRROR_X/Y` -> `NEGA/B` | `TST MIRROR_X/Y` -> `NEGA/B` | ✅ IGUAL |
| **Zero Ref** | Secuencia standard `VIA_cntl` | Secuencia standard `VIA_cntl` | ✅ IGUAL |
| **Looping** | Maneja segmentos y `next_path` | Maneja segmentos y `next_path` | ✅ IGUAL |

### Causa Raíz
Buildtools tiene un comentario admitiendo el defecto:
```rust
// Note: In buildtools, we expect the vector to be compiled with multiple _PATH0, _PATH1, etc.
// For now, we'll generate a call to the first path as a placeholder
out.push_str(&format!("    LDX #{}_PATH0  ; Load first path\n", symbol));
```
Esto ocurre porque la firma de `emit_builtin` en Buildtools no recibe `CodegenOptions`, impidiendo buscar el asset para saber cuántos paths tiene.

