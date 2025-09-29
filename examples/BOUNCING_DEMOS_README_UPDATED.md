# Demos de Rebote - Colección de Ejemplos VPy

Esta carpeta contiene una serie de demos que muestran objetos rebotando por la pantalla del Vectrex, implementadas en VPy (pseudo-Python para Vectrex).

## ⚠️ Estado del Compilador

**ACTUALIZACIÓN**: Durante las pruebas se detectó que el compilador VPy tiene limitaciones con sintaxis compleja. Los archivos originales con funciones con múltiples parámetros y llamadas a APIs no implementadas generan archivos ASM vacíos.

**✅ FUNCIONA**: `bouncing_ball_fixed.vpy` y `bouncing_ball_advanced.vpy`  
**❌ PROBLEMAS**: `bouncing_ball.vpy`, `bouncing_shapes.vpy`, `bouncing_trail.vpy`

## Archivos Funcionales

### 1. `bouncing_ball_fixed.vpy` - Demo Básica Funcional ✅
**Características:**
- Una pelota rebotando con forma de cuadrado
- Rebotes en los cuatro bordes (-100 a +100, -80 a +80)
- Variables globales para posición y velocidad
- Sintaxis simplificada compatible con el compilador actual

**Código generado:** `bouncing_ball_fixed.asm` (230 líneas de código 6809)

### 2. `bouncing_ball_advanced.vpy` - Demo Avanzada Funcional ✅
**Características:**
- Pelota con forma de diamante
- Bordes visibles de la pantalla
- Título en pantalla
- Efectos de estela según dirección de movimiento
- Contador de frames
- Mayor velocidad y complejidad visual

**Código generado:** `bouncing_ball_advanced.asm` (456 líneas de código 6809)

## Limitaciones Encontradas del Compilador VPy

Durante las pruebas se identificaron estas limitaciones:

1. **Funciones con parámetros complejos** - no siempre funcionan correctamente
2. **APIs no implementadas** - `DRAW_CIRCLE()` no existe
3. **Expresiones complejas** - algunas operaciones matemáticas anidadas fallan
4. **Sintaxis muy "pythónica"** - el compilador prefiere sintaxis más simple

## Instrucciones de Compilación

```powershell
# Compilar el proyecto primero
cd C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python
cargo build --release

# Compilar demo básica (FUNCIONA)
.\target\release\vectrex_lang.exe build examples\bouncing_ball_fixed.vpy

# Compilar demo avanzada (FUNCIONA)  
.\target\release\vectrex_lang.exe build examples\bouncing_ball_advanced.vpy

# Verificar que se generó el ASM
dir examples\*.asm
```

## Diagnóstico de Problemas

Si un archivo VPy genera un ASM vacío:

```powershell
# Verificar tokens (debe mostrar la tokenización)
.\target\release\vectrex_lang.exe lex examples\archivo.vpy

# Verificar AST (debe mostrar el árbol sintáctico)  
.\target\release\vectrex_lang.exe ast examples\archivo.vpy

# Si AST se genera pero ASM está vacío = problema en backend de generación
```

## Patrones de Código que Funcionan

**✅ Variables globales simples:**
```python
var ball_x = 0
var ball_y = 0
```

**✅ Funciones sin parámetros:**
```python
def main():
    ball_x = ball_x + vel_x
```

**✅ Condicionales simples:**
```python
if ball_x <= -90:
    ball_x = -90
    vel_x = -vel_x
```

**✅ Llamadas a APIs conocidas:**
```python
VECTREX_DRAW_LINE(x1, y1, x2, y2, intensity)
PRINT_TEXT(x, y, "texto")
```

## Patrones que Causan Problemas

**❌ Funciones con parámetros:**
```python
def draw_ball(x, y, intensity):  # Puede fallar
    DRAW_CIRCLE(x, y, radius, intensity)  # API no existe
```

**❌ Expresiones muy complejas:**
```python
# Demasiado anidado para el compilador actual
if (vel_x < 0 and vel_y < 0) or (vel_x > 0 and vel_y > 0):
```

## Recomendaciones

1. **Mantén la sintaxis simple** - evita expresiones complejas
2. **Usa solo APIs documentadas** - `VECTREX_DRAW_LINE`, `PRINT_TEXT`
3. **Funciones sin parámetros** - usa variables globales  
4. **Prueba compilación frecuentemente** - verifica que el ASM se genera
5. **Revisa ejemplos existentes** - `hello.vpy`, `triangle.vpy` funcionan bien

## Demos Funcionales Incluidas

- **bouncing_ball_fixed.vpy**: Demo básica, pelota cuadrada rebotando
- **bouncing_ball_advanced.vpy**: Demo avanzada, pelota diamante con efectos

Ambas compiladas y probadas exitosamente. ¡Listas para ejecutar en emulador o hardware real!

## Estado del Proyecto

El compilador VPy es funcional pero tiene limitaciones. Las demos básicas y intermedias funcionan perfectamente. Para proyectos más complejos, es recomendable mantener la sintaxis simple y probar la compilación frecuentemente.