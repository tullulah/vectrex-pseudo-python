# Phase 6: Mejoras Futuras Pendientes

**Fecha**: 2026-01-11  
**Estado**: Phase 6.1-6.4 COMPLETO ✅ | Phase 6.5 PARCIAL (30%)

---

## Resumen Ejecutivo

El sistema de módulos VPy está **completamente funcional** mediante compilación unificada. Las optimizaciones pendientes son **opcionales** y solo necesarias para proyectos muy grandes (>50KB).

### ✅ Lo Que Funciona Perfectamente HOY

- **Multi-file projects**: `import graphics`, `import input`, etc.
- **Dot notation**: `input.get_input()`, `graphics.draw_square()`
- **Symbol resolution**: Automática mediante unifier
- **No duplicados**: Runtime helpers consolidados
- **Build time**: <1 segundo para proyectos multi-module
- **Ejemplo real**: `examples/multi-module/` compila a 32KB sin problemas

### ⚠️ Lo Que Está Parcialmente Implementado

**Phase 6.5: Per-Module .vo Generation (30%)**

**Infraestructura Lista**:
- ✅ CLI flag `--separate-modules` exists
- ✅ VectrexObject (.vo format) fully defined
- ✅ Linker can combine multiple .vo files
- ✅ `build_separate_modules()` function skeleton

**Implementación Pendiente**:
- ❌ Compilar cada módulo sin unifier (70% del trabajo)
- ❌ Resolver referencias cross-module individualmente
- ❌ Track imports/exports por módulo separado
- ❌ Testing completo de pipeline separado

---

## Por Qué No Se Completó Phase 6.5

### Complejidad Técnica

La compilación separada requiere **reimplementar** toda la lógica de resolución de símbolos que ya existe en el unifier:

```rust
// LO QUE SE INTENTÓ (fallido):
// Compilar cada módulo ANTES del unifier

// Input.vpy compila solo → ¿Cómo sabe que main.vpy lo usa?
// Main.vpy compila solo → ¿Cómo resuelve input.get_input()?

// PROBLEMA: Chicken-and-egg
// - Necesitas el unifier para resolver símbolos
// - Pero quieres compilar sin el unifier
```

### Intento de Solución (2026-01-11)

Se intentó crear `transform_module_references()` para transformar `input.get_input()` → `INPUT_GET_INPUT()` sin el unifier completo:

```rust
pub fn transform_module_references(
    module: &Module,
    resolver: &ModuleResolver,
    module_path: &Path,
) -> Result<Module> {
    // Build name_map from all modules
    // Rewrite AST nodes...
}
```

**Resultado**: 29 errores de compilación debido a incompatibilidades con estructuras AST:
- `Item::Function` cambió de struct a tupla
- `Stmt::If` usa `body`/`else_body` no `then`/`els`
- `Stmt::Return` es tupla no struct
- Etc.

### Estimación de Trabajo Restante

- **Fixes AST**: 3-4 horas (adaptar todas las variantes)
- **Testing básico**: 2 horas
- **Edge cases**: 3-5 horas (imports circulares, símbolos conflictivos)
- **Integration testing**: 2-3 horas
- **TOTAL**: 10-14 horas de trabajo

### ROI (Return on Investment)

**Beneficio actual**: CERO
- Proyectos reales: <32KB, compilan en <1 segundo
- Sistema unificado: Funciona perfectamente
- No hay quejas de performance

**Beneficio futuro**: Potencial para proyectos >100KB
- Compilación incremental (cambiar 1 módulo, recompilar solo ese)
- Parallel compilation (compilar módulos en paralelo)
- Build cache (.vo files con timestamps)

**Conclusión**: Implementar cuando sea necesario, no especulativamente.

---

## Alternativa Recomendada (Si Se Necesita en el Futuro)

### Approach: Unifier-First + Section Extraction

En lugar de compilar módulos por separado ANTES del unifier, hacerlo DESPUÉS:

