# Análisis de SFX Existentes - Ejemplos Prácticos

Esta guía te muestra cómo **leer y entender** los SFX del proyecto actual, usándolos como referencia para crear los tuyos propios.

## 1. Ubicación de SFX en el Proyecto

```
assets/sfx/
├── jump.vsfx
├── explosion.vsfx
├── coin.vsfx
├── hit.vsfx
├── laser.vsfx
├── powerup.vsfx
├── blip.vsfx
└── [tus SFX aquí]
```

## 2. Cómo Leer un SFX Completo

Vamos a **diseccionar** el SFX de "jump" paso a paso:

### 2.1 Jump - Salto Ascendente

**Propósito**: Efecto de sonido cuando el personaje salta.

```json
{
  "version": "1.0",
  "name": "jump",
  "category": "jump",
  "duration_ms": 180
}
```
📍 **Lectura**:
- Dura 180 ms (3 frames a 50 FPS)
- Efecto corto pero audible
- Categoría para organización en editor

---

### 2.2 Oscillator (Generador de Tono Base)

```json
"oscillator": {
  "frequency": 330,
  "channel": 0,
  "duty": 50
}
```
📍 **Lectura**:
- **330 Hz** = Nota E4 (Mi4), tono medio-alto
- **Canal 0** = PSG Channel A (rojo en visualización)
- **Duty 50%** = Onda cuadrada pura

**Por qué es bueno para saltos**: 
- Frecuencia media permite escuchar cambios de pitch
- No es muy grave (no suena "golposo")
- No es muy agudo (no molesta al oído)

---

### 2.3 Envelope (Forma de Volumen)

```json
"envelope": {
  "attack": 0,
  "decay": 30,
  "sustain": 8,
  "release": 100,
  "peak": 15
}
```

**Visualización Temporal**:
```
Volumen (0-15)
     15│       ╱╲
       │      ╱  ╲___╲__
        │    ╱    decay ╲___ decay 100ms
        │   ╱0     sustain=8 
        │  ╱        
        └─────────────────── tiempo
      atk=0   decay=30ms  release=100ms
           └─ Total: 130ms de envelope
```

📍 **Lectura Línea por Línea**:
- **attack: 0** = Sin fade-in (comienza inmediatamente a volumen máximo)
- **decay: 30** = Cae de volumen máximo (15) a sustain (8) en 30ms
- **sustain: 8** = Mantiene volumen medio-bajo después del decay
- **release: 100** = Fade-out final de sustain a silencio en 100ms
- **peak: 15** = Volumen máximo (bastante fuerte)

**Por qué funciona**:
- Ataque inmediato = sonido "nítido" (satisfactorio)
- Decay rápido = simula energía del salto
- Sustain medio = mantiene el sonido sin "sobreboleo"
- Release largo = se desvanece naturalmente

---

### 2.4 Pitch Sweep (Cambio de Tono)

```json
"pitch": {
  "enabled": true,
  "start_mult": 0.6,
  "end_mult": 1.3,
  "curve": 1
}
```

**Visualización**:
```
Frequency multiplier
    1.3│                    ╱╱
       │                   ╱╱
       │                  ╱╱
       │                 ╱╱
     1.0│────────────────╱
       │               ╱
       │              ╱
     0.6│╱╱╱╱╱╱╱╱╱╱╱╱
       └─────────────────── tiempo (180ms)
            curve=1 (exponencial up)
```

📍 **Lectura**:
- **start_mult: 0.6** = Comienza a 60% de frecuencia base (330 × 0.6 = 198 Hz)
- **end_mult: 1.3** = Termina a 130% de frecuencia base (330 × 1.3 = 429 Hz)
- **curve: 1** = Interpolación exponencial (curva suave hacia arriba)

**Frecuencias Reales Durante el Efecto**:
```
Inicio (0ms):   330 Hz × 0.6 = 198 Hz  (G3 - grave)
Mitad (90ms):   ~295 Hz (entre G3 y D4)
Final (180ms):  330 Hz × 1.3 = 429 Hz  (A4 - agudo)
```

**Por qué funciona**:
- **Empieza grave** = sensación de "acumulación de energía"
- **Sube gradualmente** = como si saltara hacia arriba
- **Termina agudo** = ¡el salto ocurre!

---

### 2.5 Noise (Ruido)

