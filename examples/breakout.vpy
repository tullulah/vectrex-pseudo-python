# BREAKOUT VPy - Juego automático de pelota y ladrillos

# Variables globales del juego
var ball_x = 0
var ball_y = -100
var ball_vx = 3
var ball_vy = 3
var score = 0

# Estado de los ladrillos (1=existe, 0=destruido)
# Fila superior (y=120) - EN EL TECHO
var brick1 = 1
var brick2 = 1
var brick3 = 1
var brick4 = 1
var brick5 = 1
var brick6 = 1

# Fila media (y=100)
var brick7 = 1
var brick8 = 1
var brick9 = 1
var brick10 = 1
var brick11 = 1
var brick12 = 1

# Fila inferior (y=80)
var brick13 = 1
var brick14 = 1
var brick15 = 1
var brick16 = 1
var brick17 = 1
var brick18 = 1

# Variables de efecto visual
var flash_timer = 0
var intensity = 80

def main():
    let dummy = 0

def loop():
    # Mover pelota
    ball_x = ball_x + ball_vx
    ball_y = ball_y + ball_vy
    
    # Rebote en paredes laterales
    if ball_x > 120:
        ball_vx = -3
        flash_timer = 5
    elif ball_x < -120:
        ball_vx = 3
        flash_timer = 5
    
    # Rebote en techo
    if ball_y > 120:
        ball_vy = -3
        flash_timer = 5
    
    # Rebote en suelo
    if ball_y < -120:
        ball_vy = 3
        flash_timer = 5
    
    # COLISIONES CON LADRILLOS
    # Fila superior (y=120, EN EL TECHO)
    if ball_y > 115 and ball_y < 125:
        if ball_x > -120 and ball_x < -80 and brick1 == 1:
            brick1 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > -80 and ball_x < -40 and brick2 == 1:
            brick2 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > -40 and ball_x < 0 and brick3 == 1:
            brick3 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 0 and ball_x < 40 and brick4 == 1:
            brick4 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 40 and ball_x < 80 and brick5 == 1:
            brick5 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 80 and ball_x < 120 and brick6 == 1:
            brick6 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
    
    # Fila media (y=100)
    if ball_y > 95 and ball_y < 105:
        if ball_x > -120 and ball_x < -80 and brick7 == 1:
            brick7 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > -80 and ball_x < -40 and brick8 == 1:
            brick8 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > -40 and ball_x < 0 and brick9 == 1:
            brick9 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 0 and ball_x < 40 and brick10 == 1:
            brick10 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 40 and ball_x < 80 and brick11 == 1:
            brick11 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 80 and ball_x < 120 and brick12 == 1:
            brick12 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
    
    # Fila inferior (y=80)
    if ball_y > 75 and ball_y < 85:
        if ball_x > -120 and ball_x < -80 and brick13 == 1:
            brick13 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > -80 and ball_x < -40 and brick14 == 1:
            brick14 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > -40 and ball_x < 0 and brick15 == 1:
            brick15 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 0 and ball_x < 40 and brick16 == 1:
            brick16 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 40 and ball_x < 80 and brick17 == 1:
            brick17 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
        elif ball_x > 80 and ball_x < 120 and brick18 == 1:
            brick18 = 0
            ball_vy = -3
            score = score + 10
            flash_timer = 10
    
    # Efectos visuales simplificados
    if flash_timer > 0:
        flash_timer = flash_timer - 1
        intensity = 100
    else:
        intensity = 70
    
    # # DIBUJAR LADRILLOS - EN EL TECHO REAL (y=120, 100, 80)
    # # Fila superior (y=120) - PEGADA AL TECHO
    if brick1 == 1:
        DRAW_LINE(-120, 115, -80, 115, intensity)
        DRAW_LINE(-80, 115, -80, 125, intensity)
        DRAW_LINE(-80, 125, -120, 125, intensity)
        DRAW_LINE(-120, 125, -120, 115, intensity)
    
    if brick2 == 1:
        DRAW_LINE(-80, 115, -40, 115, intensity)
        DRAW_LINE(-40, 115, -40, 125, intensity)
        DRAW_LINE(-40, 125, -80, 125, intensity)
        DRAW_LINE(-80, 125, -80, 115, intensity)
    
    if brick3 == 1:
        DRAW_LINE(-40, 115, 0, 115, intensity)
        DRAW_LINE(0, 115, 0, 125, intensity)
        DRAW_LINE(0, 125, -40, 125, intensity)
        DRAW_LINE(-40, 125, -40, 115, intensity)
    
    if brick4 == 1:
        DRAW_LINE(0, 115, 40, 115, intensity)
        DRAW_LINE(40, 115, 40, 125, intensity)
        DRAW_LINE(40, 125, 0, 125, intensity)
        DRAW_LINE(0, 125, 0, 115, intensity)
    
    if brick5 == 1:
        DRAW_LINE(40, 115, 80, 115, intensity)
        DRAW_LINE(80, 115, 80, 125, intensity)
        DRAW_LINE(80, 125, 40, 125, intensity)
        DRAW_LINE(40, 125, 40, 115, intensity)
    
    if brick6 == 1:
        DRAW_LINE(80, 115, 120, 115, intensity)
        DRAW_LINE(120, 115, 120, 125, intensity)
        DRAW_LINE(120, 125, 80, 125, intensity)
        DRAW_LINE(80, 125, 80, 115, intensity)
    
    # Fila media (y=100)
    if brick7 == 1:
        DRAW_LINE(-120, 95, -80, 95, intensity)
        DRAW_LINE(-80, 95, -80, 105, intensity)
        DRAW_LINE(-80, 105, -120, 105, intensity)
        DRAW_LINE(-120, 105, -120, 95, intensity)
    
    if brick8 == 1:
        DRAW_LINE(-80, 95, -40, 95, intensity)
        DRAW_LINE(-40, 95, -40, 105, intensity)
        DRAW_LINE(-40, 105, -80, 105, intensity)
        DRAW_LINE(-80, 105, -80, 95, intensity)
    
    if brick9 == 1:
        DRAW_LINE(-40, 95, 0, 95, intensity)
        DRAW_LINE(0, 95, 0, 105, intensity)
        DRAW_LINE(0, 105, -40, 105, intensity)
        DRAW_LINE(-40, 105, -40, 95, intensity)
    
    if brick10 == 1:
        DRAW_LINE(0, 95, 40, 95, intensity)
        DRAW_LINE(40, 95, 40, 105, intensity)
        DRAW_LINE(40, 105, 0, 105, intensity)
        DRAW_LINE(0, 105, 0, 95, intensity)
    
    if brick11 == 1:
        DRAW_LINE(40, 95, 80, 95, intensity)
        DRAW_LINE(80, 95, 80, 105, intensity)
        DRAW_LINE(80, 105, 40, 105, intensity)
        DRAW_LINE(40, 105, 40, 95, intensity)
    
    if brick12 == 1:
        DRAW_LINE(80, 95, 120, 95, intensity)
        DRAW_LINE(120, 95, 120, 105, intensity)
        DRAW_LINE(120, 105, 80, 105, intensity)
        DRAW_LINE(80, 105, 80, 95, intensity)
    
    # Fila inferior (y=80)
    if brick13 == 1:
        DRAW_LINE(-120, 75, -80, 75, intensity)
        DRAW_LINE(-80, 75, -80, 85, intensity)
        DRAW_LINE(-80, 85, -120, 85, intensity)
        DRAW_LINE(-120, 85, -120, 75, intensity)
    
    if brick14 == 1:
        DRAW_LINE(-80, 75, -40, 75, intensity)
        DRAW_LINE(-40, 75, -40, 85, intensity)
        DRAW_LINE(-40, 85, -80, 85, intensity)
        DRAW_LINE(-80, 85, -80, 75, intensity)
    
    if brick15 == 1:
        DRAW_LINE(-40, 75, 0, 75, intensity)
        DRAW_LINE(0, 75, 0, 85, intensity)
        DRAW_LINE(0, 85, -40, 85, intensity)
        DRAW_LINE(-40, 85, -40, 75, intensity)
    
    if brick16 == 1:
        DRAW_LINE(0, 75, 40, 75, intensity)
        DRAW_LINE(40, 75, 40, 85, intensity)
        DRAW_LINE(40, 85, 0, 85, intensity)
        DRAW_LINE(0, 85, 0, 75, intensity)
    
    if brick17 == 1:
        DRAW_LINE(40, 75, 80, 75, intensity)
        DRAW_LINE(80, 75, 80, 85, intensity)
        DRAW_LINE(80, 85, 40, 85, intensity)
        DRAW_LINE(40, 85, 40, 75, intensity)
    
    if brick18 == 1:
        DRAW_LINE(80, 75, 120, 75, intensity)
        DRAW_LINE(120, 75, 120, 85, intensity)
        DRAW_LINE(120, 85, 80, 85, intensity)
        DRAW_LINE(80, 85, 80, 75, intensity)
    
    # SOLO la pelota y marco - SIN líneas extras
    DRAW_LINE(ball_x - 3, ball_y - 3, ball_x + 3, ball_y - 3, 100)
    DRAW_LINE(ball_x + 3, ball_y - 3, ball_x + 3, ball_y + 3, 100)
    DRAW_LINE(ball_x + 3, ball_y + 3, ball_x - 3, ball_y + 3, 100)
    DRAW_LINE(ball_x - 3, ball_y + 3, ball_x - 3, ball_y - 3, 100)
    
    # Marco del juego (límites reales Vectrex)
    DRAW_LINE(-127, -127, 127, -127, 30)   # Suelo
    DRAW_LINE(127, -127, 127, 127, 30)     # Pared derecha
    DRAW_LINE(127, 127, -127, 127, 30)     # Techo
    DRAW_LINE(-127, 127, -127, -127, 30)   # Pared izquierda
    
    # Reiniciar ladrillos si se destruyen todos
    if brick1 == 0 and brick2 == 0 and brick3 == 0 and brick4 == 0 and brick5 == 0 and brick6 == 0 and brick7 == 0 and brick8 == 0 and brick9 == 0 and brick10 == 0 and brick11 == 0 and brick12 == 0 and brick13 == 0 and brick14 == 0 and brick15 == 0 and brick16 == 0 and brick17 == 0 and brick18 == 0:
        # Reiniciar todos los ladrillos
        brick1 = 1
        brick2 = 1
        brick3 = 1
        brick4 = 1
        brick5 = 1
        brick6 = 1
        brick7 = 1
        brick8 = 1
        brick9 = 1
        brick10 = 1
        brick11 = 1
        brick12 = 1
        brick13 = 1
        brick14 = 1
        brick15 = 1
        brick16 = 1
        brick17 = 1
        brick18 = 1
        score = 0
        flash_timer = 20