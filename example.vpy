def add(a, b):
    return a + b

def neg(a):
    return -a

def accumulate(a, b, c, d):
    total = a
    total = total + b
    total = total + c
    total = total + d
    return total

def main():
    s = add(3, 5)
    n = neg(7)
    t = accumulate(1, 2, 3, 4)
    chain_ok = 1 < 2 < 3
    chain_fail = 3 < 2 < 1
    bw = (s & 255) | (t ^ n)
    for i in range(0, 10, 2):
        line(0, 0, i)
    if s and not n:
        line(0,0,0)
    return s + t + n + bw
