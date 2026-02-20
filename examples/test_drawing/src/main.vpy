META TITLE = "Drawing Test"

def main():
    SET_INTENSITY(127)

def loop():
    # Circle at center
    DRAW_CIRCLE(0, 0, 40, 80)

    # Polygon (triangle) in bottom-right
    DRAW_POLYGON(30, -30, 50, -50, 70, -30, 90)

    # Circle with custom segment count (octagon)
    DRAW_CIRCLE_SEG(8, -60, 0, 30, 70)

    # Line using MOVE + DRAW_TO
    MOVE(-50, 50)
    DRAW_TO(50, -50)

    # Line segment
    DRAW_LINE(-50, -50, 50, 50, 100)
