# Plan de Mejoras para VPy LSP

## Objetivo
Implementar análisis de flujo de datos para detectar variables no usadas y sugerir optimizaciones automáticas.

## Funcionalidades a Implementar

### 1. Análisis de Uso de Variables

#### 1.1 Detectar Variables No Usadas
**Comportamiento**: Variable declarada pero nunca leída
```python
player_x = 0  # ⚠️ Warning: Variable 'player_x' is declared but never used
player_y = 0
PRINT_TEXT(0, 0, "Hello")  # No usa player_x
```

**Implementación**:
- Crear struct `VariableUsage` con campos:
  - `declared: bool`
  - `initialized: bool`
  - `read_count: usize`
  - `write_count: usize`
  - `declaration_range: Range`
- Recorrer AST y trackear:
  - Declaraciones: `let x = 0` → declared=true, initialized=true, write_count++
  - Lecturas: `y = x + 1` → read_count++
  - Escrituras: `x = 5` → write_count++
- Generar diagnostic si `declared && read_count == 0`

#### 1.2 Detectar Variables Escritas pero No Leídas
**Comportamiento**: Variable modificada pero valor nunca usado
```python
counter = 0
counter = counter + 1  # ⚠️ Warning: Value assigned to 'counter' is never read
# (No hay lectura después de la última escritura)
```

**Implementación**:
- Detectar última escritura antes de salir del scope
- Si no hay lecturas después → diagnostic

#### 1.3 Detectar Variables Declaradas pero No Inicializadas
**Comportamiento**: Variable declarada sin valor inicial
```python
score  # ⚠️ Warning: Variable 'score' is declared but not initialized
if condition:
    score = 10
```

**Implementación**:
- Detectar `Item::Variable` sin `Expr` de inicialización
- Generar diagnostic con sugerencia

### 2. Sugerencias de Optimización (Code Actions)

#### 2.1 Sugerir Conversión a `const`
**Comportamiento**: Variable inicializada pero nunca modificada
```python
num_locations = 17  # 💡 Quick Fix: Convert to 'const' to save RAM
```

**Implementación**:
- En `VariableUsage`, detectar: `initialized && write_count == 1` (solo inicialización)
- Crear `CodeAction` de tipo `QuickFix`:
  ```rust
  CodeAction {
      title: "Convert to 'const' (saves 2 bytes RAM)".to_string(),
      kind: Some(CodeActionKind::QUICKFIX),
      edit: Some(WorkspaceEdit {
          changes: Some(HashMap::from([
              (uri.clone(), vec![TextEdit {
                  range: declaration_range,
                  new_text: format!("const {} = {}", var_name, value)
              }])
          ])),
          ..Default::default()
      }),
      ..Default::default()
  }
  ```

#### 2.2 Sugerir Eliminación de Variable No Usada
**Comportamiento**: Quick fix para eliminar variable completa
```python
unused_var = 42  # 💡 Quick Fix: Remove unused variable
```

**Implementación**:
- CodeAction que elimina la línea completa
- Solo si `read_count == 0 && !is_exported`

### 3. Mejoras de Diagnósticos

#### 3.1 Severity Levels
- **Error**: Variable usada sin declarar (ya implementado)
- **Warning**: Variable declarada pero no usada
- **Info**: Variable podría ser `const`
- **Hint**: Mejoras opcionales

#### 3.2 Estilo VSCode
- Variables no usadas: texto gris/tachado (via semantic tokens)
- Underline amarillo: puede ser const
- Underline azul: sugerencia de mejora

## Implementación Técnica

### Archivo: `core/src/lsp.rs`

#### Paso 1: Agregar Struct de Análisis
```rust
#[derive(Debug, Default)]
struct VariableUsage {
    declared: bool,
    initialized: bool,
    read_count: usize,
    write_count: usize,
    declaration_range: Option<Range>,
    last_write_range: Option<Range>,
    is_const: bool,
}

#[derive(Debug, Default)]
struct UsageAnalysis {
    variables: HashMap<String, VariableUsage>,
}
```

