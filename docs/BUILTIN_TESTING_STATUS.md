# Builtin Testing Status

Estado del testing incremental de builtins VPy.

## ✅ WORKING - Funcionales

### DRAW_LINE(x0, y0, x1, y1, intensity)
- **Estado**: ✅ FUNCIONA
- **Probado**: 2026-01-18
- **Notas**: 
  - Fixed memory overlap bug (TMPPTR → DRAW_LINE_ARGS)
  - Horizontal, vertical, diagonal lines funcionan correctamente
  - Segmentación automática para líneas > 127px

### DRAW_RECT(x, y, w, h, intensity)
- **Estado**: ✅ FUNCIONA
- **Probado**: 2026-01-18
- **Notas**: 
  - Dibuja rectángulos correctamente
  - Usa DRAW_LINE internamente

### PRINT_TEXT(x, y, string)
- **Estado**: ✅ FUNCIONA (con issue menor)
- **Probado**: 2026-01-18
- **Notas**: 
  - Configura VIA_cntl y llama a Reset_Pen
  - Issue conocido: Escala/posición afecta siguiente DRAW_LINE
  - No bloqueante, funcional para uso general

## ⚠️ BROKEN - No Funcionan

### PRINT_NUMBER(x, y, num)
- **Estado**: ❌ NO FUNCIONA
- **Probado**: 2026-01-18
- **Síntoma**: No dibuja números, solo 3 rayitas, pantalla parpadea
- **Prioridad**: BAJA (no crítico por ahora)
- **TODO**: Investigar conversión hex y rendering

## 🔄 PENDING - No Probados

### Input
- J1_X() - Lectura joystick X
- J1_Y() - Lectura joystick Y
- J1_BUTTON_1() - Lectura botón 1
- J1_BUTTON_2() - Lectura botón 2
- J1_BUTTON_3() - Lectura botón 3
- J1_BUTTON_4() - Lectura botón 4

### DRAW_VECTOR(name)
- **Estado**: ✅ FUNCIONA
- **Probado**: 2026-01-18
- **Notas**: "logo" se dibuja correctamente

### DRAW_VECTOR_EX(name, x, y, mirror, intensity)
- **Estado**: ✅ FUNCIONA
- **Probado**: 2026-01-18
- **Notas**: Espejos X/Y funcionan correctamente. RAM segura.

### DRAW_CIRCLE(x, y, r, intensity)
- **Estado**: ✅ FUNCIONA
- **Probado**: 2026-01-18
- **Notas**: Dibuja correctamente sin corromper memoria.

### Audio
- PLAY_MUSIC(name) - 🔄 TESTING
- PLAY_SFX(name)
- STOP_MUSIC()

### Math
- ABS(x) - ✅ FUNCIONA
- MIN(a, b) - ✅ FUNCIONA
- MAX(a, b) - ✅ FUNCIONA

### Utilities
- RESET0REF()
- WAIT_RECAL()

---
Última actualización: 2026-01-18
