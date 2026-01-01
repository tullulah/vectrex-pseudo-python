META TITLE = "Test Diagnostics"

# This should generate "unused-variable" warning
unused_var = 42

# This should generate "suggest-const" hint
num_locations = 17

player_x = 0
player_y = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    MOVE(player_x, player_y)
    player_x = player_x + 1
