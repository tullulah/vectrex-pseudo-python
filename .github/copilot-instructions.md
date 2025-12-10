# Copilot Project Instructions (Persistent Reminders)

These guidelines are critical for ongoing work in this repository. Keep them in mind for every future change.

## 0. Git Branch Strategy
- **RAMA PRINCIPAL**: `master` (NO `main`)
- Todos los merges y commits principales van a `master`
- Feature branches salen de `master` y vuelven a `master`
- NUNCA crear ramas `main` locales accidentalmente

## 0.1. PowerShell Usage
- Usuario usa Windows PowerShell v5.1 (NO PowerShell 7+).
- NUNCA usar `&&` para concatenar comandos - usar `;` en su lugar.
- Sintaxis correcta: `cd emulator; cargo build` (NO `cd emulator && cargo build`).
- PowerShell v5.1 no soporta `&&` como separador de comandos.

## 0.2. REGLA CRÍTICA: VERIFICACIÓN 1:1 OBLIGATORIA
**ANTES DE CREAR CUALQUIER ARCHIVO O API**:
1. **VERIFICAR EXISTENCIA**: Comprobar si existe en `vectrexy/libs/emulator/src/` y `vectrexy/libs/emulator/include/emulator/`
2. **LEER CÓDIGO ORIGINAL**: Examinar el .cpp/.h correspondiente LÍNEA POR LÍNEA
3. **NO ASUMIR NADA**: No inventar APIs, estructuras, o patrones sin verificar
4. **DOCUMENTAR ORIGEN**: Cada función/struct debe tener comentario "// C++ Original:" con código fuente
5. **SI NO EXISTE = NO CREAR**: Si un archivo no existe en Vectrexy, NO crearlo sin discusión explícita

### Ejemplos de INVENTOS PROHIBIDOS detectados:
- ❌ Módulo `devices/` (no existe en Vectrexy - dispositivos están directos en src/)
- ❌ `Ram::new(size)` - En Vectrexy es template fijo 1024 bytes
- ❌ `BiosRom::new(data)` - En Vectrexy es `LoadBiosRom(const char* file)`  
- ❌ `MemoryMap` como enums - En Vectrexy es namespace con struct `Mapping`
- ❌ Tests sintéticos sin verificar APIs reales

### Proceso Obligatorio:
1. `ls vectrexy/libs/emulator/src/` 
2. `cat ArchiveCorrespondiente.cpp` 
3. `cat ArchiveCorrespondiente.h`
4. Implementar EXACTAMENTE lo que dice el código original
5. NUNCA implementar tests/APIs hasta verificar paso 1-4

## 1. BIOS Usage
- Nunca generar BIOS sintética en tests ni código de ejemplo.
- Rutas válidas (RELATIVAS al workspace root, autocontenidas):
	- Primaria (assets): `ide/frontend/src/assets/bios.bin`
	- Legacy (dist empaquetado actual): `ide/frontend/dist/bios.bin`
	(Si divergen, actualizar ambas o unificar mediante script de build.)
- Si se necesita ruta en WASM/frontend, exponer una única función helper (pending) o documentar claramente.
- **CRÍTICO**: NUNCA usar rutas absolutas (C:\Users\...) ni fuera del workspace (Desktop, HOME). Proyecto debe ser autocontenido.
- **bios.bin YA ESTÁ VERSIONADO en git** - No necesita backup manual. Al clonar el repositorio, el archivo ya está incluido.

## 2. Call Stack / BIOS Tracing
- Registrar llamadas BIOS reales via `record_bios_call` únicamente en JSR/BSR hacia >= 0xF000.
- Evitar falsos positivos: no fabricar llamadas manualmente salvo hooks explícitos.
- Próximo paso pendiente: mapear direcciones desconocidas como 0xF18B a etiquetas reales revisando `bios.asm` y actualizar `record_bios_call`.
- Añadir export WASM: `bios_calls_json()` (pendiente: TODO id 13).

## 2.1. VPy Language Compilation Architecture (2025-10-01)

