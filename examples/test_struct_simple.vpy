META TITLE = "Simple Struct Test"

struct Point:
    x: int
    y: int

def main():
    p = Point()
    p.x = 100
    p.y = 50

def loop():
    WAIT_RECAL()
    p = Point()
    p.x = 10
    p.y = 20
    result = p.x + p.y
