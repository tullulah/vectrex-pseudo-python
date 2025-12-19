# Python vs VPy Language Feature Comparison

**Objetivo**: Documentar qué funcionalidades del lenguaje Python están implementadas en VPy y cuáles faltan.

**Nota**: Este documento se enfoca en características del LENGUAJE (sintaxis, control flow, tipos), NO en bibliotecas Vectrex-específicas.

---

## ✅ IMPLEMENTADO

### 1. Control Flow

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **if/elif/else** | `if x: ... elif y: ... else: ...` | ✅ | Completo |
| **while** | `while cond: ...` | ✅ | Con break/continue |
| **for range** | `for i in range(start, end, step):` | ✅ | Sintaxis: `for i = start to end step step:` |
| **break** | `break` | ✅ | Sale de loop |
| **continue** | `continue` | ✅ | Siguiente iteración |
| **switch/match** | `match x: case 1: ...` (Python 3.10+) | ✅ | `switch expr: case 1: ... default: ...` |
| **return** | `return value` | ✅ | Con/sin valor |

### 2. Variables y Asignación

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Globales** | `x = 10` (top-level) | ✅ | `var x = 10` |
| **Locales** | `x = 10` (en función) | ✅ | `let x = 10` |
| **Constantes** | No nativas | ✅ | `const X = 10` |
| **Asignación simple** | `x = expr` | ✅ | `x = expr` |
| **Asignación compuesta** | `x += 5`, `x -= 3`, etc | ✅ | `x += 5`, `x -= 3`, `x *= 2`, etc |

### 3. Operadores Aritméticos

| Operador | Python | VPy | Notas |
|----------|--------|-----|-------|
| **Suma** | `+` | ✅ | Suma entera 16-bit |
| **Resta** | `-` | ✅ | Resta entera 16-bit |
| **Multiplicación** | `*` | ✅ | Mul 16-bit |
| **División** | `/` | ✅ | División entera (trunca) |
| **División entera** | `//` | ✅ | Floor division |
| **Módulo** | `%` | ✅ | Resto |
| **Potencia** | `**` | ❌ | **NO implementado** |
| **Negación unaria** | `-x` | ✅ | `-expr` |

### 4. Operadores Bitwise

| Operador | Python | VPy | Notas |
|----------|--------|-----|-------|
| **AND** | `&` | ✅ | Bitwise AND |
| **OR** | `\|` | ✅ | Bitwise OR |
| **XOR** | `^` | ✅ | Bitwise XOR |
| **NOT** | `~` | ✅ | Bitwise NOT (complemento) |
| **Shift left** | `<<` | ✅ | Shift izquierda |
| **Shift right** | `>>` | ✅ | Shift derecha |

### 5. Operadores de Comparación

| Operador | Python | VPy | Notas |
|----------|--------|-----|-------|
| **Igual** | `==` | ✅ | Igualdad |
| **Diferente** | `!=` | ✅ | Desigualdad |
| **Menor** | `<` | ✅ | Menor que |
| **Menor igual** | `<=` | ✅ | Menor o igual |
| **Mayor** | `>` | ✅ | Mayor que |
| **Mayor igual** | `>=` | ✅ | Mayor o igual |

### 6. Operadores Lógicos

| Operador | Python | VPy | Notas |
|----------|--------|-----|-------|
| **and** | `and` | ✅ | Lógico AND (evaluación corta) |
| **or** | `or` | ✅ | Lógico OR (evaluación corta) |
| **not** | `not` | ✅ | Lógico NOT |

### 7. Funciones

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Definición** | `def func(a, b): ...` | ✅ | `def func(a, b): ...` |
| **Llamada** | `func(1, 2)` | ✅ | `func(1, 2)` |
| **Return** | `return value` | ✅ | `return value` |
| **Sin params** | `def func(): ...` | ✅ | `def func(): ...` |
| **Params posicionales** | `def f(a, b, c): ...` | ✅ | Hasta 4 params vía VAR_ARG |

### 8. Tipos de Datos Básicos

| Tipo | Python | VPy | Notas |
|------|--------|-----|-------|
| **Enteros** | `int` (ilimitado) | ✅ | 16-bit signed (-32768 a 32767) |
| **Strings** | `str` | ✅ | Para PRINT_TEXT, labels ASM |
| **Booleanos** | `True`/`False` | ⚠️ | Usa 0/1 (no keywords True/False) |

