import unused_module

META TITLE = "Tree Shaking Test"

# These should be kept (used)
player_x = 0
player_y = 0

# These should be removed (unused)
unused_global_1 = 100
unused_global_2 = 200

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    # Only calls used_function - others should be shaken
    value = unused_module.used_function()
    player_x = value
    PRINT_TEXT(-50, 50, "TREE SHAKE TEST")

# Unused functions - should be removed by tree shaking
def never_called_1():
    return 123

def never_called_2():
    return 456
