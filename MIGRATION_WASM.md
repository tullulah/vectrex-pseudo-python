# Migración al Núcleo Único Rust (WASM) – ESTADO: COMPLETADA ✅

Este documento queda como histórico de la transición: el emulador TypeScript (`ide/electron/src/emu6809.ts`) ha sido eliminado y toda la emulación reside ahora en el crate Rust `vectrex_emulator` expuesto vía WebAssembly.

## 1. Objetivo
- Reducir duplicación de lógica (flags, modos indexados, temporización, IRQ/FIRQ/NMI).
- Acelerar mantenimiento y nuevas features de emulación (VIA/PSG, precisión de ciclos) en un solo lugar.
- Reutilizar el mismo binario en Electron, VSCode, navegador o pruebas headless.

## 2. Estado Inicial (Referencia)
| Componente | TS (actual) | Rust WASM |
|------------|-------------|-----------|
| CPU 6809 subset | Sí | Sí (más actualizado) |
| Flags SUB/CMP exactas | Parcial | Implementadas |
| Indexed ampliado | Simplificado | Ampliado (parcial) |
| VIA 6522 (skeleton) | Más lógica de intercepción | Skeleton básico (extender) |
| Extracción de vectores | Intercepts BIOS | Pendiente portar |
| Métricas opcodes | Parcial | Sí (`metrics()`) |
| WAIT_RECAL frame hook | Intercept/heurística | Heurística frame_count |

## 3. API de Alto Nivel (WASM) Actual
Wrapper `WasmEmu` (ver `emulator/src/wasm_api.rs`):
```
new() -> WasmEmu
load_bios(&[u8]) -> bool
load_bin(base: u16, &[u8])
reset()
reset_stats()
step(count: u32) -> u32
run_until_wait_recal(max_instr: u32) -> u32
registers_json() -> String
metrics_json() -> String
memory_ptr() -> *const u8
// Segmentos (integrador)
integrator_segments_json() -> String             // drena
integrator_segments_peek_json() -> String        // no drena
integrator_segments_count() -> u32               // solo cuenta (no copia, no drena)
integrator_segments_ptr() -> *const u8           // staging buffer
integrator_segments_len() -> u32
integrator_segment_stride() -> u32
integrator_drain_segments()
// Controles integrador
set_integrator_merge_lines(bool)
integrator_merge_lines() -> bool
reset_integrator_segments()
set_integrator_auto_drain(bool)
integrator_auto_drain() -> bool
// Debug
loop_watch_json() -> String
// BIOS call stack (añadido 2025-09-19, TODO 13 completado)
bios_calls_json() -> String          // Últimas llamadas BIOS (array JSON de strings "FFFF:LABEL", máx 256)
clear_bios_calls()                   // Limpia buffer de llamadas BIOS
demo_triangle()
// Input
set_input_state(x: i16, y: i16, buttons: u8)
```

## 4. Pasos de Migración Ejecutados
1. Build WASM + bindings (`wasm-bindgen`).
2. Reemplazo de fachada TS por servicio WASM en renderer.
3. Sustitución de todas las referencias `globalCpu`.
4. Port de métricas y hooks a Rust únicamente.
5. Implementación del integrador y API de segmentos (JSON + memoria compartida).
6. Eliminación de intercepts BIOS legacy para vectores; ahora se generan desde integrador.
7. Render loop del panel adaptado a `run_until_wait_recal` + extracción integrador.
8. Eliminación física de `emu6809.ts` y harness.
9. Actualización README y este documento a estado final.

## 5. Posibles Extensiones Futuras
- `enable_trace(bool)` / trace estructurado incremental.
- `drain_bios_calls()` para profiling de rutinas.
- Buffer ring para segmentos (zero‑copy persistente) + versión `Uint16Array` empaquetada.
- Señales externas: `set_irq_line(level: bool)` para test harness.
- API explícita de VIA (lectura/escritura) si se encapsula memoria directa.
- Snapshot binario de registros (struct plano) para evitar JSON.

## 6. Estrategia de Testing Vigente
- Tests Rust: opcodes críticos, reset, frame_count tras BIOS, métricas de IRQ/Timer1.
- Pruebas JS (renderer): ciclo de frames incrementa, segmentos > 0 tras demo / BIOS, input refleja valores en `metrics_json()`.
- (Pendiente) Test automatizado comparando dumps de segmentos contra snapshots aprobados.

## 7. Eliminación Segura (Realizada)
Secuencia ejecutada tal cual (A→D). No quedan referencias residuales.

## 8. Rendimiento / Notas
- Acceso a memoria: usar una sola vista `new Uint8Array(wasmMemory.buffer, ptr, 65536)` y no recrearla.
- Para loops largos preferir `run_until_wait_recal` vs múltiples `step()` desde JS.
- Evitar JSON en hot-path (futuro: exponer struct plana). `registers_json()` es solo paso inicial.

## 9. Pendiente (Post-Migración)
- Audio / PSG.
- Modelo VIA completo (timings finos, timers adicionales, sincronía raster).
- Refinar heurística `run_until_wait_recal` con Timer1 auténtico.
- Optimización JSON → snapshot binario para métricas.
- Dead‑zone configurable y lectura de entrada vía VIA real.

## 10. Ejemplo de Bootstrap (Actual)
```ts
import init, { WasmEmu } from './wasm/vectrex_emulator.js';
let emu: WasmEmu;
(async()=>{
  await init();
  emu = new WasmEmu();
  emu.load_bios(await fetchBiosBytes());
  emu.reset();
  const frame = () => {
    emu.run_until_wait_recal(250_000);
    const regs = JSON.parse(emu.registers_json());
    const metrics = JSON.parse(emu.metrics_json());
    // Patrón recomendado:
    // 1) Mirar cuántos segmentos hay sin tocar el buffer
    const count = emu.integrator_segments_count();
    let segs;
    if (count === 0) {
      segs = [];
    } else if (count < 128) {
      // Coste bajo: drenar directamente en JSON
      segs = JSON.parse(emu.integrator_segments_json());
    } else {
      // Muchos segmentos: usar acceso binario
      const ptr = emu.integrator_segments_ptr();
      const len = emu.integrator_segments_len();
      const stride = emu.integrator_segment_stride();
      // Aquí convertirías cada registro (ver BeamSegmentC layout) a tu formato de render.
      // Después puedes (opcional) drenar para liberar
      // emu.integrator_drain_segments();
      segs = []; // placeholder
    }
    render(regs, metrics, segs);
    requestAnimationFrame(frame);
  };
  frame();
})();
```

---
Estado final actualizado (Sept 2025).
