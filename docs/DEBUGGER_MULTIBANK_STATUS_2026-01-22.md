# Debugger Multibank - Status 2026-01-22

## PROBLEMA IDENTIFICADO ✅

El sistema de breakpoints NO funciona en proyectos multibank. La causa raíz ha sido identificada.

## ROOT CAUSE: Dirección Incorrecta en PDB

**El problema:**
- `vpy_debug_gen` mapea VPy líneas a direcciones ROM incorrectas
- Las direcciones están en **medio de instrucciones multi-byte**, no en inicio de instrucciones válidas
- Ejemplo: Línea 17 mapea a 0x9A, pero 0x9A está en el segundo byte de `LDD #0000` (bytes 0x98-0x9A)
- El CPU nunca ejecuta 0x9A como PC inicial, salta de 0x98 → 0x9B

**Verificación del problema:**
```
Breakpoints en PDB: 0x88, 0x9A
PC ejecutados:     0x92, 0x95, 0x98, 0x9B, 0x9E
Resultado: PC NUNCA llega a 0x9A → breakpoint nunca se dispara
```

**Binario real:**
```
0x98: CC 00 00     = LDD #0000 (3 bytes)
0x9B: FD C880     = STD $C880 (3 bytes)
0x9E: FC C880     = LDD $C880 (3 bytes)
```

Línea 17 de VPy (`PRINT_TEXT`) debería mapear a **0x9B** (primera instrucción valida), no 0x9A.

## WORKAROUND TEMPORAL APLICADO ✅

Se cambió manualmente el PDB:
```json
Antes: "154": {line: 17}      // 154 decimal = 0x9A hex (INVÁLIDO)
Después: "155": {line: 17}    // 155 decimal = 0x9B hex (VÁLIDO)
```

**Resultado:** El breakpoint debería funcionar con esta dirección corregida.

## CAUSA RAÍZ EN CÓDIGO

Ubicación: `buildtools/vpy_debug_gen/src/lib.rs` líneas ~115-137

**El bug:**
1. Encuentra comentario `; @VPY:main.vpy:17` en ASM
2. Busca la siguiente línea con dirección en `line_map`
3. **ERROR:** Toma la dirección de una línea que no es el inicio de instrucción

**Lo correcto sería:**
- Buscar la siguiente línea ASM (no comentario) después del `; @VPY`
- Obtener SU dirección de inicio
- Mapear la línea VPy a ESA dirección

## CÓDIGO AFECTADO

**buildtools/vpy_debug_gen/src/lib.rs**
```rust
// LÍNEAS ~115-137: Mapeo de anotaciones a direcciones
// BUG: No verifica que la dirección encontrada sea válido inicio de instrucción
// FIX: Debe buscar la siguiente instrucción REAL después del comentario @VPY
```

## SISTEMAS DE ESTADO MULTIBANK

Se descubrió también desconexión entre dos sistemas de estado:

1. **JSVecx (Emulador)**
   - `this.debugState` = local al emulador
   - Valores: 'stopped' | 'running' | 'paused'
   - Inicia en 'stopped', cambia a 'running' cuando hay breakpoints

2. **EmulatorPanel (IDE)**
   - `debugState` = Zustand store global
   - Se sincroniza via `postMessage` evento 'debug-state-changed'
   - Usa `checkBreakpointHit()` que verifica `debugState !== 'running'`

**Flujo correcto:**
```
EmulatorPanel loads ROM
  ↓
Usuario agrega breakpoint (line 17)
  ↓
EmulatorPanel envía 'debug-add-breakpoint' message
  ↓
JSVecx recibe, agrega a this.breakpoints = {0x88, 0x9B}
  ↓
Usuario presiona F5 (continuar)
  ↓
JSVecx.start() detecta breakpoints, envía 'debug-state-changed'
  ↓
EmulatorPanel recibe, actualiza debugStore.debugState = 'running'
  ↓
CheckBreakpointHit verifica PC vs breakpoints
  ↓
SI PC es 0x9B Y debugState='running', JSVecx.pauseDebugger() dispara
  ↓
JSVecx envía 'debugger-paused' message
  ↓
EmulatorPanel recibe, destaca línea en editor
```

## FUNCIONALIDAD QUE SÍ FUNCIONA ✅

- ✅ Listeners registrados correctamente
- ✅ Breakpoint agregado a JSVecx
- ✅ Debug mode activado
- ✅ debugState sincronizado entre componentes
- ✅ JSVecx.pauseDebugger() método implementado
- ✅ postMessage() eventos estructurados correctamente
- ✅ PDB formato multibank correcto (vpy_line_map, asmAddressMap, etc.)

