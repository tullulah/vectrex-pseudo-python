def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # MARCO DE REFERENCIA
    DRAW_LINE(-50, -50, 50, -50, 40)
    DRAW_LINE(50, -50, 50, 50, 40)
    DRAW_LINE(50, 50, -50, 50, 40)
    DRAW_LINE(-50, 50, -50, -50, 40)
    
    PRINT_TEXT(-40, 60, "OPERATOR DIAGNOSIS")
    
    # TEST 1: /= (División compuesta - PROBLEMA)
    let x = 20
    DEBUG_PRINT_LABELED("before_div_eq", x)
    PRINT_TEXT(-40, 40, "X=20, X/=4 ->")
    
    x /= 4  # Debería resultar en 5
    DEBUG_PRINT_LABELED("after_div_eq", x)
    PRINT_TEXT(20, 40, "5?")
    
    # TEST 2: // (División entera - PROBLEMA)  
    let y = 17
    let result = y // 5  # Debería resultar en 3
    DEBUG_PRINT_LABELED("y_initial", y)
    DEBUG_PRINT_LABELED("y_floor_div_5", result)
    PRINT_TEXT(-40, 20, "17//5 ->")
    PRINT_TEXT(20, 20, "3?")
    
    # TEST 3: //= (División entera compuesta - PROBLEMA)
    let z = 23
    DEBUG_PRINT_LABELED("z_initial", z) 
    PRINT_TEXT(-40, 0, "Z=23, Z//=7 ->")
    
    z //= 7  # Debería resultar en 3
    DEBUG_PRINT_LABELED("z_after_floor_eq", z)
    PRINT_TEXT(20, 0, "3?")
    
    # TEST 4: *= (Multiplicación - FUNCIONA pero rápido)
    let w = 3
    DEBUG_PRINT_LABELED("w_initial", w)
    PRINT_TEXT(-40, -20, "W=3, W*=2 ->")
    
    w *= 2  # Debería resultar en 6
    DEBUG_PRINT_LABELED("w_after_mult", w)
    PRINT_TEXT(20, -20, "6?")
    
    # TEST 5: %= (Módulo compuesta - NO PROBADO)
    let v = 19
    DEBUG_PRINT_LABELED("v_initial", v)
    PRINT_TEXT(-40, -40, "V=19, V%=6 ->")
    
    v %= 6  # Debería resultar en 1
    DEBUG_PRINT_LABELED("v_after_mod", v)
    PRINT_TEXT(20, -40, "1?")