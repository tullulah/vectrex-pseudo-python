META TITLE = "Test LDB indexed"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Force generation of LDB 11,U in assembly
    # We need raw assembly for this test
