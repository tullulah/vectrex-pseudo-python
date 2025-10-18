# SUPER FIX: Completar Line Tracking Migration

## ESTADO ACTUAL
- ✅ AST actualizado: Todos los `Stmt` variants tienen `source_line: usize`
- ✅ Parser actualizado: Captura `source_line` en todos los statements
- ✅ Método helper `Stmt::source_line()` implementado
- ⏸️ codegen.rs: PARCIALMENTE actualizado (118 errores)
- ❌ backend/m6809.rs: NO actualizado (necesita llamar debug_tracker.set_line())

## ERRORES PRINCIPALES (de 118 total)

### 1. Patterns sin `source_line` (47 errores E0027)
**Solución**: Agregar `..` a todos los patterns de Stmt:
```rust
// ANTES:
Stmt::Assign { target, value }

// DESPUÉS:
Stmt::Assign { target, value, .. }
```

**Archivos**: codegen.rs, backend/m6809.rs, otros backends

**Script automatizable**: Sí
```python
# Buscar: Stmt::(\w+)\s*\{([^}]+)\}
# Verificar si contiene `..` o `source_line`
# Si no: agregar `, ..` antes del }
```

### 2. Tuple Variants (23 errores E0023)
**Problema**: `Stmt::Expr(e)` y `Stmt::Return(o)` ahora son `Stmt::Expr(e, source_line)` y `Stmt::Return(o, source_line)`

**Solución en PATTERNS**:
```rust
// ANTES:
Stmt::Expr(e) => ...
Stmt::Return(o) => ...

// DESPUÉS:
Stmt::Expr(e, _) => ...  // _ ignora source_line en patterns
Stmt::Return(o, _) => ...
```

**Solución en CONSTRUCCIONES**:
```rust
// ANTES:
Stmt::Expr(expr)
Stmt::Return(Some(val))

// DESPUÉS:
Stmt::Expr(expr, source_line_var)
Stmt::Return(Some(val), source_line_var)
```

### 3. Break/Continue ahora structs (20 errores E0533)
**Problema**: Antes eran unit variants, ahora son structs con campo

**Solución en PATTERNS**:
```rust
// ANTES:
Stmt::Break | Stmt::Continue

// DESPUÉS:
Stmt::Break { .. } | Stmt::Continue { .. }
```

**Solución en CONSTRUCCIONES**:
```rust
// ANTES:
Stmt::Break
Stmt::Continue

// DESPUÉS:
Stmt::Break { source_line: line_var }
Stmt::Continue { source_line: line_var }
```

### 4. Variable `s` no definida (11 errores E0425)
**Problema**: Script de reemplazo agregó `s.source_line()` donde no existe variable `s`

**Contextos**:
- **En `opt_stmt(s: &Stmt)`**: `s` SÍ existe, usar `s.source_line()`  
- **En `flatten_blocks(...)`**: `s` NO existe directamente, pero se itera sobre statements

**Solución**: En `flatten_blocks`, capturar source_line del statement antes de procesarlo:
```rust
// Dentro del match sobre el statement actual:
for s in stmts {
    let stmt_line = s.source_line(); // Capturar antes
    match s {
        Stmt::If { .. } => {
            // ... procesar
            out.push(Stmt::If { cond, body, elifs, else_body, source_line: stmt_line });
        }
        // ...
    }
}
```

### 5. Uso de `_` como valor (varios errores)
**Problema**: `e.clone(), _` - el `_` solo es válido en patterns del lado izquierdo

**Solución**:
```rust
// INCORRECTO:
Stmt::Expr(e.clone(), _)
Stmt::Return(o.clone(), _)

// CORRECTO:
Stmt::Expr(e.clone(), source_line)  // Usar variable real
```

### 6. CallInfo/IdentInfo usan `.line` no `.source_line` (3 errores)
**Solución**: ✅ YA ARREGLADO en último commit

## PLAN DE ACCIÓN SISTEMÁTICO

