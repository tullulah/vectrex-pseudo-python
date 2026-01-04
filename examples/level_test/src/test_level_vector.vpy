META TITLE = "Level Vector Test"
META AUTHOR = "VPy Team"

# Test 2: Draw same vector through SHOW_LEVEL
# This should produce identical output but currently doesn't work

def main():
    SET_INTENSITY(127)
    LOAD_LEVEL("test_level")

def loop():
    PRINT_TEXT(-90, 100, "LEVEL VECTOR")
    SHOW_LEVEL()
