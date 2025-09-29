META TITLE = "VECTREX" 
META COPYRIGHT = "g GCE 1998"
META MUSIC = "music1"

# Demo: Dibujo estilizado de una consola Vectrex usando macros de formas.
# Ajustado: carcasa más rectangular (menos cuadrada) aumentando altura relativa y reduciendo biseles laterales.

const I_STRONG = 0x5F
const I_MED    = 0x40
const I_DIM    = 0x28

# Nueva carcasa rectangular:
# Queremos aspecto más alto: altura ~180 (y -90..90) y algo menos de bisel horizontal.
# Puntos (CCW) ajustados manualmente:
# (-78,90) (78,90) (100,70) (100,-70) (78,-90) (-78,-90) (-100,-70) (-100,70)

var stage = 0  # variable global (contador)

def main():
    stage = stage + 1
    if stage > 400:
        stage = 400

    if stage >= 20:
        PRINT_TEXT(-60, 96, "VECTREX")  # subimos texto acorde a nueva altura

    # 0: Carcasa exterior (nueva forma rectangular)
    if stage >= 0:
        DRAW_POLYGON(8, I_STRONG, -78,90, 78,90, 100,70, 100,-70, 78,-90, -78,-90, -100,-70, -100,70)

    # 40: Marco de pantalla (subido un poco para centrar verticalmente)
    if stage >= 40:
        DRAW_POLYGON(4, I_STRONG, -72,68, 72,68, 72,-42, -72,-42)

    # 80: Interior (ajustado)
    if stage >= 80:
        DRAW_POLYGON(4, I_MED, -64,58, 64,58, 64,-34, -64,-34)

    # 120: Panel inferior (bajado algo)
    if stage >= 120:
        DRAW_POLYGON(4, I_STRONG, -72,-42, 72,-42, 72,-78, -72,-78)

    # 160: Base joystick (recolocada un poco más abajo)
    if stage >= 160:
        DRAW_CIRCLE_SEG(32, -50, -64, 28, I_MED)

    # 200: Mango joystick
    if stage >= 200:
        DRAW_CIRCLE(-50, -64, 10, I_STRONG)

    # 240: Botones (bajados en consonancia)
    if stage >= 240:
        DRAW_CIRCLE(40, -58, 12, I_MED)
        DRAW_CIRCLE(58, -56, 12, I_MED)
        DRAW_CIRCLE(40, -74, 12, I_MED)
        DRAW_CIRCLE(58, -72, 12, I_MED)
       
    # Decorativos extra desactivados por ahora
    # (Arco / Espiral / Círculo central)



