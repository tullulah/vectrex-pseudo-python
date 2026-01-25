import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type CompilerBackend = 'buildtools' | 'core';

interface SettingsState {
  compiler: CompilerBackend;
  setCompiler: (compiler: CompilerBackend) => void;
}

export const useSettings = create<SettingsState>()(
  persist(
    (set) => ({
      compiler: 'buildtools', // Default to new buildtools compiler
      setCompiler: (compiler) => set({ compiler }),
    }),
    {
      name: 'vpy-settings', // localStorage key
    }
  )
);
