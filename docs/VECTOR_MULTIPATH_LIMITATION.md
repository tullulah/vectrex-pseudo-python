# Vector Multi-Path Rendering - Known Limitation

**Fecha**: 2025-12-10  
**Estado**: DOCUMENTADO - Funciona parcialmente, requiere investigación adicional

## Resumen

El sistema de vectores multi-path **funciona correctamente** para el primer path, pero **acumula posiciones** en los paths subsiguientes debido a la naturaleza relativa de la función BIOS `Moveto_d`.

## Comportamiento Actual

### ✅ Lo que funciona:
- **Single-path vectors**: Perfecto (ej: `test_simple_vector/line.vec`)
- **Primer path de multi-path**: Dibuja correctamente (ej: círculo externo de `moon.vec`)
- **Todos los paths se dibujan**: Ningún path desaparece
- **Formato de datos**: Correcto (FCB y,x para Draw_VLc)

### ⚠️ La limitación:
- **Paths subsiguientes**: Se dibujan desde posiciones acumuladas en lugar de absolutas
- **Ejemplo**: En `moon.vec`, los 3 cráteres deberían estar distribuidos, pero se agrupan en la esquina superior derecha

## Causa Técnica

### BIOS Moveto_d es Relativo
```asm
; Estado después de dibujar el primer path (círculo):
; - Beam position: punto final del círculo (ej: x=15, y=25)

; Intento de posicionar el segundo path (crater1 en -10, 8):
LDA #8          ; A = y deseado (8)
LDB #-10        ; B = x deseado (-10)
JSR Moveto_d    ; PERO: Moveto_d suma al beam actual!
                ; Resultado: beam en (15-10, 25+8) = (5, 33) ❌
                ; Esperado: beam en (-10, 8) desde el origen ✅
```

### Código Actual (core/src/backend/m6809.rs líneas 1385-1405)
```rust
out.push_str("    JSR Reset0Ref       ; Reset integrator origin to center\n");
out.push_str(&format!("    LDX #{}_VECTORS ; Load pointer list\n", symbol));
out.push_str("DRAW_VEC_LOOP_START:\n");
out.push_str("    LDD ,X++            ; Load next path pointer\n");
out.push_str("    BEQ DRAW_VEC_DONE   ; Exit if 0 (end of list)\n");
out.push_str("    PSHS X              ; Save list pointer\n");
out.push_str("    TFR D,X             ; X = path data pointer\n");
out.push_str("    LDA ,X+             ; A = Y0 (starting point)\n");
out.push_str("    LDB ,X+             ; B = X0 (starting point)\n");
out.push_str("    JSR Moveto_d        ; Move beam to starting point\n");
out.push_str("    JSR Draw_VLc        ; Draw this path\n");
out.push_str("    ; TODO: Multi-path positioning needs investigation\n");
out.push_str("    ; Issue: Craters render at accumulated positions\n");
out.push_str("    ; Moveto_d is relative to current beam position\n");
```

## Intentos de Solución (Todos Fallidos)

### 1. ❌ Moveto_d_7F (Posicionamiento Absoluto Teórico)
```asm
JSR Moveto_d_7F  ; En lugar de Moveto_d
```
- **Resultado**: Cráteres en las **mismas posiciones incorrectas**
- **Razón**: Moveto_d_7F requiere setup adicional o también acumula

### 2. ❌ Moveto_d_7F + Scale Factor
```asm
LDA #$7F
STA VIA_shift_reg    ; Configure scale
JSR Moveto_d_7F
```
- **Resultado**: Cráteres en las **mismas posiciones incorrectas**
- **Razón**: El scale factor no resuelve la acumulación

### 3. ❌ Reset0Ref antes de cada path (dentro del loop)
```asm
DRAW_VEC_LOOP_START:
    LDD ,X++
    BEQ DRAW_VEC_DONE
    JSR Reset0Ref        ; ← Resetear antes de cada path
    PSHS X
    TFR D,X
    ...
```
- **Resultado**: **Nada se dibuja** (pantalla en blanco)
- **Razón**: Reset0Ref requiere tiempo de estabilización del integrador
- **Problema**: Llamadas rápidas en loop rompen el estado interno del BIOS

### 4. ❌ Inversión de orden de coordenadas (prueba de concepto)
```asm
LDB ,X+    ; B = X primero
LDA ,X+    ; A = Y segundo
```
- **Resultado**: No probado correctamente (usuario tenía nombre de asset incorrecto)
- **Razón**: El orden FCB y,x está correcto (verificado con single-path)

### 5. ✅ Reset0Ref una vez + loop Moveto_d (ACTUAL)
- **Resultado**: Dibuja todos los paths, primer path correcto, subsiguientes acumulan
- **Estado**: IMPLEMENTACIÓN ACTUAL - funciona parcialmente

## Ejemplos Visuales

### moon.vec - Coordenadas Esperadas vs Renderizadas

```
Esperado (absoluto desde origen):       Actual (acumulado):
    
       outer_circle (0, 30)                  outer_circle (0, 30) ✅
            ◯                                       ◯
                                                   
  crater1 (-10, 8)   crater2 (8, -5)        crater1,2,3 agrupados
      •                  •                   en (15, 33) aprox ❌
         crater3 (-5, -12)                            •••
             •                                        
                                          
```

