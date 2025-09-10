def add(a, b):
    return a + b

def main():
    let x = 0x10
    let y = 0b1010
    let s = add(x, y)
    let inv = ~s              # bitwise not
    let sh = (s << 3) >> 1    # net *4
    let r = (sh % 16) ^ (inv & 0xFF)
    acc = 0  # implicit global
    for i in range(0, 16, 2):
        acc = acc + (i & 3)
    if 1 < 2 < 3:
        acc = acc + r
    return acc + s + r
