META TITLE = "Unified Mirrors Test"
META AUTHOR = "TestBot"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    ; Test all mirror modes with the player sprite
    ; Mode 0: No mirror (normal)
    DRAW_VECTOR_EX("player", 30, 60, 0)
    
    ; Mode 1: X-axis mirror (horizontal flip)
    DRAW_VECTOR_EX("player", 90, 60, 1)
    
    ; Mode 2: Y-axis mirror (vertical flip)
    DRAW_VECTOR_EX("player", 30, 0, 2)
    
    ; Mode 3: Both axes (X and Y mirror)
    DRAW_VECTOR_EX("player", 90, 0, 3)
