def main():
    # Demo conciso de NUEVAS FUNCIONALIDADES VPy 2025
    
    let num = 100
    
    # === OPERADORES DE ASIGNACIÓN COMPUESTA ===
    num += 50     # 100 + 50 = 150
    num -= 25     # 150 - 25 = 125  
    num *= 2      # 125 * 2 = 250
    num /= 5      # 250 / 5 = 50
    num %= 7      # 50 % 7 = 1
    
    # === DIVISIÓN ENTERA ===
    let resultado = 17 // 3    # División entera: 17 // 3 = 5
    resultado //= 2            # Asignación con división entera: 5 // 2 = 2
    
    # === COMBINACIÓN DE OPERADORES ===
    let fractal_calc = 256
    fractal_calc //= 8         # 256 // 8 = 32
    fractal_calc += resultado  # 32 + 2 = 34  
    fractal_calc *= 3          # 34 * 3 = 102
    fractal_calc %= 100        # 102 % 100 = 2
    
    return fractal_calc + num  # 2 + 1 = 3

def loop():
    # Uso en loop de display
    let bright = 255
    bright //= 4    # 255 // 4 = 63
    bright += 32    # 63 + 32 = 95
    return bright