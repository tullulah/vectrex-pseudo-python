# 🎵 Sistema de Sonido VPy - Mapa de Recursos

## Visión General

VPy tiene un **sistema de audio completo** basado en:
- **PLAY_MUSIC()** - PSG music (canales A+B)
- **PLAY_SFX()** - AYFX effects (canal C)
- **AUDIO_UPDATE()** - Auto-injected, actualiza ambos sistemas cada frame

Este documento te ayuda a **encontrar exactamente lo que necesitas**.

---

## 🎯 ¿Qué Necesito?

### "Quiero crear un nuevo SFX desde cero"
👉 Comienza con: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md)**
1. Lee Sección 3 (Parámetros Detallados)
2. Copia una receta de Sección 4
3. Modifica parámetros según necesites
4. Guarda en `assets/sfx/mi_sonido.vsfx`

**Tiempo estimado**: 10-15 minutos por SFX

---

### "Quiero entender cómo funcionan los SFX existentes"
👉 Comienza con: **[GUIDE_SFX_EXAMPLES.md](GUIDE_SFX_EXAMPLES.md)**
1. Lee Sección 2 (Análisis Jump - simple)
2. Lee Sección 3 (Comparación Jump vs Coin)
3. Lee Sección 4 (Análisis Explosion - complejo)
4. Intenta recrear en el editor

**Tiempo estimado**: 20-30 minutos para comprensión completa

---

### "Quiero usar Arpeggio para acordes musicales"
👉 Comienza con: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) - Sección 3.5**
- Lista de acordes musicales (semitones)
- Ejemplos en presets existentes
- Editor visual de arpeggio en SFX Editor

**Ejemplos rápidos**:
```json
"modulation": {
  "arpeggio": true,
  "arpeggio_notes": [0, 4, 7],      // Do-Mi-Sol (mayor)
  "arpeggio_speed": 50
}
```

---

### "Necesito una referencia rápida de parámetros"
👉 Consulta: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) - Sección 2-3**

Tablas de referencia rápida:
- Oscilador (frequency, channel, duty)
- Envelope (ADSR)
- Pitch Sweep (multiplicadores)
- Noise (periodo, volumen, decay)
- Arpeggio (acordes predefinidos)

---

### "Quiero ver ejemplos de SFX comunes"
👉 Consulta: **[GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) - Sección 4**

5 recetas completas:
1. **Laser** - Tono alto que baja rápido
2. **Coin** - Acorde simple y feliz
3. **Jump** - Nota que sube
4. **Explosion** - Complejo con ruido
5. **Powerup** - Acorde ascendente

---

### "¿Cómo uso SFX en mi código VPy?"
👉 Consulta: **[VPyContext.ts](ide/frontend/src/services/contexts/VPyContext.ts)**

Sintaxis rápida:
```python
def main():
    PLAY_SFX("jump")  # Comienza SFX

def loop():
    WAIT_RECAL()  # Auto-injected: AUDIO_UPDATE()
    
    if J1_BUTTON_1():
        PLAY_SFX("coin")  # Toca moneda
    
    DRAW_VECTOR("player", x, y)
    # Audio se actualiza automáticamente
```

---

### "El SFX Editor no muestra lo que quiero"
👉 Ubicación: **`ide/frontend/src/components/SFXEditor.tsx`**

Features actuales:
- ✅ Oscillator (frecuencia, canal)
- ✅ Envelope (ADSR)
- ✅ Pitch Sweep (curva)
- ✅ Noise (ruido blanco)
- ✅ Arpeggio (acordes) - **NUEVO**
- ✅ Visualización en tiempo real

Cómo usar:
1. Abre el proyecto
2. Encuentra `assets/sfx/algo.vsfx`
3. Doble-click para abrir en SFX Editor
4. Presiona Play para escuchar
5. Ajusta sliders
6. Guarda con Ctrl+S

---

## 📚 Estructura de Documentación

```
VPy Sound System
├── [GUIDE_SFX_CREATION.md]
│   ├── 1. Introducción (qué es AYFX)
│   ├── 2. Estructura JSON base
│   ├── 3. Parámetros detallados (tablas)
│   ├── 4. Recetas comunes (5 ejemplos)
│   ├── 5. Workflow de creación manual
│   ├── 6. Tips de diseño
│   ├── 7. Limitaciones
│   └── 8. Inspiración externa
│
├── [GUIDE_SFX_EXAMPLES.md]
│   ├── 1. Ubicación de SFX
│   ├── 2. Análisis Jump (simple)
│   ├── 3. Comparación Jump vs Coin
│   ├── 4. Análisis Explosion (complejo)
│   ├── 5. Timeline visualization
│   ├── 6. Template personalizado
│   └── 7. Checklist y avanzado
│
├── [VPyContext.ts]
│   └── Documentación integrada en IDE
│       ├── PLAY_MUSIC()
│       ├── PLAY_SFX()
│       ├── AUDIO_UPDATE()
│       └── Ejemplos de código
│
└── [SFXEditor.tsx]
    └── Editor visual interactivo
        ├── Sliders para todos los parámetros
        ├── Canvas de visualización envelope
        ├── Botones de presets
        ├── Editor de arpeggio
        └── Botón Play para preview
```

