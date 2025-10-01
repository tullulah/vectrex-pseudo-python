# FRACTAL TREE DEMO - VPy Language (Versión Dinámica Simplificada)
# Fractal recursivo con trigonometría y animación

# Variables globales para control del fractal
var tree_depth = 3
var base_length = 40
var angle_delta = 30
var animation_frame = 0
var intensity_ = 120

# Variables para animación
var scale_factor = 100
var angle_offset = 0

# Variables de trabajo (OBLIGATORIO: globales en VPy)
var x1 = 0
var y1 = 0
var x2 = 0
var y2 = 0
var length1 = 0
var length2 = 0

# Ángulos para niveles
var angle1_left = 0
var angle1_right = 0
var angle2_ll = 0
var angle2_lr = 0
var angle2_rl = 0
var angle2_rr = 0

# Valores trigonométricos
var cos1_left = 0
var sin1_left = 0
var cos1_right = 0
var sin1_right = 0
var cos2_ll = 0
var sin2_ll = 0
var cos2_lr = 0
var sin2_lr = 0
var cos2_rl = 0
var sin2_rl = 0
var cos2_rr = 0
var sin2_rr = 0

# Coordenadas finales
var x2_left = 0
var y2_left = 0
var x2_right = 0
var y2_right = 0
var x3_ll = 0
var y3_ll = 0
var x3_lr = 0
var y3_lr = 0
var x3_rl = 0
var y3_rl = 0
var x3_rr = 0
var y3_rr = 0

def main():
    # Inicialización
    tree_depth = 3
    base_length = 40
    angle_delta = 30
    intensity_ = 120

def loop():
    # Animación dinámica del fractal
    animation_frame = animation_frame + 1
    if animation_frame >= 360:
        animation_frame = 0
    
    # Variación dinámica del ángulo y escala
    angle_offset = animation_frame * 3
    scale_factor = 90 + SIN(animation_frame * 5) * 30
    
    # Marco de referencia
    DRAW_LINE(-100, -100, 100, -100, 50)
    DRAW_LINE(100, -100, 100, 100, 50)
    DRAW_LINE(100, 100, -100, 100, 50)
    DRAW_LINE(-100, 100, -100, -100, 50)
    
    # === FRACTAL TREE DINÁMICO ===
    # Nivel 0 (tronco principal)
    x1 = 0
    y1 = -50
    x2 = 0
    y2 = y1 + base_length
    DRAW_LINE(x1, y1, x2, y2, intensity_)
    
    # Nivel 1 (ramas principales con animación)
    length1 = base_length * 75 / 100
    angle1_left = 270 + angle_delta + angle_offset / 3
    angle1_right = 270 - angle_delta - angle_offset / 3
    
    # Rama izquierda nivel 1
    cos1_left = COS(angle1_left)
    sin1_left = SIN(angle1_left)
    x2_left = x2 + cos1_left * length1 / scale_factor
    y2_left = y2 + sin1_left * length1 / scale_factor
    DRAW_LINE(x2, y2, x2_left, y2_left, intensity_ - 30)
    
    # Rama derecha nivel 1  
    cos1_right = COS(angle1_right)
    sin1_right = SIN(angle1_right)
    x2_right = x2 + cos1_right * length1 / scale_factor
    y2_right = y2 + sin1_right * length1 / scale_factor
    DRAW_LINE(x2, y2, x2_right, y2_right, intensity_ - 30)
    
    # Nivel 2 (sub-ramas finales)
    length2 = length1 * 65 / 100
    
    # Sub-ramas de la rama izquierda
    angle2_ll = angle1_left + angle_delta
    angle2_lr = angle1_left - angle_delta
    
    cos2_ll = COS(angle2_ll)
    sin2_ll = SIN(angle2_ll)
    x3_ll = x2_left + cos2_ll * length2 / scale_factor
    y3_ll = y2_left + sin2_ll * length2 / scale_factor
    DRAW_LINE(x2_left, y2_left, x3_ll, y3_ll, intensity_ - 50)
    
    cos2_lr = COS(angle2_lr)
    sin2_lr = SIN(angle2_lr)
    x3_lr = x2_left + cos2_lr * length2 / scale_factor
    y3_lr = y2_left + sin2_lr * length2 / scale_factor
    DRAW_LINE(x2_left, y2_left, x3_lr, y3_lr, intensity_ - 50)
    
    # Sub-ramas de la rama derecha
    angle2_rl = angle1_right + angle_delta
    angle2_rr = angle1_right - angle_delta
    
    cos2_rl = COS(angle2_rl)
    sin2_rl = SIN(angle2_rl)
    x3_rl = x2_right + cos2_rl * length2 / scale_factor
    y3_rl = y2_right + sin2_rl * length2 / scale_factor
    DRAW_LINE(x2_right, y2_right, x3_rl, y3_rl, intensity_ - 50)
    
    cos2_rr = COS(angle2_rr)
    sin2_rr = SIN(angle2_rr)
    x3_rr = x2_right + cos2_rr * length2 / scale_factor
    y3_rr = y2_right + sin2_rr * length2 / scale_factor
    DRAW_LINE(x2_right, y2_right, x3_rr, y3_rr, intensity_ - 50)