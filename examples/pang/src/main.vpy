# pang - Main entry point
# VPy game for Vectrex

META TITLE = "PANG"

screen = 0
title_intensity = 30
title_state = 0 # 0 up 1 down

def main():
    # Called once at startup
    screen =0

def loop():
    # Pantalla de presentación con logo
    SET_INTENSITY(127)
    
    if screen == 0:
        draw_title_screen()

        but1 = J1_BUTTON_1()
        but2 = J1_BUTTON_2()
        but3 = J1_BUTTON_3()
        but4 = J1_BUTTON_4()

        DEBUG_PRINT(btn1)
        DEBUG_PRINT(btn2)
        DEBUG_PRINT(btn3)
        DEBUG_PRINT(btn4)


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


    
    


    

   
