META TITLE = "Test Array Bug"

# Test case for the peephole optimization bug
# Should increment array[0] correctly: 10 → 11 → 12...
# BUG: Was compiling array[0] = array[0] + offset as array[0] * 2

array = [10, 20, 30]
offset = 1

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # This should add offset (1) to array[0]
    # BUG: Was adding array[0] to itself (doubling it)
    array[0] = array[0] + offset
    
    # Display the result
    DEBUG_PRINT(array[0])
