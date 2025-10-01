META TITLE = "ROTATING LINE"
META COPYRIGHT = "g GCE 1982"
META MUSIC = "0"

# Variables globales (usar 'var' para globales)
var angle = 0

def main():
    # Inicialización
    angle = 0

def loop():
    # Las funciones sin/cos usan ángulos 0-127 (0-127 representa 0-360°)
    # Los valores de retorno están en el rango -127 a +127
    
    # Calcular coordenadas del extremo de la línea
    # Usamos directamente los valores de sin/cos sin escalar
    # porque ya están en el rango correcto (-127 a +127)
    let x_end = cos(angle) / 2  # Dividir por 2 para radio ~63
    let y_end = sin(angle) / 2  # Dividir por 2 para radio ~63
    
    # Dibujar línea desde el centro (0,0) hasta el punto calculado
    # DRAW_LINE(x0, y0, x1, y1, intensity)
    DRAW_LINE(0, 0, x_end, y_end, 127)
    
    # Incrementar ángulo (0-127 representa ciclo completo)
    angle = angle + 1
    if angle > 127:
        angle = 0
