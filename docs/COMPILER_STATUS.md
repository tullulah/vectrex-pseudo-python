# Compiler Status (vectrex_lang)

**Última actualización:** Noviembre 15, 2025

## ESTADO ACTUAL: ENSAMBLADOR NATIVO M6809 - IMPLEMENTACIÓN MASIVA ✅

### Hito Principal: Ensamblador Binario Nativo Completo

**FECHA**: Noviembre 12-15, 2025  
**LOGRO**: Implementadas **23+ instrucciones MC6809** nuevas para eliminar dependencia de lwasm  
**IMPACTO**: rotating_line_correct.vpy progresa de 40 líneas → 242+ líneas (6x mejora)

### Instrucciones Implementadas (Sesión Nov 12-15)

#### Operaciones de Carga/Almacenamiento 16-bit
- ✅ **LDU** (Load U register): immediate (0xCE), extended (0xFE)
- ✅ **STU** (Store U register): extended (0xFF)
- ✅ **LDD indexed** (0xEC + postbyte): Soporte para ,X ,Y ,U ,S sin offset

#### Operaciones Lógicas y Aritméticas 8-bit
- ✅ **ANDB** (AND B): immediate (0xC4)
- ✅ **ASLA/ASLB** (Arithmetic Shift Left): 0x48/0x58
- ✅ **ROLA/ROLB** (Rotate Left through Carry): 0x49/0x59
- ✅ **LSRA/LSRB** (Logical Shift Right): 0x44/0x54
- ✅ **RORA/RORB** (Rotate Right through Carry): 0x46/0x56

#### Operaciones Aritméticas 16-bit
- ✅ **ADDD** (Add to D): immediate (0xC3), extended (0xF3)
- ✅ **SUBD** (Subtract from D): immediate (0x83), extended (0xB3)
- ✅ **ABX** (Add B to X): 0x3A

#### Comparaciones 16-bit
- ✅ **CMPD** (Compare D): immediate (0x1083), extended (0x10B3)

#### Operaciones de Memoria
- ✅ **CLR** (Clear memory): extended (0x7F)

#### Saltos Largos (Long Branches - 16-bit offset)
- ✅ **LBRA** (Long Branch Always): 0x16
- ✅ **LBEQ** (Long Branch if Equal): 0x1027
- ✅ **LBNE** (Long Branch if Not Equal): 0x1026
- ✅ **LBCS** (Long Branch if Carry Set): 0x1025
- ✅ **LBCC** (Long Branch if Carry Clear): 0x1024
- ✅ **LBLT** (Long Branch if Less Than): 0x102D
- ✅ **LBGE** (Long Branch if Greater or Equal): 0x102C
- ✅ **LBGT** (Long Branch if Greater Than): 0x102E
- ✅ **LBLE** (Long Branch if Less or Equal): 0x102F
- ✅ **LBMI** (Long Branch if Minus): 0x102B
- ✅ **LBPL** (Long Branch if Plus): 0x102A

#### Aliases de Instrucciones
- ✅ **BLO** (Branch if Lower): alias de BCS (0x25)
- ✅ **BHS** (Branch if Higher or Same): alias de BCC (0x24)

#### Mejoras de Branch Condicionales
- ✅ **BCS/BCC con labels**: Ahora soportan referencias a símbolos correctamente

### Resultados de Compilación

**Archivo de Prueba: rotating_line_correct.vpy**
- **Antes (Nov 12)**: Fallaba en línea 40 (LDU no soportado)
- **Progreso (Nov 15)**: Procesa hasta línea 242+ (CLR Vec_Misc_Count)
- **Mejora**: 6x más código procesado con ensamblador nativo
- **Estado actual**: Solo pendiente resolución de símbolos BIOS en second pass

**Beneficios Técnicos:**
1. ✅ Mayor cobertura de opcodes MC6809 (23 nuevas instrucciones)
2. ✅ Soporte completo de operaciones 16-bit (ADDD, SUBD, CMPD, LDD indexed)
3. ✅ Saltos largos para programas grandes (LBEQ, LBNE, etc.)
4. ✅ Modos de direccionamiento indexed básicos (,X ,Y ,U ,S)
5. ✅ Menor dependencia de lwasm fallback

## ARQUITECTURA SUBRUTINAS (Oct 2025) ✅

**FECHA**: 2025-10-01  
**PROBLEMA RESUELTO**: BRA overflow en programas grandes eliminado completamente  
**SOLUCIÓN**: Arquitectura de subrutinas JSR/RTS en lugar de código inline duplicado  

### Resultados Verificados:
- ✅ **test_vectrex_pattern.vpy**: 61 bytes (era 57, overhead JSR/RTS mínimo)
- ✅ **vectrex_console_demo.vpy**: 2138 bytes (era FALLO por overflow, ahora ÉXITO)
- ✅ **Capacidad mejorada**: Hasta 5KB disponibles para juegos complejos
- ✅ **Sin regresiones**: Ambos programas compilan y funcionan

