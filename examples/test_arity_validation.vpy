# Test file for arity validation - VPy uses Python-style syntax with parentheses

# These should show arity errors:
MOVE(100)             # Error: expects 2 args, got 1
RECT(50, 50)          # Error: expects 4 args, got 2  
CIRCLE()              # Error: expects 1 arg, got 0
INTENSITY(255, 128)   # Error: expects 1 arg, got 2
PRINT_TEXT(10, 20)    # Error: expects 3 args, got 2

# These should show syntax errors (missing parentheses):
MOVE 100, 200         # Error: function must use parentheses
RECT 10, 20, 30, 40   # Error: function must use parentheses

# These should be correct:
MOVE(100, 200)
RECT(10, 20, 30, 40)
CIRCLE(50)
INTENSITY(255)
PRINT_TEXT(10, 20, "Hello World")
ORIGIN()

# Special case - POLYGON should require at least 3 args (count + min 1 point)
POLYGON(3)            # Error: expects at least 3 args, got 1
POLYGON(3, 10)        # Error: expects at least 3 args, got 2
POLYGON(3, 10, 20)    # OK: minimum valid
POLYGON(5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100)  # OK: more args
