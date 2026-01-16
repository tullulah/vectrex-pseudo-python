META TITLE = "Math Extended Test"

angle = 0
base = 2
exp = 3
random_val = 0

def main():
    pass

def loop():
    # Test trigonometry (angle 0-127 represents 0-360°)
    sin_val = SIN(angle)
    cos_val = COS(angle)
    tan_val = TAN(angle)
    
    # Test square root
    sqrt_val = SQRT(100)
    
    # Test power
    pow_val = POW(base, exp)  # 2^3 = 8
    
    # Test arctangent
    atan_val = ATAN2(10, 5)
    
    # Test random
    random_val = RAND()
    random_range = RAND_RANGE(10, 50)
    
    # Display results on screen
    PRINT_NUMBER(-50, 60, sin_val)
    PRINT_NUMBER(-50, 40, cos_val)
    PRINT_NUMBER(-50, 20, tan_val)
    PRINT_NUMBER(0, 60, sqrt_val)
    PRINT_NUMBER(0, 40, pow_val)
    PRINT_NUMBER(0, 20, atan_val)
    PRINT_NUMBER(50, 60, random_val)
    PRINT_NUMBER(50, 40, random_range)
    
    # Increment angle for animation
    angle = (angle + 1) % 128
