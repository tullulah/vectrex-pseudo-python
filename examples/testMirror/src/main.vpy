# testMirror - Main entry point
# VPy game for Vectrex

META TITLE = "testMirror"

# === PLAYER STATE (ESENCIAL) ===
player_x = 0
player_y = -80
player_dir = 0  # 0=right, 1=left (for sprite mirroring)

def main():
    # Called once at startup
    Set_Intensity(127)

def loop():
    
    # === INPUT HANDLING (ESENCIAL) ===
    joy_x = J1_X()  # -1 (izq), 0 (centro), +1 (der)
    joy_y = J1_Y()  # -1 (abajo), 0 (centro), +1 (arriba)
    
    # === PLAYER MOVEMENT (ESENCIAL) ===
    # Movimiento horizontal
    if joy_x == -1:
        player_x = player_x - 3
        player_dir = 1  # facing left
    if joy_x == 1:
        player_x = player_x + 3
        player_dir = 0  # facing right
    
    # Movimiento vertical
    if joy_y == -1:
        player_y = player_y - 3
    if joy_y == 1:
        player_y = player_y + 3

    # === DRAW ASTRONAUT (ESENCIAL) ===
    SET_INTENSITY(127)
    DEBUG_PRINT(player_dir)
    
    # DRAW_VECTOR_EX("astronaut_armed", player_x, player_y, player_dir)
    DRAW_VECTOR_EX("player", player_x, player_y, player_dir)
        
