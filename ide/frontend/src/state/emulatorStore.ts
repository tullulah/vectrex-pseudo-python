import { create } from 'zustand';
import type { EmulatorState } from '../types/models';

interface EmulatorStore extends EmulatorState {
  setStatus: (s: EmulatorState['status']) => void;
}

const initial: EmulatorState = { status: 'stopped' };

export const useEmulatorStore = create<EmulatorStore>((set) => ({
  ...initial,
  setStatus: (status) => set({ status })
}));
