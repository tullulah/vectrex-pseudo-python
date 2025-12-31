META TITLE = "Test Various Line Sizes"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    ; Test 1: Small line (inline)
    DRAW_LINE(0, -50, 0, 0, 127)
    
    ; Test 2: Exactly 127 pixels (inline)
    DRAW_LINE(0, 0, 0, 127, 127)
    
    ; Test 3: 128 pixels (needs segment)
    DRAW_LINE(0, 0, 0, 128, 127)
    
    ; Test 4: Large line 200 pixels (needs segment)
    DRAW_LINE(0, 0, 0, 200, 127)
    
    ; Test 5: Negative large line -150 pixels (needs segment)
    DRAW_LINE(0, 0, 0, -150, 127)
