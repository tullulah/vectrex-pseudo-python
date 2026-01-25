META TITLE = "TEST INCREMENTAL"
META ROM_TOTAL_SIZE = 524288  # 512 KB total ROM
META ROM_BANK_SIZE = 16384    # 16 KB per bank (32 banks)
META MUSIC = music1

title_intensity = 127
playing = 0


def main():
    SET_INTENSITY(127)
    

def loop():
    SET_INTENSITY(title_intensity)
    
    PRINT_TEXT(0,-40,"HOLA")
    DRAW_VECTOR("logo", 0, 0)
    if(playing == 0):
        playing = 1
        PLAY_MUSIC("music1")