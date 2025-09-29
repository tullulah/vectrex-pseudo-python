# bouncing_ball_advanced.vpy - Versión avanzada con bordes y efectos

META TITLE = "BOUNCING BALL ADV"
META MUSIC = "music1"
META COPYRIGHT = "g GCE 2025"

# Variables globales
var ball_x = 0
var ball_y = 0
var vel_x = 4
var vel_y = 3
var frame_count = 0

def main():
    frame_count = frame_count + 1
    
    # Actualizar posición
    ball_x = ball_x + vel_x
    ball_y = ball_y + vel_y
    
    # Rebotes con amortiguación
    if ball_x <= -90:
        ball_x = -90
        vel_x = -vel_x
    
    if ball_x >= 90:
        ball_x = 90
        vel_x = -vel_x
    
    if ball_y <= -70:
        ball_y = -70
        vel_y = -vel_y
        
    if ball_y >= 70:
        ball_y = 70
        vel_y = -vel_y
    
    # Dibujar bordes de la pantalla
    VECTREX_DRAW_LINE(-90, -70, 90, -70, 0x30)    # Borde inferior
    VECTREX_DRAW_LINE(-90, 70, 90, 70, 0x30)      # Borde superior
    VECTREX_DRAW_LINE(-90, -70, -90, 70, 0x30)    # Borde izquierdo
    VECTREX_DRAW_LINE(90, -70, 90, 70, 0x30)      # Borde derecho
    
    # Dibujar título
    PRINT_TEXT(-80, 60, "BOUNCING BALL")
    
    # Dibujar la pelota como un diamante
    VECTREX_DRAW_LINE(ball_x, ball_y-8, ball_x+6, ball_y, 0x7F)
    VECTREX_DRAW_LINE(ball_x+6, ball_y, ball_x, ball_y+8, 0x7F)
    VECTREX_DRAW_LINE(ball_x, ball_y+8, ball_x-6, ball_y, 0x7F)
    VECTREX_DRAW_LINE(ball_x-6, ball_y, ball_x, ball_y-8, 0x7F)
    
    # Dibujar estela si está en movimiento diagonal
    if vel_x > 0:
        if vel_y > 0:
            VECTREX_DRAW_LINE(ball_x-10, ball_y-10, ball_x-5, ball_y-5, 0x20)
    
    if vel_x < 0:
        if vel_y < 0:  
            VECTREX_DRAW_LINE(ball_x+10, ball_y+10, ball_x+5, ball_y+5, 0x20)