META TITLE = "Enemy Movement Test"

# 3 enemigos con arrays
var enemy_x = [100, -90, 50]
var enemy_y = [60, 0, -40]
var enemy_dx = [-2, 2, -3]
var enemy_active = [1, 1, 1]

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Movimiento Enemy 0
    if enemy_active[0] > 0:
        let new_x = enemy_x[0] + enemy_dx[0]
        if new_x < -100:
            enemy_dx[0] = 2
        if new_x > 100:
            enemy_dx[0] = -2
        enemy_x[0] = new_x
    
    # Movimiento Enemy 1
    if enemy_active[1] > 0:
        let new_x = enemy_x[1] + enemy_dx[1]
        if new_x < -100:
            enemy_dx[1] = 2
        if new_x > 100:
            enemy_dx[1] = -2
        enemy_x[1] = new_x
    
    # Movimiento Enemy 2
    if enemy_active[2] > 0:
        let new_x = enemy_x[2] + enemy_dx[2]
        if new_x < -100:
            enemy_dx[2] = 2
        if new_x > 100:
            enemy_dx[2] = -2
        enemy_x[2] = new_x
    
    # Dibujar líneas verticales en las posiciones de los enemigos
    DRAW_LINE(enemy_x[0], enemy_y[0], enemy_x[0], enemy_y[0] + 10, 127)
    DRAW_LINE(enemy_x[1], enemy_y[1], enemy_x[1], enemy_y[1] + 10, 127)
    DRAW_LINE(enemy_x[2], enemy_y[2], enemy_x[2], enemy_y[2] + 10, 127)
