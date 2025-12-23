META TITLE = "Print Text + Vectors Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test 1: Draw vector first (should work)
    SET_INTENSITY(100)
    DRAW_VECTOR("rocket_base", -40, 20)
    
    # Test 2: Print text (this was breaking vectors after it)
    SET_INTENSITY(80)
    PRINT_TEXT(0, -40, "VPy Test")
    
    # Test 3: Draw another vector after print_text (should now work with fix)
    SET_INTENSITY(100)
    DRAW_VECTOR("platform", 40, 20)
    
    # Test 4: Try print_text again
    SET_INTENSITY(80)
    PRINT_TEXT(-30, 30, "Fixed!")
