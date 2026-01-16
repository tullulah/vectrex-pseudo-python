META TITLE = "Buildtools Builtins Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test joystick builtins (NEW in buildtools)
    x = J1_X()
    y = J1_Y()
    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    
    # Test math builtins
    abs_x = ABS(x)
    min_val = MIN(x, y)
    max_val = MAX(x, y)
    
    # Test audio (placeholder)
    AUDIO_UPDATE()
    
    # Original builtins
    PRINT_TEXT(-50, 50, "BUILDTOOLS TEST OK")
    DRAW_LINE(0, 0, 50, 50, 127)
