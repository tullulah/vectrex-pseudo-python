META TITLE = "Global Variable Sharing Test"

# Test Case 5: Can global variables be shared across modules?
# state.vpy has: counter = 0
# main.vpy: calls state.increment() which modifies counter

import state

test_counter = 0

def main():
    SET_INTENSITY(127)
    state.init_state()

def loop():
    WAIT_RECAL()
    state.increment()
    test_counter = state.get_counter()
