# VPy Project System - TODO

**Fecha inicio**: 7 de diciembre de 2025  
**Estado**: En planificación

---

## Índice

1. [Fase 1: Sistema de Proyectos](#fase-1-sistema-de-proyectos)
2. [Fase 2: Multi-archivo e Imports](#fase-2-multi-archivo-e-imports)
3. [Fase 3: Sistema de Librerías](#fase-3-sistema-de-librerías)
4. [Fase 4: Recursos Vectoriales](#fase-4-recursos-vectoriales)
5. [Fase 5: Editor de Vectores Integrado](#fase-5-editor-de-vectores-integrado)
6. [Fase 6: Interoperabilidad C/C++](#fase-6-interoperabilidad-cc)
7. [Fase 7: Toolchain Completo](#fase-7-toolchain-completo)

---

## Fase 1: Sistema de Proyectos

### 1.1 Formato de Proyecto (`.vpyproj`)

- [x] **1.1.1** Definir esquema del archivo `.vpyproj` (TOML)
- [x] **1.1.2** Crear parser de proyecto en Rust (`core/src/project/`)
- [x] **1.1.3** Validación de esquema con errores descriptivos
- [x] **1.1.4** Tests unitarios del parser

**Formato propuesto:**
```toml
[project]
name = "MiJuego"
version = "1.0.0"
author = "Nombre"
entry = "src/main.vpy"

[build]
output = "build/game.bin"
optimization = 2                    # 0-3

[sources]
vpy = ["src/**/*.vpy"]
c = ["src/**/*.c"]                  # Opcional, Fase 6
asm = ["src/**/*.asm"]              # Opcional

[resources]
vectors = ["assets/**/*.vec"]
data = ["assets/**/*.dat"]

[dependencies]
# Librerías externas (Fase 3)
vectrex-stdlib = "1.0"
my-lib = { path = "../my-library" }
```

### 1.2 Estructura de Directorios

✅ **Implementado en `create_project()`** - Crea automáticamente:

```
mi-juego/
├── game.vpyproj              # Archivo de proyecto
├── src/
│   ├── main.vpy              # Punto de entrada
│   ├── player.vpy            # Módulos
│   ├── enemies.vpy
│   └── utils/
│       └── math.vpy
├── assets/
│   ├── sprites/
│   │   ├── ship.vec
│   │   └── asteroid.vec
│   └── data/
│       └── levels.dat
├── build/                    # Generado
│   ├── game.asm
│   ├── game.o
│   ├── game.bin
│   └── game.pdb
└── .gitignore                # Archivos a ignorar
```

### 1.3 IDE: Gestión de Proyectos

- [x] **1.3.1** Comando "Nuevo Proyecto" con wizard
  - [x] Nombre del proyecto
  - [x] Ubicación
  - [ ] Template (vacío, ejemplo básico, juego completo)
- [x] **1.3.2** Comando "Abrir Proyecto" (reemplaza "Abrir Workspace")
- [x] **1.3.3** Panel de explorador de proyecto (reemplaza explorador de archivos)
  - [x] Muestra icono 📦 para proyectos .vpyproj
  - [x] Muestra nombre y versión del proyecto
  - [x] Etiqueta "VPy Project" distintiva
- [x] **1.3.4** Guardar estado del proyecto (archivos abiertos, layout)
  - [x] Guarda archivos abiertos al cerrar proyecto
  - [x] Guarda archivos abiertos al cerrar ventana
  - [x] Restaura archivos abiertos al abrir proyecto
  - [x] Restaura archivo activo
- [x] **1.3.5** Proyectos recientes en pantalla de inicio
- [x] **1.3.6** Indicador visual de proyecto activo en barra de título

### 1.4 Build desde Proyecto

- [x] **1.4.1** Comando `vpybuild` que lee `.vpyproj`
  - [x] Compilar usando entry point del proyecto
  - [x] Generar binario en ruta especificada en `build.output`
- [x] **1.4.2** Resolver rutas relativas al proyecto
- [x] **1.4.3** Crear directorio `build/` automáticamente
- [x] **1.4.4** Generar `.bin` y `.pdb` en `build/`
- [ ] **1.4.5** Build incremental (solo recompilar lo modificado) - futuro

---

## Fase 2: Multi-archivo e Imports

### 2.1 Sintaxis de Imports

```python
# Imports absolutos (desde raíz del proyecto)
from player import Player, move_player
from utils.math import clamp, lerp

# Imports relativos (desde archivo actual)
from .sprites import SHIP
from ..common import constants

# Import de librería externa
from vectrex_stdlib.graphics import draw_circle
```

### 2.2 Compilador: Resolución de Imports

- [x] **2.2.1** Parser: Soporte sintaxis `from X import Y`
- [x] **2.2.2** Parser: Soporte sintaxis `import X`
- [x] **2.2.3** Parser: Soporte `export` para marcar símbolos públicos
- [x] **2.2.4** Resolver imports relativos (`.` y `..`)
- [x] **2.2.5** Resolver imports absolutos (desde `src/`)
- [ ] **2.2.6** Resolver imports de librerías (desde `dependencies`) - Fase 3
- [x] **2.2.7** Detectar ciclos de importación
- [x] **2.2.8** Cachear módulos ya parseados

### 2.3 Compilador: Unificación de AST

- [x] **2.3.1** Cargar todos los módulos referenciados
- [x] **2.3.2** Resolver símbolos entre módulos
- [x] **2.3.3** Detectar conflictos de nombres (via prefijos automáticos)
- [x] **2.3.4** Generar AST unificado
- [ ] **2.3.5** Optimizar: eliminar código no usado (tree shaking) - Opcional

### 2.4 Compilador: Generación de Código

- [x] **2.4.1** Generar ASM único desde AST unificado
- [x] **2.4.2** Prefijos de módulo para evitar colisiones (`utils_clamp`)
- [ ] **2.4.3** Tabla de símbolos exportados para debug
- [x] **2.4.4** Actualizar `.pdb` con información multi-archivo

### 2.5 IDE: Soporte Multi-archivo

- [x] **2.5.1** Navegación "Ir a definición" entre archivos
- [x] **2.5.2** Autocompletado con símbolos de otros módulos
- [x] **2.5.3** Errores de import en tiempo real
- [x] **2.5.4** Renombrar símbolo en todo el proyecto

---

## Fase 3: Sistema de Librerías

### 3.1 Formato de Librería (`.vpylib`)

```toml
[library]
name = "vectrex-stdlib"
version = "1.0.0"
author = "VPy Team"
description = "Standard library for Vectrex development"

[exports]
# Módulos públicos
modules = ["graphics", "input", "sound", "math"]
```

**Estructura de librería:**
```
vectrex-stdlib/
├── library.vpylib
├── src/
│   ├── graphics.vpy
│   ├── input.vpy
│   ├── sound.vpy
│   └── math.vpy
└── README.md
```

### 3.2 Gestión de Librerías

- [x] **3.2.1** Crear librería desde proyecto existente (`vectrexc lib-new`)
- [ ] **3.2.2** Publicar librería (¿registro central o solo local?) - Fase futura
- [x] **3.2.3** Instalar librería por path local (en resolver)
- [ ] **3.2.4** Versionar librerías (semver) - Fase futura
- [ ] **3.2.5** Resolver dependencias transitivas - Fase futura

### 3.3 Librerías Estándar Incluidas

- [ ] **3.3.1** `vectrex-stdlib`: Funciones básicas Vectrex - Fase futura
  - [ ] `graphics`: draw_line, draw_circle, draw_text, etc.
  - [ ] `input`: read_joystick, read_buttons
  - [ ] `sound`: play_note, play_sfx
  - [ ] `math`: sin, cos, sqrt, random
- [ ] **3.3.2** Documentación de la stdlib - Fase futura

### 3.4 IDE: Gestión de Dependencias

- [ ] **3.4.1** Panel de dependencias del proyecto - Fase futura
- [ ] **3.4.2** Añadir/eliminar dependencias - Fase futura
- [ ] **3.4.3** Autocompletado de librerías disponibles - Fase futura

---

## Fase 4: Recursos Vectoriales

### 4.1 Formato `.vec` (JSON)

```json
{
  "version": "1.0",
  "name": "player_ship",
  "author": "Nombre",
  "created": "2025-12-07",
  
  "canvas": {
    "width": 256,
    "height": 256,
    "origin": "center"
  },
  
  "layers": [
    {
      "name": "body",
      "visible": true,
      "paths": [
        {
          "name": "hull",
          "intensity": 127,
          "closed": true,
          "points": [
            {"x": 0, "y": 20},
            {"x": -10, "y": -10},
            {"x": 0, "y": -5},
            {"x": 10, "y": -10}
          ]
        }
      ]
    },
    {
      "name": "detail",
      "visible": true,
      "paths": [
        {
          "name": "cockpit",
          "intensity": 80,
          "closed": true,
          "points": [
            {"x": -3, "y": 5},
            {"x": 3, "y": 5},
            {"x": 3, "y": 10},
            {"x": -3, "y": 10}
          ]
        }
      ]
    }
  ],
  
  "animations": [
    {
      "name": "thrust",
      "frames": [
        {"layer": "thrust_1", "duration": 100},
        {"layer": "thrust_2", "duration": 100}
      ]
    }
  ],
  
  "metadata": {
    "hitbox": {"x": -10, "y": -10, "w": 20, "h": 30},
    "origin": {"x": 0, "y": 0},
    "tags": ["player", "ship"]
  }
}
```

### 4.2 Compilador de Recursos

- [x] **4.2.1** Herramienta `vec2asm`: Convierte `.vec` a ASM/datos
- [ ] **4.2.2** Optimizar puntos (eliminar redundantes) - Fase futura
- [x] **4.2.3** Generar formato compatible con `DRAW_VL` de Vectrex
- [ ] **4.2.4** Soporte para múltiples formatos de salida - Fase futura
- [ ] **4.2.5** Integrar en pipeline de build - Fase futura

**Ejemplo de salida ASM:**
```asm
; Generated from ship.vec
_SHIP_VECTORS:
    DB 4                    ; num_points
    DB 127                  ; intensity
    DB 0, 20                ; point 0
    DB -10, -10             ; point 1
    DB 0, -5                ; point 2
    DB 10, -10              ; point 3
    DB $01                  ; end marker (closed)
```

### 4.3 Uso en VPy

```python
# Los recursos se importan como constantes
from assets.sprites import SHIP_VECTORS, ASTEROID_VECTORS

def loop():
    # Dibujar sprite en posición
    draw_vectorlist(player_x, player_y, SHIP_VECTORS)
```

- [ ] **4.3.1** Sintaxis para importar recursos
- [ ] **4.3.2** Función `draw_vectorlist()` en stdlib
- [ ] **4.3.3** Soporte para escala y rotación

---

## Fase 5: Editor de Vectores Integrado

### 5.1 Arquitectura del Editor

```
┌─────────────────────────────────────────────────────────────┐
│                    VPy IDE - Vector Editor                   │
├──────────┬──────────────────────────────────┬───────────────┤
│          │                                  │               │
│  Layers  │         Canvas (SVG/Canvas)      │   Properties  │
│  Panel   │                                  │   Panel       │
│          │     ┌─────────────────────┐      │               │
│ [+] body │     │                     │      │ Name: hull    │
│  └ hull  │     │    ╱╲               │      │ Intensity: 127│
│  └ wing  │     │   ╱  ╲              │      │ Closed: ✓     │
│ [ ] fx   │     │  ╱    ╲             │      │ Points: 4     │
│          │     │ ╱──────╲            │      │               │
│          │     │                     │      │ [Transform]   │
│          │     └─────────────────────┘      │ X: 0  Y: 0    │
│          │                                  │ Scale: 1.0    │
│          │  [Tools: Select|Draw|Edit|Pan]   │ Rotation: 0°  │
├──────────┴──────────────────────────────────┴───────────────┤
│ Timeline (for animations)                                    │
│ [▶] ═══════●═══════════════════════════════════════ 0:00    │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Canvas y Renderizado

- [x] **5.2.1** Canvas con renderizado estilo Vectrex (líneas con glow)
- [x] **5.2.2** Grid configurable (snap to grid)
- [x] **5.2.3** Zoom y pan (mouse wheel, drag)
- [x] **5.2.4** Ejes y origen visual
- [ ] **5.2.5** Preview en tamaño real Vectrex (330x410 px proporcional) - Fase futura
- [ ] **5.2.6** Modo "Vectrex authentic" (phosphor glow, scanlines) - Fase futura

### 5.3 Herramientas de Dibujo

- [x] **5.3.1** **Select**: Seleccionar paths/puntos
  - [x] Click para seleccionar
  - [ ] Shift+click para multi-selección
  - [ ] Arrastrar para mover
  - [ ] Handles para redimensionar
- [x] **5.3.2** **Draw Line**: Dibujar líneas punto a punto
  - [x] Click para añadir punto
  - [x] Double-click o Enter para terminar
  - [x] Escape para cancelar
- [ ] **5.3.3** **Draw Polygon**: Dibujar polígonos cerrados - Fase futura
- [ ] **5.3.4** **Draw Rectangle**: Dibujar rectángulos - Fase futura
- [ ] **5.3.5** **Edit Points**: Editar puntos individuales
  - [x] Mover puntos
  - [ ] Añadir punto en segmento
  - [ ] Eliminar punto
- [ ] **5.3.6** **Pan**: Mover vista (también con Space+drag)

### 5.4 Panel de Capas

- [x] **5.4.1** Lista de capas jerárquica
- [x] **5.4.2** Visibilidad por capa (ojo)
- [ ] **5.4.3** Bloquear capa (candado) - Fase futura
- [ ] **5.4.4** Reordenar capas (drag & drop) - Fase futura
- [ ] **5.4.5** Renombrar capas (double-click) - Fase futura
- [x] **5.4.6** Añadir/eliminar capas
- [ ] **5.4.7** Duplicar capa - Fase futura
- [ ] **5.4.8** Expandir/colapsar paths dentro de capa - Fase futura

### 5.4.x Background Image Layer (NUEVO)

- [x] **5.4.9** Cargar imagen de fondo para calcar
- [x] **5.4.10** Ajustar opacidad de imagen de fondo
- [x] **5.4.11** Toggle visibilidad de imagen de fondo
- [x] **5.4.12** Auto-detección de bordes (Sobel edge detection)
- [x] **5.4.13** Trazado automático de vectores desde imagen
- [x] **5.4.14** Simplificación de paths (Ramer-Douglas-Peucker)
- [x] **5.4.15** Configuración de umbrales de detección

### 5.5 Panel de Propiedades

- [ ] **5.5.1** Propiedades de path seleccionado
  - [ ] Nombre
  - [ ] Intensidad (0-127)
  - [ ] Cerrado/abierto
  - [ ] Lista de puntos editable
- [ ] **5.5.2** Propiedades de punto seleccionado
  - [ ] Coordenadas X, Y
- [ ] **5.5.3** Propiedades del sprite
  - [ ] Nombre
  - [ ] Canvas size
  - [ ] Origin point
  - [ ] Hitbox

### 5.6 Transformaciones

- [ ] **5.6.1** Mover (con input numérico)
- [ ] **5.6.2** Escalar (proporcional y libre)
- [ ] **5.6.3** Rotar (con snap a 15°, 45°, 90°)
- [ ] **5.6.4** Flip horizontal/vertical
- [ ] **5.6.5** Alinear objetos (izq, centro, der, arriba, medio, abajo)
- [ ] **5.6.6** Distribuir objetos uniformemente

### 5.7 Edición

- [ ] **5.7.1** Undo/Redo (Cmd+Z, Cmd+Shift+Z)
- [ ] **5.7.2** Copy/Paste paths
- [ ] **5.7.3** Duplicate (Cmd+D)
- [ ] **5.7.4** Delete (Backspace/Delete)
- [ ] **5.7.5** Select All (Cmd+A)
- [ ] **5.7.6** Deselect (Escape)

### 5.8 Atajos de Teclado

| Atajo | Acción |
|-------|--------|
| V | Select tool |
| P | Draw line/polygon |
| R | Draw rectangle |
| A | Edit points |
| Space+drag | Pan |
| Scroll | Zoom |
| Cmd+Z | Undo |
| Cmd+Shift+Z | Redo |
| Cmd+C | Copy |
| Cmd+V | Paste |
| Cmd+D | Duplicate |
| Delete | Delete selection |
| Cmd+A | Select all |
| Escape | Deselect / Cancel |
| Cmd+S | Save |
| G | Toggle grid |
| Cmd+0 | Zoom to fit |
| Cmd+1 | Zoom 100% |

### 5.9 Animaciones (Fase futura)

- [ ] **5.9.1** Timeline para animaciones frame-by-frame
- [ ] **5.9.2** Onion skinning (ver frames anteriores/siguientes)
- [ ] **5.9.3** Play/pause preview
- [ ] **5.9.4** Exportar frames como datos separados

### 5.10 Import/Export

- [ ] **5.10.1** Guardar como `.vec` (nativo)
- [ ] **5.10.2** Exportar a SVG (para uso externo)
- [ ] **5.10.3** Importar desde SVG (conversión básica)
- [ ] **5.10.4** Exportar a PNG (preview)
- [ ] **5.10.5** Exportar directo a ASM (para debug)

### 5.11 Integración con IDE

- [ ] **5.11.1** Abrir `.vec` en tab del editor (como Monaco para .vpy)
- [ ] **5.11.2** Auto-save
- [ ] **5.11.3** Preview en panel del emulador
- [ ] **5.11.4** Hot-reload: cambios en .vec se reflejan en emulador
- [ ] **5.11.5** Doble-click en recurso desde código → abrir en editor

---

## Fase 6: Interoperabilidad C/C++

### 6.1 Toolchain C

- [ ] **6.1.1** Investigar opciones: CMOC vs gcc6809
- [ ] **6.1.2** Integrar compilador C en build pipeline
- [ ] **6.1.3** Definir ABI de llamadas VPy ↔ C
- [ ] **6.1.4** Documentar convenciones (registros, stack)

### 6.2 Sintaxis VPy para C

```python
# Declarar función externa implementada en C
extern def c_multiply(a: int, b: int) -> int

# Declarar variable externa
extern c_counter: int

def loop():
    result = c_multiply(10, 20)
```

- [ ] **6.2.1** Parser: Soporte `extern def`
- [ ] **6.2.2** Parser: Soporte `extern` para variables
- [ ] **6.2.3** Generar referencias externas en ASM
- [ ] **6.2.4** Validar tipos en llamadas externas

### 6.3 Build con C

- [ ] **6.3.1** Compilar .c → .o
- [ ] **6.3.2** Compilar .vpy → .asm → .o
- [ ] **6.3.3** Linker para unir .o → .bin
- [ ] **6.3.4** Mapa de memoria para linker

---

## Fase 7: Toolchain Completo

### 7.1 Comandos CLI

| Comando | Descripción |
|---------|-------------|
| `vpy new <nombre>` | Crear nuevo proyecto |
| `vpy build` | Compilar proyecto |
| `vpy run` | Compilar y ejecutar en emulador |
| `vpy clean` | Limpiar directorio build |
| `vpy lib new <nombre>` | Crear nueva librería |
| `vpy lib add <path>` | Añadir dependencia |

- [x] **7.1.1** Implementar `vpy new` (como `vectrexc init`)
- [x] **7.1.2** Implementar `vpy build` (como `vectrexc build`)
- [ ] **7.1.3** Implementar `vpy run` - Fase futura
- [ ] **7.1.4** Implementar `vpy clean` - Fase futura
- [x] **7.1.5** Implementar `vpy lib new` (como `vectrexc lib-new`)
- [ ] **7.1.6** Implementar `vpy lib add` - Fase futura

### 7.2 IDE: Integración Completa

- [ ] **7.2.1** Menú Proyecto (Nuevo, Abrir, Cerrar, Recientes)
- [ ] **7.2.2** Menú Build (Build, Run, Clean, Rebuild)
- [ ] **7.2.3** Output panel con errores clickeables
- [ ] **7.2.4** Panel de proyecto con árbol de archivos
- [ ] **7.2.5** Configuración de proyecto en UI

---

## Priorización Sugerida

### MVP (Minimum Viable Product)
1. **Fase 1.1-1.4**: Proyecto básico `.vpyproj`
2. **Fase 2.1-2.4**: Multi-archivo con imports
3. **Fase 1.3**: IDE abre proyectos

### Versión 1.0
4. **Fase 4.1-4.3**: Recursos vectoriales básicos
5. **Fase 5.1-5.7**: Editor de vectores core
6. **Fase 3**: Sistema de librerías

### Versión 2.0
7. **Fase 5.8-5.11**: Editor de vectores completo
8. **Fase 6**: Interoperabilidad C
9. **Fase 7**: Toolchain CLI completo

---

## Notas de Implementación

### Decisiones Técnicas Pendientes

1. **Formato proyecto**: TOML (elegido por legibilidad)
2. **Linker**: ¿Desarrollar propio o usar existente (ld6809)?
3. **Editor vectores**: ¿Canvas HTML5 o SVG? (SVG probablemente mejor para manipulación)
4. **Registro de librerías**: ¿Centralizado online o solo local?

### Compatibilidad hacia atrás

- Los archivos `.vpy` sueltos seguirán funcionando sin proyecto
- El IDE detectará si hay `.vpyproj` en el directorio

---

*Última actualización: 7 de diciembre de 2025*
