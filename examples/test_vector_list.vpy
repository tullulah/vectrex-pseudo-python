META TITLE = "Malban Vector List"
META DESC = "Test DRAW_VECTOR_LIST with Malban algorithm"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Draw vector list using Malban's algorithm
    # Parameters: (list_label, y, x, scale)
    DRAW_VECTOR_LIST("SQUARE", 0, 0, 80)
