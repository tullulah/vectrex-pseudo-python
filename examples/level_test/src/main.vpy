META TITLE = "Level Debug"
META AUTHOR = "VPy Team"

# Debug SHOW_LEVEL with breakpoints

def main():
    SET_INTENSITY(127)
    LOAD_LEVEL("test_level")
    # Breakpoint 1: After LOAD_LEVEL
   

def loop():
    # Breakpoint 2: Before SHOW_LEVEL

    SHOW_LEVEL()
    # Breakpoint 3: After SHOW_LEVEL

    update_level()

    

