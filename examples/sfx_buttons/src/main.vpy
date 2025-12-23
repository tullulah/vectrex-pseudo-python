META TITLE = "Print Text + Vectors Fix Test"

def main():
    SET_INTENSITY(127)

def loop():

    
    SET_INTENSITY(80)
    PRINT_TEXT(-50, -40, "Test1")
    
    DRAW_VECTOR("vector", 0,0)
    
    SET_INTENSITY(80)
    # Draw triangle with DRAW_POLYGON to compare with DRAW_VECTOR
    DRAW_POLYGON(3, 40, -20, 55, 0, 25, 0)
    
    SET_INTENSITY(80)
    PRINT_TEXT(30, 40, "Test2")
