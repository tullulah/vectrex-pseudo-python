# IDE Diagnostics Implementation - Summary

## ✅ Completed Implementation

Se ha portado el análisis de variables del LSP al compilador del IDE. Ahora los diagnostics se generan automáticamente durante la compilación.

### 1️⃣ Cambios en el Compilador (core/src/codegen.rs)

**Nuevos Diagnostic Codes**:
- `DiagnosticCode::UnusedVariable` - Variable declarada pero nunca usada
- `DiagnosticCode::SuggestConst` - Variable que nunca cambia (sugerir const)

**Nuevas Estructuras**:
```rust
struct VariableUsage {
    declared: bool,
    initialized: bool,
    read_count: usize,
    write_count: usize,
    declaration_line: Option<usize>,
    is_const: bool,
}

struct UsageAnalysis {
    variables: HashMap<String, VariableUsage>,
}
```

**Nuevas Funciones**:
- `analyze_variable_usage(module)` - Analiza uso de variables en todo el módulo
- `analyze_statements(stmts)` - Analiza statements recursivamente
- `analyze_expr(expr)` - Detecta lecturas de variables en expresiones
- `generate_usage_diagnostics()` - Genera diagnostics para IDE

**Integración**:
- Los diagnostics se generan en `emit_asm_with_debug()` automáticamente
- Se incluyen en el campo `diagnostics` del resultado de compilación

### 2️⃣ Próximos Pasos - Integración IDE

El compilador ya genera los diagnostics. Ahora necesitas:

**A) Verificar que el IDE los recibe**:
1. Abre el IDE (`npm run electron-start` desde `ide/`)
2. Abre `examples/pang/src/main.vpy`
3. Compila (Build button o F7)
4. Verifica que aparezcan diagnostics en el editor Monaco

**B) Expected Diagnostics en Pang**:
- Línea ~15: `num_locations = 17` → ⚠️ "Variable 'num_locations' never changes - consider 'const' to save RAM (2 bytes)"
- Línea ~19: `hook_max_y = 40` → ⚠️ "Variable 'hook_max_y' never changes - consider 'const' to save RAM (2 bytes)"
- Línea ~20: `player_speed = 2` → ⚠️ "Variable 'player_speed' never changes - consider 'const' to save RAM (2 bytes)"

**C) Si NO aparecen diagnostics**:

Necesitas agregar soporte para los nuevos códigos en el IDE Monaco:

1. **Mapeo de Diagnostic Codes** (`ide/frontend/src/contexts/VPyContext.tsx` o similar):
```typescript
function mapDiagnosticToMonaco(diag: CompilerDiagnostic): monaco.editor.IMarkerData {
    let severity = monaco.MarkerSeverity.Warning;
    
    // Map new diagnostic codes
    if (diag.code === 'SuggestConst') {
        severity = monaco.MarkerSeverity.Hint; // Yellow underline
    }
    
    return {
        severity,
        message: diag.message,
        startLineNumber: diag.line || 1,
        startColumn: diag.col || 1,
        endLineNumber: diag.line || 1,
        endColumn: diag.col ? diag.col + 100 : 100,
    };
}
```

2. **Code Actions para Quick Fixes** (Monaco Editor):
```typescript
monaco.languages.registerCodeActionProvider('vpy', {
    provideCodeActions: (model, range, context) => {
        const actions: monaco.languages.CodeAction[] = [];
        
        for (const marker of context.markers) {
            if (marker.message.includes("never changes")) {
                // Quick Fix: Convert to const
                const line = model.getLineContent(marker.startLineNumber);
                const varName = extractVarName(marker.message);
                const newText = line.replace(`${varName} =`, `const ${varName} =`);
                
                actions.push({
                    title: `Convert '${varName}' to const`,
                    kind: 'quickfix',
                    edit: {
                        edits: [{
                            resource: model.uri,
                            edit: {
                                range: new monaco.Range(
                                    marker.startLineNumber, 1,
                                    marker.startLineNumber, line.length + 1
                                ),
                                text: newText
                            }
                        }]
                    }
                });
            }
            
            if (marker.message.includes("never used")) {
                // Quick Fix: Remove unused variable
                actions.push({
                    title: `Remove unused variable`,
                    kind: 'quickfix',
                    edit: {
                        edits: [{
                            resource: model.uri,
                            edit: {
                                range: new monaco.Range(
                                    marker.startLineNumber, 1,
                                    marker.startLineNumber + 1, 1
                                ),
                                text: ''
                            }
                        }]
                    }
                });
            }
        }
        
        return { actions, dispose: () => {} };
    }
});
```

### 3️⃣ Testing Workflow

1. **Archivo de prueba simple**: [test_ide_diagnostics.vpy](test_ide_diagnostics.vpy)
   - `unused_var = 42` → WARNING
   - `num_locations = 17` → HINT (suggest const)

2. **Archivo real**: `examples/pang/src/main.vpy`
   - 3 variables candidatas a const

3. **Verificar Quick Fixes**:
   - Hover sobre warning/hint
   - Debe aparecer lightbulb 💡
   - Click → "Convert to const" o "Remove unused"
   - Cambio se aplica automáticamente

### 4️⃣ Commits Realizados

1. `ee12e687` - LSP Phase 3 (Code Actions para VSCode)
2. `e0114d07` - Port to compiler for IDE (este commit)

### 5️⃣ Files Modified

- `core/src/codegen.rs`: +220 líneas
  - Nuevos diagnostic codes
  - Variable usage analysis
  - Diagnostic generation
  - Integration en emit_asm_with_debug

### 6️⃣ Next Steps

**Si funciona automáticamente** (IDE ya mapea códigos):
- ✅ Listo! Solo prueba y confirma

**Si necesitas agregar Code Actions**:
- Modificar `ide/frontend/src/contexts/VPyContext.tsx`
- Agregar `monaco.languages.registerCodeActionProvider`
- Mapear diagnostic codes a Quick Fixes

**Si necesitas mejor UI**:
- Agregar iconos para hints vs warnings
- Mejorar mensajes (más descriptivos)
- Agregar shortcuts de teclado (Cmd+. para Quick Fix)

---

**¿Quieres que implemente las Code Actions en Monaco ahora, o primero verificamos que los diagnostics ya aparecen?**
