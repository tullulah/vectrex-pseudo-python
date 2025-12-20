META TITLE = "Constructor Test"

# Test struct with constructor
# NOTE: self.field assignments in __init__ require FuncCtx awareness (TODO)
# For now, constructors can receive params and execute logic
struct Player:
    x: int
    y: int
    health: int
    
    def __init__(start_x, start_y):
        # TODO: self.x = start_x once FuncCtx supports constructor context
        # For now, fields must be set manually after instantiation
        pass
    
    def show():
        DEBUG_PRINT(self.x)
        DEBUG_PRINT(self.y)
        DEBUG_PRINT(self.health)

def main():
    pass

def loop():
    WAIT_RECAL()
    
    # Create player with constructor (allocates struct, calls init)
    player = Player(10, 20)
    
    # Manual field assignment (until self works in constructors)
    player.x = 10
    player.y = 20
    player.health = 100
    
    # Show values
    player.show()
