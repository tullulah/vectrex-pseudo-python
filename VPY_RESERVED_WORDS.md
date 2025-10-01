# VPy - Palabras Reservadas y Funciones Built-in

## ⚠️ IMPORTANTE: VPy NO es Case-Sensitive
VPy no distingue entre mayúsculas y minúsculas. Esto significa que:
- `intensity`, `INTENSITY`, `Intensity` son **todas** la misma variable
- Si existe `INTENSITY` como función built-in, no puedes usar `intensity` como variable
- Siempre usar nombres completamente diferentes (ej: `brightness`, `power`, `value`)

## Funciones de Gráficos Vectrex

### Funciones de Dibujo Básico
- `DRAW_LINE` - Dibuja línea entre dos puntos
- `DRAW_POLYGON` - Dibuja polígono con múltiples vértices  
- `DRAW_CIRCLE` - Dibuja círculo aproximado con 16 segmentos
- `DRAW_CIRCLE_SEG` - Dibuja círculo con número variable de segmentos
- `DRAW_ARC` - Dibuja arco abierto
- `DRAW_SPIRAL` - Dibuja espiral

### Funciones de Movimiento y Posicionamiento
- `MOVE` / `MOVE_TO` - Mueve el haz sin dibujar
- `RESET0REF` - Reinicia referencia de posición (0,0)

### Funciones de Control de Intensidad
- `INTENSITY` - **PALABRA RESERVADA CRÍTICA** ⚠️
- `SET_INTENSITY` - Establece intensidad del haz

### Funciones de Sistema
- `WAIT_RECAL` - Espera recalibración del sistema
- `SET_SCALE` - Establece escala de dibujo
- `PRINT_STR` - Imprime texto en pantalla

## Funciones Matemáticas

### Funciones Trigonométricas
- `SIN` / `MATH_SIN` - Función seno
- `COS` / `MATH_COS` - Función coseno  
- `TAN` / `MATH_TAN` - Función tangente

### Funciones de Comparación
- `MIN` / `MATH_MIN` - Devuelve el menor de dos valores
- `MAX` / `MATH_MAX` - Devuelve el mayor de dos valores
- `CLAMP` / `MATH_CLAMP` - Limita valor entre mínimo y máximo

### Funciones de Valor Absoluto
- `ABS` / `MATH_ABS` - Valor absoluto

## Palabras Reservadas del Sistema

### Funciones Válidas en VPy
- `main` - Función de inicialización (ejecutada una vez)
- `loop` - Función de bucle principal (ejecutada cada frame automáticamente)
- **Funciones personalizadas**: ✅ **SÍ se permiten** (ej: `def draw_tree():`, `def calculate():`)

### Reglas de Declaración de Variables

#### ⚠️ PROBLEMA ACTUAL: Bug en variables locales
El compilador VPy tiene un bug con variables `let` - las trata como globales pero no las define, causando errores "Undefined symbol".

#### ✅ ESTRATEGIA RECOMENDADA: Solo variables globales (estilo Vectrex)

**Para máxima compatibilidad y siguiendo las prácticas del Vectrex original:**

```python
# TODAS las variables como globales con 'var'
var player_x = 0
var player_y = 0
var animation_frame = 0
var temp_angle = 0
var temp_cos = 0

def main():
    # Solo asignaciones, no declaraciones
    player_x = 50
    player_y = 60

def loop():
    # Solo asignaciones, no declaraciones
    animation_frame = animation_frame + 1
    temp_angle = animation_frame * 2
    temp_cos = COS(temp_angle)
    player_x = player_x + temp_cos
```

#### ⚠️ EVITAR (hasta que se arregle el compilador):
```python
def loop():
    let x = 0  # ❌ ERROR: Undefined symbol VAR_X
```

#### 💾 Gestión de Memoria en Vectrex
- **RAM total**: 1024 bytes
- **Variables típicas**: ~50 variables × 2 bytes = 100 bytes
- **Sistema**: ~300 bytes  
- **Disponible**: ~600 bytes ✅ Suficiente para juegos complejos

#### 🎯 Beneficios del enfoque "solo globales":
1. **Sin bugs del compilador**: Variables globales funcionan perfectamente
2. **Estilo Vectrex auténtico**: Los juegos originales usaban principalmente globales
3. **Persistencia automática**: Las variables conservan valores entre frames
4. **Simplicidad**: No hay confusión entre local/global

## Ejemplos de Uso Correcto vs Incorrecto

### ❌ INCORRECTO - Conflictos de Nombres
```python
def main():
    intensity = 50  # ❌ ERROR: 'intensity' es palabra reservada
    sin = 45        # ❌ ERROR: 'sin' es función built-in
    max = 100       # ❌ ERROR: 'max' es función built-in

def loop():
    DRAW_LINE(0, 0, intensity, intensity)  # ❌ Usa variable prohibida
```

### ✅ CORRECTO - Nombres Seguros
```python
def main():
    brightness = 50   # ✅ OK: nombre diferente y claro
    angle = 45        # ✅ OK: evita 'sin'
    maximum = 100     # ✅ OK: evita 'max'

def loop():
    DRAW_LINE(0, 0, brightness, brightness)  # ✅ Usa variable permitida
```

## Recomendaciones de Naming

### Para Intensidad/Brillo
- ✅ `brightness`
- ✅ `power`
- ✅ `beam_strength`
- ✅ `intensity_val` (con sufijo)
- ❌ `intensity` (reservada)

### Para Ángulos  
- ✅ `angle`
- ✅ `rotation`
- ✅ `degrees`
- ❌ `sin`, `cos`, `tan` (reservadas)

### Para Valores Máximos/Mínimos
- ✅ `maximum`, `minimum`
- ✅ `upper_limit`, `lower_limit`
- ✅ `high_val`, `low_val`
- ❌ `max`, `min` (reservadas)

## Estrategia Segura

1. **Usar prefijos descriptivos**: `player_x`, `enemy_speed`, `game_score`
2. **Usar sufijos clarificadores**: `value_max`, `speed_min`, `angle_cos`
3. **Nombres completamente diferentes**: `brightness` en lugar de `intensity`
4. **Verificar antes de usar**: Revisar esta lista antes de declarar variables

## Arquitectura VPy Correcta

```python
# ✅ PATRÓN CORRECTO
def main():
    # Inicialización única - NO usar while True aquí
    brightness = 80
    return  # main() debe terminar rápido

def loop():
    # Código que se ejecuta cada frame automáticamente
    # El sistema llama a loop() en cada frame
    DRAW_LINE(0, 0, 100, 100, brightness)
    # NO necesita while True - loop() se llama automáticamente
```

---
**Nota**: Esta lista se basa en el análisis del código fuente del compilador VPy. 
Pueden existir más palabras reservadas. Siempre probar la compilación antes del deployment.