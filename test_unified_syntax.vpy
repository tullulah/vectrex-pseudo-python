# Prueba de sintaxis unificada VPy
# Ahora todos los comandos de vectorlist usan paréntesis igual que las funciones globales

# Funciones globales
MOVE(100, 100)
INTENSITY(255)

# VECTORLIST con sintaxis unificada
VECTORLIST unified_syntax:
    MOVE(0, 0)                              # Unificado: paréntesis
    SET_INTENSITY(128)                      # Unificado: paréntesis  
    RECT(10, 10, 50, 50)                    # Unificado: paréntesis
    CIRCLE(25, 25, 15)                      # Unificado: paréntesis
    CIRCLE(50, 50, 20, 32)                  # Unificado: paréntesis con segs opcional
    ARC(75, 75, 10, 0, 90)                  # Unificado: paréntesis
    ARC(100, 100, 15, 0, 180, 16)           # Unificado: paréntesis con segs opcional
    SPIRAL(125, 125, 5, 20, 3)              # Unificado: paréntesis
    SPIRAL(150, 150, 10, 25, 2, 64)         # Unificado: paréntesis con segs opcional

def test_function():
    MOVE(200, 200)
    RECT(0, 0, 100, 100)