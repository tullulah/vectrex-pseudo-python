# Playground Level Designer - Implementation Plan

## Objetivo
Convertir el Playground visual en un diseñador de niveles completo que permita:
1. Diseñar niveles visualmente en el IDE
2. Guardar niveles como archivos .vplay
3. Cargar y usar esos niveles en juegos VPy compilados

## Estructura Actual de .vplay

```json
{
  "version": "1.0",
  "name": "test",
  "objects": [
    {
      "id": "obj_123",
      "type": "player|enemy|obstacle|collectible|background",
      "vectorName": "bubble_large",
      "x": 0,
      "y": 50,
      "scale": 1.0,
      "rotation": 0,
      "intensity": 127,
      "physicsEnabled": true,
      "physicsType": "gravity|bounce|projectile|static",
      "gravity": 1,
      "bounceDamping": 0.85,
      "velocity": { "x": 0, "y": 0 },
      "radius": 20,
      "properties": {
        "health": 100,
        "damage": 10,
        "score": 50,
        "custom_data": "..."
      }
    }
  ]
}
```

## Plan de Implementación

### Fase 1: Extender Formato .vplay (1-2 días)
**Objetivo**: Añadir metadata y estructura para diseño de niveles

#### 1.1 Metadata de Nivel
```json
{
  "version": "2.0",
  "type": "level",
  "metadata": {
    "name": "Level 1 - Mount Fuji",
    "author": "Player Name",
    "difficulty": "easy|medium|hard",
    "timeLimit": 180,
    "targetScore": 5000,
    "description": "First level in Japan"
  },
  "worldBounds": {
    "xMin": -96,
    "xMax": 95,
    "yMin": -128,
    "yMax": 127
  },
  "layers": {
    "background": [],  // Objetos de fondo (no collision)
    "gameplay": [],    // Objetos jugables (enemies, collectibles)
    "foreground": []   // Efectos visuales encima
  },
  "spawnPoints": {
    "player": { "x": 0, "y": -100 },
    "enemies": [
      { "x": -50, "y": 50, "type": "bubble_large", "delay": 0 },
      { "x": 50, "y": 50, "type": "bubble_medium", "delay": 2.0 }
    ]
  }
}
```

#### 1.2 Tipos de Objetos Estandarizados
- **Player Start**: Punto de inicio del jugador
- **Enemy Spawn**: Spawn de enemigos con delay opcional
- **Collectible**: Items coleccionables (puntos, power-ups)
- **Obstacle**: Plataformas, paredes (collision estática)
- **Background**: Decoración visual sin collision
- **Trigger Zone**: Áreas que activan eventos (ej: siguiente oleada)

#### 1.3 Propiedades Físicas Mejoradas
```json
{
  "collision": {
    "enabled": true,
    "layer": "player|enemy|obstacle|projectile",
    "radius": 20,
    "shape": "circle|rect",
    "bounceWalls": true,
    "destroyOnCollision": false
  },
  "ai": {
    "type": "static|patrol|chase|flee|custom",
    "speed": 10,
    "waypoints": [{"x": 0, "y": 0}, {"x": 50, "y": 0}]
  }
}
```

### Fase 2: Compilador VPy - Embeber Niveles (2-3 días)
**Objetivo**: Permitir que el compilador embeba datos de niveles en ROM

#### 2.1 Asset Discovery para .vplay
Similar a como se hace con .vec y .vmus:
```rust
// core/src/main.rs - discover_assets()
fn discover_level_assets(source_path: &Path) -> Vec<LevelAssetInfo> {
    // Buscar assets/levels/*.vplay
    // Parsear JSON
    // Retornar metadata + objetos
}
```

#### 2.2 Compilación de Niveles a ROM
```rust
// core/src/levelres.rs (nuevo módulo)
pub struct LevelResource {
    name: String,
    objects: Vec<LevelObject>,
    metadata: LevelMetadata,
}

impl LevelResource {
    pub fn compile_to_asm(&self) -> String {
        // Generar datos compactos en ROM
        // Similar a vectores pero con estructura de nivel
    }
}
```