#### Paso 2: Función de Análisis
```rust
fn analyze_variable_usage(module: &Module) -> UsageAnalysis {
    let mut analysis = UsageAnalysis::default();
    
    // Fase 1: Recolectar declaraciones
    for item in &module.items {
        match item {
            Item::Variable(name, init) => {
                analysis.variables.entry(name.clone())
                    .or_default()
                    .declared = true;
                    .initialized = init.is_some();
                    .write_count = 1;
            },
            Item::Const(name, _) => {
                analysis.variables.entry(name.clone())
                    .or_default()
                    .is_const = true;
            },
            _ => {}
        }
    }
    
    // Fase 2: Analizar lecturas/escrituras en funciones
    for item in &module.items {
        if let Item::Function(_, _, stmts) = item {
            analyze_statements(stmts, &mut analysis);
        }
    }
    
    analysis
}

fn analyze_statements(stmts: &[Stmt], analysis: &mut UsageAnalysis) {
    for stmt in stmts {
        match stmt {
            Stmt::Assign(name, expr) => {
                // Escritura
                if let Some(usage) = analysis.variables.get_mut(name) {
                    usage.write_count += 1;
                }
                // Analizar lecturas en expresión
                analyze_expr(expr, analysis);
            },
            Stmt::If(cond, then_body, else_body) => {
                analyze_expr(cond, analysis);
                analyze_statements(then_body, analysis);
                if let Some(else_stmts) = else_body {
                    analyze_statements(else_stmts, analysis);
                }
            },
            // ... otros statements
        }
    }
}

fn analyze_expr(expr: &Expr, analysis: &mut UsageAnalysis) {
    match expr {
        Expr::Ident(name) => {
            // Lectura de variable
            if let Some(usage) = analysis.variables.get_mut(name) {
                usage.read_count += 1;
            }
        },
        Expr::BinOp(left, _, right) => {
            analyze_expr(left, analysis);
            analyze_expr(right, analysis);
        },
        // ... otras expresiones
    }
}
```

#### Paso 3: Generar Diagnostics
```rust
fn generate_usage_diagnostics(analysis: &UsageAnalysis, uri: &Url) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    for (name, usage) in &analysis.variables {
        // Caso 1: Declarada pero no usada
        if usage.declared && usage.read_count == 0 && !usage.is_const {
            if let Some(range) = &usage.declaration_range {
                diagnostics.push(Diagnostic {
                    range: *range,
                    severity: Some(DiagnosticSeverity::WARNING),
                    code: Some(NumberOrString::String("unused-variable".to_string())),
                    message: format!("Variable '{}' is declared but never used", name),
                    ..Default::default()
                });
            }
        }
        
        // Caso 2: Puede ser const
        if usage.initialized && usage.write_count == 1 && usage.read_count > 0 && !usage.is_const {
            if let Some(range) = &usage.declaration_range {
                diagnostics.push(Diagnostic {
                    range: *range,
                    severity: Some(DiagnosticSeverity::HINT),
                    code: Some(NumberOrString::String("suggest-const".to_string())),
                    message: format!("Variable '{}' never changes - consider 'const' to save RAM", name),
                    ..Default::default()
                });
            }
        }
    }
    
    diagnostics
}
```

