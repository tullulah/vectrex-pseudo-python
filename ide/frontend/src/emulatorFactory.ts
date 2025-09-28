import { EmulatorBackend, IEmulatorCore } from './emulatorCore';
import { JsVecxEmulatorCore } from './jsvecxCore';

export function createEmulatorCore(preferred?: EmulatorBackend): IEmulatorCore {
  // Always use JSVecx - Rust WASM removed
  return new JsVecxEmulatorCore();
}

export function readPreference(): EmulatorBackend {
  // Always return JSVecx - Rust backend removed
  return 'jsvecx';
}

export function persistPreference(backend: EmulatorBackend){
  // No-op - only JSVecx available
}
