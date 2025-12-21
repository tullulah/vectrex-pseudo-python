; Test: Center-Relative Coordinates with Mirror
; Tests that DRAW_VECTOR_EX works correctly with center-relative emission

META TITLE = "Center-Relative Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    ; Draw the player sprite centered at origin with no mirror
    SET_INTENSITY(127)
    DRAW_VECTOR_EX("player", 0, 0, 0)
    
    ; Draw the player sprite mirrored (should flip horizontally)
    SET_INTENSITY(100)
    DRAW_VECTOR_EX("player", 50, 0, 1)
