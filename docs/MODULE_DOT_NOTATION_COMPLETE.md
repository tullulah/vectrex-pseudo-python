# Multi-Module System - Dot Notation Support

## STATUS: ✅ IMPLEMENTADO (2026-01-11)

### Capacidades Completas

El sistema de módulos ahora soporta **AMBAS** sintaxis de import:

#### 1. Named Imports (desde inicio)
```python
from utils import add, mul
result = add(10, 20)  # ✅ Funciona
```

#### 2. Module Imports con Notación Dot (NUEVO)
```python
import graphics
graphics.draw_box(x, y, w, h, intensity)  # ✅ Funciona
```

### Implementación

**Archivo**: `core/src/unifier.rs`

**Cambios**:

1. **Phase 2 - Alias Resolution Mejorado** (líneas 107-121):
   - Detecta imports de módulos completos (`import graphics`)
   - Construye tabla de aliases: `"main::graphics"` → `("graphics", "*")`
   - Resuelve el module_id real comparando con módulos cargados

2. **rewrite_expr() - Expr::MethodCall** (líneas 452-488):
   - Detecta cuando `target` es un identificador de módulo importado
   - Convierte `module.function()` → `Expr::Call` con nombre unificado
   - Ejemplo: `graphics.draw_box()` → `JSR GRAPHICS_DRAW_BOX`

### Flujo de Resolución

```
Input: graphics.draw_box(x, y, w, h, 127)
  ↓
Parser: Expr::MethodCall {
  target: Expr::Ident("graphics"),
  method_name: "draw_box",
  args: [x, y, w, h, 127]
}
  ↓
Unifier: Detecta "main::graphics" en aliases
  - Lookup: module_id = "graphics"
  - Lookup: name_map[("graphics", "draw_box")] = "GRAPHICS_DRAW_BOX"
  ↓
Unifier: Convierte a Expr::Call {
  name: "GRAPHICS_DRAW_BOX",
  args: [x, y, w, h, 127]
}
  ↓
Codegen: JSR GRAPHICS_DRAW_BOX
```

### Ejemplo Funcional

**Estructura**:
```
examples/multi-module/
├── src/
│   ├── main.vpy        # import graphics
│   └── graphics.vpy    # def draw_box(), def draw_cross()
```

**main.vpy** (fragmento):
```python
import graphics

def loop():
    WAIT_RECAL()
    # Notación dot funcionando ✅
    graphics.draw_box(player_x - 10, player_y - 10, 20, 20, 127)
    graphics.draw_cross(player_x, player_y, 20, 100)
```

**Resultado**:
- ✅ Phase 3.5: Unified 9 items from 2 modules
- ✅ Phase 4: Generated 24.1 KB assembly
- ✅ Phase 6: Binary 32KB (`main.bin`)
- ✅ Llamadas correctas: `JSR GRAPHICS_DRAW_BOX`, `JSR GRAPHICS_DRAW_CROSS`

### Limitaciones Conocidas

⚠️ **Array Labels Collision**: 
- Si múltiples módulos usan arrays, los labels `ARRAY_0`, `ARRAY_1` colisionan
- **Workaround actual**: Evitar arrays en funciones de módulos importados
- **Fix pendiente**: Prefixar array labels con module_id

⚠️ **Runtime Helpers Duplicados**:
- DIV_A, MUL16, etc. no se prefixa correctamente
- **Workaround actual**: Usar operaciones simples en módulos
- **Fix pendiente**: Emitir helpers runtime una sola vez en sección compartida

### Testing

**Test 1: Named Imports** (`examples/multi-module-simple/`)
```bash
cargo run --bin vectrexc -- build examples/multi-module-simple/src/main.vpy
# ✅ 9.6KB assembly, compila correctamente
```

**Test 2: Dot Notation** (`examples/multi-module/`)
```bash
cargo run --bin vectrexc -- build examples/multi-module/src/main.vpy --bin
# ✅ 32KB binary, funciona en emulador
```

### Próximos Pasos

**Phase 6.3b: Array Label Prefixing**
- Modificar codegen para emitir `MODULE_ARRAY_0` en lugar de `ARRAY_0`
- Actualizar referencias en expresiones de indexing

**Phase 6.4: Shared Runtime Section**
- Emitir DIV_A, MUL16, RESULT, etc. UNA SOLA VEZ
- Referenciar desde todos los módulos

**Phase 6.5: Per-Module .vo Files**
- Cambiar de compilación monolítica a archivos .vo separados
- Implementar linker para combinar .vo → .bin

### Estado Actual del Sistema

```
✅ Phase 6.1: Parser (100%)
  - import module
  - from module import symbol
  - Alias support

✅ Phase 6.2: Module Resolution (100%)
  - Recursive loading
  - Path resolution (relative/absolute)
  - Cycle detection

✅ Phase 6.3: Symbol Generation (80%)
  - ✅ Named imports → unified names
  - ✅ Dot notation → unified calls
  - ⏸️ Array label prefixing
  - ⏸️ Runtime helper deduplication

⏸️ Phase 6.4: Shared Runtime (0%)
  - Per-module RAM allocation
  - Shared builtin symbols

⏸️ Phase 6.5: Build Integration (30%)
  - ✅ Monolithic compilation
  - ⏸️ Separate .vo generation
  - ⏸️ Multi-object linking
```

**Overall: Phase 6 está 65% completo** (subió de 60% con dot notation)

---
Última actualización: 2026-01-11 00:10
Commit: Implementado soporte para notación dot en imports de módulos
