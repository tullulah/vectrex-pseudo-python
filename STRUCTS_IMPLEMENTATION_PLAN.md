# Plan de Implementación: Structs Simples en VPy

**Fecha de inicio**: 19 de diciembre de 2025  
**Rama**: `feature/structs-simple`  
**Objetivo**: Implementar structs simples (Nivel 1) sin métodos ni herencia  
**Tiempo estimado**: 1-2 semanas

---

## 📋 Resumen Ejecutivo

Implementar **structs simples** en VPy para permitir agrupación de datos relacionados. Esta es la base fundamental para cualquier OOP futuro, pero con complejidad contenida.

**Scope**:
- ✅ Definición de structs con campos tipados
- ✅ Instanciación de structs
- ✅ Acceso a campos (read/write)
- ✅ Structs anidados
- ❌ NO métodos (usar funciones normales)
- ❌ NO herencia
- ❌ NO polimorfismo

---

## 🎯 Sintaxis Objetivo

```python
# Definición
struct Point:
    x: int
    y: int

struct Player:
    pos: Point      # Struct anidado
    lives: int
    score: int

# Uso
def main():
    p = Player()
    p.pos.x = 100
    p.pos.y = 50
    p.lives = 3
    p.score = 0

def loop():
    WAIT_RECAL()
    p.score = p.score + 10
    DRAW_DOT(p.pos.x, p.pos.y)
```

---

## 🏗️ Arquitectura de Implementación

### Phase 1: Parser & AST (3-4 días)

#### 1.1 Lexer Extensions
**Archivo**: `core/src/lexer.rs`

```rust
// Añadir nuevo token
pub enum Token {
    // ... tokens existentes
    Struct,      // palabra clave "struct"
    // ... resto
}

// Actualizar keyword recognition
fn is_keyword(s: &str) -> bool {
    matches!(s,
        "def" | "if" | "else" | "while" | "for" | "return" |
        "struct" | // <- NUEVO
        // ... resto
    )
}
```

**Tests necesarios**:
- [ ] `test_lex_struct_keyword()` - tokeniza "struct"
- [ ] `test_lex_struct_definition()` - parsea struct completo

---

#### 1.2 AST Extensions
**Archivo**: `core/src/ast.rs`

```rust
// Nuevo variant para Item
pub enum Item {
    Function(Function),
    StructDef(StructDef),  // <- NUEVO
    // ... resto
}

// Nueva estructura para definición de struct
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
    pub source_line: usize,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub type_annotation: Option<String>,  // "int", "Point", etc.
    pub source_line: usize,
}

// Nuevas expresiones
pub enum Expr {
    // ... expresiones existentes
    StructInit {
        struct_name: String,
        source_line: usize,
        col: usize,
    },
    FieldAccess {
        target: Box<Expr>,
        field: String,
        source_line: usize,
        col: usize,
    },
    // ... resto
}

// Nuevos assignment targets
pub enum AssignTarget {
    Ident(IdentInfo),
    Index { target: Box<Expr>, index: Box<Expr>, source_line: usize, col: usize },
    FieldAccess {  // <- NUEVO
        target: Box<Expr>,
        field: String,
        source_line: usize,
        col: usize,
    },
}
```

**Tests necesarios**:
- [ ] `test_ast_struct_def()` - crea StructDef válido
- [ ] `test_ast_field_access()` - crea FieldAccess válido
- [ ] `test_ast_struct_init()` - crea StructInit válido

---

#### 1.3 Parser Implementation
**Archivo**: `core/src/parser.rs`

