META TITLE = "Level System Test"
META AUTHOR = "VPy Team"

# Test level system builtins

def main():
    # Load level data from ROM
    level_ptr = LOAD_LEVEL("test_level")
    
    # Get world bounds
    bounds = GET_LEVEL_BOUNDS()
    # bounds contains xMin, xMax, yMin, yMax in RESULT+0/+2/+4/+6
    
    # Get object counts
    bg_count = GET_OBJECT_COUNT(0)      # Background layer
    gameplay_count = GET_OBJECT_COUNT(1) # Gameplay layer
    fg_count = GET_OBJECT_COUNT(2)       # Foreground layer
    
    # Store counts for display
    total_objects = bg_count + gameplay_count + fg_count
    
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Display level info
    PRINT_TEXT(-90, 80, "LEVEL SYSTEM TEST")
    PRINT_TEXT(-90, 60, "BUILTINS WORKING")
    
    # Get gameplay objects
    gameplay_count = GET_OBJECT_COUNT(1)
    
    # Access first gameplay object if it exists
    if gameplay_count > 0:
        obj_ptr = GET_OBJECT_PTR(1, 0)  # Layer 1 (gameplay), index 0
        PRINT_TEXT(-90, 30, "FIRST ENEMY PTR OK")
    
    # Access second gameplay object if it exists
    if gameplay_count > 1:
        obj_ptr2 = GET_OBJECT_PTR(1, 1)  # Layer 1, index 1
        PRINT_TEXT(-90, 10, "COLLECTIBLE PTR OK")
    
    # Draw simple indicator
    DRAW_LINE(-50, -50, 50, -50, 100)
    DRAW_LINE(50, -50, 50, 50, 100)
    DRAW_LINE(50, 50, -50, 50, 100)
    DRAW_LINE(-50, 50, -50, -50, 100)
