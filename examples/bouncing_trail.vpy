# bouncing_trail.vpy - Pelota rebotando con estela que se desvanece
# Efecto visual de trazas que muestran el movimiento anterior

META TITLE = "BOUNCING TRAIL"
META COPYRIGHT = "g GCE 2025"
META MUSIC = "music1"

# Constantes
const I_BRIGHT = 0x7F
const I_MEDIUM = 0x50
const I_DIM    = 0x30
const I_FAINT  = 0x18

const SCREEN_LEFT   = -90
const SCREEN_RIGHT  =  90
const SCREEN_TOP    =  70
const SCREEN_BOTTOM = -70

const BALL_SIZE = 6
const TRAIL_LENGTH = 8  # Número de posiciones anteriores a recordar

# Posición y velocidad actuales
var ball_x = 0
var ball_y = 0
var vel_x  = 4
var vel_y  = 3

# Array simulado con variables para las posiciones de la estela
# (VPy no tiene arrays, así que usamos variables individuales)
var trail_x0 = 0
var trail_y0 = 0
var trail_x1 = 0
var trail_y1 = 0
var trail_x2 = 0
var trail_y2 = 0
var trail_x3 = 0
var trail_y3 = 0
var trail_x4 = 0
var trail_y4 = 0
var trail_x5 = 0
var trail_y5 = 0
var trail_x6 = 0
var trail_y6 = 0
var trail_x7 = 0
var trail_y7 = 0

var frame_count = 0

def update_trail():
    # Desplaza todas las posiciones de la estela hacia atrás
    trail_x7 = trail_x6
    trail_y7 = trail_y6
    trail_x6 = trail_x5
    trail_y6 = trail_y5
    trail_x5 = trail_x4
    trail_y5 = trail_y4
    trail_x4 = trail_x3
    trail_y4 = trail_y3
    trail_x3 = trail_x2
    trail_y3 = trail_y2
    trail_x2 = trail_x1
    trail_y2 = trail_y1
    trail_x1 = trail_x0
    trail_y1 = trail_y0
    
    # La posición más reciente es la actual
    trail_x0 = ball_x
    trail_y0 = ball_y

def update_ball_physics():
    # Actualiza posición
    ball_x = ball_x + vel_x
    ball_y = ball_y + vel_y
    
    # Rebotes horizontales
    if ball_x <= (SCREEN_LEFT + BALL_SIZE):
        ball_x = SCREEN_LEFT + BALL_SIZE
        vel_x = -vel_x
    
    if ball_x >= (SCREEN_RIGHT - BALL_SIZE):
        ball_x = SCREEN_RIGHT - BALL_SIZE
        vel_x = -vel_x
    
    # Rebotes verticales
    if ball_y >= (SCREEN_TOP - BALL_SIZE):
        ball_y = SCREEN_TOP - BALL_SIZE
        vel_y = -vel_y
    
    if ball_y <= (SCREEN_BOTTOM + BALL_SIZE):
        ball_y = SCREEN_BOTTOM + BALL_SIZE
        vel_y = -vel_y

def draw_trail():
    # Dibuja la estela con intensidades decrecientes
    # Cada posición anterior es más tenue
    
    # Posición más antigua (más tenue)
    if trail_x7 != 0 or trail_y7 != 0:
        DRAW_CIRCLE(trail_x7, trail_y7, 2, I_FAINT)
    
    if trail_x6 != 0 or trail_y6 != 0:
        DRAW_CIRCLE(trail_x6, trail_y6, 2, I_FAINT)
    
    if trail_x5 != 0 or trail_y5 != 0:
        DRAW_CIRCLE(trail_x5, trail_y5, 3, I_FAINT)
    
    if trail_x4 != 0 or trail_y4 != 0:
        DRAW_CIRCLE(trail_x4, trail_y4, 3, I_DIM)
    
    if trail_x3 != 0 or trail_y3 != 0:
        DRAW_CIRCLE(trail_x3, trail_y3, 4, I_DIM)
    
    if trail_x2 != 0 or trail_y2 != 0:
        DRAW_CIRCLE(trail_x2, trail_y2, 4, I_MEDIUM)
    
    if trail_x1 != 0 or trail_y1 != 0:
        DRAW_CIRCLE(trail_x1, trail_y1, 5, I_MEDIUM)

def draw_current_ball():
    # Dibuja la pelota actual con máxima intensidad
    DRAW_CIRCLE(ball_x, ball_y, BALL_SIZE, I_BRIGHT)
    
    # Añade un punto central
    VECTREX_DRAW_LINE(ball_x-1, ball_y, ball_x+1, ball_y, I_BRIGHT)
    VECTREX_DRAW_LINE(ball_x, ball_y-1, ball_x, ball_y+1, I_BRIGHT)

def draw_scene_elements():
    # Título
    PRINT_TEXT(-70, 60, "TRAIL DEMO")
    
    # Bordes sutiles
    VECTREX_DRAW_LINE(SCREEN_LEFT, SCREEN_TOP, SCREEN_RIGHT, SCREEN_TOP, I_DIM)
    VECTREX_DRAW_LINE(SCREEN_LEFT, SCREEN_BOTTOM, SCREEN_RIGHT, SCREEN_BOTTOM, I_DIM)
    VECTREX_DRAW_LINE(SCREEN_LEFT, SCREEN_TOP, SCREEN_LEFT, SCREEN_BOTTOM, I_DIM)
    VECTREX_DRAW_LINE(SCREEN_RIGHT, SCREEN_TOP, SCREEN_RIGHT, SCREEN_BOTTOM, I_DIM)
    
    # Información de velocidad (opcional, parpadea)
    if (frame_count & 31) < 16:
        if vel_x > 0 and vel_y > 0:
            PRINT_TEXT(40, -60, "NE")
        if vel_x < 0 and vel_y > 0:
            PRINT_TEXT(-60, -60, "NW")
        if vel_x < 0 and vel_y < 0:
            PRINT_TEXT(-60, -60, "SW")
        if vel_x > 0 and vel_y < 0:
            PRINT_TEXT(40, -60, "SE")

def main():
    frame_count = frame_count + 1
    
    # Solo actualiza la estela cada ciertos frames para efecto más suave
    if (frame_count & 3) == 0:  # Cada 4 frames
        update_trail()
    
    # Actualiza física cada frame
    update_ball_physics()
    
    # Inicializar frame
    VECTREX_FRAME_BEGIN(I_MEDIUM)
    
    # Dibujar elementos
    draw_scene_elements()
    draw_trail()
    draw_current_ball()
    
    return 0