```rust
/// Phase 6.5 Alternative Implementation
fn build_separate_modules_v2(entry_path: &Path, ...) -> Result<()> {
    // 1️⃣ UNIFY (como siempre, usando código existente 100%)
    let mut resolver = ModuleResolver::new(project_root);
    resolver.load_project(entry_path)?;
    let unified = unifier::unify_modules(&resolver, entry_module, &options)?;
    
    // 2️⃣ COMPILE (módulo unificado completo, una sola vez)
    let (asm, _, _) = codegen::emit_asm_with_debug(
        &unified.module, 
        Target::Vectrex,
        &options
    )?;
    
    // 3️⃣ EXTRACT SECTIONS (funciones ya tienen prefijos MODULE_*)
    let sections_by_module = extract_module_sections(&asm, &unified.name_map);
    // Ejemplo:
    // - "main" → [MAIN function, LOOP function, VAR_PLAYER_X]
    // - "input" → [INPUT_GET_INPUT function, INPUT_INPUT_RESULT array]
    // - "graphics" → [GRAPHICS_DRAW_SQUARE function]
    
    // 4️⃣ CREATE .vo PER MODULE
    for (module_name, sections) in sections_by_module {
        let symbols = extract_symbols_from_sections(&sections);
        let relocations = collect_relocations(&sections, &symbols);
        
        let vo = VectrexObject {
            header: ObjectHeader { ... },
            sections,
            symbols,
            relocations,
            debug_info: DebugInfo::default(),
        };
        
        vo.save(&format!("{}.vo", module_name))?;
    }
    
    // 5️⃣ LINK (código ya implementado)
    link_cmd(&vo_files, output, base_address, title)?;
}

/// Helper: Extract sections per module from unified ASM
fn extract_module_sections(
    asm: &str, 
    name_map: &HashMap<(String, String), String>
) -> HashMap<String, Vec<Section>> {
    let mut by_module: HashMap<String, Vec<Section>> = HashMap::new();
    
    for line in asm.lines() {
        // Detect section markers
        if line.starts_with("; === RUNTIME HELPERS ===") {
            current_module = "runtime";
        } else if let Some(module_prefix) = detect_module_prefix(line) {
            current_module = module_prefix;
        }
        
        by_module.entry(current_module.clone())
            .or_default()
            .push(parse_section(line));
    }
    
    by_module
}
```

### Ventajas de Esta Alternativa

| Aspecto | Approach Actual (fallido) | Approach Alternativo |
|---------|---------------------------|----------------------|
| **Reutilización código** | 30% (solo linker) | 95% (unifier + codegen + linker) |
| **Resolución símbolos** | Reimplementar todo | Gratis (unifier) |
| **AST compatibility** | 29 errores | 0 errores |
| **Testing** | Pipeline completo nuevo | Solo extraction logic |
| **Tiempo estimado** | 10-14 horas | 2-3 horas |
| **Riesgo bugs** | Alto (nueva lógica) | Bajo (reutiliza existente) |
| **Mantenibilidad** | Doble pipeline | Pipeline único |

### Cuándo Implementar

**Triggers para implementación**:
1. Proyectos reales superan 50KB → build time >3 segundos
2. Usuarios piden compilación incremental explícitamente
3. Existe cache system (timestamps) para .vo files

**No implementar si**:
- Proyectos <50KB (actual: 32KB max)
- Build time <3 segundos (actual: <1s)
- No hay demanda de usuarios

---

## Roadmap Completo de Optimizaciones

### Phase 6.5: Per-Module .vo Generation
**Prerequisitos**: Ninguno (infraestructura ready)  
**Implementación**: Approach alternativo (unifier + extract)  
**Tiempo**: 2-3 horas  
**Beneficio**: .vo files por módulo (foundation para 6.6 y 6.7)

### Phase 6.6: Incremental Build System
**Prerequisitos**: Phase 6.5 completado  
**Tiempo**: 5-8 horas  
**Beneficio**: Solo recompilar módulos modificados

```rust
fn should_rebuild(source: &Path, vo: &Path) -> bool {
    !vo.exists() || 
    source.metadata().unwrap().modified().unwrap() >
    vo.metadata().unwrap().modified().unwrap()
}

for module in modules {
    if should_rebuild(&module.path, &module.vo_path) {
        compile_module_to_vo(module)?;
    } else {
        eprintln!("  ✓ {} up-to-date", module.name);
    }
}
```

**Ejemplo real**:
```bash
# Primera compilación: 10 módulos × 100ms = 1 segundo
vectrexc build game.vpy --separate-modules

# Cambio en input.vpy solamente
vim src/input.vpy

# Segunda compilación: 1 módulo × 100ms = 100ms
vectrexc build game.vpy --separate-modules
# Output: ✓ main.vo up-to-date
#         ✓ graphics.vo up-to-date
#         ⟳ input.vpy → input.vo
#         ✓ music.vo up-to-date
#         ...
```

