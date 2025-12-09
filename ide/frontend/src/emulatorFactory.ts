import type { EmulatorBackend, IEmulatorCore } from './emulatorCore.js';
import { JsVecxEmulatorCore } from './jsvecxCore.js';

export function createEmulatorCore(preferred?: EmulatorBackend): IEmulatorCore {
  const backend = preferred || readPreference();
  // Solo JSVecx disponible ahora que se eliminó el emulador Rust
  return new JsVecxEmulatorCore();
}

export function readPreference(): EmulatorBackend {
  if (typeof localStorage !== 'undefined') {
    try {
      const v = localStorage.getItem('emu_backend');
      if (v === 'jsvecx') return v;
    } catch { /* ignore */ }
  }
  // Query param override ?emu_backend=jsvecx
  if (typeof location !== 'undefined') {
    const p = new URLSearchParams(location.search).get('emu_backend');
    if (p === 'jsvecx') return p;
  }
  return 'jsvecx';
}

export function persistPreference(backend: EmulatorBackend){
  try { localStorage.setItem('emu_backend', backend); } catch {}
}
