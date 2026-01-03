META TITLE = "Array Bug Test"

test_array = [10, 20, 30]
index = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # This should do: test_array[0] = test_array[0] + 1
    # Expected: 10 -> 11 -> 12 -> 13...
    test_array[index] = test_array[index] + 1
    
    DEBUG_PRINT_LABELED("ARRAY[0]", test_array[0])