**Ejemplo ASM generado**:
```asm
; Level data for "fuji_level1"
_LEVEL_FUJI_1_METADATA:
    FCB 180           ; Time limit (seconds)
    FDB 5000          ; Target score
    FCB 0             ; Difficulty (0=easy, 1=medium, 2=hard)

_LEVEL_FUJI_1_PLAYER_SPAWN:
    FCB 0, -100       ; Player start X, Y

_LEVEL_FUJI_1_ENEMIES:
    FCB 5             ; Number of enemies
    ; Enemy 0: type, x, y, delay, physics_flags
    FCB 0             ; Type index (bubble_large = 0)
    FCB -50, 50       ; Position X, Y
    FCB 0             ; Spawn delay (frames)
    FCB $01           ; Physics flags (gravity enabled)
    ; ... más enemigos

_LEVEL_FUJI_1_OBJECTS:
    FCB 3             ; Number of static objects
    ; Object 0: vector_index, x, y, scale, intensity
    FDB _MOUNTAIN_VECTORS  ; Pointer to vector
    FCB 0, -80        ; Position
    FCB 127, 127      ; Scale, Intensity
```

#### 2.3 Compresión de Datos (Opcional)
- Run-Length Encoding para objetos repetidos
- Delta encoding para posiciones cercanas
- Lookup tables para tipos comunes

#### 2.4 Sistema de Físicas en VPy (PENDIENTE)
**IMPORTANTE**: Actualmente VPy NO tiene físicas implementadas en el compilador.
Al integrar niveles necesitaremos implementar:

**Físicas Básicas Necesarias**:
- ✅ Movimiento linear (ya soportado con variables x, y)
- ❌ **Gravedad**: Aplicar aceleración vertical constante
- ❌ **Colisiones**: Detección círculo-círculo, círculo-rectángulo
- ❌ **Bounce**: Invertir velocidad al colisionar con bounce damping
- ❌ **Friction**: Reducir velocidad gradualmente
- ❌ **Velocidad máxima**: Clamp de velocidades

**Opciones de Implementación**:

**Opción A: Helpers VPy** (recomendado para MVP)
```python
# Usuario implementa física en su código usando helpers
def update_physics(obj):
    # Gravedad
    obj.vel_y = obj.vel_y - 1
    
    # Aplicar velocidad
    obj.x = obj.x + obj.vel_x
    obj.y = obj.y + obj.vel_y
    
    # Colisión con suelo
    if obj.y < -100:
        obj.y = -100
        obj.vel_y = -obj.vel_y * 85 / 100  # Bounce damping
```

**Opción B: Builtins de Física** (más complejo, mejor performance)
```python
# Builtins en compilador que generan código optimizado
APPLY_GRAVITY(obj_id, gravity_strength)
CHECK_COLLISION(obj1_id, obj2_id)  # Retorna 0 o 1
APPLY_BOUNCE(obj_id, damping)
```

**Opción C: Sistema de Física Automático** (ideal, más trabajo)
```python
# Compilador genera loop de física automáticamente
# Usuario solo marca objetos con flags
obj.physics_enabled = 1
obj.gravity = 1
obj.bounce = 85
# Compilador inyecta UPDATE_PHYSICS() en loop
```

**Decisión para MVP**: Opción A (helpers en VPy) + documentación de patrones comunes.
Futuro: Migrar a Opción B/C según performance needs.

#### 2.5 Rotaciones (NO SOPORTADO AÚN)
**LIMITACIÓN CRÍTICA**: El compilador VPy actualmente NO soporta rotaciones de vectores.

**Estado Actual**:
- ✅ DRAW_VECTOR() - dibuja vector en orientación original
- ✅ DRAW_VECTOR_EX(name, x, y, mirror) - soporta espejo X/Y/XY
- ❌ **DRAW_VECTOR_ROTATED() - NO EXISTE**

