META TITLE = "TEST INCREMENTAL"
META MUSIC = music1
# META ROM_TOTAL_SIZE = 524288  # 512 KB total ROM
# META ROM_BANK_SIZE = 16384    # 16 KB per bank (32 banks)


playing = 0

def main():
    SET_INTENSITY(127)
    

def loop():
    if(playing == 0):
        playing = 1
        PLAY_SFX("coin")
        #PLAY_MUSIC("music1")
        
    #DEBUG_PRINT(playing)
    # --- HEADER ---
    PRINT_TEXT(-60, 120, "TEST SUITE")

    # --- TOP ROW: SHAPES ---
    # Two rectangles side by side
    # DRAW_RECT(-60, 70, 40, 30, 127)
    # DRAW_RECT(20, 70, 40, 30, 100)

    # # --- MATH LINES (Centered) ---
    DRAW_LINE(0, 30, ABS(-40), 70, 80)
    DRAW_LINE(0, 30, MIN(-20, 50), MAX(70, -50), 80)

    # # --- MIDDLE: CIRCLE ---
    DRAW_CIRCLE(-90, 90, 35, 127)

    # --- BOTTOM ROW: VECTORS ---
    # Normal logo on the left
    # Mirrored logo on the right (Mirror X)
    DRAW_VECTOR("logo", 0, -20)
    DRAW_VECTOR_EX("logo", 0, -110, 1, 127)