### 2.1.1 Subroutine-Based Code Generation (BREAKTHROUGH)
- **ESTADO ACTUAL**: FUNCIONANDO - Arquitectura de subrutinas implementada exitosamente
- **PROBLEMA RESUELTO**: BRA overflow en programas grandes eliminado completamente
- **ARQUITECTURA**:
  ```asm
  main:
      JSR Wait_Recal
      LDA #$80
      STA VIA_t1_cnt_lo
      JSR LOOP_BODY    ; ← Llamada a subrutina (sin límites de distancia)
      BRA main

  LOOP_BODY:           ; ← Código del loop() en subrutina separada
      [código loop...]
      RTS              ; ← Retorno a main
  ```

### 2.1.2 Beneficios Técnicos Implementados
1. **✅ ELIMINA CÓDIGO DUPLICADO**: Una sola copia del loop en `LOOP_BODY`
2. **✅ RESUELVE OVERFLOW**: JSR puede saltar a cualquier dirección (vs BRA limitado a ±127 bytes)
3. **✅ MANTIENE COMPATIBILIDAD**: Programas pequeños siguen funcionando
4. **✅ ESTRUCTURA PROFESIONAL**: Código más limpio y mantenible

### 2.1.3 Resultados de Compilación Verificados
- **test_vectrex_pattern.vpy**: 61 bytes (era 57, +4 overhead JSR/RTS aceptable)
- **vectrex_console_demo.vpy**: 2138 bytes (era FALLO por overflow, ahora ÉXITO)
- **Ambos programas**: Compilan y funcionan correctamente
- **Capacidad**: Hasta 5KB de espacio disponible para juegos complejos

### 2.1.4 Implementación Backend (m6809.rs)
- **Ubicación crítica**: `core/src/backend/m6809.rs` líneas 160-190
- **Cambio principal**: `JSR LOOP_BODY` en lugar de código inline duplicado
- **Generación automática**: `LOOP_BODY:` con contenido de función `loop()` + `RTS`
- **Mantenimiento**: Auto-loop mode optimizado para estructura Vectrex

### 2.1.5 Reglas de Desarrollo VPy
- **NUNCA volver al patrón inline**: La arquitectura de subrutinas es definitiva
- **Tests obligatorios**: Verificar tanto programas simples como complejos
- **Compilación dual**: Siempre probar test_vectrex_pattern Y vectrex_console_demo
- **Sin regresiones**: JSR/RTS es la solución estándar, no usar BRA para loops

## 3. Tests - Estructura y Reglas Obligatorias

### 3.1 Estructura de Directorios
```
tests/
├── opcodes/           # Tests de opcodes MC6809 (256 tests)
│   ├── arithmetic/    # ADD, SUB, MUL, DIV, etc.
│   ├── branch/        # BRA, BEQ, BNE, JSR, RTS, etc.
│   ├── comparison/    # CMP, TST
│   ├── data_transfer/ # LD, ST, LEA, TFR, EXG
│   ├── logic/         # AND, OR, EOR, COM, NEG
│   ├── register/      # INC, DEC, CLR por registro (A/B/D/X/Y)
│   └── stack/         # PSH, PUL, interrupt handling
└── components/        # Tests de componentes del emulador (19 tests)
    ├── integration/   # Tests de integración entre componentes
    ├── hardware/      # PSG, Screen, Shift Register, Timers
    ├── engine/        # Types, DelayedValueStore
    ├── memory/        # Dispositivos de memoria
    └── cpu/           # Funcionalidad específica CPU
```

### 3.2 Reglas de Naming y Organización
- **UN ARCHIVO POR OPCODE**: Cada opcode tiene su propio archivo `test_[opcode].rs`
- **Nombres descriptivos**: `test_adda.rs`, `test_jsr.rs`, `test_clr_indexed.rs`
- **NO duplicados**: Verificar que no existe test similar antes de crear
- **Categorización lógica**: Agrupar por funcionalidad, no por modo de direccionamiento

### 3.3 Configuración de Memoria Estándar
```rust
// CONFIGURACIÓN OBLIGATORIA en todos los tests de opcodes:
const RAM_START: u16 = 0xC800;  // Inicio de RAM de trabajo para tests
const STACK_START: u16 = 0xCFFF; // Pila inicializada al final de RAM

fn setup_emulator() -> (Emulator, Box<dyn MemoryDevice>) {
    let mut emulator = Emulator::new();
    let memory = Box::new(RamDevice::new()); // RAM mapeada en 0xC800-0xCFFF
    emulator.memory().add_device(RAM_START, memory.clone()).unwrap();
    emulator.cpu_mut().set_stack_pointer(STACK_START);
    (emulator, memory)
}
```

