# TEST SIN OPERADORES NUEVOS - VPy Language 2025
# Test usando SOLO operadores tradicionales para baseline

var counter = 0
var pos_x = 0
var pos_y = 0

def main():
    counter = 0
    pos_x = 0
    pos_y = 0

def loop():
    # Incrementar contador usando suma tradicional
    counter = counter + 1
    
    # Resetear cada 120 frames
    if counter >= 120:
        counter = 0
    
    # Calcular posiciones usando aritmética tradicional
    pos_x = counter / 2 - 30
    pos_y = SIN(counter * 6) / 4
    
    # Referencias básicas
    DRAW_LINE(-80, 0, 80, 0, 40)
    DRAW_LINE(0, -80, 0, 80, 40)
    
    # Marco
    DRAW_LINE(-100, -100, 100, -100, 30)
    DRAW_LINE(100, -100, 100, 100, 30)
    DRAW_LINE(100, 100, -100, 100, 30)
    DRAW_LINE(-100, 100, -100, -100, 30)
    
    # Objeto móvil (cuadrado)
    let size = 8
    DRAW_LINE(pos_x - size, pos_y - size, pos_x + size, pos_y - size, 127)
    DRAW_LINE(pos_x + size, pos_y - size, pos_x + size, pos_y + size, 127)
    DRAW_LINE(pos_x + size, pos_y + size, pos_x - size, pos_y + size, 127)
    DRAW_LINE(pos_x - size, pos_y + size, pos_x - size, pos_y - size, 127)
    
    # Trayectoria adicional
    let trail_x = counter / 3 - 40
    let trail_y = COS(counter * 4) / 6
    DRAW_LINE(trail_x - 2, trail_y, trail_x + 2, trail_y, 60)
    
    # Texto: "OK"
    DRAW_LINE(-20, 70, -20, 50, 100)
    DRAW_LINE(-20, 70, -10, 70, 100)
    DRAW_LINE(-10, 70, -10, 50, 100)
    DRAW_LINE(-10, 50, -20, 50, 100)
    DRAW_LINE(0, 70, 0, 50, 100)
    DRAW_LINE(0, 60, 10, 50, 100)
    DRAW_LINE(10, 70, 10, 50, 100)