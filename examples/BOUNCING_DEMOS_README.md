# Bouncing Demos - VPy Examples

Esta carpeta contiene tres demos de objetos rebotando en la pantalla del Vectrex, escritas en VPy (Vectrex Pseudo-Python).

## Demos Incluidas

### 1. bouncing_ball.vpy
**Demo básica de pelota rebotando**
- Una sola pelota que rebota en los bordes de la pantalla
- Velocidad constante con cambios de dirección en los rebotes
- Visualización de bordes de pantalla
- Intensidad variable basada en la dirección del movimiento
- Incluye contador de frames que parpadea

**Características técnicas:**
- Variables globales para posición (`ball_x`, `ball_y`) y velocidad (`vel_x`, `vel_y`)
- Detección de colisiones con bordes usando constantes de límites de pantalla
- Radio de pelota configurable
- Función de dibujado con círculo y cruz central

### 2. bouncing_shapes.vpy
**Demo avanzada con múltiples objetos**
- Círculo grande rebotando con una velocidad
- Cuadrado (polígono) rebotando con otra velocidad
- Línea rotativa que rebota y gira mientras se mueve
- Grilla de fondo que aparece y desaparece
- Diferentes intensidades para cada objeto

**Características técnicas:**
- Tres sistemas de física independientes
- Polígonos dibujados con `DRAW_POLYGON`
- Efecto de rotación simulado con múltiples líneas
- Grilla de fondo con efectos de parpadeo temporizado
- Diferentes márgenes de rebote para cada objeto

### 3. bouncing_trail.vpy
**Demo con efecto de estela**
- Pelota que deja una estela de posiciones anteriores
- Estela que se desvanece gradualmente (diferentes intensidades)
- Indicador de dirección que parpadea
- Actualización de estela cada pocos frames para suavidad

**Características técnicas:**
- Sistema de "array" simulado con 8 variables para posiciones anteriores
- Función `update_trail()` que desplaza todas las posiciones
- Intensidades decrecientes para crear efecto de desvanecimiento
- Control de timing para actualización de estela vs. física

## Conceptos de VPy Demostrados

### Variables Globales
```vpy
var ball_x = 0
var ball_y = 0
var vel_x = 3
var vel_y = 2
```

### Constantes
```vpy
const SCREEN_LEFT = -100
const I_BRIGHT = 0x7F
```

### Funciones de Dibujo
- `DRAW_CIRCLE(x, y, radius, intensity)` - Círculos
- `DRAW_POLYGON(sides, intensity, x1,y1, x2,y2, ...)` - Polígonos
- `VECTREX_DRAW_LINE(x1, y1, x2, y2, intensity)` - Líneas
- `PRINT_TEXT(x, y, "texto")` - Texto
- `VECTREX_FRAME_BEGIN(intensity)` - Inicialización de frame

### Control de Flujo
- Condicionales: `if`, `else`
- Operadores lógicos: `&`, `|`, `==`, `!=`, `<`, `>`
- Operadores aritméticos: `+`, `-`, `*`, `/`

### Patrones de Rebote
```vpy
if ball_x <= (SCREEN_LEFT + BALL_RADIUS):
    ball_x = SCREEN_LEFT + BALL_RADIUS
    vel_x = -vel_x  # Invierte velocidad
```

## Compilación y Ejecución

Para compilar cualquiera de estas demos:

1. Abrir el archivo `.vpy` en el IDE
2. Presionar F5 o usar el sistema de compilación
3. El compilador generará el archivo `.asm` correspondiente
4. Ejecutar en el emulador Vectrex integrado

## Personalización

Estas demos pueden modificarse fácilmente:

- **Velocidades**: Cambiar `vel_x`, `vel_y` para diferentes velocidades
- **Tamaños**: Modificar `BALL_RADIUS`, `BALL_SIZE` para objetos más grandes/pequeños  
- **Límites**: Ajustar constantes `SCREEN_*` para diferentes áreas de juego
- **Intensidades**: Modificar constantes `I_*` para diferentes brillos
- **Efectos**: Cambiar condiciones de parpadeo y timing

## Notas Técnicas

- VPy no tiene arrays nativos, por lo que la estela usa variables individuales
- El frame rate del Vectrex es aproximadamente 50-60 Hz
- Las intensidades van de 0x00 (apagado) a 0x7F (máximo brillo)
- Las coordenadas del Vectrex van aproximadamente de -100 a +100 en X e Y