### 9. Comentarios

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Línea** | `# comentario` | ✅ | `# comentario` |
| **Bloque** | `"""docstring"""` | ❌ | **NO implementado** |

### 10. Módulos (NUEVO)

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **import** | `import module` | ✅ | `import module` |
| **from-import** | `from module import func` | ✅ | `from module import func` |
| **import alias** | `import module as m` | ✅ | `import module as m` |
| **from-import alias** | `from m import f as g` | ✅ | `from m import f as g` |
| **import all** | `from module import *` | ✅ | `from module import *` |
| **import relativo** | `from . import x` | ✅ | `from . import x` |

---

## ❌ NO IMPLEMENTADO (Faltan)

### 1. Estructuras de Datos

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Listas** | `[1, 2, 3]` | ❌ | 🔴 ALTA | Fundamental para juegos |
| **Tuplas** | `(1, 2)` | ❌ | 🟡 MEDIA | Menos crítico |
| **Diccionarios** | `{"key": val}` | ❌ | 🟢 BAJA | Difícil en ASM |
| **Sets** | `{1, 2, 3}` | ❌ | 🟢 BAJA | No prioritario |
| **Index** | `lista[0]` | ❌ | 🔴 ALTA | Necesario con listas |
| **Slice** | `lista[1:3]` | ❌ | 🟡 MEDIA | Útil pero complejo |
| **len()** | `len(lista)` | ❌ | 🔴 ALTA | Necesario con listas |

### 2. Expresiones

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Ternario** | `x if cond else y` | ❌ | 🟡 MEDIA | Útil, no crítico |
| **Potencia** | `x ** y` | ❌ | 🟡 MEDIA | Raramente usado |
| **Walrus** | `if (x := func()): ...` | ❌ | 🟢 BAJA | Python 3.8+ |
| **Parentización** | `(a + b) * c` | ✅ | - | YA funciona |
| **Chained comparison** | `1 < x < 10` | ❌ | 🟡 MEDIA | Sintactic sugar |

### 3. Built-in Functions (Lenguaje)

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **print()** | `print(x)` | ❌ | 🔴 ALTA | Debugging crítico |
| **range()** | `range(10)` | ⚠️ | - | Solo en for loops |
| **abs()** | `abs(-5)` | ❌ | 🟡 MEDIA | Útil para física |
| **min()** | `min(a, b)` | ❌ | 🟡 MEDIA | Útil |
| **max()** | `max(a, b)` | ❌ | 🟡 MEDIA | Útil |
| **pow()** | `pow(2, 3)` | ❌ | 🟡 MEDIA | Alternativa a ** |
| **round()** | `round(3.7)` | N/A | - | Solo ints |
| **int()** | `int("42")` | ❌ | 🟢 BAJA | Conversión |
| **str()** | `str(42)` | ❌ | 🟢 BAJA | Conversión |
| **bool()** | `bool(0)` | ❌ | 🟢 BAJA | Usa 0/1 directo |
| **type()** | `type(x)` | N/A | - | No runtime types |

### 4. String Operations

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Concatenación** | `"a" + "b"` | ❌ | 🟡 MEDIA | Útil para texto |
| **Multiplicación** | `"x" * 3` | ❌ | 🟢 BAJA | Menos usado |
| **f-strings** | `f"x={x}"` | ❌ | 🟡 MEDIA | Moderno, útil |
| **format()** | `"{}".format(x)` | ❌ | 🟢 BAJA | Antiguo |
| **split()** | `"a,b".split(",")` | ❌ | 🟢 BAJA | Requiere listas |
| **join()** | `",".join(lista)` | ❌ | 🟢 BAJA | Requiere listas |

### 5. Control Flow Avanzado

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **for-in** | `for x in lista:` | ❌ | 🔴 ALTA | Necesita listas |
| **for-enumerate** | `for i, x in enumerate(l):` | ❌ | 🟡 MEDIA | Útil con listas |
| **while-else** | `while: ... else: ...` | ❌ | 🟢 BAJA | Raramente usado |
| **for-else** | `for: ... else: ...` | ❌ | 🟢 BAJA | Raramente usado |
| **try-except** | `try: ... except: ...` | ❌ | 🟢 BAJA | No exceptions en ASM |
| **with** | `with x as y: ...` | ❌ | 🟢 BAJA | Context managers |
| **pass** | `pass` | ❌ | 🟡 MEDIA | Placeholder útil |

