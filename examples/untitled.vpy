META TITLE = "ROTATING LINE"


def main():
    a =1

def loop():
    global angle = 0
    SET_INTENSITY(255)
    MOVE(0, 0)
    # Calcula las coordenadas finales de la línea
    x = 50 * cos(angle)
    y = 50 * sin(angle)
    DRAW_TO(x, y)
    # Actualiza el ángulo
    angle = angle + 0.01
    if angle > 2 * PI:
        angle = 0
