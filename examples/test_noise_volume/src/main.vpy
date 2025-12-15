META TITLE = "Noise Volume Test"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    PLAY_MUSIC("noise_test")