### 6. Funciones Avanzadas

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Default args** | `def f(x=10): ...` | ❌ | 🟡 MEDIA | Muy útil |
| **Keyword args** | `f(x=5, y=10)` | ❌ | 🟢 BAJA | Complejo en ASM |
| ***args** | `def f(*args): ...` | ❌ | 🟢 BAJA | Difícil en ASM |
| **\*\*kwargs** | `def f(**kw): ...` | ❌ | 🟢 BAJA | Difícil en ASM |
| **Lambda** | `lambda x: x*2` | ❌ | 🟢 BAJA | Funciones anónimas |
| **Decorators** | `@decorator` | ❌ | 🟢 BAJA | Meta-programming |
| **Generators** | `yield` | ❌ | 🟢 BAJA | Estado complejo |

### 7. Clases y OOP

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **class** | `class Foo: ...` | ❌ | 🟢 BAJA | No OOP en ASM típico |
| **self** | `self.x` | ❌ | 🟢 BAJA | Requiere clases |
| **Herencia** | `class B(A): ...` | ❌ | 🟢 BAJA | Muy complejo |
| **\_\_init\_\_** | `def __init__(self): ...` | ❌ | 🟢 BAJA | Constructores |

### 8. Operadores No Implementados

| Operador | Python | VPy | Prioridad | Notas |
|----------|--------|-----|-----------|-------|
| **is/is not** | `x is None` | ❌ | 🟢 BAJA | Identidad de objetos |
| **in/not in** | `x in lista` | ❌ | 🟡 MEDIA | Necesita listas |
| **Unary +** | `+x` | ❌ | 🟢 BAJA | No-op usualmente |

### 9. Misc Features

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **assert** | `assert cond, "msg"` | ❌ | 🟡 MEDIA | Debugging |
| **del** | `del x` | ❌ | 🟢 BAJA | Gestión memoria |
| **global** | `global x` | ❌ | 🟡 MEDIA | Acceso explícito |
| **nonlocal** | `nonlocal x` | ❌ | 🟢 BAJA | Closures |
| **Multiline strings** | `"""..."""` | ❌ | 🟢 BAJA | Docstrings |
| **Escape sequences** | `"\n", "\t"` | ⚠️ | - | Parcial en strings |

---

## 🎯 PRIORIDADES RECOMENDADAS

### Phase 1: Fundamentales (CRÍTICO - Sin esto no se pueden hacer juegos complejos)

1. **🔴 Listas básicas**:
   ```python
   # Declaración
   var enemies = [0, 0, 0, 0, 0]  # Array fijo de 5 elementos
   
   # Acceso
   let x = enemies[0]
   enemies[1] = 10
   
   # Tamaño
   let count = len(enemies)
   ```
   **Implementación**: Arrays estáticos en RAM, tamaño fijo en compile-time.

2. **🔴 print() para debugging**:
   ```python
   print(player_x)  # Debugging en emulador
   print("Score:", score)
   ```
   **Implementación**: Output a consola del emulador (no pantalla Vectrex).

3. **🔴 for-in sobre listas**:
   ```python
   for enemy in enemies:
       if enemy > 0:
           draw_enemy(enemy)
   ```

### Phase 2: Útiles (MEDIA - Mejoran ergonomía)

4. **🟡 abs(), min(), max()**:
   ```python
   let distance = abs(player_x - enemy_x)
   let x = max(0, min(player_x, 127))  # Clamp
   ```

5. **🟡 Operador ternario**:
   ```python
   let speed = 5 if boost else 3
   ```

6. **🟡 Default arguments**:
   ```python
   def spawn_enemy(x, y, speed=2):
       # ...
   ```

7. **🟡 pass statement**:
   ```python
   if condition:
       pass  # TODO: implementar
   ```

### Phase 3: Nice-to-have (BAJA - Conveniencia)

8. **🟢 String operations** (concatenación, f-strings)
9. **🟢 Tuplas** (inmutables, retorno múltiple)
10. **🟢 assert** (validaciones)

---

## 📊 ESTADÍSTICAS

### Implementación Actual

| Categoría | Implementado | Total | % |
|-----------|--------------|-------|---|
| Control Flow | 7 / 7 | 100% | ✅ |
| Variables | 4 / 4 | 100% | ✅ |
| Operadores Aritméticos | 6 / 7 | 86% | ⚠️ |
| Operadores Bitwise | 6 / 6 | 100% | ✅ |
| Operadores Comparación | 6 / 6 | 100% | ✅ |
| Operadores Lógicos | 3 / 3 | 100% | ✅ |
| Funciones Básicas | 5 / 5 | 100% | ✅ |
| **TOTAL BÁSICO** | **37 / 38** | **97%** | ✅ |

