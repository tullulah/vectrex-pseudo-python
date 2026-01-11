META TITLE = "Test Array"
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384

my_array = [1, 2, 3, 4]  # This is a comment

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    val = my_array[0]
    SET_INTENSITY(val)
