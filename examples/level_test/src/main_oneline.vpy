META TITLE = "LINE"


# Test with only ONE DRAW_LINE

def main():
    SET_INTENSITY(127)
    LOAD_LEVEL("test_level")

def loop():
    PRINT_TEXT(-90, 100, "ONE DRAW_LINE TEST")
    
    # Only ONE DRAW_LINE
    DRAW_LINE(-100, -100, 100, -100, 60)  # Bottom only
    
    SHOW_LEVEL()
    UPDATE_LEVEL()
