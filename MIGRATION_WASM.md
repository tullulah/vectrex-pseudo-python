# Migración al Núcleo Único Rust (WASM)

Este documento traza los pasos para eliminar el emulador TypeScript (`ide/electron/src/emu6809.ts`) y usar exclusivamente el núcleo Rust compilado a WebAssembly.

## 1. Objetivo
- Reducir duplicación de lógica (flags, modos indexados, temporización, IRQ/FIRQ/NMI).
- Acelerar mantenimiento y nuevas features de emulación (VIA/PSG, precisión de ciclos) en un solo lugar.
- Reutilizar el mismo binario en Electron, VSCode, navegador o pruebas headless.

## 2. Estado Inicial
| Componente | TS (actual) | Rust WASM |
|------------|-------------|-----------|
| CPU 6809 subset | Sí | Sí (más actualizado) |
| Flags SUB/CMP exactas | Parcial | Implementadas |
| Indexed ampliado | Simplificado | Ampliado (parcial) |
| VIA 6522 (skeleton) | Más lógica de intercepción | Skeleton básico (extender) |
| Extracción de vectores | Intercepts BIOS | Pendiente portar |
| Métricas opcodes | Parcial | Sí (`metrics()`) |
| WAIT_RECAL frame hook | Intercept/heurística | Heurística frame_count |

## 3. API de Alto Nivel (WASM)
Wrapper `WasmEmu` (ver `core/src/wasm_api.rs`):
```
new() -> WasmEmu
load_bios(&[u8]) -> bool
load_bin(base: u16, &[u8])
reset()
step(count: u32) -> u32
run_until_wait_recal(max_instr: u32) -> u32
registers_json() -> String
memory_ptr() -> *const u8
metrics() -> String
```

## 4. Pasos de Migración
1. Construir artefacto wasm + bindings (`wasm-bindgen`).
2. Crear módulo JS de fachada: `emu.ts` que re-exporta una instancia singleton o fábrica de `WasmEmu`.
3. Sustituir importaciones de `globalCpu` en:
   - `ide/electron/src/harness_*.ts`
   - `ide/electron/src/main.ts` (si aplica)
4. Reescribir helpers (stats, hard reset) usando métodos WASM.
5. Añadir API Rust para extracción de segmentos:
   - Implementar en Rust un buffer de `VectorSegment {x1,y1,x2,y2,intensity}` y función `drain_segments()`.
   - Exponer vía wasm_bindgen (Vec<f32> flateado o JSON) para evitar copia excesiva.
6. Portar lógica de intercept BIOS a Rust (añadir hooks en `CPU::step` cuando PC == direcciones BIOS). Incrementar `draw_vl_count` ya existe; expandir a acumulación de segmentos.
7. Reemplazar render loop en frontend para leer segmentos desde WASM.
8. Eliminar `emu6809.ts` y referencias residuales.
9. Actualizar README (hecho parcialmente) y limpiar dependencias TS que solo soportaban emulador.

## 5. APIs Futuras (Plan)
- `enable_trace(bool)`
- `drain_bios_calls() -> String` (JSON array)
- `drain_segments() -> Uint16Array` (paquete x1,y1,x2,y2,intensity) con escala fija
- `set_irq_line(level: bool)` (para pruebas)
- VIA register read/write wrappers (si se restringe acceso directo a memoria)

## 6. Estrategia de Testing
- Crear tests Rust (cargo test) para: reset vector, SUB/CMP flags, indexed addressing, simple BIOS call sequence increasing frame_count.
- En JS: humo: cargar bios, run_until_wait_recal, comprobar `frame_count` incrementa tras N iteraciones.

## 7. Eliminación Segura
Realizar commit en fases:
- Commit A: Introducir WASM (ya hecho).
- Commit B: Adaptar harness JS a WASM (mantener TS emu en paralelo un ciclo).
- Commit C: Quitar imports del TS emu.
- Commit D: Borrar `emu6809.ts` + tests asociados.

## 8. Rendimiento / Notas
- Acceso a memoria: usar una sola vista `new Uint8Array(wasmMemory.buffer, ptr, 65536)` y no recrearla.
- Para loops largos preferir `run_until_wait_recal` vs múltiples `step()` desde JS.
- Evitar JSON en hot-path (futuro: exponer struct plana). `registers_json()` es solo paso inicial.

## 9. Pendiente
- Buffer de segmentos en Rust.
- Modelar VIA de forma suficiente para reemplazar intercepts.
- Audio/PSG.

## 10. Ejemplo de Bootstrap en Electron (Pseudo)
```ts
import init, { WasmEmu } from '../dist-wasm/vectrex_lang.js';
let emu: WasmEmu;
(async()=>{
  await init();
  emu = new WasmEmu();
  emu.load_bios(await fetchBiosBytes());
  emu.reset();
  const loop = () => {
    emu.run_until_wait_recal(300000);
    const regs = JSON.parse(emu.registers_json());
    renderRegs(regs);
    // TODO: segmentos cuando exista API
    requestAnimationFrame(loop);
  };
  loop();
})();
```

---
Este documento se actualizará conforme se añadan las APIs de segmentos y se elimine el emulador TypeScript.
