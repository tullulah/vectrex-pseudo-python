# Análisis de Snapshots: Rust vs JSVecx

## 📊 Datos Crudos

- **Rust Emulator**: 868 vectores
- **JSVecx**: 388 vectores  
- **Ratio**: 2.24x (Rust tiene más del doble de vectores)

## 🔍 Hallazgos Críticos

### 1. DUPLICACIÓN DE VECTORES EN RUST

**Observación**: Los vectores se están renderizando DUPLICADOS en el emulador Rust.

**Evidencia**:
```
Vector #0:   -31.00  33.00  →  -31.00  33.00
Vector #90:  -31.00  33.00  →  -31.00  33.00  (DUPLICADO EXACTO)

Vector #1:   -27.00  33.00  →  -26.00  33.00
Vector #91:  -27.00  33.00  →  -26.00  33.00  (DUPLICADO EXACTO)
```

**Patrón**: Cada ~90 vectores, la secuencia se repite.

### 2. COORDENADAS DIFERENTES

**Rust Emulator** (primeros vectores del título "VECTREX"):
```
Y = 33.00 (constante para primera línea)
X: -31.00, -27.00, -20.00, -17.00, -8.00, -6.00, 2.00, 5.00, 14.00...
```

**JSVecx** (primeros vectores normalizados):
```
Y = 25.96 (constante para primera línea)
X: -6.23, 7.07, 19.63, -6.97, 6.33, 9.29, 19.63...
```

**Diferencia en Y**: 33.00 - 25.96 = **7.04 unidades** (Rust más arriba)

**Diferencia en X** (primer vector):
- Rust X0 = -31.00
- JSVecx X0 = -6.23
- Δ = **-24.77 unidades**

Pero esto NO coincide con el offset observado de -10.75 vs -4.65...

### 3. POSIBLE CAUSA DE DUPLICACIÓN

**Hipótesis 1**: El emulador Rust está renderizando cada frame DOS VECES
- Posible bug en el loop de renderizado
- `renderVectors()` llamado múltiples veces por frame

**Hipótesis 2**: Los vectores se están acumulando sin clear
- `render_context` no se limpia entre frames
- Vectores del frame anterior + vectores nuevos

**Hipótesis 3**: La BIOS está dibujando el título dos veces
- Poco probable (JSVecx no lo hace)
- Pero posible si hay diferencia en timing

## 🎯 Comparación de Coordenadas Específicas

### Título "VECTREX" - Primera Línea Horizontal

**Rust** (Y=33):
| Vector | X0 | X1 | Longitud |
|--------|-----|-----|----------|
| 0 | -31.00 | -31.00 | 0.00 |
| 1 | -27.00 | -26.00 | 1.00 |
| 2 | -20.00 | -17.00 | 3.00 |
| 3 | -17.00 | -14.00 | 3.00 |

**JSVecx** (Y=25.96):
| Vector | X0 | X1 | Longitud |
|--------|-----|-----|----------|
| 0 | -6.23 | -3.27 | 2.96 |
| 1 | 7.07 | 9.29 | 2.22 |
| 2 | 19.63 | 23.33 | 3.69 |

**Observación**: Los vectores NO se corresponden 1:1. JSVecx agrupa/optimiza diferente.

## 🚨 Problemas Identificados

### Problema 1: Duplicación de Vectores (CRÍTICO)
- **Impacto**: El emulador Rust está generando el doble de vectores necesarios
- **Efecto visual**: Posible sobre-brillantez, líneas más gruesas
- **Causa probable**: Bug en el loop de renderizado o acumulación de vectores

### Problema 2: Coordenadas No Comparables Directamente
- **Impacto**: No podemos comparar vector a vector porque no se corresponden
- **Causa**: JSVecx y Rust optimizan/agrupan vectores diferente
- **Solución**: Comparar rangos globales (min/max X/Y) en lugar de vectores individuales

### Problema 3: Diferencia en Altura Y
- **Impacto**: Rust dibuja ~7 unidades más arriba que JSVecx
- **Causa**: Posible diferencia en cálculo de offset Y o centro de pantalla

## 📐 Análisis de Rangos (Necesitamos Calcular)

Para comparar correctamente, necesitamos:

1. **Rust**: Calcular min/max X, min/max Y de TODOS los vectores
2. **JSVecx**: Calcular min/max X, min/max Y de TODOS los vectores
3. **Comparar centros**: `center_x = (min_x + max_x) / 2`
4. **Calcular offsets reales**: `Δx = center_rust - center_jsvecx`

## 🔧 Acción Requerida

### Inmediata:
1. **Investigar duplicación de vectores en Rust**
   - ¿`renderVectors()` se llama múltiples veces?
   - ¿`render_context.clear()` funciona correctamente?
   - ¿El buffer de vectores se limpia entre frames?

2. **Calcular rangos completos**
   - Script para extraer min/max X/Y de ambos snapshots
   - Comparar centros y offsets reales

### Medio plazo:
3. **Verificar correspondencia de vectores**
   - ¿Por qué JSVecx tiene vectores diferentes?
   - ¿Optimización diferente del integrador?
   - ¿Problema en conversión DAC?

## 💡 Insights

**Por qué el snapshot es invaluable**:
- Sin snapshot, no habríamos visto la duplicación (2.24x vectores)
- Confirma que el problema NO es solo de coordenadas
- Revela posible bug fundamental en el rendering loop

**Por qué las coordenadas no coinciden**:
- Diferentes algoritmos de optimización de vectores
- JSVecx puede estar combinando vectores cortos
- Rust puede estar generando micro-vectores (length=0.00)

**Próximo paso crítico**:
- **ARREGLAR LA DUPLICACIÓN PRIMERO**
- Luego comparar offsets con vectores únicos
- Solo entonces tendrá sentido la comparación de coordenadas

---

**Fecha**: 2025-10-06  
**Hallazgo clave**: Rust genera 2.24x más vectores que JSVecx (duplicación)  
**Prioridad**: CRÍTICA - Fix duplicación antes de continuar análisis de offset
