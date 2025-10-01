# bouncing_ball_fixed.vpy - Versión corregida con sintaxis simplificada

META TITLE = "BOUNCING BALL"
META MUSIC = "music1"
META COPYRIGHT = "g GCE 2025"

# Variables globales para posición de la pelota
var ball_x = 0
var ball_y = 0
var vel_x = 3
var vel_y = 2

def main():
    # Actualizar posición de la pelota
    ball_x = ball_x + vel_x
    ball_y = ball_y + vel_y
    
    # Rebote en bordes horizontales
    if ball_x <= -100:
        ball_x = -100
        vel_x = -vel_x
    
    if ball_x >= 100:
        ball_x = 100
        vel_x = -vel_x
    
    # Rebote en bordes verticales  
    if ball_y <= -80:
        ball_y = -80
        vel_y = -vel_y
        
    if ball_y >= 80:
        ball_y = 80
        vel_y = -vel_y
    
    # Dibujar la pelota como un pequeño cuadrado
    DRAW_LINE(ball_x-5, ball_y-5, ball_x+5, ball_y-5, 0x7F)
    DRAW_LINE(ball_x+5, ball_y-5, ball_x+5, ball_y+5, 0x7F)
    DRAW_LINE(ball_x+5, ball_y+5, ball_x-5, ball_y+5, 0x7F)
    DRAW_LINE(ball_x-5, ball_y+5, ball_x-5, ball_y-5, 0x7F)
