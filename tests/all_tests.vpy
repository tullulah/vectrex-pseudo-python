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
    # Comprobamos realmente que la dirección es no‑cero; devolvemos 1 si ok, 0 si falla
    if s != 0:
        return 1
    return 0

# Switch test: value 2 selects case 2 -> returns 20
def test_switch_basic():
    let acc = 0
    v = 2
    switch v:
        case 1:
            acc = 10
        case 2:
            acc = 20
        case 3:
            acc = 30
        default:
            acc = 99
    return acc

const MAGIC = 5
const BONUS = 2

def test_const_basic():
    x = MAGIC + BONUS  # should constant fold to 7
    return x

def test_string_escapes():
    s = "Line1\nLine2\tX\\Y\x21"  # contains newline, tab, backslash, hex !
    if s != 0:
        return 1
    return 0

def test_switch_dense():
    v = 4
    switch v:
        case 3:
            return 30
        case 4:
            return 40
        case 5:
            return 50
        default:
            return 99
    return 0

# Constant switch fold test: expression and cases all numeric; should fold to the matching body (case 2 -> 200)
def test_switch_const_fold():
    switch 2:
        case 1:
            return 100
        case 2:
            return 200
        case 3:
            return 300
        default:
            return 999
    return 0

def main():
    shadow_res = test_shadow()
    basic_res = test_for_basic()
    step_res = test_for_step()
    nested_res = test_for_nested()
    simple_res = test_simple_local()
    string_res = test_string()
    string_addr_res = test_string_address()
    switch_res = test_switch_basic()
    const_res = test_const_basic()
    esc_res = test_string_escapes()
    dense_res = test_switch_dense()
    const_fold_res = test_switch_const_fold()
    # Base: 15+3+6+9+6 = 39
    # + string_res (1) = 40
    # + string_addr_res (1) = 41
    # + switch_res (20) = 61
    # + const_res (7) = 68
    # + esc_res (1) = 69
    # + dense_res (40) = 109
    # + const_fold_res (200) = 309
    total = shadow_res + basic_res + step_res + nested_res + simple_res + string_res + string_addr_res + switch_res + const_res + esc_res + dense_res + const_fold_res
    return total
