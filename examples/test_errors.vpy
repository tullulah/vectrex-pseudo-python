# Test file for arity validation

# These should generate errors:
PRINT_TEXT(10, 20, "Test", "extra_arg")  # Too many args (4 instead of 3)
MOVE(10)                                 # Too few args (1 instead of 2)  
CIRCLE()                                 # Too few args (0 instead of 1)
POLYGON(2, 10, 20)                       # Too few args (3 instead of at least 3 for valid polygon)

# These should be correct:
MOVE(10, 20)
CIRCLE(50)
PRINT_TEXT(10, 20, "Hello")
POLYGON(4, 0, 0, 10, 0, 10, 10, 0, 10)

# Test syntax without parentheses (should also error):
MOVE 10, 20
CIRCLE 50