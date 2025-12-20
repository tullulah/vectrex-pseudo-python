# Python vs VPy Language Feature Comparison

**Objetivo**: Documentar qué funcionalidades del lenguaje Python están implementadas en VPy y cuáles faltan.

**Nota importante**: VPy NO es Python. Es un lenguaje inspirado en Python pero con diferencias significativas:
- **VPy usa sintaxis Python pura** - NO requiere keywords para declarar variables (actualizado 2025-12-19)
- **VPy es statically-typed 16-bit** (Python es dinamically-typed con ints ilimitados)
- **VPy compila a ASM M6809** (Python es interpretado/JIT)

Este documento compara la **sintaxis y features** para guiar el desarrollo de VPy.

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
| **pass** | `pass` | ✅ | No-op placeholder |
| **switch/match** | `match x: case 1: ...` (Python 3.10+) | ✅ | `switch expr: case 1: ... default: ...` |
| **return** | `return value` | ✅ | Con/sin valor |

### 2. Variables y Asignación

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Declaración** | `x = 10` (sin keyword) | ✅ | Sintaxis idéntica a Python |
| **Globales** | `x = 10` (top-level) | ✅ | `x = 10` (top-level, sin keyword) |
| **Locales** | `x = 10` (en función) | ✅ | `y = 20` (en función, sin keyword) |
| **Constantes** | No nativas (convención CAPS) | ✅ | `const X = 10` |
| **Asignación simple** | `x = expr` | ✅ | `x = expr` (sin redeclarar) |
| **Asignación compuesta** | `x += 5`, `x -= 3`, etc | ✅ | `x += 5`, `x -= 3`, `x *= 2`, etc |

**✅ Actualización 2025-12-19**: VPy ahora usa sintaxis Python pura. NO requiere `var`/`let` - el scope se detecta automáticamente (top-level = global, en función = local).

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
| **Strings** | `str` | ✅ | Literals en globales/locales, DEBUG_PRINT_STR |
| **Booleanos** | `True`/`False` | ⚠️ | Usa 0/1 (no keywords True/False) |

### 9. Comentarios

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Línea** | `# comentario` | ✅ | `# comentario` |
| **Bloque** | `"""docstring"""` | ❌ | **NO implementado** |

### 10. Módulos

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **import** | `import module` | ✅ | `import module` |
| **from-import** | `from module import func` | ✅ | `from module import func` |
| **import alias** | `import module as m` | ✅ | `import module as m` |
| **from-import alias** | `from m import f as g` | ✅ | `from m import f as g` |
| **import all** | `from module import *` | ✅ | `from module import *` |
| **import relativo** | `from . import x` | ✅ | `from . import x` |

### 11. Arrays y Estructuras de Datos

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Arrays** | `[1, 2, 3]` | ✅ | Arrays estáticos, tamaño fijo |
| **Index read** | `x = lista[0]` | ✅ | Acceso por índice |
| **Index write** | `lista[0] = 5` | ✅ | Asignación por índice |
| **len()** | `len(lista)` | ✅ | Retorna tamaño del array |
| **for-in** | `for x in lista:` | ✅ | Iteración sobre arrays |

### 12. Built-in Math Functions

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **abs()** | `abs(-5)` | ✅ | Valor absoluto (útil con enteros: distancias, etc) |
| **min()** | `min(a, b)` | ✅ | Mínimo de dos valores |
| **max()** | `max(a, b)` | ✅ | Máximo de dos valores |

### 13. Structs y OOP Básico

| Feature | Python | VPy | Notas |
|---------|--------|-----|-------|
| **Struct definition** | N/A (usa class) | ✅ | `struct Name: x: int` |
| **Struct instantiation** | N/A | ✅ | `obj = StructName()` |
| **Field access read** | `obj.field` | ✅ | `x = obj.field` |
| **Field access write** | `obj.field = val` | ✅ | `obj.field = value` |
| **Methods** | `def method(self):` | ✅ | `def method():` (self implícito) |
| **Self field read** | `self.x` | ✅ | `x = self.x` en métodos |
| **Self field write** | `self.x = val` | ✅ | `self.x = value` en métodos |
| **Method calls** | `obj.method(args)` | ✅ | `obj.method(args)` |
| **Constructors** | `__init__(self, x)` | ⚠️ | Params OK, self.field WIP |
| **Return self** | `return self` | ❌ | Sin method chaining |
| **Private fields** | `_field` (convención) | ❌ | Todos los campos públicos |