### 3.4 Estructura de Test por Opcode
```rust
// TEMPLATE OBLIGATORIO para tests de opcodes:
#[test]
fn test_[opcode]_[mode]_0x[hexcode]() {  // Nombre con código hex
    let (mut emulator, memory) = setup_emulator();
    
    // 1. Setup inicial - registros y memoria
    emulator.cpu_mut().set_register_a(0x42);
    memory.write(RAM_START, 0x33).unwrap();
    
    // 2. Escribir opcode y operandos en memoria
    memory.write(RAM_START + 0x100, 0x8B).unwrap(); // Opcode
    memory.write(RAM_START + 0x101, 0x42).unwrap(); // Operando si aplica
    
    // 3. Configurar PC y ejecutar
    emulator.cpu_mut().set_program_counter(RAM_START + 0x100);
    emulator.step().unwrap();
    
    // 4. Verificar resultados - registros, flags, memoria
    assert_eq!(emulator.cpu().register_a(), expected_value);
    assert_eq!(emulator.cpu().condition_codes().zero(), expected_flag);
}
```

### 3.5 Reglas de Contenido
- **BIOS real únicamente**: Usar rutas válidas de BIOS, nunca generar sintética
- **Memoria mapeada**: RAM en 0xC800-0xCFFF para todos los tests
- **Stack en 0xCFFF**: Pila siempre inicializada al final de RAM  
- **Verificación completa**: Registros, flags, memoria afectada, cycles
- **Casos edge**: Incluir casos límite (overflow, underflow, zero, negative)
- **NO side effects sintéticos**: Solo efectos reales de la instrucción
- **Timing preciso**: Verificar cycles exactos según documentación MC6809

### 3.6 Tests de Componentes
- **Separados de opcodes**: No mezclar tests de CPU con tests de hardware
- **Integración real**: Tests de integración usan componentes reales, no mocks
- **Hardware específico**: Tests de PSG, Screen, VIA separados por funcionalidad
- **Engine interno**: Tests de tipos y sistemas internos del emulador

## 3.1. BIOS Arranque Automático (Minestorm)
- La BIOS arranca AUTOMÁTICAMENTE Minestorm sin interacción del usuario.
- NO es necesaria entrada de botón o cartucho para que la BIOS progrese al copyright y luego al juego.
- La BIOS detecta ausencia de cartucho y procede automáticamente a mostrar copyright y después Minestorm.
- Tests que esperan Print_Str (0xF373) deben esperar suficientes ciclos (~2.5M) para el delay natural de la BIOS.
- No simular entradas de botón innecesariamente - la BIOS progresa sola.

## 4. Opcode / CPU Core
 Lista ilegal consolidada en `ILLEGAL_BASE_OPCODES` + helper `is_illegal_base_opcode()` (ver `cpu6809.rs`). Cualquier cambio debe reflejarse en SUPER_SUMMARY sección 24 y tests unificados.
## 5. WASM API
- Limitar tamaños de buffers exportados (ej.: trace <= 200k entries).
- Próxima adición planificada: export de call stack.

## 6. Integrator / Vector Output
- No bloquear el drenaje automático si `integrator_auto_drain` está activo.
- Evitar reintroducir backends alternativos no integrator (estandarizado).

## 7. Estilo de Parches
- Cambios mínimos y localizados; no re-formatear bloques grandes sin necesidad funcional.
- Siempre correr `cargo test -p vectrex_emulator` tras cambios en CPU o WASM API.

## 7.1. emulator_v2 - Port 1:1 desde Vectrexy
- **REGLA CRÍTICA**: NUNCA inventar implementación propia. TODO debe ser port línea-por-línea desde Vectrexy C++.
- **Referencia obligatoria**: `vectrexy/libs/emulator/` (archivos .h/.cpp en workspace)
- **IMPORTANTE**: Usar `vectrexy` NO `vectrexy` - la carpeta `vectrexy` puede haber sido modificada por nosotros.
- **Formato mandatorio**: Cada método/función debe incluir comentario `// C++ Original:` con código fuente real.
- **Verificación**: Antes de implementar, leer el archivo C++ correspondiente para entender comportamiento exacto.
- **Constantes**: Usar valores exactos del original (ej: RampUpDelay=5, VelocityXDelay=6, LineDrawScale=0.85f).
- **Estructuras de datos**: Mantener mismos campos con mismos nombres (ej: Timer2 NO tiene latch high).
- **Algoritmos**: Port exacto de lógica (ej: `assert(cycles == 1)` en DelayedValueStore, `--m_rampDelay` en Screen).
- **Excepciones permitidas**: Solo adaptaciones de sintaxis Rust (ownership, borrowing) manteniendo semántica idéntica.

