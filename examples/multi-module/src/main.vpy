# Main game - demonstrates multi-module imports with dot notation
META TITLE = "TEST"
META ROM_TOTAL_SIZE = 524288  # 512 KB total ROM
META ROM_BANK_SIZE = 16384    # 16 KB per bank (32 banks)

import graphics
import input

# Player state
player_x = 0
player_y = 0
player_size = 10

def main():
    # Initialize game
    SET_INTENSITY(127)

def loop():
    # Get input using module (stores in global input.input_result)
    input.get_input()
    # Use array values directly - no local variables
    player_x = player_x + input.input_result[0]
    player_y = player_y + input.input_result[1]
    
    # Clamp to screen bounds
    if player_x < -100:
        player_x = -100
    if player_x > 100:
        player_x = 100
    if player_y < -100:
        player_y = -100
    if player_y > 100:
        player_y = 100
    
    # Draw player using module.function() notation
    graphics.draw_box(player_x - player_size, player_y - player_size, player_size * 2, player_size * 2, 127)

    # DRAW_VECTOR(0,0,"box")  # TODO: need to create box.vec asset
    
    # Draw crosshair if fire button pressed
    if input.check_fire() == 1:
        graphics.draw_cross(player_x, player_y, 20, 100)
