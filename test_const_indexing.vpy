META TITLE = "Const Array Indexing Test"

const test_values = [10, 20, 30]

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test const array indexing
    val0 = test_values[0]  # Should be 10
    val1 = test_values[1]  # Should be 20
    val2 = test_values[2]  # Should be 30
    
    # Use values to set intensity
    SET_INTENSITY(val0 + val1)
