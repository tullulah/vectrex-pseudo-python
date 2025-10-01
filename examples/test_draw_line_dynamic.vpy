META TITLE = "DRAW LINE DYNAMIC"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

var x = 0

def main():
    let dummy = 0

def loop():
    # Test con variables - debe usar wrapper
    DRAW_LINE(x, 0, 50, 50, 255)
    x = x + 1