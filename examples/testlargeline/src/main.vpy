# testlargeline - Main entry point
# VPy game for Vectrex

META TITLE = "testlargeline"

line_length = 10

def main():
    # Called once at startup
    Set_Intensity(127)

def loop():

    # Test LONG line with variable length - grows diagonally from bottom-left to top-right
    DRAW_LINE(-127, -127, -127 + line_length, -127 + line_length, 80)
    line_length = line_length + 2
    # Wrap around if gets too long
    if line_length > 250:
        line_length = 10
