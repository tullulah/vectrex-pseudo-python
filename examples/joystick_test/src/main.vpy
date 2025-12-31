META TITLE = "JOY TEST"

# Posición del triángulo
triangle_x = 0
triangle_y = 0

# Button debounce system (same as Pang)
btn_debounce = 0
const BTN_DEBOUNCE_FRAMES = 5
startup_delay = 0

# Music test
music_started = 0

# Button states
btn1 = 0
btn2 = 0
btn3 = 0
btn4 = 0

def main():
    SET_INTENSITY(127)
    btn_debounce = 0
    startup_delay = 0

def loop():
    # Start music once - DESHABILITADO PARA TEST
    # if music_started == 0:
    #     PLAY_MUSIC("pang_theme")
    #     music_started = 1
    
    # Decrement debounce counter
    if btn_debounce > 0:
        btn_debounce = btn_debounce - 1
    
    # Startup delay
    if startup_delay < 30:
        startup_delay = startup_delay + 1
    
    # Leer joystick analógico (valores -127 a +127)
    joy_x = J1_X()
    joy_y = J1_Y()
    
    # Leer botones SIEMPRE (ver estado raw)
    btn1 = J1_BUTTON_1()
    btn2 = J1_BUTTON_2()
    btn3 = J1_BUTTON_3()
    btn4 = J1_BUTTON_4()
    
    # Trigger debounce if button pressed after startup
    if startup_delay >= 30 and btn_debounce == 0:
        if btn1 == 1 or btn2 == 1 or btn3 == 1 or btn4 == 1:
            btn_debounce = BTN_DEBOUNCE_FRAMES
    
    # Actualizar posición del triángulo
    # Dividir por 2 para movimiento más suave
    triangle_x = joy_x / 2
    triangle_y = joy_y / 2
    
    # Dibujar triángulo en la posición del joystick
    DRAW_VECTOR("tri", triangle_x, triangle_y)
    
    # Dibujar logo de Pang
    SET_INTENSITY(80)
    DRAW_VECTOR("logo", 0, 20)
    
    # Display button states
    SET_INTENSITY(100)
    
    if btn1 == 1:
        PRINT_TEXT(-120, 80, "B1:ON ")
    else:
        PRINT_TEXT(-120, 80, "B1:OFF")
    
    if btn2 == 1:
        PRINT_TEXT(-120, 60, "B2:ON ")
    else:
        PRINT_TEXT(-120, 60, "B2:OFF")
    
    if btn3 == 1:
        PRINT_TEXT(-120, 40, "B3:ON ")
    else:
        PRINT_TEXT(-120, 40, "B3:OFF")
    
    if btn4 == 1:
        PRINT_TEXT(-120, 20, "B4:ON ")
    else:
        PRINT_TEXT(-120, 20, "B4:OFF")
    
    # Show debounce and startup status
    if btn_debounce > 0:
        PRINT_TEXT(-120, -20, "DEBOUNCE")
    
    if startup_delay < 30:
        PRINT_TEXT(-120, -40, "STARTING...")


