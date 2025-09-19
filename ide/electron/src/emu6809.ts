// LEGACY EMULATOR REMOVED
// This stub file remains temporarily because deletion attempts did not remove it from the workspace.
// All functionality has migrated to the Rust WASM core (see frontend wasm wrapper). Any import from './emu6809'
// should be removed. Using this stub at runtime will throw immediately to surface accidental usage.

export const globalCpu: any = null;
export function getStats(){ throw new Error('legacy emulator removed'); }
export function resetStats(){ /* noop */ }
export function hardResetCpu(){ throw new Error('legacy emulator removed'); }
// Intentionally throw if someone still tries to construct the legacy CPU.
export class Cpu6809 { constructor(){ throw new Error('Cpu6809 legacy removed – use WASM emulator'); } }

// Optional: surface accidental residual import immediately.
if (typeof console !== 'undefined') {
	console.error('[emu6809.ts] Legacy emulator stub loaded. Remove any remaining imports.');
}