# TEST POLÍGONOS VPy - Demostración visual con polígonos regulares

# Variables globales
var counter = 0
var direction = 1
var ball_x = 0
var ball_y = 0
var ball_vx = 2
var ball_vy = 1
var intensity = 80
var test_mode = 0

def main():
    let dummy = 0

def loop():
    # Incrementar contador principal
    counter = counter + 1
    
    # TEST 1: Polígonos según contador (cambio cada 60 frames = ~1 segundo)
    if counter < 60:
        # TRIÁNGULO (3 lados)
        DRAW_LINE(0, 30, -26, -15, intensity)     # Lado 1
        DRAW_LINE(-26, -15, 26, -15, intensity)   # Lado 2
        DRAW_LINE(26, -15, 0, 30, intensity)     # Lado 3
        test_mode = 3
    elif counter < 120:
        # CUADRADO (4 lados)
        DRAW_LINE(-20, -20, 20, -20, intensity)   # Inferior
        DRAW_LINE(20, -20, 20, 20, intensity)    # Derecha
        DRAW_LINE(20, 20, -20, 20, intensity)    # Superior
        DRAW_LINE(-20, 20, -20, -20, intensity)  # Izquierda
        test_mode = 4
    elif counter < 180:
        # PENTÁGONO (5 lados) - aproximado
        DRAW_LINE(0, 25, 24, 8, intensity)       # Lado 1
        DRAW_LINE(24, 8, 15, -20, intensity)     # Lado 2
        DRAW_LINE(15, -20, -15, -20, intensity)  # Lado 3
        DRAW_LINE(-15, -20, -24, 8, intensity)   # Lado 4
        DRAW_LINE(-24, 8, 0, 25, intensity)      # Lado 5
        test_mode = 5
    else:
        # HEXÁGONO (6 lados)
        DRAW_LINE(22, 0, 11, 19, intensity)      # Lado 1
        DRAW_LINE(11, 19, -11, 19, intensity)    # Lado 2
        DRAW_LINE(-11, 19, -22, 0, intensity)    # Lado 3
        DRAW_LINE(-22, 0, -11, -19, intensity)   # Lado 4
        DRAW_LINE(-11, -19, 11, -19, intensity)  # Lado 5
        DRAW_LINE(11, -19, 22, 0, intensity)     # Lado 6
        test_mode = 6
        
    # Reset cíclico cada 240 frames (~4 segundos)
    if counter >= 240:
        counter = 0
        direction = direction * -1
        
    # TEST 2: Pelota rebotante (cuadradito pequeño)
    ball_x = ball_x + ball_vx
    ball_y = ball_y + ball_vy
    
    # Rebote en bordes
    if ball_x > 90:
        ball_vx = -2
        intensity = 100  # Brillo máximo en rebote
    elif ball_x < -90:
        ball_vx = 2
        intensity = 100
        
    if ball_y > 70:
        ball_vy = -1
        intensity = 100
    elif ball_y < -70:
        ball_vy = 1
        intensity = 100
        
    # Degradado de intensidad
    if intensity > 60:
        intensity = intensity - 1
        
    # Dibujar pelota (cuadradito de 4x4)
    DRAW_LINE(ball_x - 2, ball_y - 2, ball_x + 2, ball_y - 2, intensity)
    DRAW_LINE(ball_x + 2, ball_y - 2, ball_x + 2, ball_y + 2, intensity)
    DRAW_LINE(ball_x + 2, ball_y + 2, ball_x - 2, ball_y + 2, intensity)
    DRAW_LINE(ball_x - 2, ball_y + 2, ball_x - 2, ball_y - 2, intensity)
    
    # TEST 3: Líneas de conexión dinámicas
    if test_mode == 3:
        # Triángulo: línea desde centro a pelota cada 4 frames
        if counter % 4 == 0:
            DRAW_LINE(0, 0, ball_x, ball_y, 40)
    elif test_mode == 4:
        # Cuadrado: líneas en cruz desde esquinas
        if ball_x > 0:
            DRAW_LINE(-20, -20, 20, 20, 30)  # Diagonal
        if ball_y > 0:
            DRAW_LINE(-20, 20, 20, -20, 30)  # Diagonal inversa
    elif test_mode == 5:
        # Pentágono: círculo interno cada 3 frames
        if counter % 3 == 0:
            # Círculo aproximado con 8 líneas
            DRAW_LINE(12, 0, 8, 8, 35)
            DRAW_LINE(8, 8, 0, 12, 35)
            DRAW_LINE(0, 12, -8, 8, 35)
            DRAW_LINE(-8, 8, -12, 0, 35)
            DRAW_LINE(-12, 0, -8, -8, 35)
            DRAW_LINE(-8, -8, 0, -12, 35)
            DRAW_LINE(0, -12, 8, -8, 35)
            DRAW_LINE(8, -8, 12, 0, 35)
    elif test_mode == 6:
        # Hexágono: estrella interna
        if counter % 2 == 0:
            DRAW_LINE(0, -15, 0, 15, 45)     # Vertical
            DRAW_LINE(-13, -7, 13, 7, 45)    # Diagonal 1
            DRAW_LINE(-13, 7, 13, -7, 45)    # Diagonal 2
    
    # Marco exterior para referencia (siempre visible)
    DRAW_LINE(-95, -75, 95, -75, 25)   # Inferior
    DRAW_LINE(95, -75, 95, 75, 25)     # Derecha
    DRAW_LINE(95, 75, -95, 75, 25)     # Superior
    DRAW_LINE(-95, 75, -95, -75, 25)   # Izquierda