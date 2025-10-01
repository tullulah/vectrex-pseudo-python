# triangle.vpy - dibuja un triángulo sencillo cada frame con auto_loop

const X0 = 0
const Y0 = 0
const X1 = 40
const Y1 = 0
const X2 = 20
const Y2 = 40
const INT = 0x5F

# draw_frame ya NO hace WAIT_RECAL (lo hace el auto_loop antes de llamar a main)

def draw_frame(i):
    if (i & 32) == 0:
        VECTREX_FRAME_BEGIN(0x30)
    else:
        VECTREX_FRAME_BEGIN(INT)
    VECTREX_PRINT_TEXT(-40, 50, "TRI")
    VECTREX_PRINT_TEXT(-60, -60, "DEMO")
    # Ya no se necesita fase vectorial explícita; DRAW_LINE cambia DP internamente
    DRAW_LINE(0, 0, 10, 0, INT)
    DRAW_LINE(X0, Y0, X1, Y1, INT)
    DRAW_LINE(X1, Y1, X2, Y2, INT)
    DRAW_LINE(X2, Y2, X0, Y0, INT)
    return 0

# main se llama una vez por frame por el auto_loop: NO usar while infinito aquí

def main():
    # Removed music start to avoid unwanted sounds; just count frames
    frame_counter = frame_counter + 1
    draw_frame(frame_counter)
    return 0