```json
"noise": {
  "enabled": false,
  "period": 15,
  "volume": 12,
  "decay_ms": 100
}
```

📍 **Lectura**:
- **enabled: false** = Sin ruido en este efecto
- (El resto de parámetros se ignoran)

**Cuándo añadirías ruido**:
- Si quisieras un salto "áspero" = enabled: true
- Simularía fricción de pies contra el suelo

---

### 2.6 Modulation (Modulación/Arpeggio)

```json
"modulation": {
  "arpeggio": false,
  "arpeggio_notes": [],
  "arpeggio_speed": 50,
  "vibrato": false,
  "vibrato_depth": 0,
  "vibrato_speed": 8
}
```

📍 **Lectura**:
- **arpeggio: false** = Sin acorde, solo nota única
- arpeggio_notes está vacío
- Sin vibrato (modulación de amplitud)

---

## 3. Comparación: Jump vs Coin

Veamos cómo **coin** es diferente:

### Jump (analizado arriba)
```
- frequency: 330 Hz
- pitch: Sube de 0.6x a 1.3x
- envelope: attack=0, decay=30, sustain=8, release=100
- noise: disabled
- arpeggio: false
→ Sonido: Nota simple que sube (WHOOSH)
```

### Coin (alternativa con arpeggio)
```
- frequency: 880 Hz (más agudo)
- pitch: disabled (sin barrido)
- envelope: attack=0, decay=10, sustain=12, release=80
- noise: disabled
- arpeggio: true, notes=[0, 12], speed=60
→ Sonido: Dos notas musicales (octava), corto y feliz
```

**Diferencias Clave**:
| Parámetro | Jump | Coin |
|-----------|------|------|
| Frecuencia | 330 Hz (medio) | 880 Hz (agudo) |
| Pitch sweep | ✅ Sí (0.6→1.3) | ❌ No |
| Arpeggio | ❌ No | ✅ Sí [0,12] |
| Arp speed | - | 60ms |
| Decay | 30ms (rápido) | 10ms (muy rápido) |
| Release | 100ms (largo) | 80ms (medio) |

**Escucha la Diferencia**:
- **Jump**: Una sola nota que SUBE (dinámica de movimiento)
- **Coin**: Dos notas separadas (acorde simple, estático)

---

## 4. Análisis: Explosion (Complejo)

Explosión es el SFX más **complicado** - veamos por qué:

```json
{
  "version": "1.0",
  "name": "explosion",
  "category": "explosion",
  "duration_ms": 400,
  
  "oscillator": { "frequency": 110, "channel": 0, "duty": 50 },
  "envelope": { "attack": 5, "decay": 50, "sustain": 4, "release": 300, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 1.5, "end_mult": 0.3, "curve": -3 },
  "noise": { "enabled": true, "period": 8, "volume": 15, "decay_ms": 350 },
  "modulation": { "arpeggio": false }
}
```

### 4.1 Componentes

**Generador de Tono**:
- 110 Hz = Nota A2 (MUY grave, casi infra-sonido)
- Crea impacto de baja frecuencia

**Pitch Sweep**:
- Cae de 1.5x a 0.3x (110Hz → 33Hz)
- Curve -3 = exponencial rápida hacia abajo
- Frecuencia FINAL: 33 Hz (muy grave, casi sub-bass)
- **Efecto**: Simulación del "ruido" de explosión disminuyendo

**Noise (Ruido Blanco)**:
- period: 8 (ruido agudo)
- volume: 15 (máximo)
- decay: 350ms
- **Efecto**: Componente "rasgado" / "explosivo"

**Envelope**:
- attack: 5ms (fade-in muy rápido)
- decay: 50ms (cae desde pico a sustain)
- sustain: 4 (muy bajo, casi silencioso)
- release: 300ms (fade-out LARGO)
- **Efecto**: Impacto inicial fuerte, luego disipación larga

### 4.2 Línea de Tiempo

