def main():
    vectrex.set_origin()
    vectrex.set_intensity(80)
    # Move and print
    vectrex.move_to(0, 0)
    vectrex.print_text(0, 0, "HELLO")
    # Simple radial pattern using trig (angle 0..127 maps full circle)
    i = 0
    for a in range(0, 128, 16):
        # length scales with i
        r = 40 + i * 4
        dx = (sin(a) * r) >> 7
        dy = (cos(a) * r) >> 7
        # Placeholder draw_line (x0,y0,x1,y1,intensity)
        vectrex.draw_line(0, 0, dx, dy, 90)
        i = i + 1