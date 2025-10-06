# Test de funciones multilinea
# Este archivo testa las nuevas capacidades multilinea del parser

# Test 1: DRAW_POLYGON multilinea
def test_polygon():
    size = 10
    x = 50
    y = 50
    
    DRAW_POLYGON(4, I_MEDIUM,
                 x - size, y - size,
                 x + size, y - size,
                 x + size, y + size,
                 x - size, y + size)

# Test 2: Función con muchos parámetros
def test_complex_function():
    DRAW_COMPLEX_SHAPE(
        x=100,
        y=100,
        width=20,
        height=30,
        intensity=I_BRIGHT,
        rotation=45,
        scale=2.0
    )

# Test 3: For loop con range multilinea  
def test_for_loop():
    for i in range(0,
                   100,
                   5):
        DRAW_POINT(i, i, I_MEDIUM)

# Test 4: Expresión matemática multilinea
def test_math():
    result = (10 + 20 +
              30 + 40 +
              50)
    return result

# Test 5: Llamada anidada multilinea
def test_nested():
    DRAW_POLYGON(3, I_BRIGHT,
                 calculate_point(10,
                                20,
                                30),
                 calculate_point(40,
                                50,
                                60),
                 calculate_point(70,
                                80,
                                90))

def main():
    test_polygon()
    test_complex_function()
    test_for_loop()
    result = test_math()
    test_nested()