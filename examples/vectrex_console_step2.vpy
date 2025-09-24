META TITLE = "VECTREX STEP2"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Paso 2: carcasa + marco de pantalla (rectángulo grande interior)
# Verifica si al añadir el segundo polígono aparecen artefactos nuevos.

const I_STRONG = 0x5F

# Carcasa exterior
# (-83,75) (83,75) (95,63) (95,-63) (83,-75) (-83,-75) (-95,-63) (-95,63)
# Marco pantalla: (-70,60) (70,60) (70,-40) (-70,-40)

def main():
    DRAW_POLYGON(8, I_STRONG, -83,75, 83,75, 95,63, 95,-63, 83,-75, -83,-75, -95,-63, -95,63)
    DRAW_POLYGON(4, I_STRONG, -70,60, 70,60, 70,-40, -70,-40)
