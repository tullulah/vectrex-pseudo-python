# Compiler Status (vectrex_lang)

Fecha: 2025-09-20 (actualizado tras implementación DRAW_TO)

Este documento resume el estado actual del compilador DSL (`vectrex_lang`, carpeta `core/`), capacidades implementadas, carencias detectadas y backlog priorizado.

## 1. Alcance Actual
- Objetivo: Generar ensamblador (principalmente 6809 para Vectrex) desde un DSL inspirado en Python con bloques por indentación y DSL de listas vectoriales.
- Arquitectura front-end: lexer -> parser -> AST -> optimizaciones -> codegen -> backend específico (m6809, placeholders arm/cortexm).
- Sin fase explícita de tipos: modelo de 16 bits entero truncado.

## 2. Lexing
Capacidades:
- Indent/Dedent con ancho fijo (4 espacios) generando tokens `Indent`, `Dedent`, `Newline`.
- Números: decimal, hex (`0x`), binario (`0b`), soporte de signo vía gramática (unario) no en el literal.
- Strings: comillas dobles, escapes `\n`, `\r`, `\t`, `\\xHH` (hex byte), terminan en high-bit 1 al emitir (backend) para usar con BIOS `Print_Str_d`.
- Identificadores: case-insensitive para palabras clave re-detectadas (ej. `CONST`, `META`).
- Palabras clave: `def, let, for, in, range, while, if, elif, else, switch, case, default, return, break, continue, true, false, const, var, meta, vectorlist`.

Limitaciones / Pendiente:
- No soporte de comentarios multilínea (solo uno-línea si existe, confirmar si se desea extensión).
- No hay tokens para anotaciones / decoradores.

## 3. AST / Semántica
Nodos principales:
- `Module` con `items` y `ModuleMeta` (title/music/copyright overrides + metas arbitrary).
- Items: `Function`, `Const`, `GlobalLet` (`var`), `VectorList`.
- `VectorList` entradas: Intensity, Origin, Move, Rect, Polygon, Circle, Arc, Spiral.
- Expresiones: aritmética, bitwise, lógicas, comparaciones encadenadas, llamadas, literales número/string, identificadores, `!`, `~`, negación unaria.
- Sentencias: asignación, let, for-range (con optional step), while, if/elif/else, switch/case/default, return, break, continue, expresión.

Limitaciones:
- Sin tipos ni verificación semántica (shadowing, parámetros, número de argumentos se deja al backend/errores posteriores).
- No soporte de arrays, structs, tuplas, lambdas, closures, import/export de módulos.
- No hay macros generales (solo mini-folding de expresiones numéricas para vectorlists y optimizaciones posteriores).

## 4. Optimización (codegen.rs)
Pipeline iterativo (hasta fixpoint, máx 5 pasadas):
1. Constant folding y simplificaciones algebraicas (16-bit wrap) sobre expresiones.
2. `dead_code_elim`: elimina ramas inalcanzables simples (if con constante, while 0, for vacío).
3. `propagate_constants`: (implementación existente en archivo — no inspeccionada aún en detalle, asumir propagación; revisar en mejora futura) ; mantiene semantics.
4. `dead_store_elim`: elimina asignaciones no usadas (excepto si contienen llamada o string literal).
5. `fold_const_switches`: pliega switch completamente constante (si aplica).

Observado:
- Identidades bitwise y aritméticas aplicadas (AND con 0, OR con 0, etc.).
- Comparaciones y lógicas se pliegan cuando ambos operandos constantes.

Riesgos / Pendiente:
- No hay control de explosión de tamaño en desdoblamientos (OK por ahora: no hay inlining ni unrolling agresivo).
- Falta pase de verificación de que variables usadas fueron declaradas.

## 5. Backend 6809 (backend/m6809.rs)
- Emite prólogo, asignación de argumentos (hasta 4), stack frame (2 bytes por local 16-bit).
- Wrappers condicionales generados en función de `RuntimeUsage` (intensidad, move, draw line, vector phase, blink, silence, debug, frame begin, set origin, draw VL, wait recal fast).
- `VECTREX_DRAW_TO` ahora dibuja realmente una línea: calcula dx/dy respecto a (VCUR_X,VCUR_Y), hace clamp (-64..63) y llama a `Draw_Line_d`, luego actualiza VCUR_*.
- Opciones `CodegenOptions` específicas Vectrex: `per_frame_silence`, `debug_init_draw`, `blink_intensity`, `exclude_ram_org`, `fast_wait`.

