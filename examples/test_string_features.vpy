META TITLE = "String Features Test"
META AUTHOR = "Test"

# Test 1: Global string (already working)
var global_str = "GLOBAL"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Test 2: Local string literal (NEW feature #1)
    let local_str = "LOCAL"
    
    # Test 3: DEBUG_PRINT_STR with direct literal (NEW feature #2)
    DEBUG_PRINT_STR("LITERAL")
    
    # Test 4: DEBUG_PRINT_STR with global variable (already working)
    DEBUG_PRINT_STR(global_str)
    
    # Test 5: DEBUG_PRINT_STR with local variable (NEW feature #1)
    DEBUG_PRINT_STR(local_str)
