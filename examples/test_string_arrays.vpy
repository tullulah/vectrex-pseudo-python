META TITLE = "String Arrays Test"

const greetings = ["HELLO", "WORLD", "VECTREX"]
index = 0

def main():
    SET_INTENSITY(127)
    index = 0

def loop():
    WAIT_RECAL()
    
    # Test: Print greeting from array
    msg = greetings[index]
    PRINT_TEXT(-50, 50, msg)
    
    # Cycle through array
    index = index + 1
    if index >= 3:
        index = 0
