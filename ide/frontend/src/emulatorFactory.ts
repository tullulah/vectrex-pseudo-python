import { EmulatorBackend, IEmulatorCore } from './emulatorCore';
import { RustWasmEmulatorCore } from './rustWasmCore';
import { JsVecxEmulatorCore } from './jsvecxCore';

export function createEmulatorCore(preferred?: EmulatorBackend): IEmulatorCore {
  const backend = preferred || readPreference();
  if (backend === 'jsvecx') {
    try { return new JsVecxEmulatorCore(); } catch(e){ console.warn('[emuFactory] fallback a rust por error jsvecx', e); }
  }
  return new RustWasmEmulatorCore();
}

export function readPreference(): EmulatorBackend {
  if (typeof localStorage !== 'undefined') {
    try {
      const v = localStorage.getItem('emu_backend');
      if (v === 'jsvecx' || v === 'rust') return v;
    } catch { /* ignore */ }
  }
  // Query param override ?emu_backend=jsvecx
  if (typeof location !== 'undefined') {
    const p = new URLSearchParams(location.search).get('emu_backend');
    if (p === 'jsvecx' || p === 'rust') return p;
  }
  return 'rust';
}

export function persistPreference(backend: EmulatorBackend){
  try { localStorage.setItem('emu_backend', backend); } catch {}
}
