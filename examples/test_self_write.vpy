META TITLE = "TEST SELF WRITE"

struct Point:
    x: int
    y: int
    
    def move(dx, dy):
        # ✅ NEW: Should now support self.x = value
        self.x = self.x + dx
        self.y = self.y + dy
    
    def reset():
        self.x = 0
        self.y = 0

def main():
    pass

def loop():
    WAIT_RECAL()
    
    p = Point()
    p.x = 10
    p.y = 20
    
    # Call method that modifies internal state
    p.move(5, -3)
    
    # p.x should now be 15, p.y should be 17
    SET_INTENSITY(127)
    DRAW_LINE(p.x, p.y, p.x + 10, p.y, 127)
    
    # Reset to origin
    p.reset()
