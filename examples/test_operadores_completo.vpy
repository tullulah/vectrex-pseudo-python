def main():
    WAIT_RECAL()
    SET_INTENSITY(127)

def loop():
    # Test operadores básicos tradicionales
    let a = 10
    let b = 3
    DEBUG_PRINT_LABELED("a", a)
    DEBUG_PRINT_LABELED("b", b)
    
    # Suma tradicional
    let sum = a + b
    DEBUG_PRINT_LABELED("a+b", sum)
    
    # Resta tradicional  
    let diff = a - b
    DEBUG_PRINT_LABELED("a-b", diff)
    
    # Multiplicación tradicional
    let mult = a * b
    DEBUG_PRINT_LABELED("a*b", mult)
    
    # División tradicional
    let div = a / b
    DEBUG_PRINT_LABELED("a/b", div)
    
    # Módulo tradicional
    let mod = a % b
    DEBUG_PRINT_LABELED("a%b", mod)
    
    # TEST OPERADORES COMPUESTOS NUEVOS
    let x = 20
    DEBUG_PRINT_LABELED("x_inicial", x)
    
    # += (sabemos que funciona)
    x += 5
    DEBUG_PRINT_LABELED("x_after_+=5", x)
    
    # -= (sabemos que funciona)  
    x -= 3
    DEBUG_PRINT_LABELED("x_after_-=3", x)
    
    # *= (funciona pero muy rápido - necesita control)
    let y = 4
    DEBUG_PRINT_LABELED("y_inicial", y)
    y *= 2
    DEBUG_PRINT_LABELED("y_after_*=2", y)
    
    # /= (PROBLEMA - se queda fijo)
    let z = 16  
    DEBUG_PRINT_LABELED("z_inicial", z)
    z /= 2
    DEBUG_PRINT_LABELED("z_after_/=2", z)
    
    # %= (no probado aún)
    let w = 17
    DEBUG_PRINT_LABELED("w_inicial", w) 
    w %= 5
    DEBUG_PRINT_LABELED("w_after_%=5", w)
    
    # TEST DIVISIÓN ENTERA (PROBLEMA)
    let p = 15
    let q = 4
    DEBUG_PRINT_LABELED("p", p)
    DEBUG_PRINT_LABELED("q", q)
    
    # // (PROBLEMA - se queda fijo)
    let floor_div = p // q
    DEBUG_PRINT_LABELED("p//q", floor_div)
    
    # //= (PROBLEMA - se queda fijo)
    let r = 20
    DEBUG_PRINT_LABELED("r_inicial", r)
    r //= 3  
    DEBUG_PRINT_LABELED("r_after_//=3", r)