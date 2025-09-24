META TITLE = "SHAPES DEMO"
META COPYRIGHT = "G GCE 1982"

vectorlist shapes:
    # reset beam to (0,0)
    ORIGIN
    # move to top-left corner where square will start
    MOVE -16 -16
    # draw axis-aligned square from (-16,-16) to (16,16)
    RECT -16 -16 16 16
    # set lower intensity for subsequent inner shapes
    INTENSITY 0x3F
    # recenter before drawing the diamond (rotated square)
    ORIGIN
    # diamond (polygon with 4 points): top -> right -> bottom -> left
    POLYGON 4 0 -16 16 0 0 16 -16 0
    # recenter before circle so it's centered in diamond
    ORIGIN
    # circle centered at (0,0) radius=12 with 24 segments (can tweak segment count)
    CIRCLE 0 0 12 24
    # recenter before arc (so center of arc uses absolute coordinates)
    ORIGIN
    # arc: center (0,-16) radius=16 start=0° sweep=180° -> spans left-to-right over top of square
    ARC 0 -16 16 0 180 24
    # spiral example (commented out for now): center (0,0) rStart=20 rEnd=40 turns=2 segments=64
    #SPIRAL 0 0 20 40 2 64

def main():
    vectrex_draw_vectorlist("shapes")