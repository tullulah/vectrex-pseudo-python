# TEST SIMPLE - VPy Language
# Solo líneas básicas para test

var brightness = 127
   
def main():
    return

def loop():


    # Línea horizontal simple
    DRAW_LINE(-50, 0, 50, 0, 127)
    
    # Línea vertical simple  
    DRAW_LINE(0, -50, 0, 50, 127)
    
    # Cuadrado simple
    DRAW_LINE(-30, -30, 30, -30, 100)
    DRAW_LINE(30, -30, 30, 30, 100)
    DRAW_LINE(30, 30, -30, 30, 100)
    DRAW_LINE(-30, 30, -30, -30, 100)