---

## 🚀 Quick Start Paths

### Path 1: "Quiero un SFX Laser Rápido" (5 min)
```
1. Abre SFX Editor
2. Presiona botón "laser" (preset)
3. Presiona "Play" para escuchar
4. ¡Listo! Ya tienes un laser
```

### Path 2: "Quiero Entender Todo" (60 min)
```
1. Lee GUIDE_SFX_CREATION.md (20 min)
2. Lee GUIDE_SFX_EXAMPLES.md (25 min)
3. Abre SFX Editor (15 min)
   - Carga cada preset
   - Presiona Play
   - Cambia parámetros
   - Escucha diferencias
```

### Path 3: "Quiero Crear Mi Sonido Único" (30 min)
```
1. Elige una inspiración (GUIDE_SFX_EXAMPLES.md - Sección 5)
2. Copia una receta base (GUIDE_SFX_CREATION.md - Sección 4)
3. Crea assets/sfx/mi_sonido.vsfx
4. Abre en SFX Editor
5. Ajusta parámetros
6. Presiona Play (itera hasta que te guste)
7. Guarda
8. Usa en código: PLAY_SFX("mi_sonido")
```

---

## 🎓 Conceptos Clave Explicados

### Envelope (ADSR)
**Qué es**: Curva de volumen del sonido

```
Attack (A)   = fade-in (0-500ms)
Decay (D)    = baja a sustain (0-500ms)
Sustain (S)  = volumen de reposo (0-15)
Release (R)  = fade-out final (0-1000ms)
Peak         = volumen máximo (1-15)
```

**Efecto práctico**:
- A=0: Comienza fuerte (nítido)
- A=100: Comienza suave (fade-in)
- R=50: Corto (sonido seco)
- R=300: Largo (sonido natural)

---

### Pitch Sweep
**Qué es**: Cambio de frecuencia durante el efecto

```
start_mult = 0.5  → comienza a mitad pitch
end_mult = 2.0    → termina al doble pitch
curve = 1         → interpolación suave

Resultado: Sonido que SUBE (como "POP" de powerup)
```

---

### Arpeggio (Acordes)
**Qué es**: Toca múltiples notas en secuencia

```
[0, 4, 7]      → Do-Mi-Sol (acorde mayor)
[0, 12]        → Do-Do octava arriba
[0, 3, 7, 10]  → Do menor 7

speed: 50ms    → qué tan rápido cambia entre notas
```

---

### Noise (Ruido Blanco)
**Qué es**: Sonido sin tono específico (ruido)

```
period: 8      → ruido agudo
period: 20     → ruido grave
volume: 15     → muy fuerte
decay: 350ms   → desvanece lentamente
```

**Usa para**: explosiones, impactos, fricción

---

## 🔧 Troubleshooting

### "SFX no suena en el juego"
1. Verifica que `PLAY_SFX("nombre")` sea correcto
2. El archivo debe existir en `assets/sfx/nombre.vsfx`
3. Compila el proyecto: `cargo build --release`
4. Prueba en emulador

### "SFX suena diferente en SFX Editor vs juego"
- El editor usa Web Audio API (aproximación)
- El juego usa hardware PSG real (Vectrex)
- Es normal pequeñas diferencias

### "¿Cómo edito un SFX existente?"
1. Abre `assets/sfx/nombre.vsfx`
2. Edita JSON directamente O
3. Doble-click para abrir en SFX Editor
4. Ajusta con sliders
5. Guarda

---

## 📖 Para Más Información

- **Parámetros técnicos**: [GUIDE_SFX_CREATION.md](GUIDE_SFX_CREATION.md) Sección 3
- **Ejemplos concretos**: [GUIDE_SFX_EXAMPLES.md](GUIDE_SFX_EXAMPLES.md)
- **Uso en código**: VPyContext.ts → PLAY_SFX
- **Integración**: Busca `AUDIO_UPDATE` en copilot-instructions.md

---

## 🎵 Estado Actual

| Feature | Estado | Ubicación |
|---------|--------|-----------|
| SFX Básico | ✅ Completo | AYFX parser |
| Oscilador | ✅ Completo | frequency, channel, duty |
| Envelope | ✅ Completo | ADSR |
| Pitch Sweep | ✅ Completo | start/end multipliers |
| Noise | ✅ Completo | period, volume, decay |
| Arpeggio | ✅ Completo | [0-24] semitones |
| Editor Visual | ✅ Nuevo | SFXEditor.tsx |
| Documentación | ✅ Nuevo | GUIDE_SFX_*.md |
| Presets | ✅ 7 presets | laser, coin, jump, etc. |

---

**Última actualización**: 2025-12-23
**Versión**: 2.0 (con Arpeggio editor y guías completas)
