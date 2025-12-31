META TITLE = "Hook Y Test"

hook_y = -100

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    DEBUG_PRINT(hook_y)
