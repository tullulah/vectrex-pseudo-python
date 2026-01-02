META TITLE = "J1X Bug Test"

# Simplest possible test
raw_value = 0

def main():
    SET_INTENSITY(127)

def loop():
    
    # Just call J1_X and store it
    raw_value = J1_X()
    
    # Show if it's positive, negative, or zero
    SET_INTENSITY(127)
    PRINT_TEXT(-90, 60, "J1_X TEST")
    
    if raw_value == 0:
        PRINT_TEXT(-90, 40, "ZERO")
    elif raw_value > 0:
        PRINT_TEXT(-90, 40, "POSITIVE")
    else:
        PRINT_TEXT(-90, 40, "NEGATIVE")
