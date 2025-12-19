META TITLE = "String Init Test"

var texto = "abcd"
var count = 0

def main():
    SET_INTENSITY(127)
    DEBUG_PRINT_STR(texto)

def loop():
    WAIT_RECAL()
    count = count + 1
    DEBUG_PRINT(count)
