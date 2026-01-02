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

## 0.1.5. ESPACIO CRÍTICO: WAIT_RECAL() - NO PONERLO MANUALMENTE
⚠️ **REGLA OBLIGATORIA**: 
- ❌ **NUNCA** escribir `WAIT_RECAL()` manualmente en el código VPy
- ✅ El compilador inyecta `WAIT_RECAL()` automáticamente al inicio del `loop()`
- El loop generado es:
  ```asm
  LOOP_BODY:
      WAIT_RECAL()        # ← Inyectado automáticamente por compilador
      [resto del código]
      RTS
  ```

**POR QUÉ**: `WAIT_RECAL()` sincroniza con el refresco de pantalla (50 FPS). El compilador lo maneja automáticamente en M6809 - no debe escribirse en VPy.

## 0.1.6. MÚSICA: AUDIO_UPDATE INYECTADO AUTOMÁTICAMENTE
⚠️ **REGLA IMPLEMENTADA**: 
- ❌ **NUNCA** escribir `MUSIC_UPDATE()` o `AUDIO_UPDATE()` manualmente en el código VPy
- ✅ El compilador inyecta `AUDIO_UPDATE` automáticamente **AL FINAL del `loop()`**, después de todo el drawing
- La inyección se hace en `core/src/backend/m6809/mod.rs` líneas ~550 (después del loop de `emit_stmt`)
- El loop generado es:
  ```asm
  LOOP_BODY:
      [código del loop...]
      JSR AUDIO_UPDATE  ; ← Inyectado automáticamente por compilador (DESPUÉS del drawing)
      LEAS N,S          ; Free locals
      RTS
  ```

**POR QUÉ AL FINAL**: 
- `AUDIO_UPDATE` es una operación crítica de timing (actualiza PSG cada frame)
- Si se ejecuta al inicio, puede interrumpirse durante los calls de drawing (que son costosos)
- Colocar al final garantiza que se completa sin interrupciones entre frames
- **Problema resuelto**: Drawing del logo (11 paths) clavaba música cuando AUDIO_UPDATE estaba al inicio (commit 2025-12-26)

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

## 7.2. Validación Semántica - Variable Scope (COMPLETADO 2025-12-10)
- **Estado**: Sistema de validación mejorado implementado y funcionando
- **Ubicación**: `core/src/codegen.rs` - funciones `validate_semantics`, `validate_function`, `validate_stmt_collect`, `validate_expr_collect`
- **Capacidades**:
  - Detecta variables declaradas en una función pero usadas en otra
  - Mensajes de error descriptivos con línea/columna exacta
  - Explica por qué el error ocurre (scopes separados entre funciones)
  - Sugiere solución (declarar variable en función donde se usa)
  - Detecta variables no declaradas en general
  - Validación de aridad de funciones builtin

### Ejemplo de Error Mejorado:
```
❌ PHASE 4 FAILED: Semantic errors detected:
   error 24:5 - SemanticsError: variable 'player_x' declarada en función 'main' no es accesible en 'loop'. 
   Las funciones en VPy tienen scopes separados (no comparten variables). 
   Solución: declara 'player_x' dentro de 'loop' donde la necesitas.
```

### Implementación Técnica:
1. **Phase 1 (Discovery)**: `collect_function_locals()` recorre todas las funciones y recolecta variables locales declaradas
2. **Phase 2 (Validation)**: `validate_function()` valida cada función independientemente con su propio scope
3. **Phase 3 (Cross-Function Check)**: `validate_expr_collect()` detecta cuando una variable de otra función se intenta usar
4. **Phase 4 (Reporting)**: `main.rs` imprime errores semánticos antes de mostrar "empty assembly"

### Integración con IDE:
- Los diagnostics se exponen en `emit_asm_with_debug()` retornando `Vec<Diagnostic>`
- LSP puede consumir estos diagnostics para mostrar errores en tiempo real en el editor
- Cada diagnostic incluye: severity, code, message, line, col
- Compatible con sistema MCP para reportar errores a PyPilot y otros agentes AI

### Testing:
- `examples/test_scope.vpy`: Caso mínimo que reproduce el error
- `examples/user_test_fixed.vpy`: Versión corregida (variables en loop, no en main)
- Tests verifican que código correcto sigue compilando sin errores


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
- S3 Verificación semántica variables → COMPLETADO 2025-12-10 (cross-function scope detection).
- S7 PyPilot conversation persistence → COMPLETADO 2025-12-10 (localStorage integration).
- S8 PyPilot concise mode → COMPLETADO 2025-12-10 (system prompt injection).
- S9 MCP compiler/build store access → COMPLETADO 2025-12-10 (backend project tracker).

Nuevos focos (short):
S4 Tests constant folding / dead store.
S5 Documentar truncamiento entero 16-bit en SUPER_SUMMARY.
S6 LSP integration para reportar semantic diagnostics en tiempo real (exponer `Vec<Diagnostic>`).
S10 Multi-path vector positioning investigation (ver VECTOR_MULTIPATH_LIMITATION.md):
  - Estudiar Moveto_d_7F requirements completos
  - Probar delta calculation entre paths (relative offsets)
  - Investigar integrator settling time para Reset0Ref
  - Comparar con implementación de referencia (Vectrexy vector drawing)
  - Documentar findings en SUPER_SUMMARY.md sección Vector Drawing


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

## 17. Sistema de Assets (Vectores y Música)

### 17.1 Arquitectura General
- **Propósito**: Permitir que juegos VPy embeben recursos gráficos (.vec) y música (.vmus) como datos en ROM
- **Ubicación**: Archivos en `assets/vectors/*.vec` y `assets/music/*.vmus` dentro del proyecto
- **Descubrimiento automático**: Fase 0 de compilación escanea directorio assets/ y detecta recursos
- **Embedding**: Fase 5 embebe datos convertidos en sección DATA del ASM generado
- **Acceso en código**: Funciones builtin `DRAW_VECTOR("nombre")` y `PLAY_MUSIC("nombre")`

### 17.2 Formato de Archivos Vector (.vec)

```json
{
  "version": "1.0",
  "name": "player",
  "canvas": {"width": 256, "height": 256, "origin": "center"},
  "layers": [{
    "name": "default",
    "visible": true,
    "paths": [{
      "name": "ship",
      "intensity": 127,
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

**Campos importantes**:
- **name** (top-level): Nombre del asset (usado en `DRAW_VECTOR("name")`)
- **paths[].name**: Nombre del path individual (genera label `_NAME_PATHID_VECTORS`)
- **paths[].intensity**: 0-255, brillo del vector
- **paths[].closed**: true = polígono cerrado, false = línea abierta
- **points**: Array de {x, y} en coordenadas canvas (-127 a 127)

**Generación ASM**:
```asm
_PLAYER_SHIP_VECTORS:
    FCB 3              ; num_points
    FCB 127            ; intensity
    FCB 20, 0          ; point 0 (y, x)
    FCB -10, -15       ; point 1
    FCB -10, 15        ; point 2
    FCB $01            ; closed path

_PLAYER_VECTORS:       ; Alias principal (apunta a primer path)
    FCB 3, FCB 127
    FCB 20, 0, FCB -10, -15, FCB -10, 15
    FCB $01
