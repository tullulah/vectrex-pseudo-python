# TEST DIVISIÓN ENTERA - VPy Language 2025
# Test específico del operador // (división entera)

var test_value = 100

def main():
    test_value = 100

def loop():
    # Referencias básicas
    DRAW_LINE(-50, 0, 50, 0, 60)
    DRAW_LINE(0, -50, 0, 50, 60)
    
    # Marco
    DRAW_LINE(-80, -80, 80, -80, 40)
    DRAW_LINE(80, -80, 80, 80, 40)
    DRAW_LINE(80, 80, -80, 80, 40)
    DRAW_LINE(-80, 80, -80, -80, 40)
    
    # === TEST: DIVISIÓN ENTERA // ===
    let result = test_value // 3  # ¡Test del operador //!
    
    # Mostrar resultado como posición
    let x_pos = result / 2
    if x_pos > 60:
        x_pos = 60
    
    # Dibujar punto
    DRAW_LINE(x_pos - 3, 0, x_pos + 3, 0, 127)
    DRAW_LINE(x_pos, -3, x_pos, 3, 127)
    
    # Incrementar para siguiente frame
    test_value = test_value + 5
    if test_value > 200:
        test_value = 100
    
    # Texto: "//"
    DRAW_LINE(-30, 60, -30, 40, 100)
    DRAW_LINE(-30, 60, -20, 50, 100)
    DRAW_LINE(-20, 50, -30, 40, 100)
    DRAW_LINE(-10, 60, -10, 40, 100)
    DRAW_LINE(-10, 60, 0, 50, 100)
    DRAW_LINE(0, 50, -10, 40, 100)