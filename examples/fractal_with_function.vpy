# FRACTAL TREE DEMO - VPy Language (Con Función Personalizada)
# Demostración de que SÍ se pueden crear funciones en VPy

# Variables globales para control del fractal
var tree_depth = 3
var base_length = 40
var angle_delta = 30
var animation_frame = 0
var intensity_ = 120

# Variables para animación
var scale_factor = 100
var angle_offset = 0

def main():
    # Inicialización
    tree_depth = 3
    base_length = 40
    angle_delta = 30
    intensity_ = 120

def draw_tree():
    # FUNCIÓN PERSONALIZADA - SÍ funciona en VPy!
    
    # Nivel 0 (tronco principal)
    let x1 = 0
    let y1 = -50
    let x2 = 0
    let y2 = y1 + base_length
    DRAW_LINE(x1, y1, x2, y2, intensity_)
    
    # Nivel 1 (ramas principales con animación)
    let length1 = base_length * 75 / 100
    let angle1_left = 270 + angle_delta + angle_offset / 3
    let angle1_right = 270 - angle_delta - angle_offset / 3
    
    # Rama izquierda nivel 1
    let cos1_left = COS(angle1_left)
    let sin1_left = SIN(angle1_left)
    let x2_left = x2 + cos1_left * length1 / scale_factor
    let y2_left = y2 + sin1_left * length1 / scale_factor
    DRAW_LINE(x2, y2, x2_left, y2_left, intensity_ - 30)
    
    # Rama derecha nivel 1  
    let cos1_right = COS(angle1_right)
    let sin1_right = SIN(angle1_right)
    let x2_right = x2 + cos1_right * length1 / scale_factor
    let y2_right = y2 + sin1_right * length1 / scale_factor
    DRAW_LINE(x2, y2, x2_right, y2_right, intensity_ - 30)
    
    # Nivel 2 (sub-ramas finales)
    let length2 = length1 * 65 / 100
    
    # Sub-ramas de la rama izquierda
    let angle2_ll = angle1_left + angle_delta
    let angle2_lr = angle1_left - angle_delta
    
    let cos2_ll = COS(angle2_ll)
    let sin2_ll = SIN(angle2_ll)
    let x3_ll = x2_left + cos2_ll * length2 / scale_factor
    let y3_ll = y2_left + sin2_ll * length2 / scale_factor
    DRAW_LINE(x2_left, y2_left, x3_ll, y3_ll, intensity_ - 50)
    
    let cos2_lr = COS(angle2_lr)
    let sin2_lr = SIN(angle2_lr)
    let x3_lr = x2_left + cos2_lr * length2 / scale_factor
    let y3_lr = y2_left + sin2_lr * length2 / scale_factor
    DRAW_LINE(x2_left, y2_left, x3_lr, y3_lr, intensity_ - 50)
    
    # Sub-ramas de la rama derecha
    let angle2_rl = angle1_right + angle_delta
    let angle2_rr = angle1_right - angle_delta
    
    let cos2_rl = COS(angle2_rl)
    let sin2_rl = SIN(angle2_rl)
    let x3_rl = x2_right + cos2_rl * length2 / scale_factor
    let y3_rl = y2_right + sin2_rl * length2 / scale_factor
    DRAW_LINE(x2_right, y2_right, x3_rl, y3_rl, intensity_ - 50)
    
    let cos2_rr = COS(angle2_rr)
    let sin2_rr = SIN(angle2_rr)
    let x3_rr = x2_right + cos2_rr * length2 / scale_factor
    let y3_rr = y2_right + sin2_rr * length2 / scale_factor
    DRAW_LINE(x2_right, y2_right, x3_rr, y3_rr, intensity_ - 50)

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
    
    # Llamar a la función personalizada
    draw_tree()