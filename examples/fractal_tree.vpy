# FRACTAL TREE DEMO - VPy Language (Versión Dinámica)
# Fractal recursivo con trigonometría y animación

# Variables globales para control del fractal
var tree_depth = 4
var base_length = 50
var angle_delta = 25
var animation_frame = 0
var intensity_ = 120

# Variables para animación
var scale_factor = 100
var angle_offset = 0

def main():
    # Inicialización
    tree_depth = 4
    base_length = 50
    angle_delta = 25
    intensity_ = 120

def loop():
    # Animación dinámica
    animation_frame = animation_frame + 1
    if animation_frame >= 360:
        animation_frame = 0
    
    # Variación dinámica del ángulo y escala
    angle_offset = animation_frame * 2
    scale_factor = 80 + SIN(animation_frame * 4) * 20
    
    # Marco de referencia (opcional)
    DRAW_LINE(-120, -120, 120, -120, 60)
    DRAW_LINE(120, -120, 120, 120, 60)
    DRAW_LINE(120, 120, -120, 120, 60)
    DRAW_LINE(-120, 120, -120, -120, 60)
    
    # FRACTAL TREE DINÁMICO - Implementación completa inline
    # Nivel 0 (tronco)
    let x1 = 0
    let y1 = -60
    let x2 = 0
    let y2 = y1 + base_length
    DRAW_LINE(x1, y1, x2, y2, intensity_)
    
    # Nivel 1 (ramas principales)
    let length1 = base_length * 70 / 100
    let angle1_left = 270 + angle_delta + angle_offset / 4
    let angle1_right = 270 - angle_delta - angle_offset / 4
    
    # Rama izquierda nivel 1
    let cos1_left = COS(angle1_left)
    let sin1_left = SIN(angle1_left)
    let x2_left = x2 + cos1_left * length1 / scale_factor
    let y2_left = y2 + sin1_left * length1 / scale_factor
    DRAW_LINE(x2, y2, x2_left, y2_left, intensity_ - 20)
    
    # Rama derecha nivel 1  
    let cos1_right = COS(angle1_right)
    let sin1_right = SIN(angle1_right)
    let x2_right = x2 + cos1_right * length1 / scale_factor
    let y2_right = y2 + sin1_right * length1 / scale_factor
    DRAW_LINE(x2, y2, x2_right, y2_right, intensity_ - 20)
    
    # Nivel 2 (sub-ramas)
    let length2 = length1 * 60 / 100
    
    # Sub-ramas de la rama izquierda
    let angle2_ll = angle1_left + angle_delta
    let angle2_lr = angle1_left - angle_delta
    
    let cos2_ll = COS(angle2_ll)
    let sin2_ll = SIN(angle2_ll)
    let x3_ll = x2_left + cos2_ll * length2 / scale_factor
    let y3_ll = y2_left + sin2_ll * length2 / scale_factor
    DRAW_LINE(x2_left, y2_left, x3_ll, y3_ll, intensity_ - 40)
    
    let cos2_lr = COS(angle2_lr)
    let sin2_lr = SIN(angle2_lr)
    let x3_lr = x2_left + cos2_lr * length2 / scale_factor
    let y3_lr = y2_left + sin2_lr * length2 / scale_factor
    DRAW_LINE(x2_left, y2_left, x3_lr, y3_lr, intensity_ - 40)
    
    # Sub-ramas de la rama derecha
    let angle2_rl = angle1_right + angle_delta
    let angle2_rr = angle1_right - angle_delta
    
    let cos2_rl = COS(angle2_rl)
    let sin2_rl = SIN(angle2_rl)
    let x3_rl = x2_right + cos2_rl * length2 / scale_factor
    let y3_rl = y2_right + sin2_rl * length2 / scale_factor
    DRAW_LINE(x2_right, y2_right, x3_rl, y3_rl, intensity_ - 40)
    
    let cos2_rr = COS(angle2_rr)
    let sin2_rr = SIN(angle2_rr)
    let x3_rr = x2_right + cos2_rr * length2 / scale_factor
    let y3_rr = y2_right + sin2_rr * length2 / scale_factor
    DRAW_LINE(x2_right, y2_right, x3_rr, y3_rr, intensity_ - 40)
    
    # Nivel 3 (ramas finales)
    let length3 = length2 * 50 / 100
    
    # Ramas del extremo izquierdo-izquierdo
    let angle3_lll = angle2_ll + angle_delta / 2
    let angle3_llr = angle2_ll - angle_delta / 2
    
    let cos3_lll = COS(angle3_lll)
    let sin3_lll = SIN(angle3_lll)
    let x4_lll = x3_ll + cos3_lll * length3 / scale_factor
    let y4_lll = y3_ll + sin3_lll * length3 / scale_factor
    DRAW_LINE(x3_ll, y3_ll, x4_lll, y4_lll, intensity_ - 60)
    
    let cos3_llr = COS(angle3_llr)
    let sin3_llr = SIN(angle3_llr)
    let x4_llr = x3_ll + cos3_llr * length3 / scale_factor
    let y4_llr = y3_ll + sin3_llr * length3 / scale_factor
    DRAW_LINE(x3_ll, y3_ll, x4_llr, y4_llr, intensity_ - 60)
    
    # Ramas del extremo izquierdo-derecho
    let angle3_lrl = angle2_lr + angle_delta / 2
    let angle3_lrr = angle2_lr - angle_delta / 2
    
    let cos3_lrl = COS(angle3_lrl)
    let sin3_lrl = SIN(angle3_lrl)
    let x4_lrl = x3_lr + cos3_lrl * length3 / scale_factor
    let y4_lrl = y3_lr + sin3_lrl * length3 / scale_factor
    DRAW_LINE(x3_lr, y3_lr, x4_lrl, y4_lrl, intensity_ - 60)
    
    let cos3_lrr = COS(angle3_lrr)
    let sin3_lrr = SIN(angle3_lrr)
    let x4_lrr = x3_lr + cos3_lrr * length3 / scale_factor
    let y4_lrr = y3_lr + sin3_lrr * length3 / scale_factor
    DRAW_LINE(x3_lr, y3_lr, x4_lrr, y4_lrr, intensity_ - 60)
    
    # Ramas del extremo derecho-izquierdo
    let angle3_rll = angle2_rl + angle_delta / 2
    let angle3_rlr = angle2_rl - angle_delta / 2
    
    let cos3_rll = COS(angle3_rll)
    let sin3_rll = SIN(angle3_rll)
    let x4_rll = x3_rl + cos3_rll * length3 / scale_factor
    let y4_rll = y3_rl + sin3_rll * length3 / scale_factor
    DRAW_LINE(x3_rl, y3_rl, x4_rll, y4_rll, intensity_ - 60)
    
    let cos3_rlr = COS(angle3_rlr)
    let sin3_rlr = SIN(angle3_rlr)
    let x4_rlr = x3_rl + cos3_rlr * length3 / scale_factor
    let y4_rlr = y3_rl + sin3_rlr * length3 / scale_factor
    DRAW_LINE(x3_rl, y3_rl, x4_rlr, y4_rlr, intensity_ - 60)
    
    # Ramas del extremo derecho-derecho
    let angle3_rrl = angle2_rr + angle_delta / 2
    let angle3_rrr = angle2_rr - angle_delta / 2
    
    let cos3_rrl = COS(angle3_rrl)
    let sin3_rrl = SIN(angle3_rrl)
    let x4_rrl = x3_rr + cos3_rrl * length3 / scale_factor
    let y4_rrl = y3_rr + sin3_rrl * length3 / scale_factor
    DRAW_LINE(x3_rr, y3_rr, x4_rrl, y4_rrl, intensity_ - 60)
    
    let cos3_rrr = COS(angle3_rrr)
    let sin3_rrr = SIN(angle3_rrr)
    let x4_rrr = x3_rr + cos3_rrr * length3 / scale_factor
    let y4_rrr = y3_rr + sin3_rrr * length3 / scale_factor
    DRAW_LINE(x3_rr, y3_rr, x4_rrr, y4_rrr, intensity_ - 60)
