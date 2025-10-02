# TEST DEFINITIVO VPy - Prueba completa de características

# Variables globales
var counter = 0
var direction = 1
var ball_x = 0
var ball_y = 0
var ball_vx = 3
var ball_vy = 2
var intensity_level = 80
var test_mode = 0

def main():
    let dummy = 0

def loop():
    # Incrementar contador principal
    counter = counter + 1
    
    # TEST 1: Comparaciones IF/ELIF/ELSE básicas
    if counter < 50:
        # Línea horizontal superior en rojo (intensidad baja)
        DRAW_LINE(-80, 60, 80, 60, 40)
        test_mode = 1
    elif counter < 100:
        # Línea vertical izquierda en verde (intensidad media)
        DRAW_LINE(-80, 60, -80, -60, 70)
        test_mode = 2
    else:
        # Línea diagonal en azul (intensidad alta)
        DRAW_LINE(-80, -60, 80, 60, 100)
        test_mode = 3
        
    # TEST 2: Reset cíclico con comparación
    if counter > 120:
        counter = 0
        direction = direction * -1  # Cambiar dirección
        
    # TEST 3: Física de rebote con múltiples comparaciones
    ball_x = ball_x + ball_vx
    ball_y = ball_y + ball_vy
    
    # Rebote en bordes horizontales
    if ball_x > 100:
        ball_vx = -3
        intensity_level = 95  # Brillo máximo en rebote
    elif ball_x < -100:
        ball_vx = 3
        intensity_level = 95
        
    # Rebote en bordes verticales  
    if ball_y > 80:
        ball_vy = -2
        intensity_level = 95
    elif ball_y < -80:
        ball_vy = 2
        intensity_level = 95
        
    # TEST 4: Degradado de intensidad
    if intensity_level > 50:
        intensity_level = intensity_level - 1
        
    # TEST 5: Dibujo condicional múltiple según modo de test
    if test_mode == 1:
        # Modo 1: Cruz centrada
        DRAW_LINE(-20, 0, 20, 0, intensity_level)
        DRAW_LINE(0, -20, 0, 20, intensity_level)
    elif test_mode == 2:
        # Modo 2: Cuadrado rotado
        DRAW_LINE(-15, -15, 15, -15, intensity_level)
        DRAW_LINE(15, -15, 15, 15, intensity_level)
        DRAW_LINE(15, 15, -15, 15, intensity_level)
        DRAW_LINE(-15, 15, -15, -15, intensity_level)
    elif test_mode == 3:
        # Modo 3: Estrella
        DRAW_LINE(0, -25, -12, 8, intensity_level)
        DRAW_LINE(-12, 8, 12, 8, intensity_level)
        DRAW_LINE(12, 8, 0, -25, intensity_level)
        
    # TEST 6: Pelota rebotante con trail
    if counter % 2 == 0:  # Solo cada 2 frames para efecto trail
        # Dibujar pelota principal
        DRAW_LINE(ball_x - 3, ball_y - 3, ball_x + 3, ball_y - 3, intensity_level)
        DRAW_LINE(ball_x + 3, ball_y - 3, ball_x + 3, ball_y + 3, intensity_level)
        DRAW_LINE(ball_x + 3, ball_y + 3, ball_x - 3, ball_y + 3, intensity_level)
        DRAW_LINE(ball_x - 3, ball_y + 3, ball_x - 3, ball_y - 3, intensity_level)
        
    # TEST 7: Comparaciones complejas con operadores aritméticos
    if ball_x + ball_y > 50:
        # Línea de conexión cuando suma es alta
        DRAW_LINE(0, 0, ball_x, ball_y, 60)
    elif ball_x - ball_y > 30:
        # Línea horizontal cuando diferencia es alta
        DRAW_LINE(ball_x - 10, ball_y, ball_x + 10, ball_y, 60)
        
    # TEST 8: Bordes del área de juego
    # Marco exterior siempre visible
    DRAW_LINE(-100, -80, 100, -80, 30)   # Borde inferior
    DRAW_LINE(100, -80, 100, 80, 30)     # Borde derecho
    DRAW_LINE(100, 80, -100, 80, 30)     # Borde superior
    DRAW_LINE(-100, 80, -100, -80, 30)   # Borde izquierdo