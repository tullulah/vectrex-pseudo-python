# Copilot Project Instructions (Persistent Reminders)

These guidelines are critical for ongoing work in this repository. Keep them in mind for every future change.

## 0. PowerShell Usage
- Usuario usa Windows PowerShell v5.1 (NO PowerShell 7+).
- NUNCA usar `&&` para concatenar comandos - usar `;` en su lugar.
- Sintaxis correcta: `cd emulator; cargo build` (NO `cd emulator && cargo build`).
- PowerShell v5.1 no soporta `&&` como separador de comandos.

## 0.1. REGLA CRÍTICA: VERIFICACIÓN 1:1 OBLIGATORIA
**ANTES DE CREAR CUALQUIER ARCHIVO O API**:
1. **VERIFICAR EXISTENCIA**: Comprobar si existe en `vectrexy_backup/libs/emulator/src/` y `vectrexy_backup/libs/emulator/include/emulator/`
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
1. `ls vectrexy_backup/libs/emulator/src/` 
2. `cat ArchiveCorrespondiente.cpp` 
3. `cat ArchiveCorrespondiente.h`
4. Implementar EXACTAMENTE lo que dice el código original
5. NUNCA implementar tests/APIs hasta verificar paso 1-4

## 1. BIOS Usage
- Nunca generar BIOS sintética en tests ni código de ejemplo.
- Rutas válidas (mantenidas en sincronía, preferir la de assets para futuras referencias):
	- Primaria (assets): `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\src\assets\bios.bin`
	- Legacy (dist empaquetado actual): `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin`
	(Si divergen, actualizar ambas o unificar mediante script de build.)
- Si se necesita ruta en WASM/frontend, exponer una única función helper (pending) o documentar claramente.

## 2. Call Stack / BIOS Tracing
- Registrar llamadas BIOS reales via `record_bios_call` únicamente en JSR/BSR hacia >= 0xF000.
- Evitar falsos positivos: no fabricar llamadas manualmente salvo hooks explícitos.
- Próximo paso pendiente: mapear direcciones desconocidas como 0xF18B a etiquetas reales revisando `bios.asm` y actualizar `record_bios_call`.
- Añadir export WASM: `bios_calls_json()` (pendiente: TODO id 13).

## 3. Tests
- Tests deben usar la BIOS real (ver ruta arriba) y no escribir versiones sintéticas.
- Si un test necesita un escenario concreto, manipular RAM/cart, nunca la ROM.
- Mantener tests resilientes ante timing: usar umbrales (máx pasos) y verificar aparición de símbolos, luego endurecer cuando el etiquetado sea completo.

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
- **Referencia obligatoria**: `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\vectrexy_backup\libs\emulator\` (archivos .h/.cpp)
- **IMPORTANTE**: Usar `vectrexy_backup` NO `vectrexy` - la carpeta `vectrexy` puede haber sido modificada por nosotros.
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
`C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\vectrexy\libs\vectrexy`

Política:
1. Usar esta referencia únicamente para confirmar orden y efectos (nunca portar bloques de código textualmente — mantener originalidad y evitar problemas de copyright).
2. Si se detecta divergencia entre nuestra emulación y la referencia, primero instrumentar y demostrar con logs antes de cambiar lógica.
3. Cualquier corrección derivada debe anotar brevemente en `SUPER_SUMMARY.md` (sección CPU/VIA) el aspecto validado y la fecha.
4. Mantener comentarios críticos en español (o bilingües) al introducir cambios basados en esta verificación.

## 16. JavaScript Node.js Testing Harness (Context Preservation)

### 16.1 Scripts de Comparación Disponibles
Para evitar pérdida de contexto y mantener comparaciones Rust vs JavaScript:

#### A) test_f4eb_detailed_js.js (F4EB Loop Analysis)
- **Ubicación**: `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\test_f4eb_detailed_js.js`
- **Propósito**: Análisis específico del bucle infinito F4EB con detección automática y captura de estado VIA
- **Uso**: `node test_f4eb_detailed_js.js`
- **Características**:
  - Hook e6809_sstep personalizado para monitoring step-by-step
  - Detección automática al llegar a PC=F4EB
  - Captura completa de registros CPU y estado VIA (Timer2 en 0xD05A)
  - Logging detallado de cambios de PC y contadores de loop
  - Comparación directa con baseline Rust (Timer2=0xFF, Cycles=5342)

#### B) jsvecx_comparison.js (General Comparison Framework)
- **Ubicación**: `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\jsvecx_comparison.js`
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

---
Última actualización: (auto) mantener este archivo conforme se completen los TODOs listados.