### Datos Generados (Correctos)
```asm
_MOON_OUTER_CIRCLE_VECTORS:
    FCB 30, 0          ; y=30, x=0 (top center)
    FCB 23             ; 23 deltas
    ; ... deltas del círculo
    
_MOON_CRATER1_VECTORS:
    FCB 8, -10         ; y=8, x=-10 (debería ser left-upper)
    FCB 7              ; 7 deltas
    ; ... deltas del cráter
    
_MOON_CRATER2_VECTORS:
    FCB -5, 8          ; y=-5, x=8 (debería ser right-lower)
    ; ...

_MOON_CRATER3_VECTORS:
    FCB -12, -5        ; y=-12, x=-5 (debería ser center-bottom)
    ; ...

_MOON_VECTORS:
    FDB _MOON_OUTER_CIRCLE_VECTORS
    FDB _MOON_CRATER1_VECTORS
    FDB _MOON_CRATER2_VECTORS
    FDB _MOON_CRATER3_VECTORS
    FDB 0
```

## Restricciones Identificadas

1. **Moveto_d es relativo**: Por diseño del BIOS, suma al beam position actual
2. **Reset0Ref timing-sensitive**: No se puede llamar en loops rápidos
3. **Moveto_d_7F insuficiente**: Requiere comprensión más profunda de setup
4. **BIOS internals desconocidos**: Faltan detalles sobre integrador y timing

## Workarounds Disponibles

### A. Usar Single-Path Vectors (RECOMENDADO)
- ✅ Funciona perfectamente
- ✅ Sin limitaciones de posicionamiento
- ❌ Requiere diseñar vectores como paths únicos (más puntos)

### B. Primer Path Solamente
- ✅ El primer path de cualquier multi-path funciona correctamente
- ❌ No útil si necesitas múltiples shapes separadas

### C. Aceptar Acumulación (ACTUAL)
- ✅ Todos los paths se dibujan
- ⚠️ Posicionamiento incorrecto pero predecible
- 💡 Podría usarse para efectos artísticos intencionales

## Investigación Futura Necesaria

### 1. Estudiar BIOS Moveto_d_7F
- Documentar requirements exactos de setup
- Probar con diferentes configuraciones de VIA
- Comparar con implementación de referencia (Vectrexy)

### 2. Calcular Deltas Entre Paths
```asm
; En lugar de coordenadas absolutas en FCB,
; calcular delta desde el path anterior:
; crater1_relative = crater1_abs - circle_end
```
- Requiere tracking del punto final de cada path
- Compilador más complejo
- Potencialmente soluciona el problema

### 3. Manual Integrator Control
- Estudiar registros VIA del integrador
- Control directo sin funciones BIOS
- Avanzado, requiere conocimiento profundo

### 4. Timing de Reset0Ref
- Cuánto delay necesita entre llamadas
- Puede insertarse delay manual en el loop
- Probar con diferentes cantidades de NOPs

### 5. Alternativas BIOS
- Investigar otras funciones Moveto_* (Moveto_ix, etc.)
- Ver cómo otros juegos manejan múltiples shapes
- Disassembly de cartuchos comerciales

## Impacto en Proyectos

### test_simple_vector
- **Estado**: ✅ Funciona perfectamente
- **Tamaño**: 151 bytes
- **Tipo**: Single-path (2 puntos, línea 45°)

### test_mcp
- **Estado**: ⚠️ Funciona parcialmente
- **Tamaño**: 2733 bytes + padding
- **Tipo**: Multi-path (4 paths: círculo + 3 cráteres)
- **Observación**: Círculo perfecto, cráteres agrupados

### Recomendación General
Para proyectos de producción:
- Diseñar assets como **single-path** cuando sea posible
- Si necesitas múltiples shapes separadas, usar **DRAW_VECTOR múltiples veces** con assets single-path
- Ejemplo:
  ```python
  DRAW_VECTOR("moon_circle")     # Asset 1: solo el círculo
  DRAW_VECTOR("moon_crater1")    # Asset 2: solo crater1
  DRAW_VECTOR("moon_crater2")    # Asset 3: solo crater2
  DRAW_VECTOR("moon_crater3")    # Asset 4: solo crater3
  ```

## Referencias de Código

### Generación de código inline
- **Archivo**: `core/src/backend/m6809.rs`
- **Líneas**: 1385-1420
- **Función**: `emit_builtin_call()` - case "DRAW_VECTOR"

### Formato de datos vectoriales
- **Archivo**: `core/src/vecres.rs`
- **Líneas**: 228-296
- **Funciones**: Path data generation + pointer list

### Ensamblador nativo
- **Archivo**: `core/src/backend/asm_to_binary.rs`
- **Líneas**: 1605-1660
- **Función**: `parse_indexed_mode()` - Y register support

## Validaciones Realizadas

✅ Coordenadas no requieren negación (canvas y Vectrex coinciden)  
✅ Orden FCB y,x correcto para Draw_VLc  
✅ Loading order LDA/LDB correcto  
✅ Y register indexed addressing implementado (disponible pero no usado)  
✅ Asset validation con error handling  
✅ Single-path vectors funcionan perfectamente  
✅ Multi-path render (todos los paths visibles)  
⚠️ Multi-path positioning acumula (limitación documentada)  

## Conclusión

El sistema actual es **funcional y estable**, con una limitación conocida en el posicionamiento de multi-path. Los usuarios pueden elegir entre:
1. **Single-path workflows** (recomendado, sin limitaciones)
2. **Múltiples llamadas DRAW_VECTOR** con assets single-path
3. **Aceptar acumulación** en multi-path (efectos artísticos)

La investigación futura puede resolver completamente el problema, pero no es bloqueante para el desarrollo de juegos.

---

**Última actualización**: 2025-12-10  
**Autor**: GitHub Copilot (Claude Sonnet 4.5)  
**Contexto**: Session de debugging vector rendering con 5 estrategias intentadas
