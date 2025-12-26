META TITLE = "Test Global Variables"

counter = 0

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    counter = counter + 1