| Categoría | Faltan | Prioridad Alta | Prioridad Media | Prioridad Baja |
|-----------|--------|----------------|-----------------|----------------|
| Estructuras de Datos | 7 | 3 🔴 | 1 🟡 | 3 🟢 |
| Expresiones | 5 | 0 | 3 🟡 | 2 🟢 |
| Built-ins | 12 | 1 🔴 | 4 🟡 | 7 🟢 |
| Strings | 6 | 0 | 2 🟡 | 4 🟢 |
| Control Flow Avanzado | 7 | 1 🔴 | 2 🟡 | 4 🟢 |
| Funciones Avanzadas | 7 | 0 | 1 🟡 | 6 🟢 |
| OOP | 4 | 0 | 0 | 4 🟢 |
| Operadores | 3 | 0 | 1 🟡 | 2 🟢 |
| Misc | 9 | 0 | 2 🟡 | 7 🟢 |
| **TOTAL FALTANTE** | **60** | **5 🔴** | **16 🟡** | **39 🟢** |

---

## 🚀 ROADMAP SUGERIDO

### Sprint 1: Arrays Estáticos (1-2 semanas)
- [ ] Parser: `var lista = [1, 2, 3]`
- [ ] AST: `Expr::List(Vec<Expr>)`
- [ ] Codegen: Alocar en RAM consecutiva
- [ ] Parser: `lista[index]`
- [ ] AST: `Expr::Index { target, index }`
- [ ] Codegen: Calcular offset + cargar valor
- [ ] Parser: `lista[index] = value`
- [ ] Codegen: Calcular offset + guardar valor
- [ ] Built-in: `len(lista)` retorna tamaño
- [ ] Tests: Arrays básicos, acceso, asignación

### Sprint 2: for-in y print() (1 semana)
- [ ] Parser: `for item in lista:`
- [ ] Codegen: Iterar sobre array
- [ ] Built-in: `print(expr)` → debug output
- [ ] Built-in: `print(str, expr)` → formato
- [ ] Tests: Loops sobre arrays, debugging

### Sprint 3: Math Built-ins (3-5 días)
- [ ] `abs(x)` → valor absoluto
- [ ] `min(a, b)` → mínimo
- [ ] `max(a, b)` → máximo
- [ ] Tests: Operaciones matemáticas

### Sprint 4: Ternario y Pass (2-3 días)
- [ ] Parser: `x if cond else y`
- [ ] AST: `Expr::Ternary { cond, true_val, false_val }`
- [ ] Codegen: Branch condicional
- [ ] Parser: `pass`
- [ ] Tests: Expresiones condicionales

### Sprint 5: Default Arguments (1 semana)
- [ ] Parser: `def func(x, y=10):`
- [ ] AST: Añadir defaults a params
- [ ] Codegen: Generar código de inicialización
- [ ] Tests: Funciones con defaults

---

## 📝 NOTAS DE IMPLEMENTACIÓN

### Arrays Estáticos en M6809

```asm
; Declaración: var enemies = [0, 0, 0, 0, 0]
ENEMIES:     ; Label del array
    FDB 0    ; enemies[0]
    FDB 0    ; enemies[1]
    FDB 0    ; enemies[2]
    FDB 0    ; enemies[3]
    FDB 0    ; enemies[4]
ENEMIES_LEN: EQU 5

; Acceso: let x = enemies[2]
    LDD #ENEMIES      ; Base address
    ADDD #4           ; Offset (2 * 2 bytes)
    TFR D,X          ; Transfer to index
    LDD ,X           ; Load value
    STD RESULT

; Asignación: enemies[2] = 10
    LDD #10
    STD ENEMIES+4     ; Direct offset si constante
```

### print() Implementation

```asm
; print(value) - Debug output to emulator console
PRINT_DEBUG:
    LDA VAR_ARG0+1    ; Low byte del valor
    STA $CF00         ; Debug output area
    LDA #$42          ; Debug marker
    STA $CF01         ; Signal new output
    RTS
```

---

**Última actualización**: 2025-12-19
**Autor**: VPy Compiler Team
**Estado**: En desarrollo activo
