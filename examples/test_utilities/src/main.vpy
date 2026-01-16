# Test Utilities Builtins
# Tests all 9 utility functions

META TITLE = "Utils Test"

counter = 0

def main():
    SET_INTENSITY(127)
    
    # Test MOVE - move beam without drawing
    MOVE(0, 60)
    PRINT_TEXT(-40, 60, "MOVE OK")
    
    # Test LEN - array length (placeholder)
    length = LEN([1, 2, 3])
    PRINT_NUMBER(-40, 50, length)
    
    # Test GET_TIME - frame counter
    time = GET_TIME()
    PRINT_NUMBER(-40, 40, time)
    
    # Test PEEK - read memory
    value = PEEK(0xC800)
    PRINT_NUMBER(-40, 30, value)
    
    # Test POKE - write memory (careful!)
    POKE(0xCF00, 42)
    
    # Test WAIT - delay 3 frames
    WAIT(3)
    PRINT_TEXT(-40, 20, "WAITED")
    
    # Test BEEP - sound generation
    BEEP(100, 5)

def loop():
    WAIT_RECAL()
    
    counter = counter + 1
    
    # Test fade effects every 60 frames
    if counter == 60:
        FADE_OUT()
        counter = 0
    
    if counter == 30:
        FADE_IN()
    
    # Draw something to see fade effects
    DRAW_CIRCLE(0, 0, 30, 127)
    PRINT_NUMBER(-40, -60, counter)