```rust
// Parsear struct definition
fn parse_struct_def(&mut self) -> Result<StructDef, String> {
    // struct Point:
    self.expect(Token::Struct)?;
    let name = self.expect_ident()?;
    self.expect(Token::Colon)?;
    self.expect(Token::Newline)?;
    self.expect(Token::Indent)?;
    
    let mut fields = Vec::new();
    while !matches!(self.peek(), Some(Token::Dedent)) {
        // x: int
        let field_name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let type_ann = Some(self.expect_ident()?);
        self.expect(Token::Newline)?;
        
        fields.push(FieldDef {
            name: field_name,
            type_annotation: type_ann,
            source_line: self.line,
        });
    }
    
    self.expect(Token::Dedent)?;
    
    Ok(StructDef { name, fields, source_line: self.line })
}

// Actualizar parse_primary para field access
fn parse_primary(&mut self) -> Result<Expr, String> {
    let mut expr = /* ... parseo existente ... */;
    
    // Manejar field access: obj.field
    while matches!(self.peek(), Some(Token::Dot)) {
        self.advance(); // consume '.'
        let field = self.expect_ident()?;
        expr = Expr::FieldAccess {
            target: Box::new(expr),
            field,
            source_line: self.line,
            col: self.col,
        };
    }
    
    Ok(expr)
}

// Actualizar parse_call para struct initialization
fn parse_call(&mut self, callee: Expr) -> Result<Expr, String> {
    if let Expr::Ident(ref ident) = callee {
        // Verificar si es un struct conocido
        if self.known_structs.contains(&ident.name) {
            self.expect(Token::LParen)?;
            self.expect(Token::RParen)?;
            return Ok(Expr::StructInit {
                struct_name: ident.name.clone(),
                source_line: ident.source_line,
                col: ident.col,
            });
        }
    }
    
    // ... resto del parseo normal de llamadas
}
```

**Tests necesarios**:
- [ ] `test_parse_struct_simple()` - parsea struct básico
- [ ] `test_parse_struct_nested()` - parsea struct con campos struct
- [ ] `test_parse_field_access()` - parsea `obj.field`
- [ ] `test_parse_field_chain()` - parsea `obj.field1.field2`
- [ ] `test_parse_struct_init()` - parsea `Player()`

---

### Phase 2: Semantic Analysis (2-3 días)

#### 2.1 Type System
**Archivo**: `core/src/codegen.rs`

```rust
// Extender CodegenOptions
pub struct CodegenOptions {
    // ... campos existentes
    pub struct_defs: HashMap<String, StructDef>,  // <- NUEVO
    // ... resto
}

// Nueva función de análisis
pub fn collect_struct_defs(module: &Module) -> HashMap<String, StructDef> {
    let mut structs = HashMap::new();
    
    for item in &module.items {
        if let Item::StructDef(sdef) = item {
            if structs.contains_key(&sdef.name) {
                panic!("Duplicate struct definition: {}", sdef.name);
            }
            structs.insert(sdef.name.clone(), sdef.clone());
        }
    }
    
    structs
}
```

---

#### 2.2 Field Offset Calculation
**Archivo**: `core/src/backend/m6809/structs.rs` (NUEVO)

```rust
use std::collections::HashMap;
use crate::ast::StructDef;

#[derive(Debug, Clone)]
pub struct StructLayout {
    pub name: String,
    pub total_size: usize,      // bytes totales
    pub field_offsets: HashMap<String, usize>,  // field -> offset
}

/// Calcula layout de memoria para un struct
pub fn compute_struct_layout(
    sdef: &StructDef,
    struct_defs: &HashMap<String, StructDef>,
) -> StructLayout {
    let mut field_offsets = HashMap::new();
    let mut current_offset = 0;
    
    for field in &sdef.fields {
        field_offsets.insert(field.name.clone(), current_offset);
        
        // Calcular tamaño del campo
        let field_size = match field.type_annotation.as_deref() {
            Some("int") => 2,  // int = 2 bytes (16-bit)
            Some(type_name) => {
                // Es un struct anidado?
                if let Some(nested_struct) = struct_defs.get(type_name) {
                    let nested_layout = compute_struct_layout(nested_struct, struct_defs);
                    nested_layout.total_size
                } else {
                    panic!("Unknown type: {}", type_name);
                }
            }
            None => 2,  // default: 2 bytes
        };
        
        current_offset += field_size;
    }
    
    StructLayout {
        name: sdef.name.clone(),
        total_size: current_offset,
        field_offsets,
    }
}

/// Obtiene el offset de un campo (con soporte para acceso anidado)
pub fn get_field_offset(
    struct_name: &str,
    field_path: &[String],
    struct_layouts: &HashMap<String, StructLayout>,
    struct_defs: &HashMap<String, StructDef>,
) -> Result<usize, String> {
    let mut current_struct = struct_name;
    let mut total_offset = 0;
    
    for (i, field_name) in field_path.iter().enumerate() {
        let layout = struct_layouts.get(current_struct)
            .ok_or_else(|| format!("Unknown struct: {}", current_struct))?;
        
        let field_offset = layout.field_offsets.get(field_name)
            .ok_or_else(|| format!("Unknown field: {}.{}", current_struct, field_name))?;
        
        total_offset += field_offset;
        
        // Si no es el último campo, necesitamos el tipo del campo para continuar
        if i < field_path.len() - 1 {
            let sdef = struct_defs.get(current_struct).unwrap();
            let field_def = sdef.fields.iter()
                .find(|f| f.name == *field_name)
                .ok_or_else(|| format!("Field not found: {}", field_name))?;
            
            current_struct = field_def.type_annotation.as_deref()
                .ok_or_else(|| format!("Field {} has no type annotation", field_name))?;
        }
    }
    
    Ok(total_offset)
}
```