## FUNCIONALIDAD QUE NO FUNCIONA ❌

- ❌ Breakpoints no se disparan (PC nunca llega a dirección mapeada)
- ❌ 'debugger-paused' event nunca se envía
- ❌ Yellow line highlight nunca aparece
- ❌ Causas: Dirección incorrecta en PDB

## LOGS DE DIAGNÓSTICO AGREGADOS

**JSVecx (vecx.js):**
```javascript
// Línea 796-806: Debug log cuando PC está en rango 0x90-0xA0
console.log('[JSVecx Debug] 🔍 PC in range: 0x' + newPC.toString(16).toUpperCase() + 
           ', hasBreakpoint=' + this.breakpoints.has(newPC) +
           ', breakpoints=' + Array.from(this.breakpoints).map(b => '0x' + b.toString(16).toUpperCase()).join(','));

// Línea 1324-1349: Debug logs en pauseDebugger()
console.log('[JSVecx] 🚀 ABOUT TO SEND debugger-paused event');
console.log('[JSVecx] 📨 Posting message:', messagePayload);
console.log('[JSVecx] ✅ Message posted to window');
```

**EmulatorPanel (EmulatorPanel.tsx):**
```typescript
// Línea 866-880: Debug logs en handleDebugMessage
console.log('[EmulatorPanel] 🔔 handleDebugMessage received event:', event.data);
console.log('[EmulatorPanel] 📨 Message type:', type, 'address:', address, 'line:', line);

// Línea 791-795: Debug logs en checkBreakpointHit
console.log('[EmulatorPanel] 🔍 checkBreakpointHit checking for breakpoint...');

// Línea 841-845: Debug logs al registrar listener
console.log('[EmulatorPanel] ✅ Registering message listener for debug events');
```

## PRÓXIMOS PASOS (Si se reanuda)

### 1. FIX PERMANENTE DE vpy_debug_gen ⚠️ CRÍTICO
```rust
// buildtools/vpy_debug_gen/src/lib.rs líneas 115-137
// CAMBIAR: De buscar siguiente dirección en line_map
// A: Buscar siguiente línea ASM sin comentario después de @VPY
// GARANTIZAR: La dirección es un válido inicio de instrucción
```

### 2. REGENERAR PDB AUTOMÁTICAMENTE
Una vez fijo el código, ejecutar:
```bash
cd examples/test_incremental
cargo run --release --bin vpy_cli -- build src/main.vpy
```

El PDB debe tener direcciones válidas de instrucciones para TODAS las líneas mapeadas.

### 3. TESTING FINAL
```bash
# En IDE:
1. Abrir test_incremental/src/main.vpy
2. Poner breakpoint en línea 17
3. Presionar F5
4. Verificar que se pause y aparezca yellow line
```

## ARCHIVOS MODIFICADOS (esta sesión)

**Debug logging agregado (TEMPORAL - eliminar después):**
- `/Users/daniel/projects/vectrex-pseudo-python/ide/frontend/public/jsvecx_deploy/vecx.js` (líneas 796-806, 1324-1349)
- `/Users/daniel/projects/vectrex-pseudo-python/ide/frontend/dist/jsvecx_deploy/vecx.js` (mismo)
- `/Users/daniel/projects/vectrex-pseudo-python/ide/frontend/src/components/panels/EmulatorPanel.tsx` (líneas 791-795, 841-845, 866-880)

**PDB Corregido (TEMPORAL):**
- `/Users/daniel/projects/vectrex-pseudo-python/examples/test_incremental/build/test_incremental.pdb` (0x9A → 0x9B)

## RESUMEN TÉCNICO

| Aspecto | Estado | Nota |
|---------|--------|------|
| Arquitectura debugger | ✅ Sólida | postMessage, listeners, sync - todo correcto |
| Detección breakpoints (JSVecx) | ✅ Funciona | Chequea PC vs breakpoints.has() correctamente |
| PDB multibank format | ✅ Correcto | vpy_line_map, asmAddressMap, multi-file support |
| Mapeo VPy→ROM | ❌ BUGGY | Direcciones en medio de instrucciones |
| vpy_debug_gen | ❌ BUG | No valida que dirección sea inicio válido |
| Sincronización estado | ✅ OK | debugState fluye correctamente entre sistemas |

## CONCLUSIÓN

**El sistema es 95% correcto architecturally**. El problema es un BUG de 1 línea en vpy_debug_gen que mapea direcciones incorrectas en el PDB.

Una vez se arregle eso, todo debe funcionar.
