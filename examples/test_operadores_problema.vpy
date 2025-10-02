# TEST SIMPLE OPERADORES PROBLEMÁTICOS - VPy Language 2025
# Test específico para los operadores que fallan

var test_phase = 0
var test_frame = 0
var value = 20
var obj_x = 0
var obj_y = 0

def main():
    test_phase = 0
    test_frame = 0
    value = 20
    obj_x = 0
    obj_y = 0

def loop():
    # Incrementar frame
    test_frame = test_frame + 1
    
    # Cambiar fase cada 240 frames (4 segundos)
    if test_frame >= 240:
        test_frame = 0
        test_phase = test_phase + 1
        if test_phase >= 4:
            test_phase = 0
        value = 20
        obj_x = 0
        obj_y = 0
    
    # Marco básico
    DRAW_LINE(-100, -100, 100, -100, 40)
    DRAW_LINE(100, -100, 100, 100, 40)
    DRAW_LINE(100, 100, -100, 100, 40)
    DRAW_LINE(-100, 100, -100, -100, 40)
    DRAW_LINE(-80, 0, 80, 0, 20)
    DRAW_LINE(0, -80, 0, 80, 20)
    
    # Indicador de fase (puntos)
    if test_phase >= 0:
        DRAW_LINE(-90, 90, -85, 90, 100)
    if test_phase >= 1:
        DRAW_LINE(-80, 90, -75, 90, 100)
    if test_phase >= 2:
        DRAW_LINE(-70, 90, -65, 90, 100)
    if test_phase >= 3:
        DRAW_LINE(-60, 90, -55, 90, 100)
    
    # === FASE 0: BASELINE (operadores tradicionales) ===
    if test_phase == 0:
        obj_x = test_frame / 4
        obj_y = 0
    
    # === FASE 1: TEST /= ===
    if test_phase == 1:
        if value > 2:
            value /= 2  # ¡TEST DEL OPERADOR /=!
            obj_x = value * 6
        else:
            value = 80
    
    # === FASE 2: TEST // ===  
    if test_phase == 2:
        let temp = test_frame * 2
        obj_x = temp // 4  # ¡TEST DEL OPERADOR //!
        obj_y = temp // 8
    
    # === FASE 3: TEST //= ===
    if test_phase == 3:
        if value > 3:
            value //= 2  # ¡TEST DEL OPERADOR //=!
            obj_x = value * 5
        else:
            value = 40
    
    # Cuadrado simple
    let size = 10
    DRAW_LINE(obj_x - size, obj_y - size, obj_x + size, obj_y - size, 127)
    DRAW_LINE(obj_x + size, obj_y - size, obj_x + size, obj_y + size, 127)
    DRAW_LINE(obj_x + size, obj_y + size, obj_x - size, obj_y + size, 127)
    DRAW_LINE(obj_x - size, obj_y + size, obj_x - size, obj_y - size, 127)
    
    # Punto central
    DRAW_LINE(obj_x - 1, obj_y, obj_x + 1, obj_y, 127)
    
    # Texto de fase
    let tx = -50
    let ty = 70
    
    if test_phase == 0:
        DRAW_LINE(tx, ty, tx + 20, ty, 100)  # "INICIO"
    if test_phase == 1:
        DRAW_LINE(tx, ty + 10, tx + 10, ty, 100)  # "/"
        DRAW_LINE(tx + 15, ty, tx + 25, ty, 100)  # "="
    if test_phase == 2:
        DRAW_LINE(tx, ty + 10, tx + 8, ty, 100)   # "/"
        DRAW_LINE(tx + 10, ty + 10, tx + 18, ty, 100)  # "/"
    if test_phase == 3:
        DRAW_LINE(tx, ty + 10, tx + 8, ty, 100)   # "/"
        DRAW_LINE(tx + 10, ty + 10, tx + 18, ty, 100)  # "/"
        DRAW_LINE(tx + 25, ty, tx + 35, ty, 100)  # "="