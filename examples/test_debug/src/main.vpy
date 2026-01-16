META TITLE = "Debug Tools Test"

counter = 0
test_value = 42
my_string = "Hello Vectrex"

def main():
    pass

def loop():
    # Test DEBUG_PRINT with variable
    DEBUG_PRINT(counter)
    
    # Test DEBUG_PRINT with expression
    DEBUG_PRINT(test_value * 2)
    
    # Test DEBUG_PRINT_STR with variable
    DEBUG_PRINT_STR(my_string)
    
    # Test PRINT_NUMBER at position (shows as hex on screen)
    PRINT_NUMBER(-50, 50, counter)
    PRINT_NUMBER(-50, 30, test_value)
    PRINT_NUMBER(-50, 10, counter + test_value)
    
    # Increment counter
    counter = counter + 1
