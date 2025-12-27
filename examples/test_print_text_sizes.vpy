META TITLE = "PRINT TEXT SIZES"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Basic 3-parameter version (uses BIOS defaults)
    PRINT_TEXT(0, 90, "DEFAULT SIZE")
    
    # Small text (5 parameters)
    PRINT_TEXT(-60, 50, "SMALL TEXT", -4, 32)
    
    # Medium text  
    PRINT_TEXT(-60, 20, "MEDIUM TEXT", -6, 48)
    
    # Large text
    PRINT_TEXT(-60, -20, "LARGE TEXT", -12, 96)
