META TITLE = "Noise Volume Test"

def main():
    SET_INTENSITY(127)
    PLAY_MUSIC("Noise Volume Test")

def loop():
    WAIT_RECAL()
    # Music playing: quiet (3) -> medium (8) -> loud (15) -> quiet (3)
