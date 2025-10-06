# Vector Snapshot Feature - Comparación de Emuladores

## 📸 Funcionalidad Añadida

Se ha implementado un sistema de snapshot de vectores en ambos HTML de prueba para facilitar la comparación directa entre el emulador Rust y JSVecx.

## 🎯 Objetivo

Permitir la captura y comparación de los vectores que cada emulador genera en el mismo punto de la emulación, para identificar diferencias en las coordenadas que causan el offset visual.

## 📋 Cambios Implementados

### 1. test_wasm.html (Rust Emulator)

**Botón de Snapshot:**
- Ubicación: Sección "4. Vector Output", antes de los controles de offset
- Texto: `📸 Snapshot Vectors`
- Estado: Habilitado después de inicializar el emulador

**Panel de Snapshot:**
- Nueva sección "6. Vector Snapshot (Rust Emulator)"
- Se muestra automáticamente al capturar vectores
- Tabla con columnas:
  - `#`: Índice del vector
  - `X0, Y0`: Coordenadas de inicio
  - `X1, Y1`: Coordenadas de fin
  - `Color`: Intensidad del vector
  - `Length`: Longitud calculada del vector

**Funcionalidad:**
- Captura todos los vectores actualmente en el buffer del emulador
- Muestra coordenadas en el rango nativo del emulador Rust (aproximadamente -127 a +127)
- Botón "Clear Snapshot" para ocultar el panel

### 2. test_jsvecx.html (JSVecx)

**Botón de Snapshot:**
- Ubicación: Junto a los botones de control (Run, Pause, Reset, Analyze)
- Texto: `📸 Snapshot`

**Panel de Snapshot:**
- Nueva sección "Vector Snapshot (JSVecx - Normalized to -127..+127)"
- Se muestra automáticamente al capturar vectores
- Tabla con columnas:
  - `#`: Índice del vector
  - `X0 (DAC), Y0 (DAC)`: Coordenadas normalizadas de inicio
  - `X1 (DAC), Y1 (DAC)`: Coordenadas normalizadas de fin
  - `Color`: Intensidad del vector
  - `Length`: Longitud calculada del vector
  - `Raw X0, Raw Y0`: Coordenadas originales de JSVecx (para referencia)

**Funcionalidad:**
- Captura los vectores del último frame renderizado
- **NORMALIZACIÓN CRÍTICA**: Convierte las coordenadas internas de JSVecx al rango DAC (-127 a +127)
  - JSVecx usa internamente: X=0-33000, Y=0-41000
  - Conversión: `dac = ((raw - center) / center) * 127`
  - Esto permite comparación directa con el emulador Rust
- Muestra coordenadas raw en columnas adicionales (color gris, tamaño pequeño)
- Botón "Clear Snapshot" para ocultar el panel

## 🔍 Normalización de Coordenadas JSVecx

### Algoritmo de Conversión

```javascript
const ALG_MAX_X = 33000;  // Rango interno JSVecx X
const ALG_MAX_Y = 41000;  // Rango interno JSVecx Y
const DAC_RANGE = 127;    // Rango DAC del Vectrex real

function convertToDac(value, algMax) {
    const center = algMax / 2;
    const normalized = (value - center) / center; // -1 to +1
    return normalized * DAC_RANGE;
}

// Ejemplo:
// X raw = 16500 (centro) → DAC = 0.0
// X raw = 0 (mínimo)    → DAC = -127.0
// X raw = 33000 (máximo) → DAC = +127.0
```

### Justificación

- JSVecx usa un integrador simulado con valores arbitrarios (0-33000 para X)
- El Vectrex real usa DACs de 8 bits con rango aproximado -127 a +127
- La normalización permite comparar "manzanas con manzanas"
- Sin normalización, los valores raw de JSVecx no son comparables con el emulador Rust

## 📊 Uso Recomendado

### Workflow de Comparación

1. **Rust Emulator (test_wasm.html):**
   - Abrir en navegador
   - Load WASM → Initialize → Start/Run Frame
   - Pausar cuando se vea el título "VECTREX"
   - Click "📸 Snapshot Vectors"
   - Copiar/exportar datos de la tabla

