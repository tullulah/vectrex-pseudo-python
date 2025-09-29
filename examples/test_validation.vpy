# Test file para validar el filtrado mejorado

var stage = 0  # variable global (contador)

def main():
    stage = stage + 1
    if stage > 400:
        stage = 400

    # Estas líneas NO deben mostrar errores:
    DRAW_POLYGON(8, I_STRONG, -78,90, 78,90, 100,70, 100,-70, 78,-90, -78,-90, -100,-70, -100,70)
    DRAW_CIRCLE(-50, -64, 10, I_STRONG)
    DRAW_CIRCLE_SEG(32, -50, -64, 28, I_MED)
    
    # Esta línea SÍ debe mostrar error (función desconocida):
    # UNKNOWN_FUNCTION(1, 2, 3)