### Arquitectura Técnica:
```asm
main:
    JSR Wait_Recal
    LDA #$80  
    STA VIA_t1_cnt_lo
    JSR LOOP_BODY    ; ← Subrutina (sin límites distancia)
    BRA main

LOOP_BODY:           ; ← Código loop() separado
    [código usuario...]
    RTS              ; ← Retorno a main
```

### Impacto en Desarrollo:
- **Eliminación**: Código duplicado en assembly generado
- **Escalabilidad**: Programas grandes ahora viables (hasta 5KB+)
- **Mantenibilidad**: Estructura más limpia y profesional
- **Compatibilidad**: Programas simples mantienen funcionalidad

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
3. (S3) Verificación simple de variables – COMPLETADO 2025-09-20 (nuevo pase `validate_semantics`).
4. (S4) Tests constant folding / dead store – COMPLETADO 2025-09-20 (`core/tests/opt_pipeline.rs`).
5. (S5) Documentación truncamiento 16-bit – COMPLETADO 2025-09-20 (SUPER_SUMMARY sección 32.4).
6. (S6) Warnings variables no usadas – COMPLETADO 2025-09-20 (stderr `[warn][unused-var]`).
7. (S7) Aridad básica builtins – COMPLETADO 2025-09-20 (validación en `validate_semantics`). (Actualizado: centralizada en `BUILTIN_ARITIES` + test `builtin_arities_stable`).
8. (S8) Canal estructurado de warnings – COMPLETADO 2025-09-20 (`emit_asm_with_diagnostics`, warnings `[unused-var]`).
9. (S9) Errores semánticos estructurados – COMPLETADO 2025-09-20 (se reemplazan panics por diagnostics `Error`).
10. (S10) Códigos de diagnóstico y estructura extendida – COMPLETADO 2025-09-20 (enum `DiagnosticCode`, campos opcionales line/col aún no poblados en pase semántico).

Mid (3-6 semanas):
6. IR intermedio opcional (linear SSA-lite o tree simplificado) para separar optimizaciones de AST. (ID M1)
7. Liveness + mejora dead_store_elim (detectar efectos laterales). (ID M2)
8. Tests para switch folding y dead_code_elim. (ID M3)
9. Soporte de INCLUDE/IMPORT de módulos (sin circular). (ID M4)
10. Enriquecer diagnostics con posiciones file:line:col y códigos (seguir S8/S9). (ID M5)

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
1. S8: Canal estructurado para warnings (integración LSP) (extensión de S6).
2. S9: Convertir `SemanticsError`/`SemanticsErrorArity` panics a resultado estructurado (no abortar proceso).
3. S10: Normalizar mensaje de warnings a estructura (JSON) para futura LSP sin parseo de stderr.

## 13. Estado del Ensamblador Nativo M6809 (Nov 2025)

### Arquitectura de Tres Fases
El ensamblador nativo implementa procesamiento en tres pasadas:

1. **PRE-PASS**: Procesa directivas INCLUDE y símbolos EQU
   - Carga `VECTREX.I` con 258 símbolos BIOS
   - Resuelve expresiones aritméticas recursivamente (VAR+1, LABEL-2)
   - Búsqueda case-insensitive de símbolos

2. **PASS1**: Genera código con placeholders
   - Emite opcodes y operandos
   - Usa placeholders (0x0000) para símbolos no resueltos
   - Registra referencias a símbolos para PASS2

3. **PASS2**: Resuelve símbolos y parchea
   - Calcula offsets relativos para branches
   - Parchea referencias absolutas
   - Verifica rangos de offset

### Instrucciones Pendientes de Implementación

**Próximas en implementar (basado en análisis de archivos .vpy):**
- ⏳ **LEAX/LEAY/LEAU/LEAS**: Load Effective Address (modes indexed)
- ⏳ **CMPX/CMPY/CMPU/CMPS**: Comparaciones 16-bit de otros registros
- ⏳ **Indexed con offset numérico**: `5,X`, `-2,Y`, etc.
- ⏳ **Indexed con acumulador**: `A,X`, `B,Y`, `D,X`
- ⏳ **Indexed con auto-increment**: `,X+`, `,X++`, `,-X`, `,--X`
- ⏳ **Extended indirect**: `[addr]`
- ⏳ **PC-relative**: `label,PCR`

**Estado actual de rotating_line_correct.vpy:**
- Línea actual de fallo: 242 (resolución símbolo Vec_Misc_Count)
- Causas: Posible problema en equates BIOS o second pass
- Progreso: 242/722 líneas (33.5%) con ensamblador nativo

### Estadísticas de Cobertura de Opcodes

