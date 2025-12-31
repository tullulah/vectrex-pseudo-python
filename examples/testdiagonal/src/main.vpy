META TITLE = "Test: Diagonal Corner to Corner"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    ; Diagonal from bottom-left to top-right
    ; dy = 90 - (-90) = 180 pixels (needs 2 segments: 127 + 53)
    ; dx = 90 - (-90) = 180 pixels (needs 2 segments: 127 + 53)
    DRAW_LINE(-90, -90, 90, 90, 100)
