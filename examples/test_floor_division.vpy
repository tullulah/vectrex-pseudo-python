def main():
    let a = 17
    let b = 5
    
    # Test división entera //
    let resultado_div_entera = a // b    # 17 // 5 = 3 (división entera)
    let resultado_div_normal = a / b     # 17 / 5 = 3 (en enteros es igual)
    
    # Test con números más grandes
    let x = 100
    let y = 7
    x //= y    # x = 100 // 7 = 14
    
    # Resultado combinado
    let final = resultado_div_entera + x  # 3 + 14 = 17
    
    return final

def loop():
    return 0