**Tests necesarios**:
- [ ] `test_layout_simple()` - layout para struct simple (2 fields)
- [ ] `test_layout_nested()` - layout para struct anidado
- [ ] `test_field_offset_simple()` - offset de campo directo
- [ ] `test_field_offset_chain()` - offset de campo anidado

---

### Phase 3: Code Generation (4-5 días)

#### 3.1 Memory Allocator
**Archivo**: `core/src/backend/m6809/builtins.rs`

```rust
// Añadir al final del archivo

pub fn emit_struct_allocator(out: &mut String) {
    out.push_str("\n; === Struct Memory Allocator ===\n");
    out.push_str("; Simple stack-based allocator\n");
    out.push_str("; Input: D = size in bytes\n");
    out.push_str("; Output: D = pointer to allocated memory\n");
    out.push_str("_ALLOC_STRUCT:\n");
    out.push_str("    PSHS D          ; save size\n");
    out.push_str("    TFR S,D         ; D = stack pointer\n");
    out.push_str("    PULS X          ; X = size, adjust stack\n");
    out.push_str("    PSHS D          ; reserve space on stack\n");
    out.push_str("    LEAS -2,S       ; adjust for size value\n");
    out.push_str("    LEAS D,S        ; allocate requested bytes\n");
    out.push_str("    TFR S,D         ; D = pointer to allocated memory\n");
    out.push_str("    RTS\n");
}
```

---

#### 3.2 Struct Initialization Codegen
**Archivo**: `core/src/backend/m6809/expressions.rs`

```rust
// Añadir case para Expr::StructInit
Expr::StructInit { struct_name, source_line, col } => {
    // 1. Obtener tamaño del struct
    let layout = opts.struct_layouts.get(struct_name)
        .ok_or_else(|| format!("Unknown struct: {}", struct_name))?;
    
    // 2. Alocar memoria
    out.push_str(&format!("; Allocate struct {}\n", struct_name));
    out.push_str(&format!("    LDD #{}\n", layout.total_size));
    out.push_str("    JSR _ALLOC_STRUCT\n");
    
    // 3. Inicializar campos a cero
    if layout.total_size > 0 {
        out.push_str("    TFR D,X         ; X = base pointer\n");
        out.push_str(&format!("    LDD #0\n"));
        for offset in (0..layout.total_size).step_by(2) {
            out.push_str(&format!("    STD {},X\n", offset));
        }
        out.push_str("    TFR X,D         ; restore pointer to D\n");
    }
    
    out.push_str("    STD RESULT\n");
}

// Añadir case para Expr::FieldAccess
Expr::FieldAccess { target, field, source_line, col } => {
    // 1. Evaluar expresión target (debe retornar puntero al struct)
    emit_expr_depth(target, out, fctx, string_map, opts, depth + 1);
    
    // 2. Determinar tipo del target
    let target_type = infer_expr_type(target, fctx, opts)?;
    
    // 3. Calcular offset del campo
    let field_offset = get_single_field_offset(&target_type, field, opts)?;
    
    // 4. Cargar valor del campo
    out.push_str("    LDD RESULT\n");
    out.push_str("    TFR D,X         ; X = struct pointer\n");
    out.push_str(&format!("    LDD {},X        ; load field '{}'\n", field_offset, field));
    out.push_str("    STD RESULT\n");
}

// Helper para inferir tipo de expresión
fn infer_expr_type(expr: &Expr, fctx: &FunctionContext, opts: &CodegenOptions) -> Result<String, String> {
    match expr {
        Expr::StructInit { struct_name, .. } => Ok(struct_name.clone()),
        Expr::Ident(ident) => {
            // Buscar en locals o globals
            if let Some(local_idx) = fctx.locals.iter().position(|l| l == &ident.name) {
                // TODO: necesitamos guardar tipos de locals
                Err("Cannot infer type of local variable yet".to_string())
            } else {
                Err(format!("Unknown variable: {}", ident.name))
            }
        }
        Expr::FieldAccess { target, field, .. } => {
            let target_type = infer_expr_type(target, fctx, opts)?;
            let sdef = opts.struct_defs.get(&target_type)
                .ok_or_else(|| format!("Not a struct: {}", target_type))?;
            let field_def = sdef.fields.iter()
                .find(|f| &f.name == field)
                .ok_or_else(|| format!("Field not found: {}.{}", target_type, field))?;
            Ok(field_def.type_annotation.clone().unwrap_or("int".to_string()))
        }
        _ => Err("Cannot infer type of complex expression".to_string())
    }
}
```

