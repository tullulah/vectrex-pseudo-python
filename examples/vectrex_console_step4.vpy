META TITLE = "VECTREX STEP4"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Paso 4: añade panel inferior
# Panel inferior: (-70,-40) (70,-40) (70,-68) (-70,-68)

const I_STRONG = 0x5F
const I_MED = 0x40

def main():
    DRAW_POLYGON(8, I_STRONG, -83,75, 83,75, 95,63, 95,-63, 83,-75, -83,-75, -95,-63, -95,63)
    DRAW_POLYGON(4, I_STRONG, -70,60, 70,60, 70,-40, -70,-40)
    DRAW_POLYGON(4, I_MED, -62,52, 62,52, 62,-32, -62,-32)
    DRAW_POLYGON(4, I_STRONG, -70,-40, 70,-40, 70,-68, -70,-68)
