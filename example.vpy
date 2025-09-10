def add(a, b):
    return a + b

def main():
    x = 0x10
    y = 0b1010
    s = add(x, y)
    inv = ~s              # bitwise not
    sh = (s << 3) >> 1    # net *4
    r = (sh % 16) ^ (inv & 0xFF)
    acc = 0
    for i in range(0, 16, 2):
        acc = acc + (i & 3)
    if 1 < 2 < 3:
        acc = acc + r
    return acc + s + r
