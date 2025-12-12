# Demo de Assets - Vector Graphics y Music
# Muestra cómo usar archivos .vec y .vmus

META TITLE = "ASSETS DEMO"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

def main():
    # Inicialización
    let player_x = 0
    let player_y = 0
    let rotation = 0
    
    # Cargar y reproducir música de fondo
    PLAY_MUSIC("theme")

def loop():
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
    
    # Animación simple de rotación
    rotation = rotation + 1
    if rotation > 127:
        rotation = 0
