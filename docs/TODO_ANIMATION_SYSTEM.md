# TODO: VPy Animation System (.vanim)

## Objetivo
Crear un sistema de animaciones declarativo para reemplazar el código hardcodeado de animaciones de sprites en Pang.

## Problema Actual
```python
# Hardcoded en main.vpy (líneas 360-375)
if player_anim_frame == 1:
    DRAW_VECTOR_EX("player_walk_1", player_x, player_y, mirror_mode, 80)
elif player_anim_frame == 2:
    DRAW_VECTOR_EX("player_walk_2", player_x, player_y, mirror_mode, 80)
# ... 3 frames más

# Variables de control manual
player_anim_frame = 1
player_anim_counter = 0
player_anim_speed = 5
player_facing = 1
```

## Arquitectura Propuesta

### 1. Formato de Archivo: `.vanim` (JSON)
**Ubicación**: `assets/animations/*.vanim`

```json
{
  "version": "1.0",
  "name": "player_walk",
  "type": "animation",
  "metadata": {
    "author": "Daniel",
    "description": "Player walking cycle",
    "fps": 12,
    "loop": true
  },
  "frames": [
    {
      "id": 0,
      "vectorName": "player_walk_1",
      "duration": 5,
      "intensity": 80,
      "offsetX": 0,
      "offsetY": 0
    },
    {
      "id": 1,
      "vectorName": "player_walk_2",
      "duration": 5,
      "intensity": 80
    }
  ],
  "states": {
    "idle": {
      "animation": "player_idle",
      "mirror": false,
      "loop": true
    },
    "move_right": {
      "animation": "player_walk",
      "mirror": false,
      "loop": true,
      "triggers": ["joy_x > 0"]
    },
    "move_left": {
      "animation": "player_walk",
      "mirror": true,
      "loop": true,
      "triggers": ["joy_x < 0"]
    },
    "shoot": {
      "animation": "player_shoot",
      "mirror": false,
      "loop": false,
      "triggers": ["button_1"],
      "next_state": "idle"
    }
  },
  "controller": {
    "playable": true,
    "type": "player",
    "inputs": {
      "joy_x": "horizontal_movement",
      "joy_y": "vertical_movement",
      "button_1": "action_shoot",
      "button_2": "action_jump",
      "button_3": "action_special",
      "button_4": "action_extra"
    }
  }
}
```

### 2. Sistema de Instancias Múltiples
**CRÍTICO**: El sistema debe soportar múltiples animaciones simultáneas.

#### Ejemplo de uso:
```python
# Pang con 1 player + 8 enemigos = 9 animaciones activas
player_anim_id = CREATE_ANIM("player_walk")
enemy_anim_ids = [0, 0, 0, 0, 0, 0, 0, 0]

def spawn_enemy(index):
    enemy_anim_ids[index] = CREATE_ANIM("bubble_float")

def loop():
    WAIT_RECAL()
    
    # Player (playable - con input)
    UPDATE_ANIM(player_anim_id, player_x, player_y)
    DRAW_ANIM(player_anim_id)
    
    # Enemies (non-playable - sin input)
    for i in range(MAX_ENEMIES):
        if enemy_active[i]:
            UPDATE_ANIM(enemy_anim_ids[i], enemy_x[i], enemy_y[i])
            DRAW_ANIM(enemy_anim_ids[i])
```

#### Estructura de Datos en RAM:
```asm
; Animation Instance (12 bytes per instance)
ANIM_INSTANCE_DATA:
    ; Instance 0 (player)
    FDB _PLAYER_WALK_ANIM    ; Pointer to animation data (ROM)
    FCB 0                     ; Current frame index
    FCB 0                     ; Frame counter (for duration)
    FCB 0                     ; Current state index
    FCB 0                     ; Mirror flags (0-3)
    FDB 0                     ; X position
    FDB 0                     ; Y position
    FCB 1                     ; Active flag
    FCB 0                     ; Playable flag
    
    ; Instance 1-8 (enemies)
    ; ... same structure repeated
```

### 3. Módulos a Implementar

#### A) `core/src/animres.rs` y `buildtools/vpy_codegen/src/animres.rs`
```rust
pub struct VAnimAnimation {
    pub version: String,
    pub name: String,
    pub metadata: VAnimMetadata,
    pub frames: Vec<VAnimFrame>,
    pub states: HashMap<String, VAnimState>,
    pub controller: Option<VAnimController>,
}

pub struct VAnimFrame {
    pub id: usize,
    pub vector_name: String,
    pub duration: u8,
    pub intensity: u8,
    pub offset_x: i16,
    pub offset_y: i16,
}

pub struct VAnimState {
    pub animation: String,
    pub mirror: bool,
    pub loop_anim: bool,
    pub triggers: Vec<String>,
    pub next_state: Option<String>,
}

pub struct VAnimController {
    pub playable: bool,
    pub controller_type: String,
    pub inputs: HashMap<String, String>,
}

impl VAnimAnimation {
    pub fn load(path: &Path) -> Result<Self>;
    pub fn compile_to_asm(&self, asset_name: &str) -> String;
}
```

