META TITLE = "Constructor Test"

struct Point:
    x: int
    y: int
    
    def __init__(init_x, init_y):
        # Constructor now works with self.field!
        self.x = init_x
        self.y = init_y
    
    def print_coords():
        DEBUG_PRINT(self.x)
        DEBUG_PRINT(self.y)

def main():
    VECTREX_SET_INTENSITY(127)

def loop():
    # Create point with constructor
    p = Point(100, 200)
    
    # Verify values were set correctly by constructor
    p.print_coords()  # Should print 100, 200
    
    # Modify a field
    p.x = 42
    p.print_coords()  # Should print 42, 200
