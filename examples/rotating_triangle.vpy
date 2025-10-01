META TITLE = "SIMPLE LINE TEST"
META COPYRIGHT = "g GCE 1983"
META MUSIC = "0"

# Test: Single line to verify DRAW_LINE works

def main():
    let dummy = 0

def loop():
    # Force using DRAW_LINE wrapper by using variables
    let x1 = 10
    let y1 = 50
    let x2 = 50
    let y2 = 50
    let intensity = 255
    DRAW_LINE(x1, y1, x2, y2, intensity)
