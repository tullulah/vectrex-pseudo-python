META TITLE = "DRAW_LINE UNIFIED TEST"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

# Final test: unified DRAW_LINE function

def main():
    let dummy = 0

def loop():
    # Simple test pattern using only DRAW_LINE
    DRAW_LINE(-50, 0, 50, 0, 255)    # Horizontal line
    DRAW_LINE(0, -50, 0, 50, 255)    # Vertical line
    DRAW_LINE(-35, -35, 35, 35, 200) # Diagonal \
    DRAW_LINE(-35, 35, 35, -35, 200) # Diagonal /
