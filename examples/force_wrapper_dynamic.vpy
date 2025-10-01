META TITLE = "FORCE WRAPPER TEST"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

# Global variable to make values dynamic
var frame_count = 0

def main():
    frame_count = 0

def loop():
    # Use frame_count to make coordinates dynamic (not optimizable)
    let base_x = frame_count * 0  # Always 0 but compiler can't optimize
    let base_y = frame_count * 0  # Always 0 but compiler can't optimize
    
    # Test 1: Línea horizontal con valores "dinámicos"
    DRAW_LINE(base_x, base_y, base_x + 50, base_y, 127)
    
    # Test 2: Línea vertical con valores "dinámicos"  
    DRAW_LINE(base_x, base_y, base_x, base_y + 50, 127)
    
    # Increment frame count to keep it "dynamic"
    frame_count = frame_count + 1