```
Timeline de Explosion (400ms total):

ms     0      50     100    150    200    300    400
│      │      │      │      │      │      │      │
├──────┼──────┼──────┼──────┼──────┼──────┼──────┤
│ AMP  │█████████─────────────────╲╲╲╲╲╲╲╲╲╲╲╲╲│  Envelope
│      │█████████────────────────────────────────│  (curve)
├──────┼──────┼──────┼──────┼──────┼──────┼──────┤
│ PITCH│●●●●●●●●●●●●●●●●●●●●●╲╲╲╲╲╲╲╲╲╲╲╲╲╲│  1.5x→0.3x
├──────┼──────┼──────┼──────┼──────┼──────┼──────┤
│ NOISE│██████████████████████╲╲╲╲╲╲╲╲╲╲╲╲╲╲│  350ms decay
│      │      │      │      │      │      │      │
└──────┴──────┴──────┴──────┴──────┴──────┴──────┘

Fases:
1. (0-50ms):   Ataque: tono grave + ruido agudo
2. (50-100ms): Decay: volumen baja, pitch continúa bajando
3. (100-400ms): Release largo con ruido desapareciendo
```

### 4.3 Por Qué Suena Bien

1. **Tono grave** = impacto, "peso"
2. **Ruido agudo** = fricción, "fuego"
3. **Pitch baja** = simulación de "ruido de aire" (efecto Doppler)
4. **Release largo** = ambiente reverberación natural

---

## 5. Template: Crea Tu Propio SFX Basado en Ejemplos

### Quiero un sonido de... **Hit Corporal (golpe)**

**Inspírate en**: Explosion (ruido) + Jump (pitch) + Hit (corto)

```json
{
  "version": "1.0",
  "name": "punch",
  "category": "hit",
  "duration_ms": 120,
  
  "oscillator": { "frequency": 150, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 15, "sustain": 3, "release": 80, "peak": 14 },
  "pitch": { "enabled": true, "start_mult": 1.2, "end_mult": 0.7, "curve": -1 },
  "noise": { "enabled": true, "period": 10, "volume": 12, "decay_ms": 100 },
  "modulation": { "arpeggio": false }
}
```

**Decisiones Tomadas**:
- **Frecuencia 150 Hz** = Grave como explosion, pero más agudo
- **Pitch sweep** = Como jump, para simulación de "impacto"
- **Noise mixto** = Como explosion (ruido), pero no tanto
- **Duration 120ms** = Corto como hit, no tan largo como explosion
- **Sustain bajo** = Golpe "seco" sin reverberación

---

## 6. Checklist: Verificar un SFX Antes de Guardar

```
□ ¿version es "1.0"?
□ ¿name es único? (sin espacios ni caracteres especiales)
□ ¿category es válida? (custom, laser, explosion, jump, hit, coin, blip, powerup)
□ ¿duration_ms está en rango? (20-2000 ms recomendado)

□ ¿frequency está en rango? (55-1760 Hz)
□ ¿channel es 0-2? (A/B/C)
□ ¿duty es 0-100?

□ ¿attack + decay + release suman menos de duration_ms?
□ ¿peak es 1-15?
□ ¿sustain es 0-15 y menor que peak?

□ ¿pitch.curve es -5 a +5?
□ ¿noise.period es 0-31?
□ ¿noise.volume es 0-15?

□ ¿arpeggio_notes está vacío [] o tiene números 0-24?
□ ¿arpeggio_speed es 10-200?

□ He escuchado el efecto en el editor → ¿Suena bien?
```

---

## 7. Recursos de Referencia

### Tabla de Notas MIDI
```
C3:131   D3:147   E3:165   F3:175   G3:196   A3:220   B3:247
C4:262   D4:294   E4:330   F4:349   G4:392   A4:440   B4:494
C5:523   D5:587   E5:659   F5:698   G5:784   A5:880   B5:988
```

### Técnicas Avanzadas

**Para sonar más "jugable"** (como arcade):
- Usa duty < 50 (onda más fina)
- Add pitch sweep down (start_mult > end_mult)
- Sustain bajo (4-6)
- Release rápido (50-100ms)

**Para sonar "épico"**:
- Usa ruido con periodo bajo (5-8) = agudo
- Pitch sweep Down fuerte (curve: -3 a -5)
- Duration largo (300+ ms)
- Release LARGO (200+ ms)

**Para sonar "musical"**:
- Habilita arpeggio con acordes mayores [0,4,7]
- Sustain alto (10-12)
- Sin ruido
- Pitch sweep pequeño o disabled

---

**Próximo paso**: Abre el SFX Editor, carga `assets/sfx/jump.vsfx`, presiona Play, y experimenta cambiando cada parámetro para entender cómo afecta el sonido. 🎵