#### B) Compilación a ASM
```asm
; Animation: PLAYER_WALK
_PLAYER_WALK_ANIM:
    FCB 5                      ; Frame count
    FCB 1                      ; Loop flag
    FDB _PLAYER_WALK_FRAMES    ; Pointer to frames
    FDB _PLAYER_WALK_STATES    ; Pointer to states
    FCB 1                      ; Playable flag

_PLAYER_WALK_FRAMES:
    ; Frame 0
    FDB _PLAYER_WALK_1_VECTORS  ; Vector pointer
    FCB 5                       ; Duration (frames)
    FCB 80                      ; Intensity
    FDB 0                       ; Offset X
    FDB 0                       ; Offset Y
    
    ; Frame 1
    FDB _PLAYER_WALK_2_VECTORS
    FCB 5
    FCB 80
    FDB 0
    FDB 0
    ; ... more frames

_PLAYER_WALK_STATES:
    FCB 3                      ; State count
    ; State 0: idle
    FCB 0                      ; Animation index
    FCB 0                      ; Mirror
    FCB 1                      ; Loop
    FDB _PLAYER_IDLE_ANIM
    
    ; State 1: move_right
    FCB 0                      ; Animation index
    FCB 0                      ; Mirror
    FCB 1                      ; Loop
    FDB _PLAYER_WALK_ANIM
    
    ; State 2: move_left
    FCB 0                      ; Animation index
    FCB 1                      ; Mirror (X)
    FCB 1                      ; Loop
    FDB _PLAYER_WALK_ANIM
```

#### C) Builtins VPy
```python
# Gestión de instancias
anim_id = CREATE_ANIM("player_walk")  # Retorna ID de instancia (0-15)
DESTROY_ANIM(anim_id)

# Control manual
UPDATE_ANIM(anim_id, x, y)           # Actualiza posición, avanza frame
DRAW_ANIM(anim_id)                   # Dibuja frame actual
SET_ANIM_STATE(anim_id, "move_left") # Cambia estado
SET_ANIM_MIRROR(anim_id, 1)          # 0=normal, 1=X, 2=Y, 3=XY

# Control automático (solo playable=true)
UPDATE_ANIM_AUTO(anim_id, x, y)      # Lee input, cambia state, avanza frame
```

### 4. Asset Discovery
**Ubicación**: `core/src/main.rs` y `buildtools/vpy_codegen/src/m6809/assets.rs`

```rust
// En discover_assets()
let anims_dir = project_root.join("assets").join("animations");
if anims_dir.is_dir() {
    for entry in fs::read_dir(&anims_dir)? {
        let path = entry.path();
        if path.extension() == Some("vanim") {
            assets.push(AssetInfo {
                name: path.file_stem().to_string(),
                path: path.display().to_string(),
                asset_type: AssetType::Animation,
            });
        }
    }
}
```

### 5. IDE Integration

#### A) Panel de Animación (`AnimationPanel.tsx`)
- Lista de animaciones en `assets/animations/`
- Preview de frames (slider temporal)
- Editor visual de estados y transiciones
- Test de triggers (simular joystick/botones)

#### B) Formato de Guardado
- Crear `.vanim` desde panel
- Asociar vectores existentes como frames
- Definir states con triggers
- Marcar como playable/non-playable

### 6. Optimizaciones

#### A) Pooling de Instancias
```python
# Máximo 16 instancias simultáneas
MAX_ANIM_INSTANCES = 16
anim_pool = [0] * 16  # Pool de IDs disponibles
```

#### B) Frame Skipping
```python
# Si el juego va lento, skip frames automáticamente
if frame_time > 20:  # >20ms
    ANIM_SKIP_FRAME()
```

#### C) Culling
```python
# No actualizar animaciones off-screen
if abs(enemy_x[i]) > 150:
    PAUSE_ANIM(enemy_anim_ids[i])
```

### 7. Ejemplo Completo: Pang con Animaciones

#### `assets/animations/player_walk.vanim`
```json
{
  "version": "1.0",
  "name": "player_walk",
  "frames": [
    {"vectorName": "player_walk_1", "duration": 5, "intensity": 80},
    {"vectorName": "player_walk_2", "duration": 5, "intensity": 80},
    {"vectorName": "player_walk_3", "duration": 5, "intensity": 80},
    {"vectorName": "player_walk_4", "duration": 5, "intensity": 80},
    {"vectorName": "player_walk_5", "duration": 5, "intensity": 80}
  ],
  "states": {
    "move_right": {"mirror": false},
    "move_left": {"mirror": true}
  },
  "controller": {
    "playable": true,
    "type": "player"
  }
}
```