2. **JSVecx (test_jsvecx.html):**
   - Abrir en navegador
   - Run → Pausar cuando se vea el título "VECTREX"
   - Click "📸 Snapshot"
   - Copiar/exportar datos de la tabla

3. **Comparación:**
   - Comparar vectores en el mismo índice
   - Verificar diferencias en coordenadas X0, Y0, X1, Y1
   - Identificar patrones de offset sistemático
   - Calcular offset promedio: `Δ = X_rust - X_jsvecx`

### Puntos Clave de Comparación

- **Vector Count**: Debe ser idéntico o muy similar
- **X0, Y0**: Coordenadas de inicio de cada vector
- **X1, Y1**: Coordenadas de fin de cada vector
- **Length**: Longitud debe ser similar (indica scaling correcto)
- **Color**: Intensidad debe ser idéntica

## 🎨 Estilo Visual

### test_wasm.html (Verde en Negro)
- Fondo: `#000` (negro)
- Bordes tabla: `#555` (gris oscuro)
- Header tabla: `#333` (gris)
- Texto: `#00ff00` (verde brillante)
- Tema: Retro terminal

### test_jsvecx.html (Verde Vectrex)
- Fondo: `#000` (negro)
- Bordes tabla: `#00ff00` / `#005500` (verde brillante/oscuro)
- Header tabla: `#003300` (verde muy oscuro)
- Texto: `#00ff00` (verde brillante)
- Raw coords: `#888` (gris, tamaño reducido)
- Tema: Vectrex auténtico

## 🔧 Detalles Técnicos

### Rust Emulator (WASM)
- API usada: `emulator.getVectorCount()`, `emulator.getVector(i)`
- Coordenadas: Directas del emulador (ya normalizadas)
- Timing: Captura snapshot del buffer actual (pausado o running)

### JSVecx
- API usada: Array `vectorData` (capturado en `renderFrame()`)
- Fuente: `vecx.vectors_draw[i]` (integrador interno)
- Coordenadas raw: `v.x0, v.y0, v.x1, v.y1` (0-33000 para X, 0-41000 para Y)
- Normalización: Aplicada en tiempo de snapshot (no afecta rendering)

## ✅ Testing

### Verificación de Funcionalidad

**test_wasm.html:**
- [ ] Botón "Snapshot" habilitado después de init
- [ ] Panel aparece al hacer snapshot
- [ ] Tabla muestra todos los vectores
- [ ] Coordenadas en rango aproximado -127 a +127
- [ ] Botón "Clear" oculta el panel

**test_jsvecx.html:**
- [ ] Botón "Snapshot" funciona cuando hay vectores
- [ ] Panel aparece con tabla normalizada
- [ ] Coordenadas DAC en rango -127 a +127
- [ ] Coordenadas raw visibles en columnas adicionales
- [ ] Botón "Clear" oculta el panel

### Casos de Prueba

1. **Sin vectores**: Botón debe mostrar warning/error
2. **Con vectores**: Tabla debe mostrar todos los datos
3. **Múltiples snapshots**: Debe reemplazar snapshot anterior
4. **Después de Clear**: Panel debe ocultarse

## 📝 Notas de Implementación

- **No hay exportación a archivo**: Los snapshots se muestran solo en HTML (se puede copiar manualmente desde la tabla)
- **Snapshot es estático**: Captura el estado en el momento del click (no se actualiza automáticamente)
- **Pausar recomendado**: Para capturas precisas, pausar la emulación antes del snapshot
- **Coordinación manual**: El usuario debe capturar snapshots en el mismo punto de la emulación en ambos emuladores

## 🚀 Próximas Mejoras Posibles

- [ ] Exportar snapshots a JSON/CSV
- [ ] Comparación automática entre ambos emuladores
- [ ] Resaltado de diferencias significativas
- [ ] Gráfico de dispersión de coordenadas
- [ ] Superposición visual de vectores de ambos emuladores
- [ ] Sincronización automática de puntos de captura

---

**Fecha de implementación**: 2025-10-06  
**Propósito**: Investigación de offset visual -10.75 vs -4.65  
**Estado**: ✅ Implementado y listo para testing