## 8. Documentación
- Actualizar `SUPER_SUMMARY.md` cuando se introduce o cambia: tracing, nuevas etiquetas BIOS, métricas, o comportamiento de integrator.
- Añadir nota de migración en `MIGRATION_WASM.md` si se modifica la superficie WASM.

## 9. Rutas y Constantes Críticas
- Ruta BIOS absoluta (ver sección 1) debe quedar centralizada en helpers de test si se multiplica su uso.
- Evitar duplicación de la cadena de ruta en muchos archivos (refactor pendiente cuando aparezca el segundo uso).

## 10. Próximos TODO Prioritarios
Estado IDs previos:
- (ID 11) Mapeo completo BIOS / etiquetas → COMPLETADO 2025-09-20 (incluye Init_OS y loops intro).
- (ID 13) Export WASM `bios_calls_json()` → COMPLETADO 2025-09-19.
- (ID 5) Resumen estado compilador (`COMPILER_STATUS.md`) → COMPLETADO 2025-09-20.

Nuevos focos (short):
S3 Verificación semántica básica variables (en progreso planificación).
S4 Tests constant folding / dead store.
S5 Documentar truncamiento entero 16-bit en SUPER_SUMMARY.

## 11. Seguridad / Pureza de Entorno
- No escribir en la BIOS cargada (bus lo marca read-only); tests deben respetar esto.
- No introducir dependencias externas innecesarias en crates de núcleo.

## 12. Idioma / Comunicación
- El usuario prefiere español para instrucciones clave y recordatorios: mantener comentarios críticos en español o bilingües cuando corresponda.

## 13. Política de “No Sintético”
- “no generes nada sintético, nunca. usa la bios real.” Aplica a: tests, benchmarks, ejemplos de call stack. Excepción única: micro tests de opcode aislado (no BIOS) donde no se analiza call stack BIOS. Tampoco pongas "side effects" o "simulated". todas las implementaciones deben ser reales

### 13.1 Modo Estricto Permanente
- Se ha eliminado cualquier bandera o modo opcional: el emulador opera SIEMPRE en modo estricto.
- `record_bios_call` únicamente registra la llamada; no aplica side effects heurísticos (no altera DP, intensidad, movimientos, reset0ref, contadores) antes de que las instrucciones reales de la BIOS lo hagan.
- Si un test dependía de esos efectos sintéticos debe actualizarse para observar el cambio sólo cuando la instrucción real (ej. `TFR A,DP`) se ejecute en la BIOS.
- Cualquier nueva propuesta de "heurística" o shortcut debe rechazarse y reemplazarse por emulación fiel.

### 13.2 Lectura / Dump de BIOS en Hex
- Para inspeccionar bytes de la BIOS usar Python (PowerShell ha mostrado inconsistencias con redirecciones heredoc).
- Ejemplo rápido (no incrustar rutas distintas):
	```python
	import pathlib
	data = pathlib.Path(r'C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\dist\\bios.bin').read_bytes()
	base = 0xE000  # 8K mapeada en 0xE000
	for addr in range(0xF1AF, 0xF1AF+16):
			off = addr - base
			b = data[off]
			print(f"{addr:04X}: {b:02X}")
	```
- No usar PowerShell con `<<` para heredocs; preferir `python -c` o scripts `.py` dedicados.

## 14. Conservación de Código Funcionando
- No eliminar ni simplificar código que ya proporciona información automática útil (trazas, call stack enriquecido, anotaciones) salvo petición explícita del usuario.
- Si se considera refactor o poda, primero listar impacto (campos eliminados, UI afectada, tests) y esperar confirmación.
- Preferir extensiones incrementales (añadir campos / rutas) antes que sustituciones destructivas.
- Cualquier reducción de detalle en tracing debe justificarse y documentarse en `SUPER_SUMMARY.md` y discutirse antes.

