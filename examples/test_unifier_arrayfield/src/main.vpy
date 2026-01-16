META TITLE = "Array Field Access Test"

# Test: Can we access and modify array fields from imported modules?
# graphics.vpy will have: my_positions = [0, 0]
# main.vpy will:
#   - Read: x = graphics.my_positions[0]
#   - Write: graphics.my_positions[0] = 5

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Try to access array from imported module
    # This will be: x = GRAPHICS_MY_POSITIONS[0]
