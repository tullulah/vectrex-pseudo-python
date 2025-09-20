# Copilot Project Instructions (Persistent Reminders)

These guidelines are critical for ongoing work in this repository. Keep them in mind for every future change.

## 1. BIOS Usage
- Nunca generar BIOS sintética en tests ni código de ejemplo.
- Usar siempre la BIOS real en: `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\ide\frontend\dist\bios.bin`.
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

## 8. Documentación
- Actualizar `SUPER_SUMMARY.md` cuando se introduce o cambia: tracing, nuevas etiquetas BIOS, métricas, o comportamiento de integrator.
- Añadir nota de migración en `MIGRATION_WASM.md` si se modifica la superficie WASM.

## 9. Rutas y Constantes Críticas
- Ruta BIOS absoluta (ver sección 1) debe quedar centralizada en helpers de test si se multiplica su uso.
- Evitar duplicación de la cadena de ruta en muchos archivos (refactor pendiente cuando aparezca el segundo uso).

## 10. Próximos TODO Prioritarios
1. (ID 11) Comparar call stack con `bios.asm` y etiquetar direcciones faltantes (ej.: 0xF18B).  [Actualización 2025-09-20: 0xF18B mapeado como Init_OS]
2. (ID 13) Exportar call stack vía WASM (`bios_calls_json`).
3. (ID 5) Resumen estado compilador (análisis de `vectrex_lang` pendiente).

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

---
Última actualización: (auto) mantener este archivo conforme se completen los TODOs listados.
