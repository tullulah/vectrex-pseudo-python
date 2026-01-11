# Main game - demonstrates multi-module imports with dot notation

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
    # Main game loop
    WAIT_RECAL()
    
    # Get input using module (stores in global input.input_result)
    input.get_input()
    dx = input.input_result[0]
    dy = input.input_result[1]
    player_x = player_x + dx
    player_y = player_y + dy
    
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
    
    # Draw crosshair if fire button pressed
    if input.check_fire() == 1:
        graphics.draw_cross(player_x, player_y, 20, 100)