### Phase 6.7: Parallel Module Compilation
**Prerequisitos**: Phase 6.5 completado  
**Tiempo**: 3-5 horas  
**Beneficio**: 2-4x speedup en multi-core machines

```rust
use rayon::prelude::*;

let vo_files: Vec<PathBuf> = modules.par_iter()
    .filter(|m| should_rebuild(&m.path, &m.vo_path))
    .map(|module| compile_module_to_vo(module))
    .collect::<Result<Vec<_>>>()?;
```

**Speedup esperado**:
- 4 cores: 10 módulos × 100ms / 4 = 250ms (vs 1000ms secuencial)
- 8 cores: 10 módulos × 100ms / 8 = 125ms (vs 1000ms secuencial)

### Phase 6.8: Build Cache System
**Prerequisitos**: Phase 6.6 completado  
**Tiempo**: 5-8 horas  
**Beneficio**: Cache global de .vo files (compartido entre proyectos)

```rust
// ~/.vpy_cache/
// ├── input_a3f2b8c9.vo  (hash de input.vpy)
// ├── graphics_d4e5f6a7.vo
// └── ...

fn get_cached_vo(source: &Path) -> Option<PathBuf> {
    let hash = hash_file(source);
    let cache_path = cache_dir().join(format!("{}_{}.vo", 
        source.file_stem()?, hash));
    
    if cache_path.exists() {
        Some(cache_path)
    } else {
        None
    }
}
```

---

## Prioridades Recomendadas

### 🎯 **AHORA** (No hacer nada adicional)
- Sistema actual funciona perfectamente
- Proyectos compilan rápido (<1s)
- No hay pain points reales

### 📅 **FUTURO CERCANO** (Cuando build time >3s)
1. **Phase 6.5** (approach alternativo): 2-3 horas
2. **Phase 6.6** (incremental): 5-8 horas
3. Testing completo: 3-5 horas
4. **TOTAL**: ~2 días de trabajo

### 🔮 **FUTURO LEJANO** (Cuando proyectos >100KB)
1. **Phase 6.7** (parallel): 3-5 horas
2. **Phase 6.8** (cache): 5-8 horas
3. **TOTAL**: ~1-2 días adicionales

---

## Testing Strategy (Para Cuando Se Implemente)

### Test Cases Mínimos

**1. Basic Separate Compilation**:
```bash
vectrexc build main.vpy --separate-modules
# Verify: main.vo, input.vo, graphics.vo created
# Verify: main.bin funcional
```

**2. Incremental Rebuild**:
```bash
vectrexc build main.vpy --separate-modules  # Primera
touch src/input.vpy
vectrexc build main.vpy --separate-modules  # Solo input.vo
```

**3. Cross-Module References**:
```python
# main.vpy
import input
input.get_input()  # → INPUT_GET_INPUT()
x = input.result[0]  # → INPUT_RESULT[0]
```

**4. Symbol Conflicts**:
```python
# a.vpy
def test(): pass

# b.vpy
def test(): pass

# main.vpy
import a
import b
a.test()  # → A_TEST()
b.test()  # → B_TEST()
```

### Success Criteria

- ✅ All .vo files created
- ✅ main.bin executable
- ✅ Incremental rebuild <50% of full build time
- ✅ Parallel build 2x speedup on 4+ cores
- ✅ No regressions vs unified compilation

---

## Conclusión

### Estado Actual (2026-01-11)

**✅ COMPLETO Y FUNCIONAL**:
- Phase 6.1-6.4: Multi-module system working perfectly
- Unified compilation: Fast, reliable, battle-tested
- Real-world projects: Compile successfully (<1s)

**⏸️ PAUSADO (Baja Prioridad)**:
- Phase 6.5: Per-module .vo (30% - infraestructura ready)
  - Razón: Unified compilation es suficiente
  - Approach alternativo disponible cuando se necesite

**📅 FUTURO (Cuando Sea Necesario)**:
- Phase 6.6: Incremental builds (depends on 6.5)
- Phase 6.7: Parallel compilation (depends on 6.5)
- Phase 6.8: Build cache (depends on 6.6)

### Recomendación Final

**NO hacer nada adicional hasta que**:
1. Proyectos reales >50KB
2. Build time >3 segundos
3. Usuarios pidan compilación incremental

**Cuando sea el momento**:
- Usar approach alternativo (unifier + extract)
- 2-3 días de implementación total
- ROI positivo en ese punto

**Por ahora**:
- Sistema funciona perfectamente ✅
- Documentación completa para el futuro ✅
- Enfocarse en otras features más prioritarias ✅
