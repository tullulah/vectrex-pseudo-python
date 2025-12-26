# Test using const arrays in expressions

META TITLE = "CONST ARRAY USAGE TEST"

# Const arrays stored in ROM
const player_x = [10, 20, 30, 40]
const player_y = [50, 60, 70, 80]

# Regular variables
current_player = 0
x_offset = 0
y_offset = 0

def main():
    SET_INTENSITY(100)

def loop():
    WAIT_RECAL()
    SET_INTENSITY(current_player + 50)
