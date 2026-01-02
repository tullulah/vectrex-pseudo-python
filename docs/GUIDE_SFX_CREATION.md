# Guía Completa de Creación de SFX (Efectos de Sonido)

## 1. Introducción: Sistema AYFX

VPy utiliza **AYFX** (formato parametrizado de Richard Chadd) para efectos de sonido:
- **Lenguaje**: JSON
- **Ubicación**: `assets/sfx/*.vsfx` (ej: `assets/sfx/jump.vsfx`)
- **Canal**: PSG Canal C (registers 4/5, 6, 7, 10)
- **Formato**: Frame-based (se actualiza cada frame ~20ms en 50 FPS)

## 2. Estructura Básica de un SFX

```json
{
  "version": "1.0",
  "name": "jump",
  "category": "jump",
  "duration_ms": 180,
  
  "oscillator": {
    "frequency": 330,
    "channel": 0,
    "duty": 50
  },
  
  "envelope": {
    "attack": 0,
    "decay": 30,
    "sustain": 8,
    "release": 100,
    "peak": 14
  },
  
  "pitch": {
    "enabled": true,
    "start_mult": 0.6,
    "end_mult": 1.3,
    "curve": 1
  },
  
  "noise": {
    "enabled": false,
    "period": 15,
    "volume": 12,
    "decay_ms": 100
  },
  
  "modulation": {
    "arpeggio": false,
    "arpeggio_notes": [],
    "arpeggio_speed": 50,
    "vibrato": false,
    "vibrato_depth": 0,
    "vibrato_speed": 8
  }
}
```

## 3. Parámetros Detallados

### 3.1 Oscilador (Tono)

| Campo | Rango | Descripción | Ejemplos |
|-------|-------|-------------|----------|
| `frequency` | 55-1760 Hz | Frecuencia base del tono | 110=A2, 220=A3, 440=A4, 880=A5 |
| `channel` | 0-2 | Canal PSG (A/B/C) | 0=Canal A (rojo), 1=Canal B (verde), 2=Canal C (azul) |
| `duty` | 0-100 | Forma de onda (% pulse width) | 50=onda cuadrada pura, 25=más fino |

**Tabla de Frecuencias MIDI Comunes**:
```
C3=131Hz    E3=165Hz    G3=196Hz    C4=262Hz
D3=147Hz    F3=175Hz    A3=220Hz    D4=294Hz
E3=165Hz    G3=196Hz    B3=247Hz    E4=330Hz
F3=175Hz    A3=220Hz    C4=262Hz    F4=349Hz
G3=196Hz    B3=247Hz    D4=294Hz    G4=392Hz
A3=220Hz    C4=262Hz    E4=330Hz    A4=440Hz
B3=247Hz    D4=294Hz    F4=349Hz    B4=494Hz
```

### 3.2 Envelope (Amplitud)

Define la forma de la curva de volumen:

```
Volumen (0-15)
│
│ ╱╲
│╱  ╲___╲
├─────────────── tiempo
A D   S   R
```

| Campo | Rango | Descripción | Ejemplos |
|-------|-------|-------------|----------|
| `attack` | 0-500 ms | Tiempo para pasar de 0 a peak | 0=inmediato, 10=fade suave, 100=muy lento |
| `decay` | 0-500 ms | Tiempo de decay desde peak a sustain | 0=sin decay, 50=decay rápido, 200=muy suave |
| `sustain` | 0-15 | Volumen de mantenimiento (0-15) | 8=media, 15=máximo, 0=apagado |
| `release` | 0-1000 ms | Tiempo de release (sustain a 0) | 50=corto, 300=largo y natural |
| `peak` | 1-15 | Volumen pico durante attack/decay | 12-15=fuerte, 5-8=débil |

**Presets Comunes**:
- **Ataque rápido (laser)**: attack=0, decay=0, sustain=12, release=100
- **Ataque suave (powerup)**: attack=0, decay=20, sustain=10, release=100
- **Ataque muy lento**: attack=100, decay=200, sustain=6, release=300

### 3.3 Pitch Sweep (Barrido de Tono)

Cambia la frecuencia durante la duración del SFX:

