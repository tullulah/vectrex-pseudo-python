# test1 - Main entry point
# VPy game for Vectrex

META TITLE = "TEST 1"
META MUSIC = music1
META ROM_TOTAL_SIZE = 524288  # 512 KB total ROM
META ROM_BANK_SIZE = 16384    # 16 KB per bank (32 banks)

def main():
    # Called once at startup
    Set_Intensity(127)

def loop():
    # Game loop - called every frame
    PRINT_TEXT(0,0,"HOLA")
