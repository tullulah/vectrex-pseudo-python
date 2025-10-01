META TITLE = "STATIC LINES TEST"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

def main():
    # Initialize
    let dummy = 0

def loop():
    # Test 1: Línea horizontal usando variables (como rotating_line)
    let x0 = 0
    let y0 = 0
    let x1 = 50
    let y1 = 0
    DRAW_LINE(x0, y0, x1, y1, 127)
    
    # Test 2: Línea vertical usando variables
    let x2 = 0
    let y2 = 0
    let x3 = 0
    let y3 = 50
    DRAW_LINE(x2, y2, x3, y3, 127)