---

## ❌ NO IMPLEMENTADO (Faltan)

### 1. Estructuras de Datos Avanzadas

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Tuplas** | `(1, 2)` | ❌ | 🟡 MEDIA | Retorno múltiple, inmutables |
| **Diccionarios** | `{"key": val}` | ❌ | 🟢 BAJA | Difícil en ASM |
| **Sets** | `{1, 2, 3}` | ❌ | 🟢 BAJA | No prioritario |
| **Slice** | `lista[1:3]` | ❌ | 🟡 MEDIA | Útil pero complejo |

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
| **print()** | `print(x)` | ✅ | - | DEBUG_PRINT_STR implementado |
| **range()** | `range(10)` | ⚠️ | - | Solo en for loops |
| **pow()** | `pow(2, 3)` | ❌ | 🟡 MEDIA | Alternativa a ** |
| **round()** | `round(3.7)` | N/A | - | Solo ints |
| **int()** | `int("42")` | ❌ | 🟢 BAJA | Conversión |
| **str()** | `str(42)` | ❌ | 🟢 BAJA | Conversión |
| **bool()** | `bool(0)` | ❌ | 🟢 BAJA | Usa 0/1 directo |
| **type()** | `type(x)` | N/A | - | No runtime types |

### 4. String Operations

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Literals** | `"hello"` | ✅ | - | Globales/locales con auto-storage |
| **Concatenación** | `"a" + "b"` | ❌ | � BAJA | Útil para texto |
| **Multiplicación** | `"x" * 3` | ❌ | 🟢 BAJA | Menos usado |
| **f-strings** | `f"x={x}"` | ❌ | 🟢 BAJA | Moderno, útil |
| **format()** | `"{}".format(x)` | ❌ | 🟢 BAJA | Antiguo |
| **split()** | `"a,b".split(",")` | ❌ | 🟢 BAJA | Requiere listas |
| **join()** | `",".join(lista)` | ❌ | 🟢 BAJA | Requiere listas |

### 5. Control Flow Avanzado

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **for-enumerate** | `for i, x in enumerate(l):` | ❌ | 🟡 MEDIA | Índice + valor simultáneo |
| **while-else** | `while: ... else: ...` | ❌ | 🟢 BAJA | Raramente usado |
| **for-else** | `for: ... else: ...` | ❌ | 🟢 BAJA | Raramente usado |
| **try-except** | `try: ... except: ...` | ❌ | 🟢 BAJA | No exceptions en ASM |
| **with** | `with x as y: ...` | ❌ | 🟢 BAJA | Context managers |

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

**Nota sobre límite de parámetros**: VPy actualmente soporta **máximo 4 parámetros** por función. Este es un límite arbitrario de diseño (no técnico), fácilmente ampliable si fuera necesario. Python tiene un límite de 255 parámetros (restricción de bytecode). En la práctica, 4 parámetros son suficientes para desarrollo de juegos Vectrex.

### 7. Clases y OOP

| Feature | Python | VPy | Prioridad | Notas |
|---------|--------|-----|-----------|-------|
| **Structs básicos** | N/A | ✅ | - | `struct Name: field: int` |
| **Methods** | `def method(self):` | ✅ | - | Self implícito |
| **Self read/write** | `self.x`, `self.x = 5` | ✅ | - | Acceso completo a fields |
| **Constructors** | `__init__(self, x)` | ❌ | 🟡 MEDIA | `def __init__(x, y):` |
| **Return self** | `return self` | ❌ | 🟡 MEDIA | Method chaining |
| **Private fields** | `_field` (convención) | ❌ | 🟢 BAJA | Naming convention |
| **Properties** | `@property` | ❌ | 🟢 BAJA | Getters/setters |
| **Static methods** | `@staticmethod` | ❌ | 🟢 BAJA | Sin self |
| **Class methods** | `@classmethod` | ❌ | 🟢 BAJA | Clase como param |
| **Herencia** | `class B(A):` | ❌ | 🟢 BAJA | Muy complejo en ASM |
| **Multiple inheritance** | `class C(A,B):` | ❌ | 🟢 BAJA | Extremadamente complejo |
| **super()** | `super().method()` | ❌ | 🟢 BAJA | Requiere herencia |

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

