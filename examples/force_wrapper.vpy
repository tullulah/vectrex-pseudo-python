# FORCE WRAPPER TEST

def main():
    # No initialization needed
    let dummy = 0

def loop():
    # Test 1: Forzar wrapper con variables - línea horizontal
    let x1 = 10
    let y1 = 30  
    let x2 = 80
    let y2 = 30
    let brightness1 = 255  # Máximo brillo
    
    DRAW_LINE(x1, y1, x2, y2, brightness1)
    
    # Test 2: Forzar wrapper con variables - línea vertical  
    let x3 = 50
    let y3 = 10  
    let x4 = 50
    let y4 = 80
    let brightness2 = 255  # Máximo brillo
    
    DRAW_LINE(x3, y3, x4, y4, brightness2)
