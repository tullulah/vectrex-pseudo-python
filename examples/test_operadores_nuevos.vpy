# TEST OPERADORES NUEVOS - VPy Language 2025
# Prueba sistemática de cada operador con objeto visual simple
# Cada operador se prueba durante 2 segundos con indicación en pantalla

# Variables globales para el test
var test_frame = 0
var test_phase = 0
var object_x = 0
var object_y = 0
var object_size = 20
var test_value = 100

def main():
    # Inicialización
    test_frame = 0
    test_phase = 0
    object_x = 0
    object_y = 0
    object_size = 20
    test_value = 100

def loop():
    # Incrementar frame contador
    test_frame = test_frame + 1  # Usando suma tradicional para evitar problemas
    
    # Cambiar fase cada 120 frames (2 segundos a 60 FPS)
    if test_frame >= 120:
        test_frame = 0
        test_phase = test_phase + 1
        if test_phase >= 8:  # 8 fases total
            test_phase = 0
        # Resetear valores para cada fase
        object_x = 0
        object_y = 0
        test_value = 100
    
    # Objeto base: cuadrado en el centro (siempre visible)
    let base_x = object_x
    let base_y = object_y
    let size = object_size
    
    # Marco del objeto
    DRAW_LINE(base_x - size, base_y - size, base_x + size, base_y - size, 80)
    DRAW_LINE(base_x + size, base_y - size, base_x + size, base_y + size, 80)
    DRAW_LINE(base_x + size, base_y + size, base_x - size, base_y + size, 80)
    DRAW_LINE(base_x - size, base_y + size, base_x - size, base_y - size, 80)
    
    # Línea central para referencia
    DRAW_LINE(-127, 0, 127, 0, 30)
    DRAW_LINE(0, -127, 0, 127, 30)
    
    # === FASE 0: ESTADO INICIAL ===
    if test_phase == 0:
        # Texto: "INICIO"
        DRAW_LINE(-30, 60, -10, 60, 100)  # I
        DRAW_LINE(-30, 60, -30, 40, 100)
        DRAW_LINE(-30, 40, -10, 40, 100)
        # Objeto estático en centro
        # object_x y object_y permanecen en 0
    
    # === FASE 1: PROBANDO += ===
    if test_phase == 1:
        # Texto: "+="
        DRAW_LINE(-20, 60, 0, 60, 100)
        DRAW_LINE(-10, 70, -10, 50, 100)
        DRAW_LINE(10, 60, 30, 60, 100)
        
        # Movimiento usando +=
        object_x += 1  # ¡PRIMER TEST DEL OPERADOR +=!
    
    # === FASE 2: PROBANDO -= ===
    if test_phase == 2:
        # Texto: "-="
        DRAW_LINE(-20, 60, 0, 60, 100)
        DRAW_LINE(10, 60, 30, 60, 100)
        
        # Movimiento usando -=
        object_y -= 1  # ¡TEST DEL OPERADOR -=!
    
    # === FASE 3: PROBANDO *= ===
    if test_phase == 3:
        # Texto: "*="
        DRAW_LINE(-20, 70, -10, 50, 100)
        DRAW_LINE(-10, 70, -20, 50, 100)
        DRAW_LINE(10, 60, 30, 60, 100)
        
        # Cambio de tamaño usando *=
        object_size *= 2
        object_size = object_size / 2  # Normalizar para evitar crecimiento infinito
        if object_size > 40:
            object_size = 20
    
    # === FASE 4: PROBANDO /= ===
    if test_phase == 4:
        # Texto: "/="
        DRAW_LINE(-20, 70, -20, 50, 100)
        DRAW_LINE(-20, 70, -10, 60, 100)
        DRAW_LINE(-10, 60, -20, 50, 100)
        DRAW_LINE(10, 60, 30, 60, 100)
        
        # Movimiento usando /=
        test_value /= 2
        object_x = test_value / 10  # Convertir a posición
        if test_value < 10:
            test_value = 100
    
    # === FASE 5: PROBANDO %= ===
    if test_phase == 5:
        # Texto: "%="
        DRAW_LINE(-25, 70, -25, 60, 100)
        DRAW_LINE(-25, 70, -15, 70, 100)
        DRAW_LINE(-15, 70, -15, 60, 100)
        DRAW_LINE(-15, 60, -10, 50, 100)
        DRAW_LINE(10, 60, 30, 60, 100)
        
        # Movimiento usando %=
        test_value %= 50
        object_y = test_value - 25  # Centrar alrededor de 0
        test_value = test_value + 3  # Incrementar para siguiente iteración
    
    # === FASE 6: PROBANDO // (DIVISIÓN ENTERA) ===
    if test_phase == 6:
        # Texto: "//"
        DRAW_LINE(-25, 70, -25, 50, 100)
        DRAW_LINE(-25, 70, -15, 60, 100)
        DRAW_LINE(-15, 60, -25, 50, 100)
        DRAW_LINE(-5, 70, -5, 50, 100)
        DRAW_LINE(-5, 70, 5, 60, 100)
        DRAW_LINE(5, 60, -5, 50, 100)
        
        # Movimiento usando división entera
        let temp_calc = test_frame * 5
        object_x = temp_calc // 3  # ¡TEST DE DIVISIÓN ENTERA //!
        object_y = temp_calc // 4
    
    # === FASE 7: PROBANDO //= (DIVISIÓN ENTERA COMPUESTA) ===
    if test_phase == 7:
        # Texto: "//="
        DRAW_LINE(-30, 70, -30, 50, 100)
        DRAW_LINE(-30, 70, -20, 60, 100)
        DRAW_LINE(-20, 60, -30, 50, 100)
        DRAW_LINE(-15, 70, -15, 50, 100)
        DRAW_LINE(-15, 70, -5, 60, 100)
        DRAW_LINE(-5, 60, -15, 50, 100)
        DRAW_LINE(5, 60, 25, 60, 100)
        
        # Movimiento usando //=
        test_value //= 2  # ¡TEST DEL OPERADOR //=!
        object_x = test_value / 5
        if test_value < 5:
            test_value = 120
    
    # Indicador de fase (puntos en la esquina)
    let indicator_x = 100
    let indicator_y = 90
    let i = 0
    
    # Mostrar puntos según la fase actual
    if test_phase >= 0:
        DRAW_LINE(indicator_x, indicator_y, indicator_x + 2, indicator_y, 100)
    if test_phase >= 1:
        DRAW_LINE(indicator_x + 5, indicator_y, indicator_x + 7, indicator_y, 100)
    if test_phase >= 2:
        DRAW_LINE(indicator_x + 10, indicator_y, indicator_x + 12, indicator_y, 100)
    if test_phase >= 3:
        DRAW_LINE(indicator_x + 15, indicator_y, indicator_x + 17, indicator_y, 100)
    if test_phase >= 4:
        DRAW_LINE(indicator_x, indicator_y - 5, indicator_x + 2, indicator_y - 5, 100)
    if test_phase >= 5:
        DRAW_LINE(indicator_x + 5, indicator_y - 5, indicator_x + 7, indicator_y - 5, 100)
    if test_phase >= 6:
        DRAW_LINE(indicator_x + 10, indicator_y - 5, indicator_x + 12, indicator_y - 5, 100)
    if test_phase >= 7:
        DRAW_LINE(indicator_x + 15, indicator_y - 5, indicator_x + 17, indicator_y - 5, 100)