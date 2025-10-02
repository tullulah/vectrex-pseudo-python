def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Limpiar pantalla con un marco de referencia
    DRAW_LINE(-100, -100, 100, -100, 30)
    DRAW_LINE(100, -100, 100, 100, 30)
    DRAW_LINE(100, 100, -100, 100, 30)
    DRAW_LINE(-100, 100, -100, -100, 30)
    
    # Variables de test
    let a = 10
    let b = 3
    
    # Test 1: Operadores básicos (línea Y=80)
    let sum = a + b       # Debería ser 13
    let diff = a - b      # Debería ser 7
    let mult = a * b      # Debería ser 30
    
    PRINT_TEXT(-90, 80, "BASICOS:")
    PRINT_TEXT(-50, 80, "13")   # sum esperado
    PRINT_TEXT(-30, 80, "7")    # diff esperado
    PRINT_TEXT(-10, 80, "30")   # mult esperado
    
    # Debug output
    DEBUG_PRINT_LABELED("sum_calc", sum)
    DEBUG_PRINT_LABELED("diff_calc", diff)
    DEBUG_PRINT_LABELED("mult_calc", mult)
    
    # Test 2: Operadores compuestos (línea Y=60)
    let x = 20
    DEBUG_PRINT_LABELED("x_inicial", x)
    
    x += 5    # Debería ser 25
    DEBUG_PRINT_LABELED("x_after_plus", x)
    
    x -= 3    # Debería ser 22
    DEBUG_PRINT_LABELED("x_after_minus", x)
    
    PRINT_TEXT(-90, 60, "COMPOUND:")
    PRINT_TEXT(-50, 60, "22")   # x final esperado
    
    # Test 3: Operadores problemáticos (línea Y=40)
    let y = 16
    DEBUG_PRINT_LABELED("y_inicial", y)
    
    y /= 2    # PROBLEMA: debería ser 8, pero puede quedarse en 16
    DEBUG_PRINT_LABELED("y_after_div", y)
    
    let z = 15
    let floor_result = z // 4  # PROBLEMA: debería ser 3
    DEBUG_PRINT_LABELED("floor_15_div_4", floor_result)
    
    PRINT_TEXT(-90, 40, "PROBLEM:")
    PRINT_TEXT(-50, 40, "8")    # y esperado
    PRINT_TEXT(-30, 40, "3")    # floor esperado
    
    # Test 4: Multiplicación rápida (línea Y=20)
    let w = 2
    w *= 3    # Puede moverse muy rápido
    DEBUG_PRINT_LABELED("mult_fast", w)
    
    PRINT_TEXT(-90, 20, "FAST:")
    PRINT_TEXT(-50, 20, "6")    # w esperado