## 15. Fuente de la Verdad de Semántica (CPU/VIA)
En caso de cualquier duda sobre:
- Orden de pushes/pops de pila 6809 (RTS, interrupt frames, PSHS/PSHU, PULS/PULU)
- Manejo de temporizadores VIA (Timer1 / Timer2: expiración, recarga, limpieza de IFR, bits IER)
- Semántica de flags IFR/IER y generación de IRQ
- Secuencias de inicialización BIOS que dependan de timing real

La referencia primaria de comparación (solo lectura, para validar comportamiento, NO copiar código) es el código de la implementación de referencia localizada en:
`vectrexy/libs/vectrexy` (dentro del workspace)

Política:
1. Usar esta referencia únicamente para confirmar orden y efectos (nunca portar bloques de código textualmente — mantener originalidad y evitar problemas de copyright).
2. Si se detecta divergencia entre nuestra emulación y la referencia, primero instrumentar y demostrar con logs antes de cambiar lógica.
3. Cualquier corrección derivada debe anotar brevemente en `SUPER_SUMMARY.md` (sección CPU/VIA) el aspecto validado y la fecha.
4. Mantener comentarios críticos en español (o bilingües) al introducir cambios basados en esta verificación.

## 16. JavaScript Node.js Testing Harness (Context Preservation)

### 16.1 Scripts de Comparación Disponibles
Para evitar pérdida de contexto y mantener comparaciones Rust vs JavaScript:

#### A) test_f4eb_detailed_js.js (F4EB Loop Analysis)
- **Ubicación**: `test_f4eb_detailed_js.js` (workspace root)
- **Propósito**: Análisis específico del bucle infinito F4EB con detección automática y captura de estado VIA
- **Uso**: `node test_f4eb_detailed_js.js`
- **Características**:
  - Hook e6809_sstep personalizado para monitoring step-by-step
  - Detección automática al llegar a PC=F4EB
  - Captura completa de registros CPU y estado VIA (Timer2 en 0xD05A)
  - Logging detallado de cambios de PC y contadores de loop
  - Comparación directa con baseline Rust (Timer2=0xFF, Cycles=5342)

#### B) jsvecx_comparison.js (General Comparison Framework)
- **Ubicación**: `jsvecx_comparison.js` (workspace root)
- **Propósito**: Framework general para comparaciones Rust vs JSVecx en diferentes tamaños de test
- **Uso**: `node jsvecx_comparison.js` (ejecuta tests de 100, 500, 1000, 2000, 5000 pasos)
- **Características**:
  - Carga automática de BIOS desde ruta estándar
  - Captura de estado en cada paso con tabla formateada
  - Análisis de patrones frecuentes y estadísticas
  - Generación de archivos de comparación (jsvecx_comparison_N_steps.txt)

### 16.2 Datos Críticos para Comparación F4EB
- **Estado Rust en F4EB**: PC=F4EB, Step=1525, Cycles=5342, Timer2=0xFF, IFR=0x60, IER=0x00
- **Problema**: BIOS hace polling en Timer2 (0xD05A) esperando 0x81 pero lee 0xFF
- **Comparación Objetivo**: Verificar si JSVecx también produce Timer2=0xFF o valor diferente
- **VIA Registers**: IFR=0x60 (Timer1/Timer2 expirados), IER=0x00 (interrupts deshabilitados)

### 16.3 Protocolo de Comparación
1. **Ejecutar baseline Rust**: `cargo test test_f4eb_loop_js_vs_rust_comparison`
2. **Ejecutar comparación JavaScript**: `node test_f4eb_detailed_js.js`
3. **Comparar Timer2 values**: Rust=0xFF vs JavaScript=? 
4. **Analizar sincronización VIA**: Verificar timing Timer1/Timer2 entre emuladores
5. **Documentar discrepancias**: Actualizar SUPER_SUMMARY.md con findings

### 16.4 Dependencias y Setup
- **JSVecx path**: `jsvecx/src/deploy/js/` (utils.js, globals.js, e6809.js, vecx.js, etc.)
- **BIOS path**: `ide/frontend/dist/bios.bin` (8192 bytes, mapeada en 0xE000-0xFFFF)
- **Node.js requirement**: Compatible con Node.js estándar, sin dependencias externas
- **Cross-platform**: Scripts funcionan en Windows PowerShell y Linux/macOS

## 17. MCP (Model Context Protocol) Integration

