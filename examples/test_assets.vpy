# Test Assets System
# This program demonstrates the asset system integration

META TITLE = "ASSETS TEST"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Draw vector asset
    DRAW_VECTOR("player")
    
    # Play music asset
    PLAY_MUSIC("theme")
