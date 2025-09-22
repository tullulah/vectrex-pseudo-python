# Current Simplifications / Pending Fidelity Upgrades

(Policy: No new heuristics or silent fallbacks; this list tracks remaining gaps.)

## CPU
- Instruction cycle timings: single lump per opcode; no sub‑cycle effects for integrator timing.
- FIRQ/NMI stacking minimal; exact flag/priority nuances not fully verified.
- No STOP instruction emulation (if 6809 variant supports it) – only WAI path handled.

## VIA 6522
- Port A/B I/O direction, handshake lines (CA1/CA2/CB1/CB2) not implemented.
- Shift register: simplified continuous mode only; missing all mode variants.
- Timer2: basic countdown only; no pulse counting / PB6 side effects.
- PB7 toggle implemented but no DAC coupling or analog ramp timing.

## Vector Integrator / Display
- Model is digital: integrates abstract velocities; lacks real integrator decay, drift, or blanking settle times.
- Intensity handling simplified; no phosphor persistence or beam warmup.
- Segment generation tied to register writes rather than analog thresholds.

## Audio (PSG AY-3-8912) - COMPLETADO 2025-01-23
- ✅ COMPLETAMENTE IMPLEMENTADO: Control BC1/BDIR via VIA Port B bits 3-4, estado máquina completa (INACTIVE/LATCH ADDRESS/LATCH DATA/READ DATA), generación audio con 3 canales tono + ruido + envolventes, curva logarítmica volumen, exports WASM funcionales.
- ✅ Integración VIA completa: Port B controla PSG BC1/BDIR, Port A proporciona bus de datos.
- ✅ Audio streaming: `psg_prepare_pcm()`, `psg_pcm_ptr()`, `psg_pcm_len()` exportados y funcionales.
- Único pendiente menor: timestamp por bloque para sincronización video/audio perfecta (no afecta funcionalidad).

## Input
- Estado mínimo: snapshot joystick analógico + 4 botones vía RAM fija ($00F0..$00F2). No integración aún con puertos VIA reales ni edge detection; BIOS que espere lectura por bit en puertos puede requerir adaptación futura.

## Memory / Bus
- Zero wait states; no contention or differing timings for VIA vs RAM vs ROM.
- Illegal region (D800-DFFF) returns 0xFF; no mixed-bus artifacts.

## Interrupt / Frame Timing
- Frame boundary now ONLY on Timer1 IRQ (IFR6) – accurate trigger. No fallback heuristic remains.
- If BIOS never arms Timer1, frame_count will stay 0 (intentionally non-masked to expose issues).

## Cartridge / Banks
- Up to 48K linear; no bank-switching hardware implemented (future: paged mappers if needed).

## Accuracy Roadmap
1. Full VIA port + handshake lines.
2. Proper shift register modes & Timer2 pulse counting.
3. Analog-like integrator (time-based DAC slopes, blank settle, jitter).
4. ✅ ~~PSG audio core + mixer + timing~~ - COMPLETADO 2025-01-23.
5. Input matrix / analog joystick scaling.
6. Bank switching / external hardware (if required by target ROMs).
7. Verified cycle timings per addressing mode (not just opcode). 

(Keep this file updated as features graduate from "simplified" to "accurate".)
