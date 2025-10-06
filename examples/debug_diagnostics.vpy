# Test file for debugging diagnostics flow
# Este archivo contiene errores intencionales para probar el flujo de diagnósticos

# Error 1: MOVE con aridad incorrecta (necesita 2, tiene 3)
MOVE(100, 200, 300)

# Error 2: CIRCLE sin paréntesis (debe usar sintaxis Python)
CIRCLE 50, 100, 25

# Error 3: POLYGON con demasiados argumentos (máximo 20, aquí 21)
POLYGON(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22)

# Error 4: PRINT_TEXT con 4 argumentos en lugar de 3
PRINT_TEXT(100, 200, "Hello", "Extra")

# Error 5: Función inexistente
INVALID_FUNCTION(123)

xczc
