import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface EmulatorSettings {
  // Audio settings
  audioEnabled: boolean;
  
  // Overlay settings
  overlayEnabled: boolean;
  
  // Last ROM selection
  lastRomPath: string | null;
  lastRomName: string | null;
  
  // Actions
  setAudioEnabled: (enabled: boolean) => void;
  setOverlayEnabled: (enabled: boolean) => void;
  setLastRom: (path: string | null, name: string | null) => void;
}

export const useEmulatorSettings = create<EmulatorSettings>()(
  persist(
    (set) => ({
      // Default values
      audioEnabled: true,
      overlayEnabled: true,
      lastRomPath: null,
      lastRomName: null,
      
      // Actions
      setAudioEnabled: (enabled) => set({ audioEnabled: enabled }),
      setOverlayEnabled: (enabled) => set({ overlayEnabled: enabled }),
      setLastRom: (path, name) => set({ lastRomPath: path, lastRomName: name }),
    }),
    {
      name: 'vpy-emulator-settings',
      partialize: (state) => ({
        audioEnabled: state.audioEnabled,
        overlayEnabled: state.overlayEnabled,
        lastRomPath: state.lastRomPath,
        lastRomName: state.lastRomName,
      })
    }
  )
);