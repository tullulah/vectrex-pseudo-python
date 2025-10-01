# DRAW LINE POSITIVE COORDINATES TEST

def main():
    # No initialization needed
    let dummy = 0

def loop():
    # Test 1: DRAW_LINE (wrapper) - línea horizontal solo positivos
    let x1 = 10
    let y1 = 30  
    let x2 = 80
    let y2 = 30
    let brightness = 255  # Máximo brillo
    
    DRAW_LINE(x1, y1, x2, y2, brightness)
    
    # Test 2: DRAW_LINE (directo) - línea vertical solo positivos  
    DRAW_LINE(50, 10, 50, 80, 255)
