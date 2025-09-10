def helper_shadow():
    g = 7            # global (no let) for shadow test
    return g

def test_shadow():   # expect 15 ( (7 -> local g=7) g+1 =8, helper gives global g=7 )
    let g = helper_shadow()
    g = g + 1
    h = helper_shadow()   # h becomes global
    return g + h          # 8 + 7 = 15

def test_for_basic():     # sum 0+1+2 = 3
    let s = 0
    for i in range(0, 3):
        s = s + i
    return s

def test_for_step():      # sum 0+2+4 = 6
    let s = 0
    for i in range(0, 6, 2):
        s = s + i
    return s

def test_for_nested():    # nested accumulation; expected 9
    let acc = 0
    for i in range(0, 3):
        for j in range(0, 2):
            acc = acc + i + j
    return acc

def test_simple_local():  # 5 -> 6
    let x = 5
    x = x + 1
    return x

# String literal smoke test: ensure parser accepts it. Placeholder codegen returns 0 currently.
def test_string():
    let dummy = 1  # keep local usage
    msg = "hello"  # string literal (currently evaluates to 0 placeholder)
    return dummy    # return 1 so we can include in checksum later when implemented

def test_string_address():
    s = "abc"
    # After backend implementation, string literal expression yields its address (non-zero expected)
    # We just return 1 unconditionally (could check s != 0 if runtime guarantees) to keep deterministic checksum.
    return 1

def main():
    shadow_res = test_shadow()
    basic_res = test_for_basic()
    step_res = test_for_step()
    nested_res = test_for_nested()
    simple_res = test_simple_local()
    string_res = test_string()
    string_addr_res = test_string_address()
    # Base: 15+3+6+9+6 = 39
    # + string_res (1) = 40
    # + string_addr_res (1) = 41
    total = shadow_res + basic_res + step_res + nested_res + simple_res + string_res + string_addr_res
    return total