| Campo | Rango | Descripción | Ejemplos |
|-------|-------|-------------|----------|
| `enabled` | true/false | Habilitar barrido de tono | |
| `start_mult` | 0.1-4.0 | Multiplicador inicial de frecuencia | 0.5=mitad pitch, 1.0=normal, 2.0=doble |
| `end_mult` | 0.1-4.0 | Multiplicador final de frecuencia | |
| `curve` | -5 a +5 | Curvatura del interpolación | -3=exponencial down (laser), 0=lineal, +2=exponencial up (powerup) |

**Ejemplos de Sweeps**:
- **Laser (down)**: start=2.0, end=0.5, curve=-2
- **Powerup (up)**: start=0.8, end=1.5, curve=2
- **Jump (up)**: start=0.6, end=1.3, curve=1
- **Explosion (down)**: start=1.5, end=0.3, curve=-3

### 3.4 Noise (Ruido)

Añade ruido blanco (útil para explosiones, efectos percusivos):

| Campo | Rango | Descripción | Ejemplos |
|-------|-------|-------------|----------|
| `enabled` | true/false | Habilitar ruido | |
| `period` | 0-31 | Período del ruido (más bajo=más agudo) | 8=agudo, 15=medio, 31=grave |
| `volume` | 0-15 | Volumen del ruido | 12-15=fuerte, 5-8=débil |
| `decay_ms` | 10-1000 | Fade-out del ruido | 100=rápido, 300=lento |

**Presets**:
- **Explosión**: period=8, volume=15, decay=350
- **Impacto (hit)**: period=12, volume=14, decay=80

### 3.5 Arpeggio (Acordes)

Toca múltiples notas en secuencia (acorde):

| Campo | Rango | Descripción | Ejemplos |
|-------|-------|-------------|----------|
| `enabled` | true/false | Habilitar arpeggio | |
| `arpeggio_notes` | [0-24] | Offsets de semitonos | [0,4,7]=Do-Mi-Sol (acorde mayor) |
| `arpeggio_speed` | 10-200 ms | Tiempo entre notas del acorde | 40=rápido, 60=normal, 100=lento |

**Acordes Musicales (en semitonos desde nota base)**:
```
Do Mayor (C)       [0, 4, 7]        (Do-Mi-Sol)
Do Menor (Cm)      [0, 3, 7]        (Do-Mib-Sol)
Do 7 (C7)          [0, 4, 7, 10]    (Do-Mi-Sol-Sib)
Do Mayor 7 (Cmaj7) [0, 4, 7, 11]    (Do-Mi-Sol-Si)
Acorde vacío       [0, 12]          (Do-Do arriba)
Quintas abiertas   [0, 7]           (Do-Sol)
Acorde 5ª          [0, 7, 12]       (Do-Sol-Do arriba)
```

**Ejemplos en Presets**:
- **Coin**: [0, 12] = octava simple, speed=60
- **Powerup**: [0, 4, 7, 12] = acorde mayor, speed=40

## 4. Recetas de SFX Comunes

### 4.1 Laser Clásico
```json
{
  "name": "laser",
  "duration_ms": 150,
  "oscillator": { "frequency": 880, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 0, "sustain": 12, "release": 100, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 2.0, "end_mult": 0.5, "curve": -2 },
  "noise": { "enabled": false },
  "modulation": { "arpeggio": false }
}
```
**Efecto**: Tono alto que baja rápidamente (como disparo de láser)

### 4.2 Moneda (Coin/Pickup)
```json
{
  "name": "coin",
  "duration_ms": 120,
  "oscillator": { "frequency": 880, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 10, "sustain": 12, "release": 80, "peak": 15 },
  "pitch": { "enabled": false },
  "noise": { "enabled": false },
  "modulation": {
    "arpeggio": true,
    "arpeggio_notes": [0, 12],
    "arpeggio_speed": 60
  }
}
```
**Efecto**: Nota ascendente alegre (acorde simple en octavas)

### 4.3 Salto (Jump)
```json
{
  "name": "jump",
  "duration_ms": 180,
  "oscillator": { "frequency": 330, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 30, "sustain": 8, "release": 100, "peak": 14 },
  "pitch": { "enabled": true, "start_mult": 0.6, "end_mult": 1.3, "curve": 1 },
  "noise": { "enabled": false },
  "modulation": { "arpeggio": false }
}
```
**Efecto**: Nota que sube durante el salto, volumen que decae