#### Paso 4: Implementar Code Actions
```rust
#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn code_action(&self, params: CodeActionParams) -> LspResult<Option<CodeActionResponse>> {
        let uri = &params.text_document.uri;
        let docs = self.docs.lock().unwrap();
        let text = match docs.get(uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        
        let module = parse_with_filename(text, uri.path());
        let analysis = analyze_variable_usage(&module);
        
        let mut actions = Vec::new();
        
        for diagnostic in &params.context.diagnostics {
            if let Some(NumberOrString::String(code)) = &diagnostic.code {
                match code.as_str() {
                    "suggest-const" => {
                        // Extraer nombre de variable del mensaje
                        if let Some(var_name) = extract_var_name(&diagnostic.message) {
                            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                                title: format!("Convert '{}' to const", var_name),
                                kind: Some(CodeActionKind::QUICKFIX),
                                edit: Some(create_const_conversion_edit(uri, diagnostic.range, var_name)),
                                ..Default::default()
                            }));
                        }
                    },
                    "unused-variable" => {
                        if let Some(var_name) = extract_var_name(&diagnostic.message) {
                            actions.push(CodeActionOrCommand::CodeAction(CodeAction {
                                title: format!("Remove unused variable '{}'", var_name),
                                kind: Some(CodeActionKind::QUICKFIX),
                                edit: Some(create_remove_line_edit(uri, diagnostic.range)),
                                ..Default::default()
                            }));
                        }
                    },
                    _ => {}
                }
            }
        }
        
        Ok(Some(actions))
    }
}
```

### Archivo: `vscode-extension/src/extension.ts`

#### Habilitar Code Actions en Cliente
```typescript
const clientOptions: LanguageClientOptions = {
  documentSelector: [{ language: 'vpy' }],
  synchronize: {
    fileEvents: vscode.workspace.createFileSystemWatcher('**/*.vpy')
  },
  // Habilitar code actions
  middleware: {
    provideCodeActions: async (document, range, context, token, next) => {
      const actions = await next(document, range, context, token);
      return actions;
    }
  }
};
```

## Testing

### Test Cases

#### Test 1: Variable No Usada
```python
# Input:
unused_var = 42
player_x = 0
PRINT_TEXT(0, 0, "Hello")

# Expected Diagnostic:
# Line 1: Warning "Variable 'unused_var' is declared but never used"
# Quick Fix: "Remove unused variable 'unused_var'"
```

#### Test 2: Sugerir Const
```python
# Input:
num_levels = 17
current_level = 0

def loop():
    current_level = (current_level + 1) % num_levels

# Expected Diagnostic:
# Line 1: Hint "Variable 'num_levels' never changes - consider 'const'"
# Quick Fix: "Convert 'num_levels' to const"
```

#### Test 3: Variable Escrita pero No Leída
```python
# Input:
counter = 0
counter = counter + 1
# (No más uso de counter)

# Expected Diagnostic:
# Line 2: Warning "Value assigned to 'counter' is never read"
```

## Cronograma de Implementación

1. **Fase 1 (2-3 horas)**:
   - Implementar `VariableUsage` y `UsageAnalysis`
   - Análisis básico de declaraciones y lecturas
   - Test unitarios

2. **Fase 2 (2-3 horas)**:
   - Generar diagnostics para variables no usadas
   - Integrar con `textDocument/publishDiagnostics`
   - Testing en VSCode

3. **Fase 3 (3-4 horas)**:
   - Implementar code actions
   - Quick fixes para conversión a const
   - Quick fixes para eliminación de variables
   - Testing end-to-end

4. **Fase 4 (1-2 horas)**:
   - Refinamiento de UX (mensajes claros)
   - Optimización de performance
   - Documentación

**Total estimado**: 8-12 horas de desarrollo

## Notas de Implementación

- **Performance**: Análisis debe ser incremental (solo re-analizar función modificada)
- **Scope**: Considerar scopes de funciones (variables locales vs globales)
- **Edge cases**: Variables que se leen antes de escribir (error ya detectado)
- **Const validation**: Verificar que el valor inicial sea constante (no una expresión variable)

## Referencias

- [LSP Spec - Code Actions](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction)
- [LSP Spec - Diagnostics](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnostic)
- [VSCode API - Diagnostics](https://code.visualstudio.com/api/language-extensions/programmatic-language-features#provide-diagnostics)
