META TITLE = "SHAPE COMP"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Demo de composición de formas vectoriales usando macros:
#  - Hexágono central
#  - Triángulo superior como "flecha"
#  - Cuadrado inferior
#  - Círculo interior y anillo exterior segmentado
#  - Arcos laterales decorativos
#  - Espiral tipo galaxia a la izquierda
#  - Texto de cabecera
# Todas las figuras usan un solo Reset0Ref por figura (optimización en macros).

const I_FULL = 0x5F
const I_DIM  = 0x30
const I_LOW  = 0x18

# Coordenadas base
const CX = 0
const CY = 0

def main():
    PRINT_TEXT(-70, 70, "VECTOR DEMO")

    # Hexágono central (radio ~28)
    DRAW_POLYGON(6, I_FULL,  28, 0,  14, 24,  -14, 24,  -28, 0,  -14, -24, 14, -24)

    # Triángulo superior (punta arriba)
    DRAW_POLYGON(3, I_FULL, 0, 55, -25, 5, 25, 5)

    # Cuadrado inferior (semilado 22)
    DRAW_POLYGON(4, I_FULL, -22, -55, 22, -55, 22, -11, -22, -11)

    # Círculo interior (diámetro 40) y anillo exterior segmentado (diámetro 80)
    DRAW_CIRCLE(0, 0, 40, I_DIM)
    DRAW_CIRCLE_SEG(48, 0, 0, 80, I_LOW)

    # Arcos laterales (derecha e izquierda) 180° cada uno
    DRAW_ARC(32, 90, 0, 35, -90, 180, I_FULL)   # derecha
    DRAW_ARC(32, -90, 0, 35, 90, 180, I_FULL)   # izquierda

    # Espiral (galaxia) a la izquierda superior
    DRAW_SPIRAL(96, -110, 60, 5, 55, 2, I_DIM)

    # Espiral pequeña decorativa abajo derecha
    DRAW_SPIRAL(64, 100, -60, 4, 30, 1, I_DIM)

    # Círculo pequeño resaltando el centro
    DRAW_CIRCLE_SEG(24, 0, 0, 16, I_FULL)
