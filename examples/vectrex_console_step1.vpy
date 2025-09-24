META TITLE = "VECTREX STEP1"
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Paso 1: SOLO la carcasa exterior para comprobar que se dibuja estable.
# Si esto ya se ve "extraño" el problema está en la macro de polígono o en el backend.
# Si se ve bien, pasaremos al paso 2 añadiendo el marco (bezel) de la pantalla.

const I_STRONG = 0x5F

# Carcasa exterior (octágono con biseles) coordenadas CCW
# (-83,75) (83,75) (95,63) (95,-63) (83,-75) (-83,-75) (-95,-63) (-95,63)

def main():
    DRAW_POLYGON(8, I_STRONG, -83,75, 83,75, 95,63, 95,-63, 83,-75, -83,-75, -95,-63, -95,63)
