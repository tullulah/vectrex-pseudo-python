# pang - Main entry point
# VPy game for Vectrex

META TITLE = "PANG"

screen = 0
title_intensity = 30
title_state = 0 # 0 up 1 down

def main():
    # Called once at startup
    screen = 0
    PLAY_MUSIC("pang_theme")  # ¡Reproduce la música!

def loop():
    # Pantalla de presentación con logo
    SET_INTENSITY(127)
    
    if screen == 0:
        draw_title_screen()

        btn1 = J1_BUTTON_1()
        btn2 = J1_BUTTON_2()
        btn3 = J1_BUTTON_3()
        btn4 = J1_BUTTON_4()

        if (btn1 == 1 or btn2 == 1 or btn3 == 1 or btn4 == 1):
            #PLAY_SFX("coin")
            screen = 1 



def draw_title_screen():
    # Dibujar logo centrado
    DRAW_VECTOR("logo", 0, 20)

    SET_INTENSITY(title_intensity)
    PRINT_TEXT(-90, -40, "PRESS A BUTTON")
    PRINT_TEXT(-50, -60, "TO START")

    if (title_state == 0):
        title_intensity += 1

    if (title_state == 1):
        title_intensity -= 1    

    if (title_intensity == 127):
        title_state = 1
    
    if (title_intensity == 30):
        title_state = 0
