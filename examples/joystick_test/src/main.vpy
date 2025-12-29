META TITLE = "JOY TEST"

# Posición del triángulo
triangle_x = 0
triangle_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    # Leer joystick analógico (valores -127 a +127)
    joy_x = J1_X()
    joy_y = J1_Y()
    
    # Actualizar posición del triángulo
    # Dividir por 2 para movimiento más suave
    triangle_x = joy_x / 2
    triangle_y = joy_y / 2
    
    # Mostrar valores de debug
    DEBUG_PRINT(joy_x)
    
    # Dibujar triángulo en la posición del joystick
    DRAW_VECTOR("tri", triangle_x, triangle_y)


