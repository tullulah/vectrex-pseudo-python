META TITLE = "Array Init Test"

# 8-bit array
small_values = [1, 2, 3, 4]

# 16-bit array
large_values = [1000, 2000, 3000, 4000]

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    PRINT_TEXT(-50, 50, "ARRAY TEST")
