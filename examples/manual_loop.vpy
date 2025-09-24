# manual_loop.vpy - ejemplo con bucle de frame explícito cuando se usa --no-auto-loop
# El usuario es responsable de llamar a WAIT_RECAL y RESET0REF (expuestos como built-ins futuros o wrappers).
# Mientras no exista un wrapper directo, se puede simular un frame con funciones estándar ya generadas:
# Usaremos una función frame() que sería llamada desde main() en un bucle infinito.

# Nota: Hasta que haya wrappers explícitos para WAIT_RECAL / RESET0REF, este ejemplo
# sólo ilustra la estructura Python-like; el backend todavía inserta las BIOS calls
# si auto_loop está activo. Con --no-auto-loop, main() debe no retornar.

def frame(counter):
    # Aquí irían: VECTREX_SET_ORIGIN(), VECTREX_SET_INTENSITY(0x5F), etc.
    # Dibujar texto básico
    VECTREX_PRINT_TEXT(0, 0, "MANUAL LOOP")
    return counter + 1

def main():
    c = 0
    while 1:
        c = frame(c)
        # Futuro: invocar wrappers WAIT_RECAL() y VECTREX_SET_ORIGIN() aquí
    return 0
