# bouncing_ball.vpy - Demo de una pelota rebotando por la pantalla
# Utiliza variables globales para mantener posición y velocidad entre frames

META TITLE = "BOUNCING BALL DEMO"
META COPYRIGHT = "g GCE 2025"
META MUSIC = "music1"

# Constantes de intensidad
const I_BRIGHT = 0x7F
const I_MEDIUM = 0x50
const I_DIM    = 0x30

# Límites de la pantalla (aproximados para Vectrex)
const SCREEN_LEFT   = -100
const SCREEN_RIGHT  =  100
const SCREEN_TOP    =  80
const SCREEN_BOTTOM = -80

# Radio de la pelota
const BALL_RADIUS = 8

# Variables globales para posición y velocidad de la pelota
var ball_x = 0
var ball_y = 0
var vel_x  = 3
var vel_y  = 2

# Contador de frames para efectos
var frame_count = 0

def draw_ball(x, y, intensity):
    # Dibuja la pelota como un círculo
    DRAW_CIRCLE(x, y, BALL_RADIUS, intensity)
    
    # Añade una pequeña cruz en el centro para mejor visibilidad
    DRAW_LINE(x-3, y, x+3, y, intensity)
    DRAW_LINE(x, y-3, x, y+3, intensity)

def update_ball_position():
    # Actualiza la posición de la pelota
    ball_x = ball_x + vel_x
    ball_y = ball_y + vel_y
    
    # Rebote en bordes horizontales (izquierda y derecha)
    if ball_x <= (SCREEN_LEFT + BALL_RADIUS):
        ball_x = SCREEN_LEFT + BALL_RADIUS
        vel_x = -vel_x  # Invierte velocidad horizontal
    
    if ball_x >= (SCREEN_RIGHT - BALL_RADIUS):
        ball_x = SCREEN_RIGHT - BALL_RADIUS
        vel_x = -vel_x  # Invierte velocidad horizontal
    
    # Rebote en bordes verticales (arriba y abajo)
    if ball_y >= (SCREEN_TOP - BALL_RADIUS):
        ball_y = SCREEN_TOP - BALL_RADIUS
        vel_y = -vel_y  # Invierte velocidad vertical
    
    if ball_y <= (SCREEN_BOTTOM + BALL_RADIUS):
        ball_y = SCREEN_BOTTOM + BALL_RADIUS
        vel_y = -vel_y  # Invierte velocidad vertical

def draw_borders():
    # Dibuja los bordes de la pantalla para visualizar los límites
    # Borde superior
    DRAW_LINE(SCREEN_LEFT, SCREEN_TOP, SCREEN_RIGHT, SCREEN_TOP, I_DIM)
    # Borde inferior  
    DRAW_LINE(SCREEN_LEFT, SCREEN_BOTTOM, SCREEN_RIGHT, SCREEN_BOTTOM, I_DIM)
    # Borde izquierdo
    DRAW_LINE(SCREEN_LEFT, SCREEN_TOP, SCREEN_LEFT, SCREEN_BOTTOM, I_DIM)
    # Borde derecho
    DRAW_LINE(SCREEN_RIGHT, SCREEN_TOP, SCREEN_RIGHT, SCREEN_BOTTOM, I_DIM)

def draw_info():
    # Muestra información en pantalla
    PRINT_TEXT(-90, 70, "BOUNCING BALL")
    PRINT_TEXT(-90, -70, "DEMO")
    
    # Muestra contador de frames (opcional)
    if (frame_count & 63) < 32:  # Parpadea cada ~1 segundo
        PRINT_TEXT(50, 70, "FRAME")

def main():
    # Inicialización: posición inicial en el centro
    ball_x = 0
    ball_y = 0
    vel_x = 3
    vel_y = 2
    frame_count = 0

def loop():
    frame_count = frame_count + 1
    
    # Actualizar física de la pelota
    update_ball_position()
    
    # Dibujar elementos de la pantalla
    draw_borders()
    draw_info()
    
    # Calcular intensidad de la pelota basada en velocidad (efecto visual)
    let ball_intensity = I_BRIGHT
    if (vel_x < 0 and vel_y < 0) or (vel_x > 0 and vel_y > 0):
        ball_intensity = I_MEDIUM  # Intensidad diferente en diagonales
    
    # Dibujar la pelota
    draw_ball(ball_x, ball_y, ball_intensity)
