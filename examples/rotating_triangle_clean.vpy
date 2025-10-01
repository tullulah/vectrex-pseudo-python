META TITLE = "ROTATING TRIANGLE"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

# Example: Rotating triangle using sin/cos and DRAW_LINE
# Demonstrates: sin(), cos(), DRAW_LINE() unified function

# Global variable for rotation angle
var angle = 0

def main():
    # Initialize angle
    angle = 0

def loop():
    # Calculate triangle vertices using sin/cos (0-127 angle system)
    # Triangle with 3 points rotating around center
    
    # Point 1: Top vertex
    let x1 = cos(angle) / 2
    let y1 = sin(angle) / 2
    
    # Point 2: Bottom-right vertex (120° offset)
    let angle2 = angle + 42  # 42 ≈ 120° in 0-127 system
    if angle2 > 127:
        angle2 = angle2 - 128
    let x2 = cos(angle2) / 2  
    let y2 = sin(angle2) / 2
    
    # Point 3: Bottom-left vertex (240° offset)
    let angle3 = angle + 85  # 85 ≈ 240° in 0-127 system
    if angle3 > 127:
        angle3 = angle3 - 128
    let x3 = cos(angle3) / 2
    let y3 = sin(angle3) / 2
    
    # Draw triangle sides using unified DRAW_LINE function
    DRAW_LINE(x1, y1, x2, y2, 255)  # Side 1-2
    DRAW_LINE(x2, y2, x3, y3, 255)  # Side 2-3  
    DRAW_LINE(x3, y3, x1, y1, 255)  # Side 3-1
    
    # Increment angle for rotation
    angle = angle + 1
    if angle > 127:
        angle = 0