~~1. **🔴 Listas básicas**~~ ✅ **COMPLETADO (2025-12-19)**:
   ```python
   # Python Y VPy (sintaxis idéntica):
   enemies = [0, 0, 0, 0, 0]  # Array fijo
   x = enemies[0]             # Acceso
   enemies[1] = 10            # Asignación
   count = len(enemies)       # Tamaño
   ```
   **Implementación**: Arrays estáticos en RAM, tamaño fijo en compile-time.

~~2. **🔴 print() para debugging**~~ ✅ **COMPLETADO (2025-12-19)**:
   ```python
   # Python:
   print("Score:", score)
   
   # VPy:
   DEBUG_PRINT_STR("Score:")  # Literal directo
   DEBUG_PRINT_STR(texto)     # Variable global/local
   DEBUG_PRINT(score)         # Numérico
   ```
   **Implementación**: DEBUG_PRINT_STR con protocolo C000-C00F.

~~3. **🔴 for-in sobre listas**~~ ✅ **COMPLETADO (2025-12-19)**:
   ```python
   # Python Y VPy (sintaxis idéntica):
   for enemy in enemies:
       if enemy > 0:
           draw_enemy(enemy)
   ```

### Phase 2: Útiles (MEDIA - Mejoran ergonomía)

~~4. **🟡 abs(), min(), max()**~~ ✅ **COMPLETADO (2025-12-19)**:
   ```python
   # Python Y VPy (sintaxis idéntica):
   distance = abs(player_x - enemy_x)  # ✅ Valor absoluto para distancias
   x = max(0, min(player_x, 127))      # ✅ Clamp con min/max
   ```
   **Nota**: abs() es útil con enteros - distancias, velocidades, colisiones.

~~5. **🟡 Structs con self write**~~ ✅ **COMPLETADO (2025-12-20)**:
   ```python
   # Python Y VPy (sintaxis similar):
   struct Entity:
       x: int
       dx: int
       
       def update():
           self.x = self.x + self.dx  # ✅ Modifica estado interno
       
       def bounce(limit):
           if self.x > limit:
               self.dx = -self.dx      # ✅ Cambia dirección
   
   entity = Entity()
   entity.x = 100
   entity.update()  # ✅ Objeto se modifica internamente
   ```
   **Implementación**: LEAX para punteros de structs locales, VAR_ARG0 en métodos.

6. **🟡 Operador ternario**:
   ```python
   speed = 5 if boost else 3
   ```

~~7. **🟡 global keyword**~~ → **DEPRECADO**: VPy no necesita `global` explícito:
   ```python
   # Python:
   def func():
       global score  # ← Necesario
       score = score + 10
   
   # VPy:
   def func():
       score = score + 10  # ✅ Ya funciona sin 'global'
   ```

8. **🟡 Constructores (__init__)** → **PARCIALMENTE COMPLETADO (2025-12-20)**:
   ```python
   struct Player:
       x: int
       y: int
       
       def __init__(start_x, start_y):
           # ⚠️ LIMITACIÓN: self.field assignment WIP (FuncCtx awareness needed)
           # Por ahora: params funcionan, pero fields se asignan después
           pass
   
   player = Player(10, 20)  # ✅ Aloca struct + llama init con params
   player.x = 10  # Asignación manual por ahora
   player.y = 20
   ```
   **Implementado**: Parser, AST, codegen, instanciación con args
   **Pendiente**: FuncCtx awareness para self.field en constructor

### Phase 3: Nice-to-have (BAJA - Conveniencia)

9. **🟢 Return self (method chaining)**:
   ```python
   entity.move(5).rotate(90).draw()  # Chaining
   ```

10. **🟢 Default arguments**:
   ```python
   def spawn_enemy(x, y, speed=2):
       # ...
   ```

11. **🟢 String operations** (concatenación, f-strings)
12. **🟢 Tuplas** (inmutables, retorno múltiple)
13. **🟢 assert** (validaciones)

---

## 📊 ESTADÍSTICAS

### Implementación Actual

