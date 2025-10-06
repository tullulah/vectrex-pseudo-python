# 🎯 ANÁLISIS FINAL: Comparación Rust vs JSVecx

## 📊 Datos Calculados de los Snapshots

### Rust Emulator (868 vectores - DUPLICADOS)
```
X Range: -73.00 to +75.00  (148 unidades de ancho)
Y Range: -108.00 to +110.00 (218 unidades de alto)
Center: (1.00, 1.00)
```

### JSVecx (388 vectores - Normalizado a DAC)
```
X Range: -84.40 to +86.36  (170.76 unidades de ancho)
Y Range: -101.10 to +99.52 (200.62 unidades de alto)
Center: (0.98, -0.79)
```

## 🔍 Comparación de Centros

| Emulador | Center X | Center Y | Offset del Origen |
|----------|----------|----------|-------------------|
| **Rust** | **1.00** | **1.00** | ~1 unidad desplazado |
| **JSVecx** | **0.98** | **-0.79** | Casi centrado en origen |

**Diferencia de Centros**:
- **Δ X = 1.00 - 0.98 = 0.02** (prácticamente idéntico!)
- **Δ Y = 1.00 - (-0.79) = 1.79** (Rust ligeramente más arriba)

## 🚨 HALLAZGO CRÍTICO: Los Centros Son CASI IDÉNTICOS

**Conclusión Sorprendente**:
- El centro X de ambos emuladores está a ~1 unidad del origen (0, 0)
- **NO hay offset de -10.75 vs -4.65** en los datos raw de vectores
- El offset visual que observamos **NO está en los vectores**, está en el **RENDERING**

## 🎨 ¿Dónde Está el Offset Entonces?

### Teoría 1: Transformación de Renderizado
El offset -10.75 vs -4.65 está en la **conversión de coordenadas a píxeles**:

**Rust (test_wasm.html)**:
```javascript
const VECTREX_RANGE = 256;
const scale = canvas.width / VECTREX_RANGE;  // 600 / 256 = 2.34375
const centerX = canvas.width / 2;             // 300

// Conversión a píxeles:
const x0 = centerX + ((vec.x0 + offsetX) * scale);
```

**JSVecx (osint_render)**:
```javascript
// JSVecx usa transform de canvas diferente
ctx.setTransform(1, 0, 0, 1, canvas.width / 2, canvas.height / 2);
```

### Teoría 2: Diferencia en Coordenadas DAC vs Integrador

**Rust**: Usa coordenadas DAC directas (-127 a +127)
- Center = 1.00 → ligeramente desplazado del cero real

**JSVecx**: Normaliza desde integrador (0-33000) a DAC
- Center = 0.98 → casi perfectamente centrado

**Posible causa del offset visual**:
- La conversión de JSVecx introduce un offset de ~16500/33000 = 0.5 → multiplicado por escala = offset visual
- Rust usa DAC directo sin normalización adicional

## 📏 Análisis de Rangos

### Ancho (X):
- **Rust**: 148 unidades
- **JSVecx**: 170.76 unidades
- **Ratio**: JSVecx es 1.15x más ancho (15% más grande)

### Alto (Y):
- **Rust**: 218 unidades
- **JSVecx**: 200.62 unidades
- **Ratio**: Rust es 1.09x más alto (9% más grande)

**Observación**: Los rangos son ligeramente diferentes, pero no explican el offset de -10.75 vs -4.65.

## 🐛 Problema de Duplicación de Vectores

### Evidencia Concreta:
```
Rust: 868 vectores
JSVecx: 388 vectores
Ratio: 2.24x
```

**Patrón de Duplicación**:
- Los vectores 0-89 se repiten como vectores 90-179
- Luego vectores 180-269, etc.
- Cada bloque de ~90 vectores se duplica

### Causa Probable:

**Hipótesis A**: `renderVectors()` en JavaScript se llama MÚLTIPLES VECES
```javascript
// En test_wasm.html - función loop()
function loop() {
    if (emulator.isRunning()) {
        emulator.runFrame(CYCLES_PER_FRAME);
        updateMetrics();
        updateRegisters();
        renderVectors();  // ← Se llama aquí
        animationId = requestAnimationFrame(loop);
    }
}

// Y también en btnRunFrame:
document.getElementById('btnRunFrame').addEventListener('click', () => {
    emulator.runFrame(CYCLES_PER_FRAME);
    renderVectors();  // ← Y aquí
});
```

**Hipótesis B**: Los vectores NO se limpian entre frames
- El buffer de vectores en WASM acumula en lugar de reemplazar
- Cada frame AÑADE vectores en lugar de REEMPLAZAR

**Hipótesis C**: La BIOS realmente está dibujando dos veces
- Poco probable (JSVecx solo tiene 388 vectores)
- Pero posible si hay diferencia en timing de frames

## 🎯 Conclusión Principal

### ✅ LO QUE DESCUBRIMOS:

1. **Los centros son casi idénticos** (Δ < 2 unidades)
   - Rust: (1.00, 1.00)
   - JSVecx: (0.98, -0.79)

2. **El offset -10.75 vs -4.65 NO está en los vectores**
   - Está en el RENDERING (conversión a píxeles)
   - Posiblemente en la escala o transform del canvas

3. **Rust genera 2.24x vectores** (duplicación)
   - 868 vs 388 vectores
   - Bug crítico que debe investigarse

4. **Los rangos son similares pero no idénticos**
   - JSVecx ligeramente más ancho
   - Rust ligeramente más alto

### ❌ LO QUE NO ES EL PROBLEMA:

- ❌ NO es un problema de coordenadas DAC raw
- ❌ NO es offset en los datos de los vectores
- ❌ NO es diferencia en cálculo de integrador (los centros coinciden)

### 🔧 SIGUIENTE PASO:

1. **ARREGLAR DUPLICACIÓN DE VECTORES**
   - Investigar por qué Rust genera 868 en lugar de ~388
   - Verificar que `render_context.clear()` funcione
   - Asegurar que vectores no se acumulen

2. **INVESTIGAR TRANSFORMACIÓN DE RENDERING**
   - Comparar `centerX + (vec.x0 * scale)` entre ambos
   - Verificar si JSVecx aplica un offset adicional en `osint_render()`
   - Medir offset visual DESPUÉS de arreglar duplicación

3. **VALIDAR ESCALA Y CENTRO**
   - Verificar que `VECTREX_RANGE = 256` sea correcto
   - Comparar con JSVecx `ALG_MAX_X = 33000`
   - Calcular ratio real: 33000 / 256 = 128.9 (¿debería ser 127?)

---

**Hallazgo Clave**: El offset NO está en los vectores, está en el rendering. Los centros son casi idénticos (~1 unidad de diferencia), pero el offset visual es de ~6 unidades. La duplicación de vectores es un bug separado que debe arreglarse primero.

**Acción Inmediata**: Investigar `renderVectors()` y el sistema de buffering de vectores en WASM.
