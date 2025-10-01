# Test file to verify diagnostic accumulation from LSP and compiler
# This should trigger both LSP errors (unknown function) and compiler errors

# LSP error: unknown function
unknown_function()

# Another LSP error: deprecated function usage (should trigger warning)
MOVETO(10, 20)  

# Valid VPy code with unified syntax
MOVE(50, 50)
INTENSITY(128)

# VECTORLIST with unified syntax
VECTORLIST test_vectors:
    MOVE(0, 0)
    SET_INTENSITY(255)
    RECT(10, 10, 100, 100)