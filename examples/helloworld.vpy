# Vectrex pseudo-python Hello World demo
# Shows text and a simple triangle using built-in intrinsics.

# Coordinates are 16-bit signed (interpretation depends on future vector engine scaling)

def main():
    set_origin()
    print_text(0, 0, "HELLO VECTREX")
    # Triangle points (0,0)->(50,0)->(25,40)
    move_to(0, 0)
    draw_to(50, 0, 15)
    draw_to(25, 40, 15)
    draw_to(0, 0, 15)
    return 0
