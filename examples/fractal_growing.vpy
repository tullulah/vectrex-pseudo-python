# FRACTAL TREE DEMO - VPy Language (Crecimiento Gradual)
# Árbol fractal que crece cada X ciclos

# Variables globales para control del fractal
var tree_depth = 0  # Empezará en 0 y crecerá
var base_length = 10  # Empezará pequeño
var angle_delta = 30
var animation_frame = 0
var intensity_ = 120

# Variables para animación y crecimiento
var scale_factor = 100
var angle_offset = 0
var growth_cycle = 0
var max_depth = 3
var growth_speed = 60  # Cada 60 frames = 1 segundo aprox

# Estado de crecimiento
var current_depth = 0
var current_length = 10
var is_growing = 1

# Variables de trabajo (todas globales)
var x1 = 0
var y1 = 0
var x2 = 0
var y2 = 0
var length1 = 0
var length2 = 0
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
    current_depth = 0
    current_length = 10
    max_depth = 3
    growth_speed = 60
    intensity_ = 120
    is_growing = 1

def loop():
    # Contador de animación
    animation_frame = animation_frame + 1
    if animation_frame >= 1200:  # Ciclo más corto para ver mejor
        animation_frame = 0
    
    # === LÓGICA DE CRECIMIENTO SIMPLIFICADA ===
    # Cada 100 frames = cambio de nivel (más lento y visible)
    growth_phase = animation_frame / 100
    
    # Determinar nivel actual basado en el ciclo
    if growth_phase < 1:
        current_depth = 0  # Solo tronco
    elif growth_phase < 2:
        current_depth = 1  # Tronco + nivel 1
    elif growth_phase < 3:
        current_depth = 2  # Tronco + nivel 1 + nivel 2
    elif growth_phase < 4:
        current_depth = 3  # Árbol completo
    elif growth_phase < 5:
        current_depth = 2  # Vuelve a nivel 2
    elif growth_phase < 6:
        current_depth = 1  # Vuelve a nivel 1
    else:
        current_depth = 0  # Vuelve a solo tronco
    
    # Ajustar longitud del tronco según el nivel
    current_length = 20 + current_depth * 10
    
    # Variación dinámica del ángulo y escala (animación fluida)
    angle_offset = animation_frame * 2
    scale_factor = 80 + SIN(animation_frame * 3) * 20
    
    # Marco de referencia + indicador de nivel actual
    DRAW_LINE(-100, -100, 100, -100, 40)
    DRAW_LINE(100, -100, 100, 100, 40)
    DRAW_LINE(100, 100, -100, 100, 40)
    DRAW_LINE(-100, 100, -100, -100, 40)
    
    # Indicador visual del nivel actual (líneas en la esquina)
    if current_depth >= 1:
        DRAW_LINE(-90, -90, -80, -90, 120)  # Línea nivel 1
    if current_depth >= 2:
        DRAW_LINE(-90, -85, -80, -85, 120)  # Línea nivel 2
    if current_depth >= 3:
        DRAW_LINE(-90, -80, -80, -80, 120)  # Línea nivel 3
    
    # === FRACTAL TREE CON CRECIMIENTO GRADUAL ===
    
    # NIVEL 0 (tronco principal) - Siempre se dibuja
    x1 = 0
    y1 = -60
    x2 = 0
    y2 = y1 + current_length
    DRAW_LINE(x1, y1, x2, y2, intensity_)
    
    # NIVEL 1 (ramas principales) - Solo si current_depth >= 1
    if current_depth >= 1:
        length1 = current_length * 70 / 100
        angle1_left = 270 + angle_delta + angle_offset / 4
        angle1_right = 270 - angle_delta - angle_offset / 4
        
    #     # Rama izquierda nivel 1
    #     cos1_left = COS(angle1_left)
    #     sin1_left = SIN(angle1_left)
    #     x2_left = x2 + cos1_left * length1 / scale_factor
    #     y2_left = y2 + sin1_left * length1 / scale_factor
    #     DRAW_LINE(x2, y2, x2_left, y2_left, intensity_ - 30)
        
    #     # Rama derecha nivel 1  
    #     cos1_right = COS(angle1_right)
    #     sin1_right = SIN(angle1_right)
    #     x2_right = x2 + cos1_right * length1 / scale_factor
    #     y2_right = y2 + sin1_right * length1 / scale_factor
    #     DRAW_LINE(x2, y2, x2_right, y2_right, intensity_ - 30)
        
    #     # NIVEL 2 (sub-ramas) - Solo si current_depth >= 2
    #     if current_depth >= 2:
    #         length2 = length1 * 65 / 100
            
    #         # Sub-ramas de la rama izquierda
    #         angle2_ll = angle1_left + angle_delta
    #         angle2_lr = angle1_left - angle_delta
            
    #         cos2_ll = COS(angle2_ll)
    #         sin2_ll = SIN(angle2_ll)
    #         x3_ll = x2_left + cos2_ll * length2 / scale_factor
    #         y3_ll = y2_left + sin2_ll * length2 / scale_factor
    #         DRAW_LINE(x2_left, y2_left, x3_ll, y3_ll, intensity_ - 50)
            
    #         cos2_lr = COS(angle2_lr)
    #         sin2_lr = SIN(angle2_lr)
    #         x3_lr = x2_left + cos2_lr * length2 / scale_factor
    #         y3_lr = y2_left + sin2_lr * length2 / scale_factor
    #         DRAW_LINE(x2_left, y2_left, x3_lr, y3_lr, intensity_ - 50)
            
    #         # Sub-ramas de la rama derecha
    #         angle2_rl = angle1_right + angle_delta
    #         angle2_rr = angle1_right - angle_delta
            
    #         cos2_rl = COS(angle2_rl)
    #         sin2_rl = SIN(angle2_rl)
    #         x3_rl = x2_right + cos2_rl * length2 / scale_factor
    #         y3_rl = y2_right + sin2_rl * length2 / scale_factor
    #         DRAW_LINE(x2_right, y2_right, x3_rl, y3_rl, intensity_ - 50)
            
    #         cos2_rr = COS(angle2_rr)
    #         sin2_rr = SIN(angle2_rr)
    #         x3_rr = x2_right + cos2_rr * length2 / scale_factor
    #         y3_rr = y2_right + sin2_rr * length2 / scale_factor
    #         DRAW_LINE(x2_right, y2_right, x3_rr, y3_rr, intensity_ - 50)
            
    #         # NIVEL 3 - Solo si current_depth >= 3 (opcional, para más detalle)
    #         if current_depth >= 3:
    #             # Pequeñas ramas adicionales en los extremos
    #             DRAW_LINE(x3_ll, y3_ll, x3_ll - 5, y3_ll + 8, intensity_ - 70)
    #             DRAW_LINE(x3_lr, y3_lr, x3_lr + 5, y3_lr + 8, intensity_ - 70)
    #             DRAW_LINE(x3_rl, y3_rl, x3_rl - 5, y3_rl + 8, intensity_ - 70)
    #             DRAW_LINE(x3_rr, y3_rr, x3_rr + 5, y3_rr + 8, intensity_ - 70)