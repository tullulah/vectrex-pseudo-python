# testlargeline - Main entry point
# VPy game for Vectrex

META TITLE = "testlargeline"

def main():
    # Called once at startup
    Set_Intensity(127)

def loop():
    # Test LONG line using DRAW_LINE with absolute coordinates
    # This line goes from (0, -100) to (0, 72), dy = 172 pixels
    # Requires segmentation: segment 1 draws 127, segment 2 draws 45
    DRAW_LINE(0, -100, 0, 75, 80)
