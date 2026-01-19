META TITLE = "TEST INCREMENTAL"
META MUSIC = music1
# META ROM_TOTAL_SIZE = 524288  # 512 KB total ROM
# META ROM_BANK_SIZE = 16384    # 16 KB per bank (32 banks)

title_intensity = 30

def main():
    SET_INTENSITY(127)
    

def loop():
    SET_INTENSITY(title_intensity)
    DRAW_VECTOR("logo", 0, 0)
    PRINT_TEXT(0,-40,"HOLA")
    