META TITLE = "SPINNING STAR"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

# Example: Spinning star with multiple rotating lines
# Demonstrates: sin(), cos(), loops, trigonometric patterns

var angle = 0

def main():
    angle = 0

def loop():
    # Draw 8 lines from center creating a star pattern
    let i = 0
    while i < 8:
        # Calculate angle for this spoke
        let spoke_angle = angle + (i * 16)  # 16 = 128/8 for 8 spokes
        
        # Keep angle in 0-127 range
        if spoke_angle > 127:
            spoke_angle = spoke_angle - 128
        
        # Calculate endpoint (divide by 2 for smaller radius)
        let x = cos(spoke_angle) / 2
        let y = sin(spoke_angle) / 2
        
        # Draw line from center to spoke endpoint
        DRAW_LINE(0, 0, x, y, 100)
        
        i = i + 1
    
    # Rotate the entire star
    angle = angle + 1
    if angle > 127:
        angle = 0
