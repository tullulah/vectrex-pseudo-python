META TITLE = "Test Multi-Bank Variable"
META ROM_TOTAL_SIZE = 524288
META ROM_BANK_SIZE = 16384

# Simple variable
my_var = 10

# Simple array variable
my_array = [127, 100, 80, 60]  # Visible intensities

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    val = my_array[0]
    SET_INTENSITY(val)
