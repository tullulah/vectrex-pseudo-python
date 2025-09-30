# Archivo de prueba para verificar LSP - funciones unificadas

# Funciones unificadas a nivel global (con paréntesis)
MOVE(100, 100)
SET_INTENSITY(255)

# Funciones específicas de dibujo directo (solo globales)
RECT(10, 10, 50, 50)
CIRCLE(25)

# VECTORLIST usando las mismas funciones unificadas (con espacios)
VECTORLIST my_vectors:
    MOVE 0 0                # mismo nombre que función global
    SET_INTENSITY 128       # mismo nombre que función global  
    SET_ORIGIN              # función unificada
    MOVE 50 50              # misma función

# Esto simplifica mucho el lenguaje - un solo conjunto de nombres
# Sintaxis: paréntesis en global, espacios en vectorlist

def my_function():
    MOVE(200, 200)          # función unificada (global)
    SET_INTENSITY(64)       # función unificada (global)