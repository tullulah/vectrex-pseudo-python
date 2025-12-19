META TITLE = "Array Test Global"

# Arrays globales para demostración
var numbers = [10, 20, 30]
var positions_x = [-50, 0, 50]

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    
    # Leer y modificar el primer elemento
    let first = numbers[0]
    numbers[0] = first + 1
    
    # Leer diferentes posiciones
    let pos0 = positions_x[0]
    let pos1 = positions_x[1]
    let pos2 = positions_x[2]
    
    # Usar valores (para que no se optimicen)
    let sum = pos0 + pos1 + pos2
