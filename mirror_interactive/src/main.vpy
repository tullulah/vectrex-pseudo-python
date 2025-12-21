META TITLE = "Mirror Interactive - Joystick Control"
META AUTHOR = "VPy Demo"

def main():
    SET_INTENSITY(127)

def loop():
    # Leer joystick
    joy_x = J1_X()  # -1 (left), 0 (center), +1 (right)
    joy_y = J1_Y()  # -1 (down), 0 (center), +1 (up)
    
    # Inicializar posiciones (necesario declarar antes de usar en if)
    pos_x = 64
    pos_y = 64
    
    # Calcular posición X basada en joystick
    if joy_x == -1:
        pos_x = 30
    elif joy_x == 1:
        pos_x = 98
    
    # Calcular posición Y basada en joystick
    if joy_y == -1:
        pos_y = 30
    elif joy_y == 1:
        pos_y = 98
    
    # Inicializar modo mirror
    mirror = 0
    
    # Si va a la derecha (+X), aplicar mirror X (modo 1)
    if joy_x > 0:
        mirror = 1
    
    # Si va hacia abajo (-Y), sumar mirror Y (modo +2)
    if joy_y < 0:
        mirror = mirror + 2
    
    # Dibujar nave con posición y mirror dinámicos
    DRAW_VECTOR_EX("spaceship", pos_x, pos_y, mirror)

