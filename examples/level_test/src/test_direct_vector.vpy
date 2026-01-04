META TITLE = "Direct Vector Test"
META AUTHOR = "VPy Team"

# Test 1: Draw vector directly with DRAW_VECTOR_EX
# This uses the SAME function as level system (Draw_Sync_List_At_With_Mirrors)

def main():
    SET_INTENSITY(127)

def loop():
    PRINT_TEXT(-90, 100, "DIRECT VECTOR EX")
    DRAW_VECTOR_EX("mountain", 0, 0, 0, 0)  # center, no mirror, no intensity override