#### `assets/animations/bubble_float.vanim`
```json
{
  "version": "1.0",
  "name": "bubble_float",
  "frames": [
    {"vectorName": "bubble_large_1", "duration": 8, "intensity": 100},
    {"vectorName": "bubble_large_2", "duration": 8, "intensity": 100}
  ],
  "controller": {
    "playable": false,
    "type": "enemy"
  }
}
```

#### `main.vpy` (refactorizado)
```python
# Antes: 60 líneas de if/elif hardcodeado
# Después: 10 líneas con animaciones

player_anim = CREATE_ANIM("player_walk")
enemy_anims = [0] * MAX_ENEMIES

def spawn_enemy(index, size):
    if size == 4:
        enemy_anims[index] = CREATE_ANIM("bubble_huge")
    elif size == 3:
        enemy_anims[index] = CREATE_ANIM("bubble_large")
    # ...

def loop():
    WAIT_RECAL()
    
    # Player (automático con input)
    UPDATE_ANIM_AUTO(player_anim, player_x, player_y)
    DRAW_ANIM(player_anim)
    
    # Enemies (manual sin input)
    for i in range(MAX_ENEMIES):
        if enemy_active[i]:
            UPDATE_ANIM(enemy_anims[i], enemy_x[i], enemy_y[i])
            DRAW_ANIM(enemy_anims[i])
```

### 8. Testing Strategy

#### Unit Tests
- `animres.rs`: Parsing de .vanim
- Frame duration calculations
- State transitions

#### Integration Tests
- Multiple instances (16 simultáneas)
- Playable vs non-playable
- Memory usage (12 bytes * 16 = 192 bytes)

#### Real-world Test
- Pang con player + 8 enemies
- Verificar no hay lag
- Binary size impact

## Prioridades de Implementación

1. **Phase 1**: Formato .vanim + animres.rs (solo frames básicos)
2. **Phase 2**: Builtins CREATE/UPDATE/DRAW_ANIM
3. **Phase 3**: Sistema de instancias múltiples
4. **Phase 4**: States y transiciones
5. **Phase 5**: Controller integration (playable)
6. **Phase 6**: IDE AnimationPanel
7. **Phase 7**: Optimizaciones (pooling, culling)

## Notas de Diseño

### ¿Por qué no usar PLAY_ANIM()?
- Las animaciones se dibujan cada frame (no son fire-and-forget como música)
- Necesitamos control fino de posición (x, y) cada frame
- Por eso: UPDATE_ANIM() + DRAW_ANIM() separados

### ¿Por qué instancias en lugar de nombres?
- Múltiples enemigos pueden usar la misma animación "bubble_large"
- Cada uno necesita su propio frame counter
- IDs numéricos (0-15) son más eficientes que strings

### ¿Qué pasa con las colisiones?
- Las animaciones solo manejan VISUAL
- Colisiones siguen en código VPy (enemy_x[], enemy_y[], sizes)
- Animaciones son decorativas sobre la lógica de juego

## Referencias
- Sistema actual: `examples/pang/src/main.vpy` líneas 360-375
- Vectors existentes: `examples/pang/assets/vectors/player_walk_*.vec`
- Similar a: Aseprite animations, Unity Animator, Godot AnimationPlayer

---
**Status**: Phase 1 COMPLETE ✅  
**Fecha**: 2026-01-25
**Branch**: feature/pang-level-format (continuar aquí)

## Progress Log

### 2026-01-25 - Phase 1 Complete ✅
**Completed**:
- ✅ Created `core/src/animres.rs` (310 lines)
- ✅ Created `buildtools/vpy_codegen/src/animres.rs` (310 lines)
- ✅ Implemented `VAnimAnimation` struct with full .vanim v1.0 support
- ✅ Implemented `load()` method with JSON parsing + validation
- ✅ Implemented `compile_to_asm()` and `compile_to_asm_with_name()` methods
- ✅ Generated ASM data structures (frames table, state table, controller flags)
- ✅ Added unit tests: `test_parse_simple_animation` and `test_compile_to_asm` ✅
- ✅ Registered module in `lib.rs` for both compilers
- ✅ Created example file `examples/pang/assets/animations/player_walk.vanim`

**Generated ASM Structure**:
```asm
_PLAYER_WALK_ANIM:
    FCB 5           ; num_frames
    FCB 2           ; num_states
    FCB $01         ; controller_flags (mirror_on_left)
    FDB _PLAYER_WALK_FRAMES
    FDB _PLAYER_WALK_STATES

_PLAYER_WALK_FRAMES:
    ; Frame 0-4 (10 bytes each)
    ; vector_ptr, duration, intensity, offset_x, offset_y, mirror

_PLAYER_WALK_STATES:
    ; State: walking (5 frames, loop=1)
    ; State: idle (1 frame, loop=1)
```

**Next Phase**: Phase 2 - Builtin Functions (CREATE_ANIM, UPDATE_ANIM, DRAW_ANIM)

