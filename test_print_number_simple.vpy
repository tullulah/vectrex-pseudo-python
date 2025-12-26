META TITLE = "Test PRINT_NUMBER"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Test con valor conocido: 0
    PRINT_NUMBER(0, 0, 0)