---

#### 3.3 Field Assignment Codegen
**Archivo**: `core/src/backend/m6809/statements.rs`

```rust
// Actualizar emit_assignment para manejar AssignTarget::FieldAccess
Stmt::Assignment { target, value, .. } => {
    match target {
        // ... casos existentes (Ident, Index)
        
        AssignTarget::FieldAccess { target: obj_expr, field, .. } => {
            // 1. Evaluar valor a asignar
            emit_expr_depth(value, out, fctx, string_map, opts, depth + 1);
            out.push_str("    LDD RESULT\n");
            out.push_str("    PSHS D          ; save value\n");
            
            // 2. Evaluar target object
            emit_expr_depth(obj_expr, out, fctx, string_map, opts, depth + 1);
            
            // 3. Determinar tipo y calcular offset
            let target_type = infer_expr_type(obj_expr, fctx, opts)?;
            let field_offset = get_single_field_offset(&target_type, field, opts)?;
            
            // 4. Escribir valor en campo
            out.push_str("    LDD RESULT\n");
            out.push_str("    TFR D,X         ; X = struct pointer\n");
            out.push_str("    PULS D          ; restore value\n");
            out.push_str(&format!("    STD {},X        ; store to field '{}'\n", field_offset, field));
        }
    }
}
```

---

### Phase 4: Testing & Validation (2-3 días)

#### 4.1 Unit Tests
**Archivo**: `core/tests/structs.rs` (NUEVO)

```rust
#[test]
fn test_simple_struct_allocation() {
    let source = r#"
struct Point:
    x: int
    y: int

def main():
    p = Point()
"#;
    // Verificar que compila sin errores
    // Verificar ASM genera _ALLOC_STRUCT call
    // Verificar size correcto (4 bytes)
}

#[test]
fn test_field_write_read() {
    let source = r#"
struct Point:
    x: int
    y: int

def main():
    p = Point()
    p.x = 100
    p.y = 50
    result = p.x + p.y
"#;
    // Verificar que compila
    // Verificar offsets correctos (0, 2)
    // Verificar STD/LDD con offsets
}

#[test]
fn test_nested_struct() {
    let source = r#"
struct Point:
    x: int
    y: int

struct Player:
    pos: Point
    lives: int

def main():
    p = Player()
    p.pos.x = 10
    p.lives = 3
"#;
    // Verificar layout correcto
    // Player: [pos(4 bytes), lives(2 bytes)] = 6 bytes total
    // p.pos.x offset = 0
    // p.lives offset = 4
}
```

---

#### 4.2 Integration Tests
**Archivo**: `examples/test_structs_game.vpy`

```python
META TITLE = "Structs Demo"

struct Bullet:
    x: int
    y: int
    vx: int
    vy: int
    active: int

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Create bullet
    b = Bullet()
    b.x = 0
    b.y = 0
    b.vx = 5
    b.vy = 3
    b.active = 1
    
    # Update position
    if b.active:
        b.x = b.x + b.vx
        b.y = b.y + b.vy
        
        # Draw bullet
        if b.x > -100 and b.x < 100:
            if b.y > -100 and b.y < 100:
                DRAW_DOT(b.x, b.y)
```