### 17.1 Arquitectura General
- **Propósito**: Exponer IDE y emulador a agentes AI (PyPilot, Copilot, etc.)
- **Implementación Dual**:
  - **Electron Backend**: `ide/electron/src/mcp/server.ts` - Servidor interno IPC
  - **External Server**: `ide/mcp-server/server.js` - Servidor stdio para AIs externos
- **Comunicación**: External server → IPC (puerto 9123) → Electron → IDE state
- **Total de herramientas**: 22 tools (7 editor, 2 compiler, 3 emulator, 2 debugger, 8 project)

### 17.2 Convenciones de Naming
- **Tool Names en External Server**: snake_case (`editor_write_document`, `project_create_vector`)
- **Tool Names en Electron Server**: slash-separated (`editor/write_document`, `project/create_vector`)
- **Conversión automática**: External server convierte **PRIMER guión bajo** a slash: `editor_write_document` → `editor/write_document`
- **CRÍTICO**: NO convertir todos los guiones bajos - solo el primero (ej: `project_create_vector` → `project/create_vector`, NO `project/create/vector`)

### 17.3 Herramientas Disponibles

#### Editor (7 tools)
- `editor/list_documents`: Lista documentos abiertos
- `editor/read_document`: Lee contenido de documento
- `editor/write_document`: **Crea O actualiza** documento (auto-abre en editor si es nuevo)
- `editor/get_diagnostics`: Obtiene errores de compilación/lint
- `editor/replace_range`: Reemplaza texto en rango específico
- `editor/insert_at`: Inserta texto en posición
- `editor/delete_range`: Elimina texto en rango

#### Compiler (2 tools)
- `compiler/build`: Compila programa VPy
- `compiler/get_errors`: Obtiene últimos errores de compilación

#### Emulator (3 tools)
- `emulator/run`: Ejecuta ROM compilada
- `emulator/get_state`: Estado actual (PC, registros, cycles)
- `emulator/stop`: Detiene ejecución

#### Debugger (2 tools)
- `debugger/add_breakpoint`: Añade breakpoint en línea
- `debugger/get_callstack`: Obtiene call stack actual

#### Project (8 tools)
- `project/get_structure`: Estructura del proyecto
- `project/read_file`: Lee archivo del proyecto
- `project/write_file`: Escribe archivo general
- `project/create`: Crea nuevo proyecto (muestra dialog si no hay path)
- `project/close`: Cierra proyecto actual
- `project/open`: Abre proyecto existente
- `project/create_vector`: **Crea archivo .vec con validación JSON**
- `project/create_music`: **Crea archivo .vmus con validación JSON**

### 17.4 Validación JSON para Assets

#### Vector Files (.vec) - FORMATO OBLIGATORIO JSON
```json
{
  "version": "1.0",
  "name": "shape",
  "canvas": {"width": 256, "height": 256, "origin": "center"},
  "layers": [{
    "name": "default",
    "visible": true,
    "paths": [{
      "name": "line1",
      "intensity": 127,
      "closed": false,
      "points": [{"x": 0, "y": 0}, {"x": 10, "y": 10}]
    }]
  }]
}
```

**Ejemplo triángulo cerrado**:
```json
{
  "layers": [{
    "paths": [{
      "closed": true,
      "points": [
        {"x": 0, "y": 20},
        {"x": -15, "y": -10},
        {"x": 15, "y": -10}
      ]
    }]
  }]
}
```

#### Music Files (.vmus) - FORMATO OBLIGATORIO JSON
```json
{
  "version": "1.0",
  "name": "My Song",
  "author": "Composer Name",
  "tempo": 120,
  "ticksPerBeat": 24,
  "totalTicks": 384,
  "notes": [
    {
      "id": "note1",
      "note": 60,
      "start": 0,
      "duration": 48,
      "velocity": 12,
      "channel": 0
    }
  ],
  "noise": [
    {
      "id": "noise1",
      "start": 0,
      "duration": 24,
      "period": 15,
      "channels": 1
    }
  ],
  "loopStart": 0,
  "loopEnd": 384
}
```

**CAMPOS OBLIGATORIOS**:
- **note**: Número MIDI (0-127, 60=Do central, 72=Do5)
- **velocity**: Volumen (0-15, 15=máximo)
- **period**: Período de ruido (0-31, menor=tono más alto)
- **channels**: Máscara de bits para ruido (1=A, 2=B, 4=C, 7=todos)
- **id**: Identificador único para cada nota/evento de ruido

