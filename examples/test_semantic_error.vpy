# Test file for semantic error detection
META title="Test PRINT_TEXT Arity Error"

def main
    # This should cause a semantic error - PRINT_TEXT expects 3 arguments but we're passing 4
    PRINT_TEXT 100, 50, "Hello", "World"
RETURN