**Implicaciones para Niveles**:
- Objetos en .vplay tienen campo `rotation` pero se ignora en compilación
- Solo se pueden usar orientaciones fijas (0°, 90°, 180°, 270° via mirror)
- Rotación arbitraria requiere pre-rotar vectores en editor

**Soluciones Temporales**:
1. **Pre-rotación**: Crear múltiples versiones del mismo vector
   - `ship_0.vec`, `ship_45.vec`, `ship_90.vec`, etc.
   - Playground genera versiones automáticamente

2. **Mirror combinations**: Usar espejos para 4 orientaciones básicas
   ```python
   # 0° = normal, 90° = mirror_y, 180° = mirror_xy, 270° = mirror_x
   if rotation == 0:   DRAW_VECTOR_EX("ship", x, y, 0)
   if rotation == 90:  DRAW_VECTOR_EX("ship", x, y, 2)
   if rotation == 180: DRAW_VECTOR_EX("ship", x, y, 3)
   if rotation == 270: DRAW_VECTOR_EX("ship", x, y, 1)
   ```

3. **Limitación de diseño**: Niveles solo usan objetos sin rotación
   - Válido para Pang (burbujas son círculos)
   - Limitante para shooters o platformers

**Implementación Futura de Rotaciones** (fuera de scope de MVP):
```python
# API deseada
DRAW_VECTOR_ROTATED("ship", x, y, angle)  # angle en grados 0-359

# Implementación en BIOS (costosa):
# - Rotar cada punto del vector
# - Usar tablas de sin/cos
# - ~100-200 cycles por vector

# Alternativa: lookup table de vectores pre-rotados
# - Generar 36 versiones (cada 10°) en compilación
# - DRAW_VECTOR_ROTATED busca versión más cercana
# - Trade-off: ROM space vs CPU time
```

**Decisión para MVP**: 
- Niveles ignoran campo `rotation` (siempre 0°)
- Playground muestra rotación visualmente pero no se exporta
- Documentar limitación en tutorial
- Implementar rotaciones en fase posterior (Fase 6+)

### Fase 3: API VPy para Cargar Niveles (2-3 días)
**Objetivo**: Builtins en VPy para acceder a datos de nivel en runtime

#### 3.1 Nuevas Funciones Builtin
```python
# Cargar nivel en memoria (parsear ROM a structs)
LOAD_LEVEL("fuji_level1")

# Obtener metadata
time_limit = GET_LEVEL_TIME()
target_score = GET_LEVEL_TARGET_SCORE()

# Obtener spawn del jugador
player_x, player_y = GET_PLAYER_SPAWN()

# Iterar enemigos a spawnear
enemy_count = GET_ENEMY_COUNT()
for i in range(enemy_count):
    enemy_type, x, y, delay = GET_ENEMY_DATA(i)
    # Crear enemigo en juego

# Obtener objetos de background
obj_count = GET_LEVEL_OBJECT_COUNT()
for i in range(obj_count):
    vector_name, x, y, scale = GET_LEVEL_OBJECT(i)
    DRAW_VECTOR_EX(vector_name, x, y, 0)  # Dibujar decoración
```

#### 3.2 Implementación en Compiler
```rust
// core/src/backend/m6809/builtins.rs

// LOAD_LEVEL: Cargar punteros a estructuras de nivel
fn emit_load_level(out: &mut String, level_name: &str, opts: &CodegenOptions) {
    // Verificar que nivel existe en opts.level_assets
    // LDX #_LEVEL_NAME_METADATA
    // STX CURRENT_LEVEL_PTR
}

// GET_ENEMY_COUNT: Leer byte de cantidad
fn emit_get_enemy_count(out: &mut String) {
    // LDX CURRENT_LEVEL_PTR
    // LDB LEVEL_ENEMY_OFFSET,X  ; Offset fijo conocido
    // STB RESULT
}

// GET_ENEMY_DATA: Acceder a array de enemigos
fn emit_get_enemy_data(out: &mut String, index: u16) {
    // Calcular offset: base + (index * ENEMY_STRUCT_SIZE)
    // LDX CURRENT_LEVEL_PTR
    // LDD #index
    // Multiplicar por tamaño de struct
    // Cargar datos en RESULT (tipo, x, y, delay)
}
```

