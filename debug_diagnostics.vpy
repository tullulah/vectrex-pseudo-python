# Test file for debugging diagnostics
META title="Debug Diagnostics"

def main
    # Missing parenthesis - should cause syntax error
    PRINT_TEXT 100, 50, "Hello"
    # Too many arguments - should cause semantic error  
    PRINT_TEXT 10, 20, "Test", "Extra"
RETURN