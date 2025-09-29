# bouncing_shapes.vpy - Demo con múltiples formas rebotando
# Muestra círculos, líneas y polígonos rebotando con diferentes velocidades

META TITLE = "BOUNCING SHAPES"
META COPYRIGHT = "g GCE 2025"
META MUSIC = "music1"

# Constantes de intensidad
const I_BRIGHT = 0x7F
const I_MEDIUM = 0x50
const I_DIM    = 0x30
const I_FAINT  = 0x20

# Límites de la pantalla
const SCREEN_LEFT   = -100
const SCREEN_RIGHT  =  100
const SCREEN_TOP    =  80
const SCREEN_BOTTOM = -80

# Variables para círculo grande
var circle_x = -30
var circle_y = 20
var circle_vx = 2
var circle_vy = -3

# Variables para cuadrado (dibujado como polígono)
var square_x = 40
var square_y = -30
var square_vx = -1
var square_vy = 4

# Variables para línea que rota
var line_x = 0
var line_y = 0
var line_vx = 3
var line_vy = 1
var line_angle = 0

# Contador global
var frame_count = 0

def update_circle():
    # Actualiza posición del círculo
    circle_x = circle_x + circle_vx
    circle_y = circle_y + circle_vy
    
    # Rebotes con margen de 15 unidades
    if circle_x <= (SCREEN_LEFT + 15):
        circle_x = SCREEN_LEFT + 15
        circle_vx = -circle_vx
    
    if circle_x >= (SCREEN_RIGHT - 15):
        circle_x = SCREEN_RIGHT - 15
        circle_vx = -circle_vx
    
    if circle_y >= (SCREEN_TOP - 15):
        circle_y = SCREEN_TOP - 15
        circle_vy = -circle_vy
    
    if circle_y <= (SCREEN_BOTTOM + 15):
        circle_y = SCREEN_BOTTOM + 15
        circle_vy = -circle_vy

def update_square():
    # Actualiza posición del cuadrado
    square_x = square_x + square_vx
    square_y = square_y + square_vy
    
    # Rebotes con margen de 10 unidades
    if square_x <= (SCREEN_LEFT + 10):
        square_x = SCREEN_LEFT + 10
        square_vx = -square_vx
    
    if square_x >= (SCREEN_RIGHT - 10):
        square_x = SCREEN_RIGHT - 10
        square_vx = -square_vx
    
    if square_y >= (SCREEN_TOP - 10):
        square_y = SCREEN_TOP - 10
        square_vy = -square_vy
    
    if square_y <= (SCREEN_BOTTOM + 10):
        square_y = SCREEN_BOTTOM + 10
        square_vy = -square_vy

def update_line():
    # Actualiza posición de la línea
    line_x = line_x + line_vx
    line_y = line_y + line_vy
    line_angle = line_angle + 2  # Rota mientras se mueve
    
    # Rebotes con margen de 20 unidades
    if line_x <= (SCREEN_LEFT + 20):
        line_x = SCREEN_LEFT + 20
        line_vx = -line_vx
    
    if line_x >= (SCREEN_RIGHT - 20):
        line_x = SCREEN_RIGHT - 20
        line_vx = -line_vx
    
    if line_y >= (SCREEN_TOP - 20):
        line_y = SCREEN_TOP - 20
        line_vy = -line_vy
    
    if line_y <= (SCREEN_BOTTOM + 20):
        line_y = SCREEN_BOTTOM + 20
        line_vy = -line_vy

def draw_bouncing_circle():
    # Dibuja círculo grande
    DRAW_CIRCLE(circle_x, circle_y, 12, I_BRIGHT)

def draw_bouncing_square():
    # Dibuja cuadrado como polígono de 4 lados
    size = 8
    DRAW_POLYGON(4, I_MEDIUM, square_x - size, square_y - size, 
    square_x + size, square_y - size, square_x + size, square_y + size,
    square_x - size, square_y + size)

def draw_bouncing_line():
    # Dibuja línea rotativa
    # Calcula puntos de la línea basados en el ángulo
    # Simplificado: línea horizontal que "rota" visualmente con múltiples segmentos
    length = 15
    
    # Línea principal
    DRAW_LINE(line_x - length, line_y, line_x + length, line_y, I_MEDIUM)
    
    # Líneas adicionales para efecto de rotación
    if (line_angle & 8) != 0:
        DRAW_LINE(line_x, line_y - length, line_x, line_y + length, I_DIM)
    
    if (line_angle & 16) != 0:
        VECTREX_DRAW_LINE(line_x - 10, line_y - 10, line_x + 10, line_y + 10, I_FAINT)
        VECTREX_DRAW_LINE(line_x - 10, line_y + 10, line_x + 10, line_y - 10, I_FAINT)

def draw_background():
    # Dibuja una grilla de fondo sutil
    if (frame_count & 127) < 64:  # Aparece/desaparece lentamente
        # Lines verticales
        VECTREX_DRAW_LINE(-50, SCREEN_TOP, -50, SCREEN_BOTTOM, I_FAINT)
        VECTREX_DRAW_LINE(0, SCREEN_TOP, 0, SCREEN_BOTTOM, I_FAINT)
        VECTREX_DRAW_LINE(50, SCREEN_TOP, 50, SCREEN_BOTTOM, I_FAINT)
        
        # Líneas horizontales
        VECTREX_DRAW_LINE(SCREEN_LEFT, 40, SCREEN_RIGHT, 40, I_FAINT)
        VECTREX_DRAW_LINE(SCREEN_LEFT, 0, SCREEN_RIGHT, 0, I_FAINT)
        VECTREX_DRAW_LINE(SCREEN_LEFT, -40, SCREEN_RIGHT, -40, I_FAINT)

def main():
    frame_count = frame_count + 1
    
    # Inicializar frame
    VECTREX_FRAME_BEGIN(I_MEDIUM)
    
    # Actualizar físicas
    update_circle()
    update_square()
    update_line()
    
    # Dibujar fondo
    draw_background()
    
    # Dibujar título
    PRINT_TEXT(-80, 70, "BOUNCING")
    PRINT_TEXT(-80, -75, "SHAPES")
    
    # Dibujar objetos rebotando
    draw_bouncing_circle()
    draw_bouncing_square()
    draw_bouncing_line()
    
    return 0