### Fase 4: UI del Playground - Herramientas de Diseño (3-4 días)
**Objetivo**: Mejorar UX del playground para diseño de niveles

#### 4.1 Modos de Edición
- **Object Mode**: Arrastrar/colocar objetos
- **Physics Mode**: Configurar física y colisiones
- **Path Mode**: Dibujar rutas de patrullaje
- **Trigger Mode**: Definir zonas de activación

#### 4.2 Paleta de Objetos
```tsx
// Categorías de objetos disponibles
interface ObjectPalette {
  enemies: {
    bubble_large: VectorInfo,
    bubble_medium: VectorInfo,
    // ...
  },
  obstacles: {
    platform: VectorInfo,
    wall: VectorInfo,
  },
  collectibles: {
    coin: VectorInfo,
    powerup: VectorInfo,
  }
}
```

#### 4.3 Panel de Propiedades
- Editor visual para propiedades de objeto seleccionado
- Presets comunes (enemy_slow, enemy_fast, static_wall)
- Preview de comportamiento físico

#### 4.4 Grid y Snap
- Grilla visual opcional (8x8, 16x16, 32x32)
- Snap to grid para posicionamiento preciso
- Rulers con coordenadas Vectrex

#### 4.5 Layers Panel
- Toggle visibilidad de capas (background/gameplay/foreground)
- Lock layers para evitar editar accidentalmente
- Reordenar objetos dentro de capa (z-order)

#### 4.6 Test Mode in Playground
- Botón "Test Level" que simula física localmente
- No compila, solo preview interactivo
- Useful para iterar rápidamente

### Fase 5: Ejemplo Completo - Pang Levels (2 días)
**Objetivo**: Demostrar sistema completo con niveles reales de Pang

#### 5.1 Convertir Niveles Existentes
Tomar los 17 niveles de Pang y crearlos como .vplay:
- `assets/levels/01_mount_fuji.vplay`
- `assets/levels/02_mount_keirin.vplay`
- etc.

#### 5.2 Refactorizar Código Pang
```python
# Antes: Hardcoded
const location_names = ["MOUNT FUJI - JAPAN", ...]
current_location = 0

# Después: Level-based
current_level = 0
LOAD_LEVEL("01_mount_fuji")

def init_level():
    # Cargar spawn del jugador
    player_x, player_y = GET_PLAYER_SPAWN()
    
    # Crear enemigos desde nivel
    enemy_count = GET_ENEMY_COUNT()
    for i in range(enemy_count):
        enemy_type, x, y, delay = GET_ENEMY_DATA(i)
        spawn_enemy(enemy_type, x, y, delay)
    
    # Dibujar background desde nivel
    draw_level_background()

def loop():
    WAIT_RECAL()
    update_game()
    
    if level_complete():
        current_level += 1
        LOAD_LEVEL(level_names[current_level])
        init_level()
```

#### 5.3 Sistema de Progresión
- Guardar nivel actual en EEPROM (si disponible)
- Unlock de niveles conforme se completan
- High scores por nivel

### Fase 6: Optimizaciones y Polish (2-3 días)

#### 6.1 Validación de Niveles
- Compilador valida que todos los vectores existan
- Advertencias si objetos están fuera de bounds
- Error si nivel no tiene player spawn

#### 6.2 Level Packing
- Múltiples niveles en un solo .vplay (level pack)
- Metadatos de campaña (nombre, descripción, orden)

#### 6.3 Export/Import
- Exportar nivel a JSON legible
- Importar niveles de otros proyectos
- Plantillas de niveles comunes

#### 6.4 Documentation
- Tutorial de cómo crear primer nivel
- Ejemplos de diferentes tipos de juego:
  - Platformer level (obstacles + enemies)
  - Bullet hell (spawn patterns)
  - Puzzle (static objects + triggers)

## Timeline Estimado

