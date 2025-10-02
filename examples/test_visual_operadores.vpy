# TEST VISUAL OPERADORES - VPy Language 2025
# Test súper simple y visual para ver qué operador falla

# Variables globales
var test_frame = 0
var test_phase = 0
var object_x = 0
var object_y = 0
var value = 10

def main():
    test_frame = 0
    test_phase = 0
    object_x = 0
    object_y = 0
    value = 10

def loop():
    # Incrementar frame usando suma tradicional (sabemos que funciona)
    test_frame = test_frame + 1
    
    # Cambiar fase cada 180 frames (3 segundos)
    if test_frame >= 180:
        test_frame = 0
        test_phase = test_phase + 1
        if test_phase >= 7:
            test_phase = 0
        # Reset valores
        object_x = 0
        object_y = 0
        value = 10
    
    # Marco de referencia fijo (para confirmar que algo se dibuja)
    DRAW_LINE(-120, -120, 120, -120, 40)
    DRAW_LINE(120, -120, 120, 120, 40)
    DRAW_LINE(120, 120, -120, 120, 40)
    DRAW_LINE(-120, 120, -120, -120, 40)
    
    # Líneas centrales
    DRAW_LINE(-100, 0, 100, 0, 30)
    DRAW_LINE(0, -100, 0, 100, 30)
    
    # Indicador de fase (barras verticales en la parte superior)
    let bar_x = -80
    let bar_y = 100
    let bar_count = 0
    
    # Dibujar barras según la fase
    while bar_count <= test_phase:
        DRAW_LINE(bar_x + bar_count * 25, bar_y, bar_x + bar_count * 25, bar_y - 20, 127)
        bar_count = bar_count + 1
    
    # === FASE 0: BASELINE (sin operadores nuevos) ===
    # EL POLÍGONO DEBE MOVERSE DE IZQUIERDA A DERECHA
    if test_phase == 0:
        object_x = test_frame / 3 - 30
        object_y = 0
    
    # === FASE 1: OPERADOR += ===
    # EL POLÍGONO DEBE MOVERSE HACIA LA DERECHA PROGRESIVAMENTE
    if test_phase == 1:
        object_x += 1  # TEST +=
        if object_x > 60:
            object_x = -60
    
    # === FASE 2: OPERADOR -= ===
    # EL POLÍGONO DEBE MOVERSE HACIA ARRIBA PROGRESIVAMENTE
    if test_phase == 2:
        object_y -= 1  # TEST -=
        if object_y < -60:
            object_y = 60
    
    # === FASE 3: OPERADOR *= ===
    # EL POLÍGONO DEBE MOVERSE HORIZONTALMENTE CON ACELERACIÓN (CONTROLADA)
    if test_phase == 3:
        value *= 2  # TEST *=
        object_x = value / 20  # Dividir por más para ralentizar
        if value > 200:  # Límite más bajo para evitar overflow
            value = 5
    
    # === FASE 4: OPERADOR /= ===
    # EL POLÍGONO DEBE MOVERSE HORIZONTALMENTE CON DESACELERACIÓN
    if test_phase == 4:
        if value > 2:  # Evitar división que resulte en valores muy pequeños
            value /= 2  # TEST /=
            object_x = value * 8  # Multiplicar por más para hacer visible
        else:
            value = 80  # Resetear a valor seguro
    
    # === FASE 5: OPERADOR // (división entera) ===
    # EL POLÍGONO DEBE MOVERSE EN DIAGONAL
    if test_phase == 5:
        let temp = test_frame * 3  # Aumentar velocidad base
        if temp > 0:  # Verificar que no sea cero
            object_x = temp // 4  # TEST // con divisor seguro
            object_y = temp // 6  # TEST // con divisor diferente
        else:
            object_x = 0
            object_y = 0
    
    # === FASE 6: OPERADOR //= (división entera compuesta) ===
    # EL POLÍGONO DEBE MOVERSE HORIZONTALMENTE CON PATRÓN ESPECÍFICO  
    if test_phase == 6:
        if value > 4:  # Evitar que value se vuelva muy pequeño
            value //= 2  # TEST //=
            object_x = value * 4  # Multiplicar para hacer visible
        else:
            value = 60  # Resetear a valor seguro
    
    # Dibujar objeto principal (POLÍGONO DE 4 LADOS - CUADRADO SIMPLE)
    let size = 12
    
    # Cuadrado simple (no rombo) - más fácil de ver
    DRAW_LINE(object_x - size, object_y - size, object_x + size, object_y - size, 127)  # Lado superior
    DRAW_LINE(object_x + size, object_y - size, object_x + size, object_y + size, 127)  # Lado derecho
    DRAW_LINE(object_x + size, object_y + size, object_x - size, object_y + size, 127)  # Lado inferior
    DRAW_LINE(object_x - size, object_y + size, object_x - size, object_y - size, 127)  # Lado izquierdo
    
    # Punto central del polígono
    DRAW_LINE(object_x - 1, object_y, object_x + 1, object_y, 127)
    DRAW_LINE(object_x, object_y - 1, object_x, object_y + 1, 127)
    
    # Debug info: mostrar el valor actual como líneas en la esquina inferior derecha
    let debug_x = 80
    let debug_y = -90
    let debug_value = value / 10  # Escalar para visualización
    
    # Mostrar value como línea horizontal (longitud proporcional al valor)
    if debug_value > 0 and debug_value < 20:
        DRAW_LINE(debug_x, debug_y, debug_x + debug_value, debug_y, 60)
    
    # Mostrar object_x como línea vertical
    let debug_obj_x = object_x / 5  # Escalar
    if debug_obj_x > -10 and debug_obj_x < 10:
        DRAW_LINE(debug_x, debug_y, debug_x, debug_y + debug_obj_x, 80)
    
    # TEXTO DESCRIPTIVO para cada fase (simple y claro)
    let text_x = -100
    let text_y = 70
    
    if test_phase == 0:
        # "INICIO" - líneas simples
        DRAW_LINE(text_x, text_y, text_x, text_y + 20, 100)      # I
        DRAW_LINE(text_x + 10, text_y, text_x + 10, text_y + 20, 100)  # N (línea izquierda)
        DRAW_LINE(text_x + 20, text_y, text_x + 20, text_y + 20, 100)  # I
        DRAW_LINE(text_x + 30, text_y, text_x + 30, text_y + 20, 100)  # C
        DRAW_LINE(text_x + 40, text_y, text_x + 40, text_y + 20, 100)  # O (línea izquierda)
        
    if test_phase == 1:
        # "+=" - muy simple
        DRAW_LINE(text_x + 5, text_y + 10, text_x + 15, text_y + 10, 100)  # línea horizontal +
        DRAW_LINE(text_x + 10, text_y + 5, text_x + 10, text_y + 15, 100)  # línea vertical +
        DRAW_LINE(text_x + 25, text_y + 10, text_x + 35, text_y + 10, 100)  # línea = arriba
        DRAW_LINE(text_x + 25, text_y + 12, text_x + 35, text_y + 12, 100)  # línea = abajo
        
    if test_phase == 2:
        # "-=" - simple
        DRAW_LINE(text_x + 5, text_y + 10, text_x + 15, text_y + 10, 100)   # línea horizontal -
        DRAW_LINE(text_x + 25, text_y + 10, text_x + 35, text_y + 10, 100)  # línea = arriba
        DRAW_LINE(text_x + 25, text_y + 12, text_x + 35, text_y + 12, 100)  # línea = abajo
        
    if test_phase == 3:
        # "*=" - simple
        DRAW_LINE(text_x + 5, text_y + 5, text_x + 15, text_y + 15, 100)    # diagonal * \
        DRAW_LINE(text_x + 15, text_y + 5, text_x + 5, text_y + 15, 100)    # diagonal * /
        DRAW_LINE(text_x + 25, text_y + 10, text_x + 35, text_y + 10, 100)  # línea = arriba
        DRAW_LINE(text_x + 25, text_y + 12, text_x + 35, text_y + 12, 100)  # línea = abajo
        
    if test_phase == 4:
        # "/=" - simple
        DRAW_LINE(text_x + 5, text_y + 15, text_x + 15, text_y + 5, 100)    # diagonal /
        DRAW_LINE(text_x + 25, text_y + 10, text_x + 35, text_y + 10, 100)  # línea = arriba
        DRAW_LINE(text_x + 25, text_y + 12, text_x + 35, text_y + 12, 100)  # línea = abajo
        
    if test_phase == 5:
        # "//" - doble diagonal
        DRAW_LINE(text_x + 5, text_y + 15, text_x + 12, text_y + 5, 100)    # primera diagonal /
        DRAW_LINE(text_x + 15, text_y + 15, text_x + 22, text_y + 5, 100)   # segunda diagonal /
        
    if test_phase == 6:
        # "//=" - triple símbolo
        DRAW_LINE(text_x + 2, text_y + 15, text_x + 8, text_y + 5, 100)     # primera diagonal /
        DRAW_LINE(text_x + 10, text_y + 15, text_x + 16, text_y + 5, 100)   # segunda diagonal /
        DRAW_LINE(text_x + 25, text_y + 10, text_x + 35, text_y + 10, 100)  # línea = arriba
        DRAW_LINE(text_x + 25, text_y + 12, text_x + 35, text_y + 12, 100)  # línea = abajo