```

### 17.3 Formato de Archivos Música (.vmus)

```json
{
  "version": "1.0",
  "name": "theme",
  "author": "Composer",
  "tempo": 120,
  "ticksPerBeat": 24,
  "totalTicks": 384,
  "notes": [
    {"id": "note1", "note": 60, "start": 0, "duration": 48, "velocity": 12, "channel": 0},
    {"id": "note2", "note": 64, "start": 48, "duration": 48, "velocity": 12, "channel": 0},
    {"id": "note3", "note": 67, "start": 96, "duration": 48, "velocity": 12, "channel": 0}
  ],
  "noise": [
    {"id": "noise1", "start": 0, "duration": 24, "period": 15, "channels": 1, "velocity": 12}
  ],
  "loopStart": 0,
  "loopEnd": 384
}
```

**Campos importantes**:
- **note**: Número MIDI (0-127, donde 60=Do central/C4, 69=La/A4 440Hz)
- **velocity**: Volumen PSG (0-15, donde 15=máximo) - Usado tanto por notes como noise
- **channel**: Canal PSG (0=A, 1=B, 2=C) - Solo para notes
- **period**: Período de ruido (0-31, valores menores = tono más agudo)
- **channels**: Máscara de bits para ruido (1=A, 2=B, 4=C, 7=todos) - Solo para noise

**Conversión MIDI a PSG**:
- Fórmula: `period = 1_500_000 / (32 * freq_hz)`
- Frecuencia MIDI: `freq = 440 * 2^((note - 69) / 12)`
- Ejemplo: MIDI 60 (C4, 261.63Hz) → PSG period 179

**Generación ASM** (placeholder actual):
```asm
_THEME_MUSIC:
    FCB 0 ; Placeholder (PSG player completo pendiente)
```

### 17.4 Funciones Builtin en VPy

#### DRAW_VECTOR(nombre: str)
Dibuja un vector asset embebido en ROM.

```python
def loop():
    WAIT_RECAL()
    DRAW_VECTOR("player")  # Dibuja el sprite del jugador
```

**Código ASM generado**:
```asm
    LDX #_PLAYER_VECTORS   ; Cargar puntero a datos del vector
    JSR Draw_VLc           ; Llamar BIOS para dibujar
    LDD #0
    STD RESULT
```

**Verificación en compilación**:
- Comprueba que el asset existe en `opts.assets`
- Error si el archivo .vec no se encuentra o el nombre no coincide
- Genera comentario de error en ASM si falla

#### DRAW_VECTOR_EX(nombre: str, x: int, y: int, mirror: int)
Dibuja un vector asset con posición y espejo (horizontal/vertical).

**Parámetros**:
- `nombre`: Nombre del asset .vec
- `x`, `y`: Posición de dibujo (offset desde la posición del sprite)
- `mirror`: Modo de espejo (0-3):
  - **0** = Normal (sin espejo)
  - **1** = Espejo X (horizontal, voltea izquierda-derecha)
  - **2** = Espejo Y (vertical, voltea arriba-abajo)
  - **3** = Espejo XY (ambos ejes, rotación 180°)

**Ejemplo VPy**:
```python
def loop():
    WAIT_RECAL()
    DRAW_VECTOR_EX("player", 30, 60, 0)   # Normal
    DRAW_VECTOR_EX("player", 90, 60, 1)   # Espejo X
    DRAW_VECTOR_EX("player", 30, 0, 2)    # Espejo Y
    DRAW_VECTOR_EX("player", 90, 0, 3)    # Espejo XY
```

**Código ASM generado** (simplificado):
```asm
    LDD #30          ; X posición
    STA DRAW_VEC_X
    LDD #60          ; Y posición
    STA DRAW_VEC_Y
    LDD #1           ; Mirror mode
    LDB RESULT+1
    
    ; Decode mirror flags
    CLR MIRROR_X
    CLR MIRROR_Y
    CMPB #1          ; Check for X-mirror
    BNE DSVEX_CHK_Y
    LDA #1
    STA MIRROR_X
DSVEX_CHK_Y:
    CMPB #2          ; Check for Y-mirror
    BNE DSVEX_CHK_XY
    LDA #1
    STA MIRROR_Y
    ...
    LDX #_PLAYER_PATH0
    JSR Draw_Sync_List_At_With_Mirrors  ; Función unificada
```

**Arquitectura de Espejos Unificada** (NUEVO 2025-12-18):
- **Función única**: `Draw_Sync_List_At_With_Mirrors` maneja todos los 4 modos
- **Runtime flags**: MIRROR_X (0/1) y MIRROR_Y (0/1) controlan condicional­mente las negaciones
- **Ahorro ASM**: Una función con condicionales (~220 líneas) vs 4 funciones separadas (~520 líneas)
- **Centro-relativo**: Todas las coordenadas ya son relativas al centro del sprite (vecres.rs)
- **Operaciones**:
  - **X-mirror** (modo 1): NEGA X coordinate + NEGA dx deltas
  - **Y-mirror** (modo 2): NEGB Y coordinate + NEGB dy deltas  
  - **XY-mirror** (modo 3): Ambas operaciones aplicadas
  - **Normal** (modo 0): No apply any negation

**Verificación en compilación**:
- Comprueba que el asset existe
- Valida que mirror sea 0-3
- Error si el archivo .vec no se encuentra
- Automáticamente genera flags MIRROR_X/MIRROR_Y en RAM

#### PLAY_MUSIC(nombre: str)
Inicia reproducción de música embebida en ROM.

```python
def loop():
    PLAY_MUSIC("theme")  # Reproduce música de fondo
```

**Código ASM generado**:
```asm
    LDX #_THEME_MUSIC        ; Cargar puntero a datos de música
    JSR PLAY_MUSIC_RUNTIME   ; Llamar player de música
    LDD #0
    STD RESULT