Limitaciones:
- No tracking de registro usado / allocation: uso directo de variables globales (`VAR_ARG*`, `VCUR_X/Y`).
- No soporte banco/align a pesar de `_bank_size` placeholder.

## 6. Otros Backends
- `arm`, `cortexm`: presentes (no analizados en detalle) — posible estado placeholder o parcial.
- Sin IR intermedio independiente (backend trabaja directo sobre AST optimizada).

## 7. WASM / LSP
- `wasm_api.rs` inicialmente presente, ahora la emulación se trasladó a crate `vectrex_emulator`. Archivo residual indica API previa (posible limpieza futura o migrar wrappers específicos del compilador si fuese necesario).
- `lsp.rs` (no leído aún) provee diagnósticos de posiciones (tests indican verificación de mapping tokens->diagnostics).

## 8. Testing Actual
- Tests de diagnósticos (posiciones) en `core/tests/diagnostics_positions.rs`.
- Ausencia de smoke test mínimo (parse + codegen de ejemplo trivial) = brecha detectada.
- Sin tests de optimizaciones (constant folding, dead store) para garantizar invariantes.

## 9. Riesgos / Debilidades
- Ausencia de verificación semántica: errores silenciosos si variable no declarada antes de uso o aridad incorrecta de llamadas.
- Falta test formal para wrappers del backend (regresión riesgo al modificar emitters).
- `VECTREX_DRAW_TO` incompleto puede inducir usuarios a esperar dibujo real.
- Sin control de overflow int (se trunca siempre a 16 bits — documentar claramente).

## 10. Backlog Prioritario (Short / Mid / Long)
Short (1-2 semanas):
1. (S1) Smoke test básico – COMPLETADO 2025-09-20 (`core/tests/smoke_compile.rs`).
2. (S2) `VECTREX_DRAW_TO` real – COMPLETADO 2025-09-20.
3. (S3) Añadir verificación simple de variables (pendiente).
4. (S4) Tests constant folding (pendiente).
5. (S5) Documentar truncamiento 16-bit también en `SUPER_SUMMARY.md` (pendiente).

Mid (3-6 semanas):
6. IR intermedio opcional (linear SSA-lite o tree simplificado) para separar optimizaciones de AST. (ID M1)
7. Liveness + mejora dead_store_elim (detectar efectos laterales). (ID M2)
8. Detección de aridad en llamadas a builtins / wrappers. (ID M3)
9. Tests para switch folding y dead_code_elim. (ID M4)
10. Soporte de INCLUDE/IMPORT de módulos (sin circular). (ID M5)

Long (6+ semanas):
11. Sistema de tipos básico (int vs maybe future fixed-point). (ID L1)
12. Optimización vectorlist: coalescer movimientos consecutivos y normalizar intensidades redundantes. (ID L2)
13. Backend bank/align real + linker map. (ID L3)
14. Emisión opcional a formato binario directo (ensamblador integrado). (ID L4)
15. Pipeline de depuración: mapa fuente->dirección (DWARF-like minimal). (ID L5)

## 11. Métricas / KPIs Iniciales (a definir)
- Tiempo de compilación (ms) para smoke test.
- Recuento de optimizaciones aplicadas (pliegues, stores eliminados) — instrumentar contadores.
- Cobertura tests: % de nodos AST visitados en suite (meta inicial: >60%).

## 12. Próximos Pasos Inmediatos (actualizado)
1. S3: Verificación semántica básica de variables.
2. S4: Tests constant folding / dead store.
3. S5: Doc truncamiento 16-bit en `SUPER_SUMMARY.md`.

---
Notas de mantenimiento: mantener este archivo actualizado cuando se cierren IDs. Añadir fecha y breve changelog al inicio.

---
Changelog:
- 2025-09-20: Añadido smoke test (S1) y wrapper `VECTREX_DRAW_TO` implementado (S2). Actualizada sección backend y backlog.
