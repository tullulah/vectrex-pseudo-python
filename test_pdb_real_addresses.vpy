# Test PDB Real Addresses
# Este archivo prueba que el .pdb contiene direcciones reales (no 0x0000)

def setup():
    VECTREX_WAIT_RECAL()

def main():
    setup()

def loop():
    VECTREX_INTENSITY(0x7F)
    VECTREX_MOVE_TO(50, 50)
    VECTREX_DRAW_TO(0, 0)
