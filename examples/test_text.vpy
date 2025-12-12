META TITLE = "Test: Print Text"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    PRINT_TEXT("HELLO WORLD", 0, 50)