| Fase | Duración | Prioridad | Notas |
|------|----------|-----------|-------|
| Fase 1: Extender .vplay | 1-2 días | **CRÍTICO** | Formato v2.0, schema |
| Fase 2: Compilador | 2-3 días | **CRÍTICO** | Sin físicas/rotaciones |
| Fase 2.5: Físicas en VPy | 3-4 días | **ALTO** | Gravity, collision, bounce |
| Fase 3: API VPy | 2-3 días | **CRÍTICO** | Load/read levels |
| Fase 4: UI Playground | 3-4 días | **MEDIO** | Layers, grid, test |
| Fase 5: Ejemplo Pang | 2 días | **ALTO** | Demo funcional |
| Fase 6: Polish | 2-3 días | **BAJO** | Optimizaciones |
| Fase 7: Rotaciones | 4-5 días | **FUTURO** | Fuera de MVP |
| **MVP TOTAL** | **15-20 días** | | Sin rotaciones |
| **FULL TOTAL** | **19-25 días** | | Con rotaciones |

## Decisiones de Diseño Clave

### 1. ¿Embedded vs External Levels?
**Decisión**: Embedded en ROM (como .vec y .vmus)
- ✅ No necesita filesystem en runtime
- ✅ Faster loading (no I/O)
- ✅ Funciona en hardware real Vectrex
- ❌ No permite DLC/modding sin recompilar

### 2. ¿Formato Binario Compacto vs JSON?
**Decisión**: JSON en disco, binario en ROM
- .vplay files son JSON legible (fácil debug, versionable en Git)
- Compilador convierte a formato binario compacto para ROM
- Best of both worlds

### 3. ¿API Imperativa vs Declarativa?
**Decisión**: Híbrida
```python
# Imperativa (más control)
for i in range(GET_ENEMY_COUNT()):
    enemy = GET_ENEMY_DATA(i)
    spawn_enemy(enemy)

# Declarativa (más simple)
SPAWN_LEVEL_ENEMIES()  # Hace todo automáticamente
```

### 4. ¿Scripting en Niveles?
**Decisión**: No en MVP, posible en futuro
- Por ahora: datos estáticos (posiciones, propiedades)
- Futuro: mini-scripting para triggers complejos
  ```json
  "triggers": [{
    "type": "on_all_enemies_dead",
    "action": "spawn_wave_2"
  }]
  ```

## Riesgos y Mitigaciones

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| Niveles muy grandes no caben en ROM | Media | Alto | Compresión, límites en editor |
| Performance de rendering con muchos objetos | Media | Medio | Culling, limit objetos visibles |
| Formato .vplay cambia mucho | Alta | Bajo | Versionado, migraciones automáticas |
| API VPy muy verbosa | Baja | Medio | Helpers de alto nivel, macros |
| **No hay sistema de físicas en VPy** | **Alta** | **Alto** | Opción A: helpers en código usuario, documentar patrones |
| **Rotaciones no soportadas** | **Alta** | **Medio** | Ignorar rotación en MVP, pre-rotar vectores, mirrors |

## Próximos Pasos Inmediatos

1. ✅ **Crear rama** `feature/playground-level-designer`
2. 📝 **Definir schema JSON v2.0** para .vplay (con validación)
3. 🔨 **Implementar `levelres.rs`** (parser JSON → ASM)
4. 🧪 **Proof of concept**: 
   - Crear nivel simple `test_level.vplay`
   - Compilar con embebido
   - Leer datos en VPy con `LOAD_LEVEL()`
5. 🎮 **Demo funcional**: Pang carga nivel 1 desde .vplay

## Referencias

- Vector asset system: `core/src/vecres.rs`
- Music asset system: `core/src/musres.rs`
- Asset discovery: `core/src/main.rs` lines 500-550
- Builtin functions: `core/src/backend/m6809/builtins.rs`
- Playground UI: `ide/frontend/src/components/panels/PlaygroundPanel.tsx`

---
**Status**: Planning Phase
**Branch**: `feature/playground-level-designer`
**Updated**: 2026-01-03