| Categoría | Implementado | Total | % |
|-----------|--------------|-------|---|
| Control Flow | 7 / 7 | 100% | ✅ |
| Variables | 6 / 6 | 100% | ✅ |
| Operadores Aritméticos | 6 / 7 | 86% | ⚠️ |
| Operadores Bitwise | 6 / 6 | 100% | ✅ |
| Operadores Comparación | 6 / 6 | 100% | ✅ |
| Operadores Lógicos | 3 / 3 | 100% | ✅ |
| Funciones Básicas | 5 / 5 | 100% | ✅ |
| Strings | 2 / 2 | 100% | ✅ |
| Arrays & Iteration | 5 / 5 | 100% | ✅ |
| Math Builtins | 3 / 3 | 100% | ✅ |
| Structs & OOP | 8 / 11 | 73% | ⚠️ |
| **TOTAL BÁSICO** | **57 / 61** | **93%** | ✅ |

| Categoría | Faltan | Prioridad Alta | Prioridad Media | Prioridad Baja |
|-----------|--------|----------------|-----------------|----------------|
| Estructuras de Datos | 4 | 0 | 1 🟡 | 3 🟢 |
| Expresiones | 5 | 0 | 3 🟡 | 2 🟢 |
| Built-ins | 6 | 0 | 1 🟡 | 5 🟢 |
| Strings | 5 | 0 | 0 | 5 🟢 |
| Control Flow Avanzado | 6 | 0 | 2 🟡 | 4 🟢 |
| Funciones Avanzadas | 7 | 0 | 1 🟡 | 6 🟢 |
| OOP Avanzado | 9 | 0 | 2 🟡 | 7 🟢 |
| Operadores | 3 | 0 | 1 🟡 | 2 🟢 |
| Misc | 9 | 0 | 2 🟡 | 7 🟢 |
| **TOTAL FALTANTE** | **54** | **0 🔴** | **13 🟡** | **41 🟢** |

**Mejoras recientes**:
- ✅ **2025-12-19**: String literals locales, DEBUG_PRINT_STR, len(), MIN(), MAX()
- ✅ **2025-12-20**: Self write support (self.field = value), method chaining preparation

---

## 🚀 ROADMAP SUGERIDO

### ✅ Sprint 0: Strings y Debug (COMPLETADO 2025-12-19)
- [x] String literals en variables locales (`let texto = "HOLA"`)
- [x] DEBUG_PRINT_STR con literals directos
- [x] len() builtin para arrays
- [x] MIN() y MAX() builtins

### Sprint 1: Arrays Estáticos (1-2 semanas) - **PRÓXIMO**
- [ ] Parser: `var lista = [1, 2, 3]`
- [ ] AST: `Expr::List(Vec<Expr>)`
- [ ] Codegen: Alocar en RAM consecutiva
- [ ] Parser: `lista[index]`
- [ ] AST: `Expr::Index { target, index }`
- [ ] Codegen: Calcular offset + cargar valor
- [ ] Parser: `lista[index] = value`
- [ ] Codegen: Calcular offset + guardar valor
- [ ] Tests: Arrays básicos, acceso, asignación

### Sprint 2: for-in (1 semana)
- [ ] Parser: `for item in lista:`
- [ ] Codegen: Iterar sobre array
- [ ] Tests: Loops sobre arrays

### Sprint 3: Math Built-ins (3-5 días)
- [ ] `abs(x)` → valor absoluto
- [x] `min(a, b)` → mínimo (YA IMPLEMENTADO)
- [x] `max(a, b)` → máximo (YA IMPLEMENTADO)
- [ ] Tests: Operaciones matemáticas

### Sprint 4: Ternario (2-3 días)
- [ ] Parser: `x if cond else y`
- [ ] AST: `Expr::Ternary { cond, true_val, false_val }`
- [ ] Codegen: Branch condicional
- [x] Parser: `pass` ✅ **COMPLETADO 2025-12-19**
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

**Última actualización**: 2025-12-19 (21:18)
**Autor**: VPy Compiler Team
**Estado**: En desarrollo activo

**Cambios recientes**:
- ✅ **SINTAXIS PYTHON PURA**: Eliminados keywords var/let (sintaxis idéntica a Python)
- ✅ Arrays estáticos con index access `[1,2,3]`, `lista[0]`, `lista[i]=x`
- ✅ `for-in` sobre arrays: `for item in lista:`
- ✅ Math builtins: `abs()`, `min()`, `max()`
- ✅ String literals en locales y DEBUG_PRINT_STR
- ✅ `len()` builtin para arrays
- 🎯 **NO quedan features críticas pendientes** - VPy cubre lo esencial para juegos