### Fase 1: Arreglar patterns (automatizable)
```python
import re

with open('core/src/codegen.rs', 'r') as f:
    content = f.read()

# Fix pattern matches faltantes
patterns_to_fix = [
    (r'Stmt::Break(?!\s*\{)', r'Stmt::Break { .. }'),
    (r'Stmt::Continue(?!\s*\{)', r'Stmt::Continue { .. }'),
    (r'Stmt::Expr\(([^,)]+)\)(?!\s*,)', r'Stmt::Expr(\1, _)'),  # Solo en patterns
    (r'Stmt::Return\(([^,)]+)\)(?!\s*,)', r'Stmt::Return(\1, _)'),
]

for old, new in patterns_to_fix:
    content = re.sub(old, new, content)

# Agregar .. a patterns sin él
def ensure_ellipsis(match):
    stmt_type = match.group(1)
    fields = match.group(2)
    if '..' in fields or 'source_line' in fields:
        return match.group(0)
    # Agregar .. antes del cierre
    return f'Stmt::{stmt_type} {{{fields}, .. }}'

content = re.sub(r'Stmt::(\w+)\s*\{([^}]+)\}', ensure_ellipsis, content)

with open('core/src/codegen.rs', 'w') as f:
    f.write(content)
```

### Fase 2: Arreglar construcciones de Stmt
**Manual**: Revisar cada lugar donde se construye un Stmt y agregar `source_line`

Ubicaciones típicas:
- `opt_stmt()` - ✅ Parcialmente hecho
- `flatten_blocks()` - ❌ Necesita source_line de statement original
- `cp_stmt()` - ❌ Necesita source_line de statement original

**Patrón**:
```rust
fn process_stmt(s: &Stmt) -> Stmt {
    let source_line = s.source_line(); // Capturar primero
    match s {
        Stmt::Assign { target, value, .. } => {
            Stmt::Assign {
                target: target.clone(),
                value: process_expr(value),
                source_line, // Usar capturado
            }
        }
        // ...
    }
}
```

### Fase 3: Actualizar backend/m6809.rs
**Crítico**: Aquí es donde se GENERA el código ASM

**Cambio necesario**:
```rust
fn compile_stmt(&mut self, stmt: &Stmt) {
    // NUEVO: Registrar línea fuente para debugging
    self.debug_tracker.set_line(stmt.source_line());
    
    match stmt {
        Stmt::Expr(call, _) => self.compile_expr_stmt(call),
        // ...resto igual
    }
}
```

**Ubicación**: `core/src/backend/m6809.rs` en función `compile_stmts()` o similar

### Fase 4: Actualizar otros backends
- `cortexm.rs` - si existe
- `arm.rs` - si existe  
Similar a m6809.rs

### Fase 5: Verificar y test
1. `cargo build` → Debe compilar sin errores
2. Compilar `test_debug_simple.vpy`
3. Verificar que `.pdb` tiene `lineMap` poblado:
```json
{
  "lineMap": {
    "7": "0xC810",
    "10": "0xC820",
    // etc.
  }
}
```

## DECISIÓN CRÍTICA: ENFOQUE

**Opción A: Continuar fix manual**
- Pros: Control total, aprendizaje profundo
- Contras: Tedioso, propenso a errores, ~100 cambios restantes

**Opción B: Script Python comprehensivo**
- Pros: Rápido, consistente, repetible
- Contras: Puede necesitar ajustes manuales después

**Opción C: Híbrido (RECOMENDADO)**
1. Script para patterns automáticos (Fase 1)
2. Manual para construcciones complejas (Fase 2)
3. Manual para backend integration (Fase 3-4)

## PRÓXIMO COMANDO
```bash
# Crear y ejecutar script de Fase 1
python fix_stmt_patterns_comprehensive.py

# Luego revisar manualmente lo que quede
cargo build 2>&1 | grep "error\[" | wc -l
```

## RESULTADO ESPERADO
Al terminar:
- ✅ 0 errores de compilación
- ✅ .pdb con lineMap completo
- ✅ Breakpoints funcionando en IDE
- ✅ Debug stepping operacional
