META TITLE = "VECTREX STEP6"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Paso 6: añade mango joystick (círculo pequeño)

const I_STRONG = 0x5F
const I_MED = 0x40

def main():
    DRAW_POLYGON(8, I_STRONG, -83,75, 83,75, 95,63, 95,-63, 83,-75, -83,-75, -95,-63, -95,63)
    DRAW_POLYGON(4, I_STRONG, -70,60, 70,60, 70,-40, -70,-40)
    DRAW_POLYGON(4, I_MED, -62,52, 62,52, 62,-32, -62,-32)
    DRAW_POLYGON(4, I_STRONG, -70,-40, 70,-40, 70,-68, -70,-68)
    DRAW_CIRCLE_SEG(40, -50, -54, 28, I_MED)
    DRAW_CIRCLE(-50, -54, 10, I_STRONG)