**LÍMITES DE TAMAÑO (ACTUALIZADO)**:
✅ **Límite ampliado**: max_tokens aumentado de 2000 a 8000 (hasta ~100 notas aprox)
⚠️ **Recomendación**: Mantener canciones bajo ~80-100 notas totales para evitar truncamiento
💡 **Mejor práctica**: Para canciones largas, usar loops cortos + loopStart/loopEnd para repetición
💡 **Ventaja de loops**: Archivos más pequeños, más eficientes, mismo efecto musical

#### Validación Implementada
- **`project/create_vector`**: Valida JSON antes de crear archivo
  - Verifica campos obligatorios: `version`, `layers` (array)
  - Rechaza formatos inventados (VECTOR_START, MOVE, DRAW_TO, etc.)
  - Error muestra formato correcto con ejemplo
  
- **`project/create_music`**: Valida JSON antes de crear archivo
  - Verifica campos obligatorios: `version`, `tempo`, `notes` (array)
  - Rechaza formatos no-JSON
  - Error muestra formato correcto con ejemplo

### 17.5 Comportamiento de Creación de Archivos
- **Auto-apertura**: Todos los archivos creados se abren automáticamente en el editor
- **Auto-detección de lenguaje**: `.vpy` → VPy, `.vec`/`.vmus`/`.json` → JSON
- **Creación de directorios**: Automática si no existen (`assets/vectors/`, `assets/music/`)
- **Normalización de URI**: Helper `normalizeUri()` maneja:
  - Nombres de archivo simples (`"game.vpy"`)
  - Rutas relativas (`"src/main.vpy"`)
  - Rutas absolutas (`"/Users/.../file.vpy"`)
  - URIs completos (`"file:///path/to/file"`)

### 17.6 Guías para AI Integration

#### Creating New Files:
✅ **Use `editor/write_document`**: Create .vpy files, general text files (creates + opens automatically)
✅ **Use `project/create_vector`**: Create .vec files (validates JSON structure)
✅ **Use `project/create_music`**: Create .vmus files (validates JSON structure)
❌ **Don't use `editor/read_document`**: Fails if file doesn't exist yet ("Document not found")
❌ **Don't use `editor/replace_range`**: Requires file to be open first + requires LINES not offsets

#### Editing Existing Files:
1. **For complete replacement**: Use **`editor/write_document`** (replaces entire content, works always)
2. **For partial edits**:
   - First: **`editor/list_documents`** (verify file is open)
   - Then: **`editor/replace_range`** (requires `startLine`/`endLine`, NOT character offsets)
   - Or: **`editor/insert_at`** / **`editor/delete_range`**

#### Common Mistakes:
❌ Calling `editor/read_document` on file that isn't open → "Document not found: game.vmus. Use editor/write_document to CREATE new files"
❌ Calling `editor/replace_range` with `start`/`end` offsets → "Missing line parameters (startLine/endLine REQUIRED, NOT character offsets)"
❌ Inventing text formats for .vec/.vmus → "Rejected: Must be valid JSON"
✅ Using `editor/write_document` for new OR existing files → Always works
✅ Using `project/create_music` for .vmus → JSON validated automatically, helpful error messages

#### Tool Rules:
- **NO inventar herramientas**: Solo usar las 22 herramientas registradas
- **NO inventar formatos**: Archivos .vec y .vmus son SIEMPRE JSON
- **Usar herramientas especializadas**: `project/create_vector` en lugar de `editor/write_document` para vectores (valida JSON)
- **Aprender de errores**: La validación JSON enseña el formato correcto mediante feedback
- **Nombres correctos**: Verificar con `tools/list` antes de llamar herramientas

### 17.7 Debugging MCP
- **Logs External Server**: `ide/mcp-server/server.js` escribe a stderr
- **Logs Electron**: `ide/electron/src/mcp/server.ts` usa console.log
- **Test IPC**: Puerto 9123 debe estar disponible
- **Tool not found**: Verificar conversión de nombre (snake_case → slash-separated)
- **JSON validation errors**: Verificar estructura completa en mensaje de error

---
Última actualización: 2025-12-10 - Añadida sección 17 (MCP Integration completa)
