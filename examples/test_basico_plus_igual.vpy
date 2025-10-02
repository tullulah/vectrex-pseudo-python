# TEST BÁSICO += - VPy Language 2025
# Test minimalista para diagnosticar el operador +=

var x = 0

def main():
    x = 0

def loop():
    # Marco de referencia simple
    DRAW_LINE(-100, -100, 100, -100, 50)
    DRAW_LINE(100, -100, 100, 100, 50)
    DRAW_LINE(100, 100, -100, 100, 50)
    DRAW_LINE(-100, 100, -100, -100, 50)
    
    # Líneas centrales
    DRAW_LINE(-80, 0, 80, 0, 30)
    DRAW_LINE(0, -80, 0, 80, 30)
    
    # === ÚNICO TEST: OPERADOR += ===
    x += 2  # ¡ESTE ES EL ÚNICO OPERADOR NUEVO QUE PROBAMOS!
    
    # Resetear si sale de rango
    if x > 160:
        x = -160
    
    # Objeto móvil MUY visible
    let pos = x / 2
    
    # Cruz grande
    DRAW_LINE(pos - 20, 0, pos + 20, 0, 127)
    DRAW_LINE(pos, -20, pos, 20, 127)
    
    # Círculo alrededor
    DRAW_LINE(pos - 15, -15, pos + 15, -15, 80)
    DRAW_LINE(pos + 15, -15, pos + 15, 15, 80)
    DRAW_LINE(pos + 15, 15, pos - 15, 15, 80)
    DRAW_LINE(pos - 15, 15, pos - 15, -15, 80)
    
    # Indicador de que += está funcionando (punto que parpadea)
    if x < 80:
        DRAW_LINE(-90, 90, -85, 90, 127)
        DRAW_LINE(-90, 85, -85, 85, 127)