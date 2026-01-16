# Test Level System Builtins
# Tests all 6 level system functions

META TITLE = "Level Test"

def main():
    SET_INTENSITY(127)
    
    # Load test level
    LOAD_LEVEL("test_level")
    
    # Get level info
    width = GET_LEVEL_WIDTH()
    height = GET_LEVEL_HEIGHT()
    
    # Display level dimensions
    DEBUG_PRINT(-50, 60, "Level loaded!")
    PRINT_NUMBER(-50, 50, width)
    PRINT_NUMBER(-50, 40, height)

def loop():
    WAIT_RECAL()
    
    # Render level
    SHOW_LEVEL()
    
    # Update level (placeholder for now)
    UPDATE_LEVEL()
    
    # Get tile at (0, 0)
    tile = GET_LEVEL_TILE(0, 0)
    
    # Display tile info
    PRINT_NUMBER(-50, -60, tile)
