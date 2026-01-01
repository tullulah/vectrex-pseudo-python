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
  
  // Emulator state (stopped/running) - persisted across IDE restarts
  emulatorWasRunning: boolean;
  
  // Last compiled binary from project (auto-load on startup if project opens)
  lastCompiledBinary: string | null; // Full path to .bin file
  lastCompiledProject: string | null; // Project path this binary belongs to
  
  // Actions
  setAudioEnabled: (enabled: boolean) => void;
  setOverlayEnabled: (enabled: boolean) => void;
  setLastRom: (path: string | null, name: string | null) => void;
  setEmulatorRunning: (running: boolean) => void;
  setLastCompiledBinary: (binPath: string | null, projectPath: string | null) => void;
}

export const useEmulatorSettings = create<EmulatorSettings>()(
  persist(
    (set) => ({
      // Default values
      audioEnabled: true,
      overlayEnabled: true,
      lastRomPath: null,
      lastRomName: null,
      emulatorWasRunning: false, // Default: don't auto-start
      lastCompiledBinary: null,
      lastCompiledProject: null,
      
      // Actions
      setAudioEnabled: (enabled) => set({ audioEnabled: enabled }),
      setOverlayEnabled: (enabled) => set({ overlayEnabled: enabled }),
      setLastRom: (path, name) => set({ lastRomPath: path, lastRomName: name }),
      setEmulatorRunning: (running) => set({ emulatorWasRunning: running }),
      setLastCompiledBinary: (binPath, projectPath) => set({ 
        lastCompiledBinary: binPath,
        lastCompiledProject: projectPath 
      }),
    }),
    {
      name: 'vpy-emulator-settings',
      partialize: (state) => ({
        audioEnabled: state.audioEnabled,
        overlayEnabled: state.overlayEnabled,
        lastRomPath: state.lastRomPath,
        lastRomName: state.lastRomName,
        emulatorWasRunning: state.emulatorWasRunning,
        lastCompiledBinary: state.lastCompiledBinary,
        lastCompiledProject: state.lastCompiledProject,
      })
    }
  )
);