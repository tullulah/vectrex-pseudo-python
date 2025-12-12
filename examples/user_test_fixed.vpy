# Demo de Assets - Vector Graphics y Music
# Muestra cómo usar archivos .vec y .vmus

META TITLE = "ASSETS DEMO"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

def main():
    # Inicialización - sin código por ahora
    SET_INTENSITY(127)

def loop():
    # Variables locales de loop (se reinicializan cada frame)
    # NOTA: VPy no soporta variables compartidas entre main() y loop()
    # Cada función tiene su propio scope
    let player_x = 0
    let player_y = 0
    
    # Limpiar pantalla y centrar origen
    WAIT_RECAL()
    SET_ORIGIN()
    
    # Dibujar nave del jugador usando asset vectorial
    SET_INTENSITY(255)
    MOVE(player_x, player_y)
    DRAW_VECTOR("player")
    
    # Dibujar enemigos en diferentes posiciones
    SET_INTENSITY(200)
    MOVE(-60, 40)
    DRAW_VECTOR("enemy")
    
    MOVE(60, 40)
    DRAW_VECTOR("enemy")
    
    # Dibujar bala
    SET_INTENSITY(180)
    MOVE(0, -30)
    DRAW_VECTOR("bullet")
    
    # Título y controles
    SET_INTENSITY(150)
    PRINT_TEXT(-60, 90, "ASSETS DEMO")
    PRINT_TEXT(-60, 80, "PLAYER - ENEMIES - BULLET")
    PRINT_TEXT(-60, -90, "MUSIC: THEME.VMUS")

