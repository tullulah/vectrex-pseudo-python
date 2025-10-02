def main():
    let x = 10
    let y = 5
    
    # Test compound assignment operators
    x += y  # x should be 15
    y -= 2  # y should be 3
    x *= 2  # x should be 30
    y /= 3  # y should be 1
    x %= 7  # x should be 2
    
    # Test with expressions
    x += y * 3  # x should be 5
    
    return x

def loop():
    return 0