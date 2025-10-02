# FRACTAL TREE MEJORADO - VPy Language 2025
# Usando NUEVOS OPERADORES: +=, -=, *=, /=, %=, //, //=
# Optimizado con operadores de asignación compuesta

# Variables globales para animación y configuración
var frame_counter = 0
var tree_depth = 5
var base_length = 60
var angle_spread = 30
var brightness = 127  # Evitamos 'intensity' que es reservada

def main():
    # Inicialización
    frame_counter = 0
    tree_depth = 5
    base_length = 60
    angle_spread = 30
    brightness = 127

def loop():
    # === ANIMACIÓN USANDO OPERADORES COMPUESTOS ===
    frame_counter += 1  # ¡Nuevo operador +=!
    
    # Resetear contador usando módulo compuesto
    frame_counter %= 360  # ¡Nuevo operador %=!
    
    # === PARÁMETROS DINÁMICOS CON DIVISIÓN ENTERA ===
    let angle_variation = frame_counter // 6  # ¡Nueva división entera //!
    let scale_factor = 100  # Factor de escala fijo
    let pulse_effect = SIN(frame_counter * 4) // 10  # Efecto sutil de pulsación
    
    # === MARCO DE REFERENCIA ===
    let border_bright = brightness // 2  # División entera para intensidad del marco
    DRAW_LINE(-100, -100, 100, -100, border_bright)
    DRAW_LINE(100, -100, 100, 100, border_bright)
    DRAW_LINE(100, 100, -100, 100, border_bright)
    DRAW_LINE(-100, 100, -100, -100, border_bright)
    
    # === FRACTAL TREE CON NUEVOS OPERADORES ===
    
    # TRONCO PRINCIPAL
    let trunk_x1 = 0
    let trunk_y1 = -70
    let trunk_x2 = 0
    let trunk_y2 = trunk_y1 + base_length
    DRAW_LINE(trunk_x1, trunk_y1, trunk_x2, trunk_y2, brightness)
    
    # === NIVEL 1: RAMAS PRINCIPALES ===
    let branch_length = base_length * 3  # Usaremos /= para calcular
    branch_length //= 4  # ¡División entera compuesta! 75% del tamaño base
    
    # Ángulos dinámicos con variación
    let left_angle = 270 + angle_spread + angle_variation
    let right_angle = 270 - angle_spread - angle_variation
    
    # Rama izquierda nivel 1
    let left_cos = COS(left_angle)
    let left_sin = SIN(left_angle)
    let left_x = trunk_x2 + left_cos * branch_length / scale_factor
    let left_y = trunk_y2 + left_sin * branch_length / scale_factor
    
    let branch_bright = brightness
    branch_bright -= 25  # ¡Operador -= para reducir brillo!
    DRAW_LINE(trunk_x2, trunk_y2, left_x, left_y, branch_bright)
    
    # Rama derecha nivel 1
    let right_cos = COS(right_angle)
    let right_sin = SIN(right_angle)
    let right_x = trunk_x2 + right_cos * branch_length / scale_factor
    let right_y = trunk_y2 + right_sin * branch_length / scale_factor
    DRAW_LINE(trunk_x2, trunk_y2, right_x, right_y, branch_bright)
    
    # === NIVEL 2: SUB-RAMAS ===
    let sub_length = branch_length
    sub_length *= 3   # ¡Operador *=!
    sub_length //= 5  # ¡División entera compuesta! 60% del nivel anterior
    
    # Sub-ramas izquierdas
    let ll_angle = left_angle + angle_spread
    let lr_angle = left_angle - angle_spread
    
    let ll_cos = COS(ll_angle)
    let ll_sin = SIN(ll_angle)
    let ll_x = left_x + ll_cos * sub_length / scale_factor
    let ll_y = left_y + ll_sin * sub_length / scale_factor
    
    let sub_bright = branch_bright
    sub_bright -= 20  # ¡Operador -= otra vez!
    DRAW_LINE(left_x, left_y, ll_x, ll_y, sub_bright)
    
    let lr_cos = COS(lr_angle)
    let lr_sin = SIN(lr_angle)
    let lr_x = left_x + lr_cos * sub_length / scale_factor
    let lr_y = left_y + lr_sin * sub_length / scale_factor
    DRAW_LINE(left_x, left_y, lr_x, lr_y, sub_bright)
    
    # Sub-ramas derechas
    let rl_angle = right_angle + angle_spread
    let rr_angle = right_angle - angle_spread
    
    let rl_cos = COS(rl_angle)
    let rl_sin = SIN(rl_angle)
    let rl_x = right_x + rl_cos * sub_length / scale_factor
    let rl_y = right_y + rl_sin * sub_length / scale_factor
    DRAW_LINE(right_x, right_y, rl_x, rl_y, sub_bright)
    
    let rr_cos = COS(rr_angle)
    let rr_sin = SIN(rr_angle)
    let rr_x = right_x + rr_cos * sub_length / scale_factor
    let rr_y = right_y + rr_sin * sub_length / scale_factor
    DRAW_LINE(right_x, right_y, rr_x, rr_y, sub_bright)
    
    # === NIVEL 3: RAMAS FINALES ===
    let final_length = sub_length
    final_length //= 2  # ¡División entera! 50% del nivel anterior
    
    # Solo algunas ramas finales para no sobrecargar
    let final_bright = sub_bright
    final_bright -= 15  # ¡Último uso de -=!
    
    # Finales de rama izquierda-izquierda
    let lll_angle = ll_angle + angle_spread // 2
    let llr_angle = ll_angle - angle_spread // 2
    
    let lll_cos = COS(lll_angle)
    let lll_sin = SIN(lll_angle)
    let lll_x = ll_x + lll_cos * final_length / scale_factor
    let lll_y = ll_y + lll_sin * final_length / scale_factor
    DRAW_LINE(ll_x, ll_y, lll_x, lll_y, final_bright)
    
    let llr_cos = COS(llr_angle)
    let llr_sin = SIN(llr_angle)
    let llr_x = ll_x + llr_cos * final_length / scale_factor
    let llr_y = ll_y + llr_sin * final_length / scale_factor
    DRAW_LINE(ll_x, ll_y, llr_x, llr_y, final_bright)
    
    # Finales de rama derecha-derecha
    let rrl_angle = rr_angle + angle_spread // 2
    let rrr_angle = rr_angle - angle_spread // 2
    
    let rrl_cos = COS(rrl_angle)
    let rrl_sin = SIN(rrl_angle)
    let rrl_x = rr_x + rrl_cos * final_length / scale_factor
    let rrl_y = rr_y + rrl_sin * final_length / scale_factor
    DRAW_LINE(rr_x, rr_y, rrl_x, rrl_y, final_bright)
    
    let rrr_cos = COS(rrr_angle)
    let rrr_sin = SIN(rrr_angle)
    let rrr_x = rr_x + rrr_cos * final_length / scale_factor
    let rrr_y = rr_y + rrr_sin * final_length / scale_factor
    DRAW_LINE(rr_x, rr_y, rrr_x, rrr_y, final_bright)
    
    # === EFECTOS ADICIONALES CON NUEVOS OPERADORES ===
    
    # Hojas/frutos en las puntas (usando frame para animación)
    let leaf_frame = frame_counter
    leaf_frame //= 10  # División entera para ralentizar
    leaf_frame %= 4    # Módulo para ciclar entre 0-3
    
    # Solo dibujar hojas cada 4 frames para efecto parpadeante
    if leaf_frame == 0:
        let leaf_bright = final_bright // 2
        # Pequeños círculos como hojas
        DRAW_LINE(lll_x-2, lll_y, lll_x+2, lll_y, leaf_bright)
        DRAW_LINE(llr_x-2, llr_y, llr_x+2, llr_y, leaf_bright)
        DRAW_LINE(rrl_x-2, rrl_y, rrl_x+2, rrl_y, leaf_bright)
        DRAW_LINE(rrr_x-2, rrr_y, rrr_x+2, rrr_y, leaf_bright)