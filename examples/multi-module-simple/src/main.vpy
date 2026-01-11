# Main using direct imports (without module prefix)

from utils import add, mul

x = 0

def main():
    SET_INTENSITY(127)
    x = add(10, 20)

def loop():
    WAIT_RECAL()
    result = mul(x, 2)
    SET_INTENSITY(result)