**Tests**:
- [ ] Compila sin errores
- [ ] Ejecuta en emulador
- [ ] Bullet se mueve correctamente
- [ ] No memory leaks (stack crece/decrece correctamente)

---

#### 4.3 Error Cases
**Archivo**: `core/tests/structs_errors.rs` (NUEVO)

```rust
#[test]
#[should_panic(expected = "Duplicate struct definition")]
fn test_duplicate_struct() {
    let source = r#"
struct Point:
    x: int
    y: int

struct Point:
    z: int
"#;
    // Debe fallar en semantic analysis
}

#[test]
#[should_panic(expected = "Unknown field")]
fn test_unknown_field() {
    let source = r#"
struct Point:
    x: int

def main():
    p = Point()
    p.z = 10  # ERROR: field 'z' no existe
"#;
}

#[test]
#[should_panic(expected = "Unknown struct")]
fn test_unknown_struct() {
    let source = r#"
def main():
    p = Player()  # ERROR: struct 'Player' no definido
"#;
}
```

---

### Phase 5: Documentation & Polish (1-2 días)

#### 5.1 Documentation Updates

**Archivos a actualizar**:

1. **PYTHON_VS_VPY.md**:
```markdown
### 7. Clases y OOP

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **struct** | N/A | ✅ | - | Structs simples (sin métodos) |
| **class** | `class Foo: ...` | ⚠️ | 🟡 MEDIA | Solo structs por ahora |
| **field access** | `obj.field` | ✅ | - | Read/write de campos |
| **self** | `self.x` | ❌ | 🟢 BAJA | Requiere métodos |
| **Herencia** | `class B(A): ...` | ❌ | 🟢 BAJA | Muy complejo |
```

2. **ide/frontend/src/services/contexts/docs/vpy-language.md**:
```markdown
## Structs (NEW!)

VPy ahora soporta structs simples para agrupar datos relacionados:

\`\`\`python
struct Point:
    x: int
    y: int

struct Player:
    pos: Point
    lives: int
    score: int

def main():
    p = Player()
    p.pos.x = 100
    p.pos.y = 50
    p.lives = 3
\`\`\`

**Limitaciones**:
- No soporta métodos (usar funciones normales)
- No soporta herencia
- No soporta constructores personalizados
- Campos siempre se inicializan a cero

**Buenas prácticas**:
- Usar structs para datos relacionados (Player, Enemy, Bullet)
- Preferir structs sobre múltiples variables globales
- Mantener structs pequeños (<8 campos) por limitaciones de RAM
\`\`\`
```

3. **MANUAL.md** - Añadir sección completa de structs con ejemplos

---

#### 5.2 LSP Support (opcional, pero recomendado)

**Archivo**: `core/src/lsp.rs`

```rust
// Añadir autocompletion para campos de structs
pub fn complete_field_access(
    struct_name: &str,
    struct_defs: &HashMap<String, StructDef>,
) -> Vec<CompletionItem> {
    if let Some(sdef) = struct_defs.get(struct_name) {
        sdef.fields.iter().map(|field| {
            CompletionItem {
                label: field.name.clone(),
                kind: CompletionItemKind::Field,
                detail: field.type_annotation.clone(),
                documentation: None,
            }
        }).collect()
    } else {
        vec![]
    }
}

// Añadir hover info para structs
pub fn hover_struct_definition(
    struct_name: &str,
    struct_defs: &HashMap<String, StructDef>,
) -> Option<String> {
    struct_defs.get(struct_name).map(|sdef| {
        let mut info = format!("struct {}\n", sdef.name);
        for field in &sdef.fields {
            info.push_str(&format!("  {}: {}\n", 
                field.name,
                field.type_annotation.as_deref().unwrap_or("int")
            ));
        }
        info
    })
}
```

---

## 📊 Cronograma Detallado

| Fase | Días | Tareas | Entregables |
|------|------|--------|-------------|
| **Phase 1** | 3-4 | Parser & AST | Token `struct`, AST nodes, parse tests |
| **Phase 2** | 2-3 | Semantic analysis | Type checking, layout computation |
| **Phase 3** | 4-5 | Code generation | Allocator, field access ASM, assignments |
| **Phase 4** | 2-3 | Testing | Unit tests, integration tests, error cases |
| **Phase 5** | 1-2 | Documentation | Docs, examples, LSP support |
| **TOTAL** | **12-17 días** | | **Structs simples funcionales** |

