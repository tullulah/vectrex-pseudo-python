META TITLE = "64 Bank Test"
META MUSIC = 1
META ROM_TOTAL_SIZE = 1048576  # 1MB = 64 banks
META ROM_BANK_SIZE = 16384     # 16KB per bank

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    PRINT_TEXT(-70, 0, "64 BANKS - 1MB ROM")
