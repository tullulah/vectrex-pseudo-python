# Archivo de prueba para verificar LSP - expresiones top-level

# Funciones globales (deben funcionar directamente)
MOVE(100, 100)
INTENSITY(255)
RECT(10, 10, 50, 50)
CIRCLE(25)

def my_function():
    MOVE(200, 200)         # función global válida
    return 42