META TITLE = "PAC VECTOR MAZE"
META COPYRIGHT = "G GCE 1982"

# Simplified Pac-Man style maze layout.
# Coordinate system centered at (0,0). Walls drawn as rectangles / line segments.
# You can tweak sizes or add more segments; keep total vectors modest for flicker.

vectorlist maze:
    INTENSITY 0x7F
    # --- Outer border (with side tunnels gap at mid Y) ---
    ORIGIN
    MOVE -68 -68
    RECT -68 -68 68 -67          # top edge
    ORIGIN
    MOVE -68 67
    RECT -68 67 68 68            # bottom edge
    # Left side split (gap from -8..+8)
    ORIGIN
    MOVE -68 -68
    RECT -68 -68 -67 -8          # left upper
    ORIGIN
    MOVE -68 8
    RECT -68 8 -67 68            # left lower
    # Right side split (gap from -8..+8)
    ORIGIN
    MOVE 67 -68
    RECT 67 -68 68 -8            # right upper
    ORIGIN
    MOVE 67 8
    RECT 67 8 68 68              # right lower

    # --- Top area bars ---
    ORIGIN
    MOVE -56 -52
    RECT -56 -52 56 -51          # top interior bar
    ORIGIN
    MOVE -40 -40
    RECT -40 -40 40 -39          # second interior bar
    # Small top center notch
    ORIGIN
    MOVE -8 -60
    RECT -8 -60 8 -59            # small bar above center

    # --- Upper mid horizontal corridors ---
    ORIGIN
    MOVE -60 -24
    RECT -60 -24 -10 -23         # left upper corridor segment
    ORIGIN
    MOVE 10 -24
    RECT 10 -24 60 -23           # right upper corridor segment
    ORIGIN
    MOVE -60 -8
    RECT -60 -8 60 -7            # full width corridor (connects tunnels)

    # --- Central area (ghost house) ---
    ORIGIN
    MOVE -28  -4
    RECT -28 -4 28 -3             # ghost box top
    ORIGIN
    MOVE -28 20
    RECT -28 20 28 21             # ghost box bottom
    ORIGIN
    MOVE -28 -4
    RECT -28 -4 -27 20            # ghost box left
    ORIGIN
    MOVE 27 -4
    RECT 27 -4 28 20              # ghost box right
    # Door posts (gap center)
    ORIGIN
    MOVE -8 6
    RECT -8 6 -7 14               # left door post
    ORIGIN
    MOVE 7 6
    RECT 7 6 8 14                 # right door post

    # --- Lower mid horizontal corridors ---
    ORIGIN
    MOVE -60 8
    RECT -60 8 60 9               # full width corridor below house
    ORIGIN
    MOVE -60 24
    RECT -60 24 -10 25            # left lower corridor
    ORIGIN
    MOVE 10 24
    RECT 10 24 60 25              # right lower corridor

    # --- Bottom area bars ---
    ORIGIN
    MOVE -56 40
    RECT -56 40 56 41             # upper bottom bar
    ORIGIN
    MOVE -40 52
    RECT -40 52 40 53             # lower bottom bar

    # --- Vertical pillars / dividers ---
    ORIGIN
    MOVE -40 -40
    RECT -40 -40 -39 -8           # upper left vertical
    ORIGIN
    MOVE -40 8
    RECT -40 8 -39 40             # lower left vertical
    ORIGIN
    MOVE 39 -40
    RECT 39 -40 40 -8             # upper right vertical
    ORIGIN
    MOVE 39 8
    RECT 39 8 40 40               # lower right vertical
    ORIGIN
    MOVE -8 -24
    RECT -8 -24 -7 -8             # mid left small vertical
    ORIGIN
    MOVE 7 -24
    RECT 7 -24 8 -8               # mid right small vertical
    ORIGIN
    MOVE -8 8
    RECT -8 8 -7 24               # mid left lower small vertical
    ORIGIN
    MOVE 7 8
    RECT 7 8 8 24                 # mid right lower small vertical

    # --- Center pillars bottom ---
    ORIGIN
    MOVE -4 60
    RECT -4 60 -3 68
    ORIGIN
    MOVE 3 60
    RECT 3 60 4 68

vectorlist pellets:
    # Dimmer intensity for pellets
    INTENSITY 0x30
    # Row of pellets near top
    ORIGIN
    MOVE -48 -32
    RECT -49 -33 -47 -31  # pellet 1
    ORIGIN
    MOVE -36 -32
    RECT -37 -33 -35 -31  # pellet 2
    ORIGIN
    MOVE -24 -32
    RECT -25 -33 -23 -31  # pellet 3
    ORIGIN
    MOVE -12 -32
    RECT -13 -33 -11 -31  # pellet 4
    ORIGIN
    MOVE 0 -32
    RECT -1 -33 1 -31     # pellet 5
    ORIGIN
    MOVE 12 -32
    RECT 11 -33 13 -31    # pellet 6
    ORIGIN
    MOVE 24 -32
    RECT 23 -33 25 -31    # pellet 7
    ORIGIN
    MOVE 36 -32
    RECT 35 -33 37 -31    # pellet 8
    ORIGIN
    MOVE 48 -32
    RECT 47 -33 49 -31    # pellet 9

    # Power pellets (bigger squares) corners just inside outer boundary
    INTENSITY 0x50
    ORIGIN
    MOVE -52 -52
    RECT -54 -54 -50 -50  # power pellet TL
    ORIGIN
    MOVE 50 -52
    RECT 48 -54 52 -50    # power pellet TR
    ORIGIN
    MOVE -52 52
    RECT -54 50 -50 54    # power pellet BL
    ORIGIN
    MOVE 50 52
    RECT 48 50 52 54      # power pellet BR

vectorlist actors:
    INTENSITY 0x5F
    # Player starting position marker (small diamond)
    ORIGIN
    MOVE 0 24
    POLYGON 4 0 -2 2 0 0 2 -2 0
    # Ghost placeholders (three small circles) inside ghost house
    CIRCLE -12 0 3 12
    CIRCLE 0 0 3 12
    CIRCLE 12 0 3 12


def main():
    vectrex_draw_vectorlist("maze")
    vectrex_draw_vectorlist("pellets")
    vectrex_draw_vectorlist("actors")
