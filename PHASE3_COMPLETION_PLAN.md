# Phase 3 Unifier Completion Plan - Incremental Implementation

## Objetivo
Terminar completamente Phase 3 antes de continuar. Nada de features a medias.

## Trabajo A Completar (Incremental)

### 1️⃣ CIRCULAR IMPORT DETECTION (0.5 - 1 hora)
**Estado**: NO existe
**Ubicación**: core/src/unifier.rs - antes de Phase 1
**Qué hace**: Detecta cuando A→B→A y reporta error claro
**Implementación**:
- Función `detect_circular_imports()` que recorre el grafo de imports
- Build dependency graph
- Check for cycles usando DFS
- Reportar error con módulos involucrados

**Validación**: 
- Test case A imports B, B imports A → error
- Test case A imports B, B imports C, C imports A → error
- Test case normal imports → no error

### 2️⃣ NAME CONFLICT DETECTION (0.5 - 1 hora)
**Estado**: NO existe  
**Ubicación**: core/src/unifier.rs - Phase 1 (export collection)
**Qué hace**: Detecta cuando múltiples módulos exportan mismo símbolo
**Implementación**:
- Mientras se colectan exports, verificar duplicados
- Si hay conflicto: warning (no error) con nombre de módulos
- Recomendación: usar `module.symbol()` notation
**Validación**:
- input.vpy exports `get_input`, graphics.vpy exports `get_input` → warning
- Normal case sin conflictos → no warning

### 3️⃣ MISSING MODULE VALIDATION (0.5 horas)
**Estado**: Silenciosamente ignorado
**Ubicación**: core/src/unifier.rs - Phase 2 (alias building)
**Qué hace**: Valida que módulos importados existen
**Implementación**:
- En Phase 2, cuando se procesa import, verificar que módulo existe
- Si no existe: error claro "Cannot find module: xyz"
**Validación**:
- import nonexistent → error
- import existing → OK

### 4️⃣ TREE SHAKING IMPLEMENTATION (2 - 3 horas)
**Estado**: Disabled pero campo existe
**Ubicación**: core/src/unifier.rs - nueva Phase después de 4
**Qué hace**: Elimina símbolos no usados
**Implementación**:
- Phase 4.5: Collect used symbols starting from main/loop functions
- Phase 5: Filter items que no estén en used set
- Recalcular name_map solo con símbolos usados
**Validación**:
- Project sin tree_shake → todos los símbolos
- Project con tree_shake → solo símbolos usados
- Binario con tree_shake debe ser más pequeño

### 5️⃣ TEST SUITE PARA PHASE 3 (2 - 3 horas)
**Estado**: Incompleto (AST cambió)
**Ubicación**: core/tests/ o nuevo archivo  
**Qué hace**: Tests unitarios para todas las fases
**Implementación**:
- Revisar AST actual (Module, Item, ImportDecl, ImportSymbols)
- Crear fixtures que construyan AST valido
- Tests para cada fase + cada nueva feature
- Mínimo 20+ tests

---

## Orden de Implementación (Incremental)

```
✅ 1. Circular import detection (basada en analyzer/resolver)
   └─ Validar con test cases
   
✅ 2. Name conflict detection
   └─ Validar con test cases
   
✅ 3. Missing module validation  
   └─ Validar con test cases
   
✅ 4. Tree shaking
   └─ Validar con binarios más pequeños
   
✅ 5. Comprehensive test suite
   └─ Validar que todo integra
```

Cada paso:
1. Implement feature
2. Compile & verify no errors
3. Run existing multi-module examples → deben compilar
4. Create specific test case
5. Verify test case passes
6. Move to next step

---

## Tiempo Total Estimado
- Circular imports: 1 hora
- Name conflicts: 1 hora
- Missing modules: 0.5 horas
- Tree shaking: 2.5 horas
- Test suite: 2.5 horas
- Integration/validation: 1.5 horas

**TOTAL**: 8-9 horas para Phase 3 100% completo

---

## Success Criteria
✅ Circular imports detectados → error claro
✅ Name conflicts detectados → warning
✅ Missing modules detectados → error claro
✅ Tree shaking funciona cuando enabled
✅ 30+ tests para todas fases
✅ Todos los ejemplos multi-módulo siguen compilando
✅ Nada "a medias" quedado
✅ Phase 3 CLOSED, completamente robusto

---

## Qué NO tocamos
- Phase 1-2 (loader, lexer) - funcionan perfectamente
- Phase 2b parser - 28/28 tests passing
- Phase 4+ (codegen, etc) - esperan a Phase 3

---

**Status**: Ready to start. Esperando OK para comenzar.
