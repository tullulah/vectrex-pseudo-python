META TITLE = "Noise Test"

song = -1

def main():
    SET_INTENSITY(80)
    

def loop():
    #MUSIC_UPDATE()

    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    btn3 = J1_BUTTON_3()
    btn4 = J1_BUTTON_4()

    if(btn1 == 1):
        if(song != 1):
            PLAY_MUSIC("minimal_noise")
            song = 1

    if(btn2 == 1):
        if(song != 2):
            PLAY_MUSIC("pang_theme")
            song = 2

    if(song == 1):
        PRINT_TEXT(0, 0, "SONG 1")
    if(song == 2):
        PRINT_TEXT(0, 0, "SONG 2")

    DRAW_VECTOR("test", 0, 20)
 
    
    
    
    