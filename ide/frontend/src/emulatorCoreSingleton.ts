// Singleton compartido de IEmulatorCore para toda la UI.
// Selecciona backend según preferencia persistida (localStorage ?emu_backend=jsvecx).
// Expone window.emuCore (y alias transitional window.globalEmu) para debugging manual.
import { createEmulatorCore } from './emulatorFactory.js';
import type { IEmulatorCore } from './emulatorCore.js';

export const emuCore: IEmulatorCore = createEmulatorCore();

try {
  // @ts-ignore
  (window as any).emuCore = emuCore;
  // Mantener alias hasta completar refactor total (scripts externos, consola navegador).
  // @ts-ignore
  if (!(window as any).globalEmu) (window as any).globalEmu = emuCore;
} catch { /* entornos SSR / tests */ }
