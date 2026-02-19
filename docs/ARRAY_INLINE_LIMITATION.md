# Array Inline Limitation (2026-01-11)

## Problema Identificado

Los arrays literales inline con elementos variables locales NO se pueden almacenar en ROM.

### Ejemplo Problemático

```python
def get_input():
    x = J1_X()
    y = J1_Y()
    result = [x, y]  # ❌ PROBLEMA: x,y son variables locales
    return result
```

### Por Qué Falla

1. **Arrays en ROM**: El sistema actual emite arrays literales en ROM (sección DATA)
2. **Variables locales**: `x` e `y` son variables locales (están en el stack, no tienen `VAR_*`)
3. **Conflicto**: No se puede referenciar `VAR_X` en ROM porque no existe como símbolo global

### Código Generado (Incorrecto)

```asm
; === INLINE ARRAY LITERALS ===
ARRAY_GET_INPUT_0:
    FDB VAR_X   ; ❌ ERROR: VAR_X no existe (es local)
    FDB VAR_Y   ; ❌ ERROR: VAR_Y no existe (es local)
```

## Solución Actual

### Workaround: Usar Arrays Globales

```python
# Global array to store input
input_result = [0, 0]

def get_input():
    x = J1_X()
    y = J1_Y()
    input_result[0] = x  # ✅ Asignación a array global
    input_result[1] = y
    return 0
```

### Código Generado (Correcto)

```asm
; Global array in ROM
ARRAY_INPUT_RESULT:
    FDB 0
    FDB 0

; Runtime assignment (in function)
get_input:
    JSR J1X_BUILTIN
    STD RESULT
    ; Load array base address
    LDD #VAR_INPUT_RESULT_DATA
    ; ... indexing code ...
    ; Store x to array[0]
```

## Solución Ideal (No Implementada)

Para soportar arrays inline con variables locales se necesitaría:

### 1. Runtime Array Allocation

```python
result = [x, y]  # Necesita alocar espacio en stack/RAM
```

**Compilación requerida**:
```asm
; Allocate 2 words on stack
LEAS -4,S       ; Reserve 4 bytes (2 elements × 2 bytes)

; Store x to array[0]
LDD VAR_X_LOCAL ; Load local x (from stack offset)
STD 0,S         ; Store to array[0]

; Store y to array[1]
LDD VAR_Y_LOCAL ; Load local y
STD 2,S         ; Store to array[1]

; Return pointer to array
TFR S,D         ; D = stack pointer (array address)
STD RESULT
```

### 2. Problemas de Implementación

- **Lifetime**: ¿Cuándo se libera el array? (al salir de función, stack corrupto)
- **Retorno**: Si se retorna el array, el puntero apunta a stack frame dealocado
- **Complejidad**: Necesita heap allocation o static allocation por función

### 3. Alternativas Evaluadas

**A) Stack-allocated arrays (temporal)**
- ✅ Rápido
- ❌ No se puede retornar (lifetime issues)
- ❌ Complejo para nested calls

**B) Static allocation per function**
```asm
GET_INPUT_ARRAY_BUFFER:
    FDB 0, 0  ; Static buffer (no thread-safe)
```
- ✅ Simple
- ❌ No reentrant
- ❌ Limitado a 1 array por función

**C) Heap allocation**
- ✅ Flexible
- ❌ Necesita malloc/free implementation
- ❌ Overhead muy alto para Vectrex

## Recomendación

**NO implementar arrays inline con variables locales por ahora.**

### Pattern Recomendado

1. **Arrays constantes**: ✅ Usar `const`
   ```python
   const lookup_table = [10, 20, 30, 40]
   ```

2. **Arrays globales mutables**: ✅ Usar global
   ```python
   buffer = [0, 0, 0, 0]
   def process():
       buffer[0] = compute()
   ```

3. **Retornar múltiples valores**: ✅ Usar global o parámetros out
   ```python
   result_x = 0
   result_y = 0
   def get_input():
       result_x = J1_X()
       result_y = J1_Y()
   ```

## Estado del Código

### Implementación Actual (2026-01-11)

✅ **Collector implementado**: `collect_inline_array_literals()`
- Traversa todo el AST
- Genera labels únicos por función: `ARRAY_FUNCNAME_0`
- Emite datos al final del módulo

❌ **Limitación**: Solo funciona con arrays de constantes numéricas

### Archivos Modificados

- `core/src/codegen.rs`: Agregado `inline_arrays: Vec<(String, Vec<Expr>)>`
- `core/src/backend/m6809/collectors.rs`: `collect_inline_array_literals()`
- `core/src/backend/m6809/expressions.rs`: Matching de inline arrays
- `core/src/backend/m6809/mod.rs`: Emisión de INLINE ARRAY LITERALS section

### Tests

- `examples/multi-module/src/input.vpy`: Modificado para usar array global
- `examples/multi-module/src/main.vpy`: Modificado para acceder vía `input.input_result[0]`

## Próximos Pasos

1. **Validación en compilación**: Detectar arrays inline con variables y mostrar error descriptivo
2. **Documentar en LSP**: Agregar hint en editor cuando se usa pattern incorrecto
3. **Ejemplo en GUIDE**: Documentar pattern correcto en guía de usuario
4. **Phase 6.3 completion**: Resolver issue multi-module (unifier no está corriendo)

---

**Fecha**: 2026-01-11  
**Estado**: Documentado - No implementar por complejidad  
**Alternativa**: Usar arrays globales + assignment statements
