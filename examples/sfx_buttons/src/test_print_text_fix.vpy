META TITLE = "Print Text + Vectors Fix Test"

def main():
    SET_INTENSITY(127)

def loop():
        
    # Test: Print text followed by vector drawing
    # This tests the fix for DP=$D0 management in PRINT_TEXT
    
    SET_INTENSITY(80)
    PRINT_TEXT(-50, -40, "Test1")
    
    # Simple shape with lines - no vector assets needed
    MOVE_TO(0, 0)
    DRAW_TO(30, 0)
    DRAW_TO(30, 30)
    DRAW_TO(0, 30)
    DRAW_TO(0, 0)
    
    SET_INTENSITY(80)
    PRINT_TEXT(30, 40, "Test2")
