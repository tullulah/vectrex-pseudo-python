# DRAW LINE COMPARISON TEST

def main():
    # No initialization needed
    let dummy = 0

def loop():
    # Test 1: DRAW_LINE (wrapper) - línea horizontal
    let x1 = -50
    let y1 = 30  
    let x2 = 50
    let y2 = 30
    let brightness = 255  # Máximo brillo
    
    DRAW_LINE(x1, y1, x2, y2, brightness)
    
    # Test 2: DRAW_LINE (directo) - línea vertical  
    DRAW_LINE(-30, -50, -30, 50, 255)
