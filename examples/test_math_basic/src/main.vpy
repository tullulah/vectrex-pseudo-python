META TITLE = "Test Math Basic"

result_abs = 0
result_min = 0
result_max = 0
result_clamp = 0

def main():
    SET_INTENSITY(127)
    
    # Test ABS
    result_abs = ABS(-42)  # Should be 42
    
    # Test MIN
    result_min = MIN(10, 25)  # Should be 10
    
    # Test MAX
    result_max = MAX(10, 25)  # Should be 25
    
    # Test CLAMP
    result_clamp = CLAMP(50, 0, 100)  # Should be 50 (in range)

def loop():
    WAIT_RECAL()
    
    # Display results
    PRINT_TEXT(-70, 60, "ABS(-42):")
    
    PRINT_TEXT(-70, 40, "MIN(10,25):")
    
    PRINT_TEXT(-70, 20, "MAX(10,25):")
    
    PRINT_TEXT(-70, 0, "CLAMP(50,0,100):")
    
    # Test with variables
    val1 = -15
    val2 = 30
    
    abs_val = ABS(val1)      # 15
    min_val = MIN(val1, val2)  # -15
    max_val = MAX(val1, val2)  # 30
    clamped = CLAMP(val2, 0, 20)  # 20 (clamped to max)
