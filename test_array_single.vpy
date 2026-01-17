META TITLE = "TEST ARRAY SINGLE"
META MUSIC = music1
# Simple variable
my_var = 10

# Simple array variable with visible intensities
my_array = [127, 100, 80, 60]

def main():
    SET_INTENSITY(127)

def loop():
    val = my_array[0]
    SET_INTENSITY(val)
    PRINT_TEXT(-70, 70, "ARRAY TEST")