---

## ✅ Checklist de Completion

### Parser & AST
- [ ] Token `struct` en lexer
- [ ] `StructDef` AST node
- [ ] `FieldAccess` expression
- [ ] `StructInit` expression
- [ ] `FieldAccess` assignment target
- [ ] Tests de parsing

### Semantic Analysis
- [ ] `collect_struct_defs()` function
- [ ] `compute_struct_layout()` function
- [ ] `get_field_offset()` function
- [ ] Type inference para struct fields
- [ ] Validación de campos existentes
- [ ] Validación de structs definidos

### Code Generation
- [ ] `_ALLOC_STRUCT` allocator
- [ ] Codegen para `StructInit`
- [ ] Codegen para `FieldAccess` (read)
- [ ] Codegen para `FieldAccess` (write)
- [ ] Soporte para structs anidados
- [ ] Inicialización a cero de campos

### Testing
- [ ] 10+ unit tests (structs.rs)
- [ ] 5+ integration tests (examples/)
- [ ] 5+ error case tests
- [ ] Test de structs anidados
- [ ] Test de memory allocation

### Documentation
- [ ] PYTHON_VS_VPY.md actualizado
- [ ] vpy-language.md actualizado
- [ ] MANUAL.md sección structs
- [ ] Ejemplos en examples/
- [ ] LSP autocompletion (opcional)

---

## 🚀 Getting Started

```bash
# Verificar que estamos en la rama correcta
git branch  # Debe mostrar: * feature/structs-simple

# Primer paso: Añadir token struct al lexer
# Editar: core/src/lexer.rs

# Segundo paso: Extender AST
# Editar: core/src/ast.rs

# Tercer paso: Implementar parser
# Editar: core/src/parser.rs

# Ejecutar tests continuamente
cargo test --lib

# Test específico de structs (cuando exista)
cargo test test_struct
```

---

## 🎯 Objetivos de Calidad

- **Cobertura de tests**: >80% de código nuevo
- **Documentación**: 100% de funciones públicas documentadas
- **Ejemplos**: Mínimo 3 ejemplos funcionales en `examples/`
- **Error messages**: Mensajes claros y útiles para usuarios
- **Performance**: Overhead <10% vs código sin structs

---

## 📝 Notas Importantes

### Limitaciones Conocidas

1. **Allocator simple**: Stack-based, no hay free() explícito
   - Structs viven en stack frame de función
   - Se liberan automáticamente al salir de función
   - No soporta structs globales (por ahora)

2. **Type inference limitado**:
   - Solo funciona para casos simples
   - Puede requerir anotaciones explícitas
   - Mejorará en iteraciones futuras

3. **Sin métodos**:
   - Usar funciones normales: `update_player(p)`
   - Struct es primer parámetro
   - Convención de naming: `verb_structname()`

### Decisiones de Diseño

**¿Por qué stack allocation?**
- Más simple de implementar
- Automáticamente gestiona memoria
- Compatible con 1KB RAM del Vectrex
- Suficiente para la mayoría de casos de uso

**¿Por qué no métodos?**
- Reduce complejidad significativamente
- Funciones normales son suficientes
- Permite implementación más rápida
- Métodos pueden añadirse después

**¿Por qué no herencia?**
- Complejidad muy alta
- Overhead de memoria significativo
- No necesario para juegos Vectrex
- Composition over inheritance

---

## 🔄 Future Work (Post-Structs)

Una vez implementados structs simples, posibles extensiones:

1. **Constructores personalizados** (1 semana):
   ```python
   struct Point:
       x: int
       y: int
       
       def __init__(x: int, y: int):
           self.x = x
           self.y = y
   ```

2. **Métodos** (2 semanas):
   ```python
   struct Point:
       x: int
       y: int
       
       def distance(self):
           return ABS(self.x) + ABS(self.y)
   ```

3. **Structs globales** (1 semana):
   ```python
   g_player = Player()  # Allocado en sección DATA
   ```

4. **Arrays de structs** (1 semana):
   ```python
   bullets = [Bullet()] * 10
   ```

---

**Autor**: Daniel Ferrer  
**Última actualización**: 19 de diciembre de 2025  
**Status**: 🟢 Ready to start