**Categoría completada:**
- ✅ Aritmética 8-bit: ADDA, ADDB, SUBA, SUBB, ANDA, ANDB, ORA, EORA
- ✅ Aritmética 16-bit: ADDD, SUBD (immediate + extended)
- ✅ Comparaciones 8-bit: CMPA, CMPB
- ✅ Comparaciones 16-bit: CMPD (immediate + extended)
- ✅ Load/Store 8-bit: LDA, LDB, STA, STB (direct + extended)
- ✅ Load/Store 16-bit: LDD, STD, LDX, LDY, STX, STY, LDU, STU
- ✅ Branches cortos: BEQ, BNE, BCC, BCS, BLE, BGT, BLT, BGE, BPL, BMI, BVC, BVS, BHI, BLS, BLO, BHS
- ✅ Branches largos: LBRA, LBEQ, LBNE, LBCS, LBCC, LBLT, LBGE, LBGT, LBLE, LBMI, LBPL
- ✅ Control de flujo: JSR, RTS, BRA
- ✅ Registros: CLRA, CLRB, INCA, INCB, DECA, DECB
- ✅ Shifts/Rotates: ASLA, ASLB, ROLA, ROLB, LSRA, LSRB, RORA, RORB
- ✅ Especiales: ABX, TFR
- ✅ Memoria: CLR (extended)
- ✅ Indexed básico: LDD ,X (sin offset)

**Categorías pendientes:**
- ⏳ LEA instructions (Load Effective Address)
- ⏳ Indexed avanzado (offsets, auto-increment, indirect)
- ⏳ Comparaciones extendidas (CMPX, CMPY, CMPU, CMPS)
- ⏳ Stack operations extendidas (PSHS/PULS con múltiples registros)
- ⏳ Bit manipulation (BITA, BITB, TST)
- ⏳ Multiply/Divide (MUL, DAA)

### Métricas de Mejora

| Métrica | Antes (Nov 12) | Después (Nov 15) | Mejora |
|---------|---------------|------------------|---------|
| Instrucciones implementadas | ~40 | ~63 | +57.5% |
| Líneas procesadas (rotating_line_correct.vpy) | 40 | 242 | +505% |
| Archivos que compilan 100% nativo | 2 | 3+ | +50%+ |
| Dependencia lwasm fallback | Alta | Media | Reducida |

## 14. Roadmap de Implementación (Nov-Dic 2025)

### Sprint 1 (Completado - Nov 12-15)
- ✅ Implementar 23 instrucciones básicas MC6809
- ✅ Soporte long branches
- ✅ Indexed mode básico (,X sin offset)
- ✅ Operaciones 16-bit (ADDD, SUBD, CMPD)

### Sprint 2 (En Progreso - Nov 16-22)
- ⏳ Resolver problemas de símbolos BIOS en second pass
- ⏳ Implementar LEA instructions (LEAX, LEAY, LEAU, LEAS)
- ⏳ Indexed con offsets numéricos (5,X, -2,Y)
- ⏳ Comparaciones extendidas (CMPX, CMPY)
- 🎯 **Meta**: rotating_line_correct.vpy compila 100% nativo

### Sprint 3 (Pendiente - Nov 23-30)
- ⏳ Indexed con acumuladores (A,X B,Y D,X)
- ⏳ Auto-increment/decrement (,X+ ,-X ,X++ ,--X)
- ⏳ PC-relative addressing (label,PCR)
- ⏳ Extended indirect ([addr])
- 🎯 **Meta**: Todos los ejemplos .vpy compilan sin lwasm

### Sprint 4 (Futuro - Dic 2025)
- ⏳ Bit manipulation (BITA, BITB, TST)
- ⏳ MUL/DAA opcodes
- ⏳ Optimizaciones de código generado
- ⏳ Verificación exhaustiva de timing
- 🎯 **Meta**: Paridad completa con lwasm

---
Notas de mantenimiento: mantener este archivo actualizado cuando se cierren IDs. Añadir fecha y breve changelog al inicio.

---
Changelog:
- 2025-11-15: Añadida sección completa de ensamblador nativo M6809 con estadísticas de progreso, instrucciones implementadas y roadmap detallado.
- 2025-11-12-15: Implementación masiva de 23 instrucciones MC6809 (LDU, STU, ADDD, SUBD, CMPD, LBEQ, long branches, indexed básico, shifts/rotates).
- 2025-10-01: Arquitectura de subrutinas implementada (JSR/RTS) para resolver BRA overflow.
- 2025-09-20: Añadido smoke test (S1) y wrapper `VECTREX_DRAW_TO` implementado (S2). Actualizada sección backend y backlog.
- 2025-09-20: Pase semántico básico (`validate_semantics`) marca error en uso/asignación de variable no declarada (S3 completado).
- 2025-09-20: Tests optimización (S4), doc truncamiento 16-bit (S5) y warnings unused-var (S6) completados.
- 2025-09-20: Aridad builtins validada (S7) añade panics `SemanticsErrorArity` para mismatch.
- 2025-09-20: Refactor: tabla centralizada `BUILTIN_ARITIES` + helper `expected_builtin_arity`, añadido test `core/tests/builtin_arities.rs`.
- 2025-09-20: S8/S9: introducido canal `emit_asm_with_diagnostics` (warnings y errores estructurados). Panics `SemanticsError*` sustituidos por diagnostics `Error`; tests actualizados.
- 2025-09-20: S10: añadidos codes (`UnusedVar`, `UndeclaredVar`, `UndeclaredAssign`, `ArityMismatch`) y assertions de tests migradas a codes; groundwork para posiciones.
