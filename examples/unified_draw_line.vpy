META TITLE = "UNIFIED DRAW_LINE TEST"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

def main():
    let dummy = 0

def loop():
    # Test unified DRAW_LINE - should always work consistently
    DRAW_LINE(10, 30, 80, 30, 255)   # Línea horizontal
    DRAW_LINE(50, 10, 50, 80, 255)   # Línea vertical  
    DRAW_LINE(20, 20, 70, 70, 255)   # Línea diagonal
