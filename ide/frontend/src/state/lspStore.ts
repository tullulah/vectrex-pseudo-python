import { create } from 'zustand';
import type { DiagnosticModel } from '../types/models.js';

interface LspState {
  connected: boolean;
  locale: string;
  setConnected: (c: boolean) => void;
  setLocale: (loc: string) => void;
  applyDiagnostics: (uri: string, diags: DiagnosticModel[]) => void;
}

export const useLspStore = create<LspState>((set) => ({
  connected: false,
  locale: 'en',
  setConnected: (c) => set({ connected: c }),
  setLocale: (loc) => set({ locale: loc }),
  applyDiagnostics: (_uri, _diags) => { /* will route into editorStore later */ },
}));