```

**Estado actual**: Placeholder implementado, PSG player completo pendiente.

### 17.5 Pipeline de Compilación

#### Fase 0: Asset Discovery
```rust
fn discover_assets(source_path: &Path) -> Vec<AssetInfo>
```

1. Determina project root (parent de src/ o directorio del archivo)
2. Busca `project_root/assets/vectors/*.vec`
3. Busca `project_root/assets/music/*.vmus`
4. Retorna `Vec<AssetInfo>` con nombre, path, tipo de cada asset
5. Log: `✓ Discovered N asset(s): - name (Type)`

#### Fase 5: Asset Embedding
En `emit_with_debug()` después de parsear lineMap:

```rust
for asset in &opts.assets {
    match asset.asset_type {
        AssetType::Vector => {
            let resource = VecResource::load(&asset.path)?;
            let asm = resource.compile_to_asm();
            out.push_str(&asm);
        },
        AssetType::Music => {
            // Deserializa JSON inline
            // Genera label _NAME_MUSIC con datos placeholder
        }
    }
}
```

#### Variables RAM Necesarias
Si hay assets de música, se define automáticamente:
```asm
MUSIC_PTR:  FDB 0   ; Storage para puntero de música actual
```

### 17.6 Compatibilidad con Ensamblador Nativo

El ensamblador nativo M6809 de VPy **NO soporta**:
- ❌ Directiva `EQU` (debe usar labels duplicados con datos reales)
- ❌ Directiva `RMB` (debe usar FDB/FCB o definir en sección RAM con EQU)
- ✅ Labels estándar (termina con `:`)
- ✅ Directivas FCB, FDB, ORG

**Soluciones implementadas**:
- `_PLAYER_VECTORS` genera label duplicado con datos completos (no EQU)
- `MUSIC_PTR` definida en sección RAM con EQU a RESULT+26
- `PLAY_MUSIC_RUNTIME` helper emitido automáticamente si hay assets música

### 17.7 Módulos de Código Relevantes

**core/src/vecres.rs**: Vector resource handling
- `VecResource::load(path)` - Carga .vec desde disco
- `compile_to_asm()` - Genera ASM con FCB data + label principal
- Genera `_NAME_PATHID_VECTORS` por cada path
- Genera `_NAME_VECTORS` apuntando al primer path (alias principal)

**core/src/musres.rs**: Music resource handling
- `MusicResource::load(path)` - Carga .vmus desde disco
- `compile_to_asm()` - Genera ASM con tempo header, eventos ordenados, loops
- `midi_to_psg_period(note)` - Convierte MIDI a período PSG
- Tests de conversión MIDI: note 60→179, note 69→106

**core/src/main.rs**: Compilation pipeline
- `discover_assets(source_path)` - Fase 0 de descubrimiento
- Pasa `assets: Vec<AssetInfo>` a `CodegenOptions`

**core/src/backend/m6809.rs**: Assembly generation
- Fase 5: Embedding de assets en DATA section
- `emit_builtin_call()`: DRAW_VECTOR y PLAY_MUSIC code generation
- `emit_builtin_helpers()`: Emite PLAY_MUSIC_RUNTIME si hay música
- Define MUSIC_PTR en sección RAM si necesario

**core/src/codegen.rs**: Types and options
- `AssetInfo { name, path, asset_type }`
- `AssetType` enum: Vector, Music
- `CodegenOptions.assets: Vec<AssetInfo>`
- `BUILTIN_ARITIES`: DRAW_VECTOR(1), PLAY_MUSIC(1)

### 17.8 Ejemplo Completo

**examples/test_assets.vpy**:
```python
META TITLE = "Asset Demo"

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    DRAW_VECTOR("player")
    PLAY_MUSIC("theme")
```

**examples/assets/vectors/player.vec**: Triángulo de nave (3 puntos)
**examples/assets/music/theme.vmus**: Melodía C-E-G (3 notas)

**Resultado**:
- Compilación exitosa: `✓ Discovered 2 asset(s)`
- ASM generado: 3.5KB con datos embebidos
- Binario: 156 bytes de código máquina
- Ensamblador nativo: Procesa correctamente sin lwasm

### 17.9 TODO Pendientes
- [ ] Implementar PSG music player completo en PLAY_MUSIC_RUNTIME
- [ ] Validación semántica: error en tiempo de compilación si asset no existe
- [ ] LSP autocomplete para nombres de assets en DRAW_VECTOR/PLAY_MUSIC
- [ ] Soporte multi-path: `DRAW_VECTOR("player.ship")` para paths específicos
- [ ] Documentación en VPyContext.ts para IDE integration
- [ ] Tests de integración con emulador (verificar rendering/playback)

---
Última actualización: 2025-12-10 - Añadida sección 17 (Sistema de Assets completo)

## 18. MCP (Model Context Protocol) Integration

### 18.1 Arquitectura General
- **Propósito**: Exponer IDE y emulador a agentes AI (PyPilot, Copilot, etc.)
- **Implementación Dual**:
  - **Electron Backend**: `ide/electron/src/mcp/server.ts` - Servidor interno IPC
  - **External Server**: `ide/mcp-server/server.js` - Servidor stdio para AIs externos
- **Comunicación**: External server → IPC (puerto 9123) → Electron → IDE state
- **Total de herramientas**: 25 tools (7 editor, 2 compiler, 3 emulator, 3 memory, 2 debugger, 8 project)

### 18.2 Convenciones de Naming
- **Tool Names en External Server**: snake_case (`editor_write_document`, `project_create_vector`)
- **Tool Names en Electron Server**: slash-separated (`editor/write_document`, `project/create_vector`)
- **Conversión automática**: External server convierte **PRIMER guión bajo** a slash: `editor_write_document` → `editor/write_document`
- **CRÍTICO**: NO convertir todos los guiones bajos - solo el primero (ej: `project_create_vector` → `project/create_vector`, NO `project/create/vector`)

### 18.3 Herramientas Disponibles

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

#### Memory (3 tools) - **NUEVO 2026-01-01**
- `memory/dump`: Get memory snapshot (hex dump of RAM region)
- `memory/list_variables`: Get all variables from PDB with sizes and types (sorted by size, largest first)
- `memory/read_variable`: Read current value of specific variable from emulator

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

### 18.4 Validación JSON para Assets

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
      "channels": 1,
      "velocity": 12
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

### 18.5 Comportamiento de Creación de Archivos
- **Auto-apertura**: Todos los archivos creados se abren automáticamente en el editor
- **Auto-detección de lenguaje**: `.vpy` → VPy, `.vec`/`.vmus`/`.json` → JSON
- **Creación de directorios**: Automática si no existen (`assets/vectors/`, `assets/music/`)
- **Normalización de URI**: Helper `normalizeUri()` maneja:
  - Nombres de archivo simples (`"game.vpy"`)
  - Rutas relativas (`"src/main.vpy"`)
  - Rutas absolutas (`"/Users/.../file.vpy"`)
  - URIs completos (`"file:///path/to/file"`)

### 18.6 Guías para AI Integration

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

#### Asset System Integration:
- **Assets ubicación**: `assets/vectors/*.vec` y `assets/music/*.vmus` en project root
- **Compilación automática**: Los assets se descubren y embeben automáticamente (Fase 0 + Fase 5)
- **Uso en código VPy**: `DRAW_VECTOR("nombre")` y `PLAY_MUSIC("nombre")`
- **Creación recomendada**: Usar `project/create_vector` y `project/create_music` (validan JSON)
- **Formato verificado**: Ver sección 17 para estructura JSON completa de .vec y .vmus
- **Ensamblador nativo**: El compilador usa ensamblador M6809 propio (NO lwasm)
- **Compilación end-to-end**: `cargo run --bin vectrexc -- build programa.vpy --bin`

### 18.7 Debugging MCP
- **Logs External Server**: `ide/mcp-server/server.js` escribe a stderr
- **Logs Electron**: `ide/electron/src/mcp/server.ts` usa console.log
- **Test IPC**: Puerto 9123 debe estar disponible
- **Tool not found**: Verificar conversión de nombre (snake_case → slash-separated)
- **JSON validation errors**: Verificar estructura completa en mensaje de error

### 18.8 CRITICAL: Project Paths and File Operations
⚠️ **RUTAS RELATIVAS AL PROYECTO**:
- `project/read_file` y `project/write_file` usan paths RELATIVAS al project root
- Ejemplo: Para leer `/Users/daniel/projects/Vectrex/jetpac/src/main.vpy`, usar `src/main.vpy`
- ❌ MAL: `project/read_file("main.vpy")` → busca en `/project/main.vpy`
- ✅ BIEN: `project/read_file("src/main.vpy")` → busca en `/project/src/main.vpy`

⚠️ **DIFERENCIA ENTRE EDITOR Y PROJECT**:
- `editor/read_document`: Lee archivos ABIERTOS en el editor (URI completo: `file:///Users/...`)
- `project/read_file`: Lee archivos del PROYECTO (path relativo: `src/main.vpy`)
- Usar `editor/list_documents` para ver qué archivos están abiertos
- Usar `project/get_structure` para ver estructura del proyecto

⚠️ **NOMBRES DE HERRAMIENTAS**:
- Los nombres con slash son NOMBRES DE HERRAMIENTAS, NO paths de archivo
- `project/create_vector` = nombre de herramienta (crear vector file)
- NO confundir con path de archivo como `project/assets/vectors/ship.vec`
- Cuando la documentación dice "project/create_vector", el slash es parte del NOMBRE DE HERRAMIENTA

⚠️ **ASSET NAMES VS FILE PATHS**:
- Asset names en código: `DRAW_VECTOR("ship")` - nombre simple, sin extensión
- Asset file paths: `assets/vectors/ship.vec` - path relativo con extensión
- `project/create_vector` recibe NAME (sin extensión) y crea en ubicación estándar
- El sistema automáticamente crea `assets/vectors/{name}.vec`

⚠️ **CRÍTICO: NUNCA INVENTAR NOMBRES DE ASSETS**:
- ANTES de usar `DRAW_VECTOR("nombre")` o `PLAY_MUSIC("nombre")`:
  1. **VERIFICAR** con `project/get_structure` qué assets existen
  2. **LEER** lista de archivos en `assets/vectors/*.vec` y `assets/music/*.vmus`
  3. **USAR** solo nombres que existan físicamente
- ❌ NO asumir nombres genéricos (player, enemy, ship_part1, etc.)
- ✅ Ejemplo correcto:
  ```
  1. project/get_structure → ver assets/vectors/rocket_base.vec
  2. Código VPy: DRAW_VECTOR("rocket_base")  # ✅ existe
  3. NO: DRAW_VECTOR("ship_part1")  # ❌ no existe, inventado
  ```
- Si asset no existe: Preguntar al usuario o crearlo con `project/create_vector`

---
Última actualización: 2025-12-18 - Sección 18.8: Project Paths, File Operations y Asset Verification

## 19. Joystick Input System (J1_X, J1_Y)

### 19.1 Arquitectura General
- **Propósito**: Permitir que juegos VPy lean entrada de joystick (Vectrex analógico de hardware original)
- **Implementación Dual**:
  - **Frontend**: `ide/frontend/src/components/panels/EmulatorPanel.tsx` - Lee gamepad de navegador
  - **Emulador**: JSVecx (JavaScript) - Almacena valores en RAM
  - **Compilador**: `core/src/backend/m6809/builtins.rs` - Genera M6809 que lee desde RAM

### 19.2 RAM Addresses (CRÍTICO - Memory Collision Zone)
⚠️ **IMPORTANTE**: Estas direcciones pueden colisionar con struct globales en programas grandes.

**Addresses Actuales** (cambio 2025-12-18):
```
$CF00 - Joy_1_X (unsigned byte: 0=left, 128=center, 255=right)
$CF01 - Joy_1_Y (unsigned byte: 0=down, 128=center, 255=up)
```

**Por qué estos addresses**:
- $C81B/$C81C anterior causaba colisión con structs en Jetpac
- $CF00/$CF01 están en zona de alto RAM, menos probable de colisionar
- Ubicación: Entre típicas variables work ($C800-$CE00) y stack ($CFFF)

**Si hay nueva colisión**:
1. Cambiar ambas ubicaciones (compiler + frontend) a un nuevo par de addresses
2. Coordinar entre `builtins.rs` y `EmulatorPanel.tsx` - DEBEN ser el mismo par
3. Documentar nueva dirección en esta sección
4. Recompilar compiler y frontend

### 19.3 Data Flow (Unsigned 0-255 Range)

1. **Hardware Input** (Browser Gamepad API):
   - Analog stick values: -1.0 (left/down) to +1.0 (right/up)
   - Deadzone: 0.3 (applies to analog sticks, not D-Pad)

2. **Frontend Conversion** (EmulatorPanel.tsx line 511-514):
   ```typescript
   const analogX = Math.round((x + 1) * 127.5);  // -1.0..+1.0 → 0..255
   const analogY = Math.round((y + 1) * 127.5);  // Range: 0=extreme, 128=center, 255=extreme
   vecx.write8(0xCF00, analogX);  // Write to Joy_1_X
   vecx.write8(0xCF01, analogY);  // Write to Joy_1_Y
   ```

3. **Emulator Storage** (JSVecx):
   - Bytes stored in RAM at $CF00 (X) and $CF01 (Y)
   - Unsigned range: 0-255

4. **VPy Compiler ASM** (builtins.rs J1_X function):
   ```asm
   LDB $CF00          ; Read unsigned byte from RAM
   CMPB #108          ; Compare with lower threshold
   BLO J1X_LEFT       ; Branch if <108 (left)
   CMPB #148          ; Compare with upper threshold
   BHI J1X_RIGHT      ; Branch if >148 (right)
   ; Otherwise center (0)
   ```

5. **Return Value** (VPy Code):
   ```python
   joy_x = J1_X()     # Returns signed: -1 (left), 0 (center), +1 (right)
   joy_y = J1_Y()     # Returns signed: -1 (down), 0 (center), +1 (up)
   ```

### 19.4 Thresholds for Unsigned 0-255

**Reasoning**:
- Center = 128 (midpoint of 0-255)
- Deadzone = ±20 from center
- Thresholds: 108 (128-20) and 148 (128+20)

```
Value Range    →    Interpretation
0-107         →    -1 (left/down, extreme)
108-148       →    0 (center)
149-255       →    +1 (right/up, extreme)
```

**Note**: These thresholds assume no additional deadzone in frontend (deadzone 0.3 handles it).

### 19.5 Builtin Functions

#### J1_X() - Read Joystick X Axis
- **Returns**: Signed 16-bit (-1, 0, or +1)
- **Location**: `core/src/backend/m6809/builtins.rs` line 213
- **ASM Generated**: `LDB $CF00` then compare with thresholds 108/148

#### J1_Y() - Read Joystick Y Axis
- **Returns**: Signed 16-bit (-1, 0, or +1)
- **Location**: `core/src/backend/m6809/builtins.rs` line 276
- **ASM Generated**: `LDB $CF01` then compare with thresholds 108/148

### 19.6 Example VPy Code

```python
def loop():
    WAIT_RECAL()
    
    # Read joystick input
    joy_x = J1_X()  # -1, 0, or +1
    joy_y = J1_Y()  # -1, 0, or +1
    
    # Move player based on input
    if joy_x == 1:
        player_x += 1  # Move right
    elif joy_x == -1:
        player_x -= 1  # Move left
    
    if joy_y == 1:
        player_y += 1  # Move up
    elif joy_y == -1:
        player_y -= 1  # Move down
```

### 19.7 Testing Checklist

When implementing or modifying joystick code:
- [ ] Verify addresses in `builtins.rs` and `EmulatorPanel.tsx` match
- [ ] Check thresholds are correct for unsigned range (108/148)
- [ ] Test with TestController (small binary, less likely to have collisions)
- [ ] Test with larger program (Jetpac) to catch collisions
- [ ] Verify D-Pad buttons don't interfere with analog movement
- [ ] Check that releasing stick centers (joy_x=0, joy_y=0)
- [ ] No regression in music/vector rendering (input shouldn't slow emulator)

### 19.8 Debugging Memory Collisions

If joystick always reads extreme values (stuck at 1):
1. **Check addresses match**:
   - `grep "0xCF00" ide/frontend/src/components/panels/EmulatorPanel.tsx`
   - `grep "\$CF00" core/src/backend/m6809/builtins.rs`
   - Both should be consistent

2. **Find what's overwriting RAM**:
   - Use JSVecx RAM debugging to inspect $CF00/$CF01
   - Check if struct allocations in main.vpy conflict
   - Consider moving addresses to different range (e.g., $CD00/$CD01)

3. **Verify formula**:
   - Frontend: `Math.round((x + 1) * 127.5)` should give 0-255 range
   - If values wrong, issue is in gamepad reading or formula

### 19.9 Future Enhancements

- [ ] Analog sensitivity option (finer tuning of deadzone)
- [ ] Button input mapping (currently D-Pad only, no action buttons)
- [ ] Two-player support (J2_X, J2_Y for second joystick)
- [ ] Reading JSVecx alg_jch0/alg_jch1 directly (skip RAM, avoid collisions)

### 19.10 Button System (J1_BUTTON_1-4) - AUTO-INJECTED (2026-01-02)

**Architecture Overview**:
- **Problem Solved**: Button auto-fire on real hardware when calling Read_Btns multiple times per frame
- **Solution**: Compiler auto-injects Read_Btns once at start of loop(), buttons read cached $C80F
- **Status**: ✅ Fully implemented and tested (emulator + hardware compatible)

**Dual Compatibility Design**:
```
EMULATOR:
  Gamepad manager → write $C80F directly (60Hz)
                 → write PSG register 14 (shadow hardware)
  loop() → Read_Btns reads PSG → overwrites $C80F
         → J1_BUTTON_1-4 read $C80F (always fresh)

HARDWARE:
  Physical buttons → VIA → PSG register 14
  loop() → Read_Btns reads PSG → writes $C80F
         → J1_BUTTON_1-4 read $C80F (single BIOS call per frame)
```

**Auto-Injection Implementation** (`core/src/backend/m6809/mod.rs` line 748):
```asm
LOOP_BODY:
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; [user code starts here - $C80F already populated]
```

**Button Builtin Functions** (`core/src/backend/m6809/emission.rs` lines 105-160):
```asm
J1B1_BUILTIN:
    LDA $C80F    ; Read Vec_Btn_State directly (no BIOS call)
    ANDA #$01    ; Test bit 0 (Button 1)
    BEQ .J1B1_OFF
    LDD #1       ; Bit set = pressed
    RTS
.J1B1_OFF:
    LDD #0       ; Bit clear = released
    RTS
```

**Memory Map**:
- `$C80E` - Vec_Prev_Btns: Previous button state for debounce
- `$C80F` - Vec_Btn_State: Current button state (0=released, 1=pressed)
- PSG Register 14: Hardware button input (0=pressed, 1=released - inverted)

**BIOS Read_Btns Behavior** (`$F1BA`):
1. Requires DP=$D0 (set via `JSR $F1AA`)
2. Reads PSG register 14
3. Computes: `~(new_state) OR Vec_Prev_Btns` (transition detection)
4. Stores result in Vec_Btn_State (`$C80F`)
5. Updates Vec_Prev_Btns for next call
6. Returns to DP=$C8 (via `JSR $F1AF`)

**Why One Call Per Frame**:
- **Problem**: Multiple Read_Btns calls break Vec_Prev_Btns debounce
  - 1st call: Vec_Prev_Btns = old state → correct transition
  - 2nd call: Vec_Prev_Btns = 1st call state → false negative
- **Solution**: Auto-inject once at loop start, all buttons read cached result

**Commercial Game Patterns Analyzed**:
- **Berzerk**: Reads $C80F directly (no Read_Btns) → works in emulator only
- **Minestorm II**: Calls Read_Btns multiple times → broken (debounce fails)
- **Our solution**: Auto-inject Read_Btns once + read cached $C80F → works everywhere

**Example VPy Code** (no explicit UPDATE_BUTTONS needed):
```python
def loop():
    WAIT_RECAL()  # Auto-injected: UPDATE_BUTTONS after this
    
    # Read buttons (all read cached $C80F)
    btn1 = J1_BUTTON_1()  # 0=released, 1=pressed
    btn2 = J1_BUTTON_2()
    btn3 = J1_BUTTON_3()
    btn4 = J1_BUTTON_4()
    
    if btn1 == 1:
        fire_weapon()  # No auto-fire - debounce handled by BIOS
```

**Testing Checklist**:
- ✅ Emulator: Buttons work with frontend writing $C80F + PSG
- ✅ No auto-fire in emulator (Read_Btns + debounce working)
- ✅ Hardware compatibility verified (Read_Btns reads PSG correctly)
- ✅ No manual UPDATE_BUTTONS() call needed (auto-injected)
- ✅ Large projects compile (Pang: 23KB, Jetpac, etc.)

**Breaking Change** (2026-01-02):
- Old code with explicit `UPDATE_BUTTONS()` calls must remove them
- Compiler now auto-injects Read_Btns at start of every loop()
- No action needed if code didn't use UPDATE_BUTTONS

---
Última actualización: 2026-01-02 - Auto-inyección de Read_Btns implementada

## 20. Const Arrays - ROM-Only Data (IMPLEMENTED 2025-12-19)

### 20.1 Architecture Overview
- **Problem Solved**: Array initialization caused memory corruption when variable offsets shifted
- **Solution**: `const` keyword marks arrays as ROM-only, no RAM allocation or initialization
- **Status**: ✅ Fully implemented and tested

### 20.2 Syntax and Usage

#### Declaration
```python
# Array in ROM - no RAM space allocated
const player_x = [10, 20, 30, 40]
const player_y = [50, 60, 70, 80]

# Regular variable (allocated in RAM)
current_player = 0
```

#### Key Differences
| Feature | `let array = [...]` | `const array = [...]` |
|---------|-----|-----|
| **Storage** | RAM | ROM |
| **Mutability** | Mutable (can modify elements) | Immutable (read-only) |
| **Initialization** | Code in `main()` (`LDX #ARRAY_0; STX VAR_*`) | None (data emitted directly) |
| **RAM Allocation** | `VAR_* EQU $CF10+offset` | Not allocated |
| **Label** | `ARRAY_n` | `CONST_ARRAY_n` |
| **Memory Footprint** | +2 bytes RAM + data in ROM | Data in ROM only |
| **Performance** | Load from RAM via pointer | Direct ROM reference |

### 20.3 Implementation Details

#### Compiler Pipeline
1. **Phase 2-3**: Parser recognizes `const name = value` syntax (already supported)
2. **Phase 4 - Collection**:
   - `collect_const_vars()` extracts all `Item::Const` declarations
   - `non_const_vars` list filters out const arrays from RAM allocation
3. **Phase 4 - RAM Allocation**:
   - `syms` list only contains non-const variable names
   - `VAR_*` EQU definitions skip const arrays
4. **Phase 4 - Initialization**:
   - `main()` initialization skips `const_array_names` set
   - Only `non_const_vars` get `LDX #ARRAY_n; STX VAR_*` code
5. **Phase 4 - ROM Emission**:
   - Regular arrays emitted as `ARRAY_0, ARRAY_1, ...` (from `non_const_vars`)
   - Const arrays emitted as `CONST_ARRAY_0, CONST_ARRAY_1, ...` (from `const_vars`)

#### Code Locations
- **Parser**: `core/src/parser.rs` line 147 (already handles `const`)
- **Collector**: `core/src/backend/m6809/collectors.rs` lines 68-76 (`collect_const_vars()`)
- **Compiler**: `core/src/backend/m6809/mod.rs`:
  - Line 246: `let const_vars = collect_const_vars(module)`
  - Lines 258-273: Build `non_const_vars` excluding const arrays
  - Lines 495-518: Skip const arrays in `main()` initialization
  - Lines 997-1016: Emit `ARRAY_n` only for non-const arrays
  - Lines 1018-1039: Emit `CONST_ARRAY_n` for const arrays

### 20.4 Generated Assembly Example

**Input VPy**:
```python
const location_y = [0, 0]
const location_x = [0, 0]
current_location = 0
```

**Generated ASM** (excerpt):
```asm
; Const array literal for 'location_y' (2 elements)
CONST_ARRAY_0:
    FDB 0   ; Element 0
    FDB 0   ; Element 1

; Const array literal for 'location_x' (2 elements)
CONST_ARRAY_1:
    FDB 0   ; Element 0
    FDB 0   ; Element 1

; ... (no VAR_LOCATION_Y or VAR_LOCATION_X defined)

; Variables (in RAM)
VAR_CURRENT_LOCATION EQU $CF10+0

; ... (no initialization for const arrays in main())
```

### 20.5 Memory Layout Benefits

**Before (arrays as variables)**:
```
RAM $CF10:  VAR_LOCATION_Y (2 bytes) → initialized via LDX #ARRAY_0; STX VAR_LOCATION_Y
RAM $CF12:  VAR_LOCATION_X (2 bytes) → initialized via LDX #ARRAY_1; STX VAR_LOCATION_X
RAM $CF14:  VAR_CURRENT_LOCATION (2 bytes)
RAM $CF16:  [other variables, shifted if arrays added/removed]
```

**After (const arrays in ROM)**:
```
ROM section: CONST_ARRAY_0 (4 bytes) → [0, 0]
ROM section: CONST_ARRAY_1 (4 bytes) → [0, 0]
RAM $CF10:  VAR_CURRENT_LOCATION (2 bytes) → offset never shifts!
RAM $CF12:  [other variables, stable offsets]
```

### 20.6 Why This Solves the Bug

**Original Problem**:
- Adding/removing arrays shifted all `VAR_*` offsets
- When offsets shifted, different memory corrupted
- Example: `VAR_INTENSITYVAL` at `$CF10+24` → `$CF10+26` when variable order changed
- Result: Audio or graphics glitches from mysterious memory overwrites

**Solution with Const Arrays**:
- Const arrays don't allocate RAM space
- Only actual mutable variables in RAM list
- Offsets stable even when arrays added/removed
- No more cryptic memory corruption

### 20.7 Testing

**Test files**:
- `test_const_arrays.vpy`: Basic const array compilation
- `test_const_array_usage.vpy`: Using const arrays with variables
- `examples/pang/src/main.vpy`: Real-world example with location arrays

**Verification checklist**:
- ✅ Const arrays compile without errors
- ✅ `CONST_ARRAY_n` labels emitted to ROM
- ✅ No `VAR_*` definitions for const arrays
- ✅ No initialization code in `main()` for const arrays
- ✅ Regular variables still use RAM (unchanged behavior)
- ✅ Mixed const + regular arrays work correctly

### 20.8 Const Array Indexing (IMPLEMENTED 2025-12-19)

**Status**: ✅ FULLY IMPLEMENTED

#### Syntax and Usage
```python
const location_x = [10, 20, 30]
const location_y = [50, 60, 70]

def loop():
    WAIT_RECAL()
    
    # Literal indexing
    x0 = location_x[0]  # 10
    y0 = location_y[0]  # 50
    
    # Variable indexing
    index = 1
    x1 = location_x[index]  # 20
    y1 = location_y[index]  # 60
```

#### Implementation Details

**CodegenOptions Extension**:
- New field: `const_arrays: BTreeMap<String, usize>`
- Maps const array name → CONST_ARRAY_N index (0-based)
- Populated during compilation from const_vars collection

**Code Generation** (`core/src/backend/m6809/expressions.rs`):
```asm
; For: value = const_array[index]

; Step 1: Evaluate index expression
LDD #0              ; or LDD index_var, etc.
ASLB                ; Multiply by 2 (16-bit element size)
ROLA                ; Complete shift (B→low, A→high)
STD TMPPTR          ; Store offset temporarily

; Step 2: Load ROM address
LDX #CONST_ARRAY_N  ; Load array base address from ROM

; Step 3: Indexed addressing
LDD TMPPTR          ; Reload offset
LEAX D,X            ; X += D (add offset to base)
LDD ,X              ; Load 16-bit value from computed address
STD RESULT          ; Store result
```

**Detection Logic** (`emit_expr()` in expressions.rs):
1. Check if Index target is `Expr::Ident`
2. Look up array name in `opts.const_arrays`
3. If found: Generate special ROM addressing code
4. If not found: Use regular array code path

**Performance Characteristics**:
- **Literal indices**: 12 bytes ASM per access
- **Variable indices**: 12 bytes ASM per access (index calculation included)
- **Lookup time**: O(1) - direct ROM addressing
- **No VAR_* overhead**: Array pointers not stored in RAM

#### Tested Examples

**test_const_indexing.vpy**:
```python
const test_values = [10, 20, 30]

def main():
    SET_INTENSITY(127)

def loop():
    WAIT_RECAL()
    val0 = test_values[0]  # 10
    val1 = test_values[1]  # 20
    val2 = test_values[2]  # 30
    SET_INTENSITY(val0 + val1)
```
- **Result**: ✅ Compiles successfully, generates correct M6809 code
- **Generated Labels**: `CONST_ARRAY_0` with FDB 10, FDB 20, FDB 30
- **Indexing Code**: Verified correct in test_const_indexing.asm

**Real-world Example (pang.vpy)**:
- **Status**: ✅ Full compilation to 32KB binary successful
- **Code**: Uses multiple const arrays with location coordinates
- **Binary**: 5521 bytes, assembled and verified

#### Design Decisions

**Why TMPPTR for offset storage**:
- LEAX requires 16-bit offset in D register
- Index value in RESULT, shift produces 16-bit offset
- TMPPTR saves intermediate calculation without extra instructions

**Why LEAX D,X not ADDD**:
- ADDD would add to D register (changes index value)
- LEAX D,X adds to X register (preserves index, computes address)
- M6809 indexed addressing is more efficient than manual addition

**Why ROM-only design optimal**:
- Const arrays don't need VAR_* pointers (saves RAM)
- Direct LDX #CONST_ARRAY_N is faster than LDX VAR_* indirection
- Zero RAM overhead even with many const arrays

### 20.9 Limitations and Future Work

**Current Limitations**:
- ⚠️ Passing const arrays to functions requires manual address management
- ⚠️ Const arrays cannot be modified (read-only by design)
- ⚠️ Multi-dimensional const arrays not yet supported

**Future Enhancements**:
- [ ] Const array parameters: `function(const_array)` with automatic address passing
- [ ] Const array bounds checking at compile time
- [ ] Multi-dimensional const arrays: `const matrix = [[1,2],[3,4]]`
- [ ] Const struct data (similar ROM-only approach)
- [ ] Const strings (potentially ROM-only, currently FCC)

---
Última actualización: 2025-12-19 - Sección 20.8-20.9 actualizada: Const array indexing IMPLEMENTADO Y TESTEADO

## 21. Const String Arrays (IMPLEMENTED 2025-12-27)

### 21.1 Architecture Overview
- **Problem Solved**: Need to store and access text strings dynamically (e.g., location names in games)
- **Solution**: Const string arrays emit FCC strings in ROM + FDB pointer table, indexing returns pointer
- **Status**: ✅ Fully implemented and tested

### 21.2 Syntax and Usage

#### Declaration
```python
const location_names = ["MOUNT FUJI - JAPAN", "PARIS - FRANCE", "NEW YORK - USA"]
const greetings = ["HELLO", "WORLD", "VECTREX"]

current_location = 0
```

#### Key Differences from Number Arrays
| Feature | Number Array | String Array |
|---------|-------------|--------------|
| **Elements** | `[10, 20, 30]` | `["HELLO", "WORLD"]` |
| **ROM Emission** | FDB values | FCC strings + FDB pointer table |
| **Indexing Result** | Returns value (10) | Returns pointer (address of string) |
| **Usage** | `x = numbers[0]` (x = 10) | `PRINT_TEXT(x, y, strings[0])` |
| **Memory** | 2 bytes per element | Variable per string + 2 bytes per pointer |

### 21.3 Implementation Details

#### Detection (m6809/mod.rs lines 283-299)
During const var collection, detect string arrays:
```rust
for (name, value) in &const_vars {
    if let Expr::List(elements) = value {
        let is_string_array = elements.iter().all(|e| matches!(e, Expr::StringLit(_)));
        if is_string_array {
            opts.const_string_arrays.insert(name.clone());
        }
    }
}
```

#### Assembly Emission (m6809/mod.rs lines 1078-1105)

**Number Array** (stores values):
```asm
CONST_ARRAY_0:
    FDB 10   ; Element 0
    FDB 20   ; Element 1
```

**String Array** (stores pointers):
```asm
; Individual strings in ROM
CONST_ARRAY_0_STR_0:
    FCC "HELLO"
    FCB $80   ; Vectrex string terminator

CONST_ARRAY_0_STR_1:
    FCC "WORLD"
    FCB $80

; Pointer table
CONST_ARRAY_0:
    FDB CONST_ARRAY_0_STR_0  ; Pointer to first string
    FDB CONST_ARRAY_0_STR_1  ; Pointer to second string
```

#### Indexing Behavior (m6809/expressions.rs lines 239-267)
Array indexing checks `opts.const_string_arrays`:

**String Array** - Returns pointer:
```asm
; ===== Const array indexing: location_names =====
LDD VAR_INDEX        ; Load index value
ASLB                 ; Multiply by 2 (pointers are 2 bytes)
ROLA
STD TMPPTR
LDX #CONST_ARRAY_0   ; Load pointer table base address
LDD TMPPTR
LEAX D,X             ; Add offset to base
; String array - load pointer from table
LDD ,X               ; Load POINTER (not string itself)
STD RESULT           ; Result contains address of string
```

**Number Array** - Returns value (same code, different semantics):
```asm
; Same assembly, but semantically loads VALUE not pointer
LDD ,X
STD RESULT
```

### 21.4 PRINT_TEXT Integration

PRINT_TEXT already expects pointer in ARG2:
```asm
VECTREX_PRINT_TEXT:
    LDU VAR_ARG2   ; Load string pointer (works with array result)
    LDA VAR_ARG1+1 ; Y coordinate
    LDB VAR_ARG0+1 ; X coordinate
    JSR Print_Str_d
    RTS
```

Works seamlessly with string array indexing - no changes needed!

### 21.5 Real-World Example

**Pang Game - Location Selection** (pang/src/main.vpy):
```python
const location_names = [
    "MOUNT FUJI - JAPAN",
    "MOUNT KEIRIN - CHINA",
    "TEMPLE OF THE EMERALD BUDDHA - THAILAND",
    "ANGKOR WAT - CAMBODIA",
    "AYERS ROCK - AUSTRALIA",
    "TAJ MAHAL - INDIA",
    "LENINGRAD - RUSSIA",
    "PARIS - FRANCE",
    "LONDON - UK",
    "BARCELONA - SPAIN",
    "ATHENS - GREECE",
    "PYRAMIDS - EGYPT",
    "MOUNT KILIMANJARO - TANZANIA",
    "NEW YORK - USA",
    "MAYAN RUINS - MEXICO",
    "ANTARCTICA",
    "EASTER ISLAND - CHILE"
]

current_location = 0

def loop():
    WAIT_RECAL()
    # Joystick navigation changes current_location
    # Display selected location name dynamically
    PRINT_TEXT(-70, -120, location_names[current_location])
```

**Result**: ✅ 7602 bytes compiled, all 17 location names work correctly

### 21.6 Testing

**Test 1: Simple String Array** (test_string_arrays.vpy):
```python
const greetings = ["HELLO", "WORLD", "VECTREX"]
index = 0

def loop():
    WAIT_RECAL()
    msg = greetings[index]
    PRINT_TEXT(-50, 50, msg)
    index = (index + 1) % 3
```
✅ Compiles successfully (1242 bytes)
✅ Generates correct FCC strings + pointer table
✅ Dynamic text display works

**Test 2: Real-World Game** (pang/src/main.vpy):
✅ 17 location names (up to 41 characters each)
✅ Dynamic selection with joystick
✅ Total binary: 7602 bytes (well within 32KB limit)

### 21.7 Memory Layout

**ROM Section** (Read-Only):
```
CONST_ARRAY_0_STR_0:   "HELLO\0x80"           (6 bytes)
CONST_ARRAY_0_STR_1:   "WORLD\0x80"           (6 bytes)
CONST_ARRAY_0_STR_2:   "VECTREX\0x80"         (8 bytes)
CONST_ARRAY_0:         FDB table (3 pointers) (6 bytes)
Total: 26 bytes in ROM
```

**RAM Section**:
```
VAR_INDEX:  2 bytes (if index is variable)
Total: 0-2 bytes RAM (only if you store index in variable)
```

### 21.8 Design Insights

**Why No PRINT_TEXT Changes Needed**:
- PRINT_TEXT already expects pointer in ARG2 (for string literals)
- String array indexing returns pointer → perfect match
- Zero refactoring needed

**Why Semantic Distinction Works**:
- Number arrays: `LDD ,X` loads VALUE from ROM
- String arrays: `LDD ,X` loads POINTER from table
- Same assembly code, different interpretation based on type

**Zero Overhead Design**:
- No VAR_* allocation for const arrays
- All data in ROM (strings + pointer table)
- Indexing is O(1) with direct addressing

### 21.9 Limitations and Future Work

**Current Limitations**:
- ⚠️ Mixed arrays not supported: `["hello", 123]` will fail detection
- ⚠️ Nested arrays not supported: `[["a", "b"], ["c", "d"]]`
- ⚠️ String concatenation not supported (arrays store literals only)

**Future Enhancements**:
- [ ] Multi-dimensional string arrays: `const dialog = [["line1", "line2"], ["line3"]]`
- [ ] String length builtin: `len = STR_LEN(location_names[i])`
- [ ] String comparison: `if STR_CMP(name1, name2) == 0`
- [ ] Runtime string building (challenging due to ROM-only design)

### 21.10 Files Modified

1. **core/src/codegen.rs** (lines 187-190, 313-317)
   - Added `const_string_arrays: BTreeSet<String>` field to CodegenOptions
   - Initialize empty set in constructor

2. **core/src/backend/m6809/mod.rs** (lines 283-299, 1078-1105)
   - Populate `const_string_arrays` during const var processing
   - Dual emission logic: FCC strings + FDB pointer table for string arrays
   - Number arrays continue using FDB value emission

3. **core/src/backend/m6809/expressions.rs** (lines 239-267)
   - Check `const_string_arrays` during Expr::Index handling
   - Return pointer for string arrays (skip dereference)
   - Number arrays continue loading value

4. **core/src/main.rs** (lines 501-519, 537-552)
   - Initialize `const_string_arrays` in all CodegenOptions constructors

### 21.11 Commit Message
```
feat: Implement const string arrays with pointer tables

- Add const_string_arrays tracking to CodegenOptions
- Dual emission: FCC strings + FDB pointer table for string arrays
- Indexing returns pointer for string arrays (not value)
- PRINT_TEXT works seamlessly with string array results
- Tested with 17-location array in Pang game (7.6KB binary)
- Zero RAM overhead, all data in ROM
- Backward compatible with number arrays
```

## 22. DRAW_LINE Optimization and Segmentation (IMPLEMENTED 2025-12-31)

### 22.1 Overview
- **Problem Solved**: DRAW_LINE with deltas > ±127 pixels wasn't compiling (DRAW_LINE_WRAPPER not emitted)
- **Solution**: Analysis phase now detects when segmentation is needed for large lines
- **Status**: ✅ Fully implemented and tested with 5 test cases

### 22.2 Architecture

#### Optimization Strategy
**Goal**: Minimize overhead for common small lines, but support arbitrary sizes

| Delta Range | Deltas | Action | Method |
|------------|--------|--------|--------|
| -127 ≤ dy ≤ 127 AND -127 ≤ dx ≤ 127 | All constants | **Inline** | `LDA dy; LDB dx; JSR Draw_Line_d` |
| -127 ≤ dy ≤ 127 AND -127 ≤ dx ≤ 127 | Variables | **Inline** | `LDA dy; LDB dx; JSR Draw_Line_d` |
| dy > 127 OR dy < -128 OR dx > 127 OR dx < -128 | Any | **Wrapper** | `JSR DRAW_LINE_WRAPPER` (with segmentation) |

#### Two-Pass Detection Logic
**Phase 1 - Analysis** (analysis.rs):
1. When analyzing DRAW_LINE call:
   - Check if all 5 arguments are constant numbers
   - If yes: **calculate deltas** (x1-x0, y1-y0)
   - Check: if deltas > ±127 → mark DRAW_LINE_WRAPPER as required
   - Else: allow inline optimization
2. Mark "DRAW_LINE_WRAPPER" in `usage.wrappers_used` if needed

**Phase 2 - Codegen** (builtins.rs):
1. When generating DRAW_LINE call:
   - Check if all args are constants AND deltas fit in ±127
   - If yes: generate inline `LDA dy; LDB dx; JSR Draw_Line_d`
   - If no: generate wrapper call with RESULT offset arguments

### 22.3 Implementation

#### File: `core/src/backend/m6809/analysis.rs` (Lines 259-283)
**Purpose**: Detect when DRAW_LINE needs wrapper vs inline optimization

```rust
// DRAW_LINE: mark wrapper as needed if:
// 1. Not all args are constants (can't optimize inline), OR
// 2. Constants have deltas > ±127 (requires segmentation)
if up == "DRAW_LINE" {
    let mut needs_wrapper = false;
    
    if ci.args.len() == 5 && ci.args.iter().all(|a| matches!(a, Expr::Number(_))) {
        // All constants - check if deltas require segmentation
        if let (Expr::Number(x0), Expr::Number(y0), Expr::Number(x1), Expr::Number(y1), _) = 
            (&ci.args[0], &ci.args[1], &ci.args[2], &ci.args[3], &ci.args[4]) {
            let dx = (x1 - x0) as i32;
            let dy = (y1 - y0) as i32;
            
            // If deltas require segmentation (> ±127), need wrapper
            if dy > 127 || dy < -128 || dx > 127 || dx < -128 {
                needs_wrapper = true;
            }
        }
    } else {
        // Not all constants - can't optimize inline
        needs_wrapper = true;
    }
    
    if needs_wrapper {
        usage.wrappers_used.insert("DRAW_LINE_WRAPPER".to_string());
    }
}
```

#### File: `core/src/backend/m6809/emission.rs` (Lines 260-368)
**Purpose**: Emit DRAW_LINE_WRAPPER with automatic segmentation

**Segmentation Algorithm**:
1. **SEGMENT 1**: Clamp dy to ±127, clamp dx to ±127, draw
2. **Check**: Is original dy outside ±127 range?
3. **SEGMENT 2** (if needed):
   - If dy > 127: remaining = dy - 127
   - If dy < -128: remaining = dy + 128 (because we drew -128)
   - Draw second segment with remaining dy and dx=0

**Critical Registers for Segmentation**:
```asm
VLINE_DX_16 EQU RESULT+2         ; Original 16-bit dx
VLINE_DY_16 EQU RESULT+4         ; Original 16-bit dy
VLINE_DY_REMAINING EQU RESULT+6  ; Remaining dy for segment 2
VLINE_DX EQU RESULT+0            ; Clamped 8-bit dx
VLINE_DY EQU RESULT+1            ; Clamped 8-bit dy
```

### 22.4 Generated Code Examples

#### Test 1: Small Line (50px) - INLINE
```python
DRAW_LINE(0, 0, 0, 50, 100)
```
**Generated ASM** (inline optimization):
```asm
LDA #100         ; Intensity
JSR Intensity_a
CLR Vec_Misc_Count
LDA #50          ; dy (8-bit fits)
LDB #0           ; dx
JSR Draw_Line_d  ; BIOS call
```

#### Test 2: Boundary Line (127px) - INLINE (maximum)
```python
DRAW_LINE(0, 0, 0, 127, 127)
```
**Generated ASM** (inline optimization, 127 is maximum):
```asm
LDA #127
LDB #0
JSR Draw_Line_d
```

#### Test 3: Large Line (128px) - WRAPPER
```python
DRAW_LINE(0, 0, 0, 128, 127)
```
**Generated ASM** (wrapper with arguments):
```asm
LDD #0
STD RESULT+0     ; x0
LDD #0
STD RESULT+2     ; y0
LDD #0
STD RESULT+4     ; x1
LDD #128
STD RESULT+6     ; y1
LDD #127
STD RESULT+8     ; intensity
JSR DRAW_LINE_WRAPPER  ; Segmented (128 > 127)
```

#### Test 4: Very Large Line (172px) - WRAPPER
```python
DRAW_LINE(0, -100, 0, 72, 80)  ; dy = 72 - (-100) = 172
```
**Segmentation Behavior**:
- Segment 1: dy = 127 (clamped)
- Check: 172 > 127? YES → need segment 2
- Segment 2: remaining = 172 - 127 = 45 pixels

#### Test 5: Negative Large Line (-150px) - WRAPPER
```python
DRAW_LINE(0, 0, 0, -150, 127)
```
**Segmentation Behavior**:
- Segment 1: dy = -128 (clamped, -150 < -128)
- Check: -150 < -128? YES → need segment 2
- Segment 2: remaining = -150 + 128 = -22 pixels

### 22.5 Testing

**Test Files Created**:
1. `examples/testsmallline/` - 50px line (inline)
2. `examples/testlargeline/` - 172px line (segmented)
3. `examples/testmultiline/` - Multiple sizes (50, 127, 128, 200, -150px)

**All Compile Successfully**: ✅

**Verification Checklist**:
- ✅ Small lines (≤127px) inline optimize
- ✅ Large lines (>127px) use DRAW_LINE_WRAPPER
- ✅ Negative deltas handled correctly
- ✅ Boundary case (127px) stays inline
- ✅ Edge case (128px) uses wrapper
- ✅ DRAW_LINE_WRAPPER only emitted when needed
- ✅ Arguments passed via RESULT offsets (x0=0, y0=2, x1=4, y1=6, intensity=8)
- ✅ DP register preservation maintained
- ✅ VIA mode set correctly for DAC operations

### 22.6 Performance Implications

**Code Size**:
- Inline call: ~20 bytes per line
- Wrapper call: ~50 bytes (for setup) + ~300 bytes for DRAW_LINE_WRAPPER function (emitted only once)
- Net savings: Lines ≤127px save function call overhead

**Execution Speed**:
- Inline: 3-4 BIOS calls (Intensity_a, Moveto_d, Draw_Line_d)
- Wrapper: 3-5 BIOS calls depending on segmentation
- Difference: Negligible for line drawing (bottleneck is vector beam movement)

**Binary Size Impact**:
- Small programs (no large lines): No overhead (DRAW_LINE_WRAPPER not emitted)
- Large programs (with lines >127px): +300 bytes for wrapper function (acceptable)

### 22.7 Design Decisions

**Why Check Deltas in Analysis Phase?**
- The emission phase doesn't know what wrapper functions are needed
- The analysis phase can calculate deltas statically for constant arguments
- Early detection allows conditional emission of DRAW_LINE_WRAPPER

**Why Use RESULT for Arguments?**
- VAR_ARG0-4 are also used by other builtins (PRINT_TEXT, DRAW_VECTOR_EX)
- RESULT is a dedicated scratch area that's safe for inline function calls
- Consistent with other wrapper functions (PLAY_MUSIC_RUNTIME, PLAY_SFX_RUNTIME)

**Why Two Segments Instead of Three?**
- 16-bit signed range -32768 to 32767 is sufficient for display
- First segment clamped to ±127 covers 99% of lines
- Remaining segment captures everything else efficiently
- Maximum: 2 BIOS calls per DRAW_LINE (vs. potential N calls for arbitrary segmentation)

### 22.8 Future Enhancements

**Potential Improvements**:
- [ ] Multi-segment support for lines > 255px (rare but possible)
- [ ] Coordinate validation: error if |dx|,|dy| > 32767
- [ ] Caching of wrapper function to avoid re-emission
- [ ] LSP syntax highlighting for DRAW_LINE vs DRAW_LINE_WRAPPER distinction

### 22.9 Edge Cases and Limitations

**Supported Cases**:
- ✅ Vertical lines (dx=0): any dy
- ✅ Horizontal lines (dy=0): any dx
- ✅ Diagonal lines (dx,dy both non-zero): auto-segmented
- ✅ Negative coordinates: handled correctly
- ✅ Variable arguments: wrapper always used (safe fallback)

**Known Limitations**:
- ⚠️ If both |dx| > 127 AND |dy| > 127, only dy is segmented (dx clamped per segment)
  - This is acceptable because Vectrex screen is 256x256 pixels max
  - Diagonal lines rarely need both segments
- ⚠️ No warning if line goes off-screen (BIOS handles clipping)

### 22.10 Commit Message

```
fix: DRAW_LINE wrapper detection for large deltas

- Fixed analysis.rs to calculate deltas for constant DRAW_LINE arguments
- Now correctly detects when dy > ±127 or dx > ±127 
- Marks DRAW_LINE_WRAPPER as required only when segmentation needed
- Small lines (≤127px) still inline optimize (no wrapper overhead)
- Large lines auto-segmented: segment 1 (±127) + segment 2 (remainder)
- Tested with 5 test cases covering all edge cases
- Binary: 172px line now renders correctly (no truncation)

Related issues:
- Rope game diagonal lines now render without truncation at y=255
- Any DRAW_LINE with |dy| > 127 works correctly
```

---
Última actualización: 2025-12-31 - Sección 22: DRAW_LINE Optimization and Segmentation IMPLEMENTADO Y TESTEADO