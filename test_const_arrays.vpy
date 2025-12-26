# Test const arrays implementation

META TITLE = "CONST ARRAYS TEST"

# Const arrays stored in ROM only
const coords_x = [10, 20, 30]
const coords_y = [40, 50, 60]

# Regular variable to test differentiation
location = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Just do something simple to verify compilation works
    # and that const arrays don't cause memory corruption
    SET_INTENSITY(100)
