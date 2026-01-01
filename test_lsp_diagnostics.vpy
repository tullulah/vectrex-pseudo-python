# Test file for LSP diagnostics
# Expected diagnostics:
# Line 8: Warning - unused_var is declared but never used
# Line 9: Hint - num_locations never changes - consider 'const'

META TITLE = "LSP Test"

unused_var = 42
num_locations = 17
player_x = 0
player_y = 0
current_location = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Use player_x and player_y (so they don't trigger warnings)
    MOVE(player_x, player_y)
    
    # Use num_locations (read but never written)
    current_location = (current_location + 1) % num_locations
    
    # Write to player_x and player_y
    player_x = player_x + 1
    player_y = player_y + 1
    
    PRINT_TEXT(0, 0, "LSP Test")
