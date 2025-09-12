META TITLE = "VDBG MIN"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Debug mínimo: texto + pequeño cuadrado 20x20 centrado cerca del origen.
# Sirve para comprobar si el pipeline básico (Wait_Recal, Intensity, Reset0Ref, Moveto/Draw) funciona.

const I = 0x5F

def main():
    PRINT_TEXT(-20, 0, "DBG")
    # Pequeño cuadrado: (0,0)->(20,0)->(20,20)->(0,20)
    DRAW_POLYGON(4, I, 0,0, 20,0, 20,20, 0,20)
