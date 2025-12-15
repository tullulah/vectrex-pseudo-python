META TITLE = "Noise Test"

def main():
    SET_INTENSITY(80)
    PLAY_MUSIC("minimal_noise")

def loop():
    MUSIC_UPDATE()
    #PRINT_TEXT(0, 0, "NOISE")
    DRAW_VECTOR("test", 0, 20)
    
    
    
    