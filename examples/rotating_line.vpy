META TITLE = "ROTATING LINE"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

# Example: Rotating line using trigonometric functions
# Demonstrates: sin(), cos(), DRAW_LINE(), variable scoping

# Global variable for rotation angle
var angle = 0

def main():
    # Initialize angle
    angle = 0

def loop():
    # sin() and cos() use angles 0-127 (represents 0-360°)
    # Return values are in range -127 to +127
    
    # Calculate endpoint of line using trig functions
    # Divide by 2 to keep values in ±63 range (required for DRAW_LINE)
    let x_end = cos(angle) / 2
    let y_end = sin(angle) / 2
    
    # Draw line from center (0,0) to calculated endpoint
    # DRAW_LINE(x0, y0, x1, y1, intensity)
    DRAW_LINE(0, 0, x_end, y_end, 127)
    
    # Increment angle (0-127 represents full 360° rotation)
    angle = angle + 1
    if angle > 127:
        angle = 0