### 4.4 Explosión
```json
{
  "name": "explosion",
  "duration_ms": 400,
  "oscillator": { "frequency": 110, "channel": 0, "duty": 50 },
  "envelope": { "attack": 5, "decay": 50, "sustain": 4, "release": 300, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 1.5, "end_mult": 0.3, "curve": -3 },
  "noise": { "enabled": true, "period": 8, "volume": 15, "decay_ms": 350 },
  "modulation": { "arpeggio": false }
}
```
**Efecto**: Tono grave que cae rápido + ruido blanco fuerte (explosión clásica)

### 4.5 Impacto (Hit)
```json
{
  "name": "hit",
  "duration_ms": 100,
  "oscillator": { "frequency": 220, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 10, "sustain": 6, "release": 50, "peak": 15 },
  "pitch": { "enabled": false },
  "noise": { "enabled": true, "period": 12, "volume": 14, "decay_ms": 80 },
  "modulation": { "arpeggio": false }
}
```
**Efecto**: Sonido corto de impacto (ruido + tono grave)

### 4.6 Powerup (Ascendente)
```json
{
  "name": "powerup",
  "duration_ms": 200,
  "oscillator": { "frequency": 440, "channel": 0, "duty": 50 },
  "envelope": { "attack": 0, "decay": 20, "sustain": 10, "release": 100, "peak": 15 },
  "pitch": { "enabled": true, "start_mult": 0.8, "end_mult": 1.5, "curve": 2 },
  "noise": { "enabled": false },
  "modulation": {
    "arpeggio": true,
    "arpeggio_notes": [0, 4, 7, 12],
    "arpeggio_speed": 40
  }
}
```
**Efecto**: Acorde mayor que sube en pitch (sonido de "obtener poder")

## 5. Workflow de Creación Manual

### Paso 1: Crear archivo base
```bash
# Crear archivo en assets/sfx/
# assets/sfx/custom_sound.vsfx
```

### Paso 2: Definir parámetros básicos
1. **Elige frecuencia base**: ¿Grave (110-220 Hz)? ¿Medio (330-440 Hz)? ¿Agudo (880-1760 Hz)?
2. **Define duración**: ¿Corto (50-150 ms)? ¿Medio (150-300 ms)? ¿Largo (300+ ms)?
3. **Elige envelope simple**: ADSR con attack=0, decay=30, sustain=8, release=100

### Paso 3: Prueba y ajusta
1. Abre el SFX Editor
2. Carga tu archivo
3. Presiona "Play" para escuchar
4. Ajusta parámetros visualmente
5. Guarda cuando esté bien

### Paso 4: Añade características avanzadas
- ¿Necesita pitch sweep? Habilita `pitch.enabled` y ajusta start/end
- ¿Necesita ruido? Habilita `noise.enabled`
- ¿Necesita acorde? Habilita `modulation.arpeggio` y define notas

## 6. Tips de Diseño de SFX

| Efecto Deseado | Técnica | Parámetros |
|---|---|---|
| Sonido **punchy** (nítido) | Attack=0, decay corto, sustain bajo | Percusivo, corto |
| Sonido **suave** (fuzzy) | Attack pequeño, decay largo | Más natural |
| Sonido **retro** 8-bit | Duty < 50, pitch sweep fuerte | Clásico arcade |
| Sonido **grave** (bass) | Frecuencia baja (55-110 Hz) | Impactante |
| Sonido **agudo** (treble) | Frecuencia alta (1320+ Hz) | Brillante |
| Sonido **grave a agudo** | Pitch sweep up (start < end) | Ascendente |
| Sonido **agudo a grave** | Pitch sweep down (start > end) | Descendente |
| Acorde (armónico) | Arpeggio con notas [0, 4, 7, 12] | Musical |

## 7. Limitaciones y Notas

⚠️ **Importante**:
- Canal C es **solo para SFX** (música usa A+B)
- Máx. duración: ~2 segundos (más impacta rendimiento)
- Arpeggio se ejecuta **secuencialmente** (no acordes simultáneos)
- Máx. volumen (15) puede **saturar** en Vectrex real
- Frecuencias < 55 Hz o > 1760 Hz causan comportamiento indefinido

## 8. Ejemplos de Inspiración

Busca inspiración en:
- **SFXR** (herramienta web clásica): genera SFX retro aleatorios
- **Soundly**: editor profesional de SFX
- **Zapsplat**: librería libre de SFX
- **Juegos Atari**: para sonidos 8-bit auténticos

---

**Última actualización**: 2025-12-23
**Formato**: AYFX (Richard Chadd system)
