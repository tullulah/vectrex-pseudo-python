# TEST SIMPLE OPERADORES - VPy Language 2025
# Test ultra-simplificado para diagnosticar problemas con nuevos operadores

# Una sola variable global para el test
var test_counter = 0

def main():
    test_counter = 0

def loop():
    # Líneas de referencia básicas
    DRAW_LINE(-50, 0, 50, 0, 60)    # Línea horizontal
    DRAW_LINE(0, -50, 0, 50, 60)    # Línea vertical
    
    # Marco simple
    DRAW_LINE(-80, -80, 80, -80, 40)
    DRAW_LINE(80, -80, 80, 80, 40)
    DRAW_LINE(80, 80, -80, 80, 40)
    DRAW_LINE(-80, 80, -80, -80, 40)
    
    # === TEST 1: OPERADOR += ===
    test_counter += 1  # ¡Aquí está el test del operador +=!
    
    # Dibujar punto que se mueve basado en test_counter
    let x_pos = test_counter / 2
    let y_pos = 0
    
    # Limitar posición para que no salga de pantalla
    if x_pos > 60:
        test_counter = 0
    
    # Dibujar punto móvil (pequeña cruz)
    DRAW_LINE(x_pos - 3, y_pos, x_pos + 3, y_pos, 127)
    DRAW_LINE(x_pos, y_pos - 3, x_pos, y_pos + 3, 127)
    
    # Texto indicativo: "+="
    DRAW_LINE(-30, 60, -10, 60, 100)  # Línea horizontal
    DRAW_LINE(-20, 70, -20, 50, 100)  # Línea vertical
    DRAW_LINE(0, 60, 20, 60, 100)     # Signo igual