import { create } from 'zustand';
import type { DebugState } from '../types/models';

interface DebugStore extends DebugState {
  setRegisters: (r: Record<string,string>) => void;
  setVariables: (vars: Array<{name:string;value:string}>) => void;
  setConstants: (c: Array<{name:string;value:string}>) => void;
  setPC: (pc: number) => void;
  setCycles: (cycles: number) => void;
  reset: () => void;
}

const initial: DebugState = {
  registers: {},
  pc: 0,
  cycles: 0,
  variables: [],
  constants: [],
};

export const useDebugStore = create<DebugStore>((set) => ({
  ...initial,
  setRegisters: (r) => set({ registers: r }),
  setVariables: (variables) => set({ variables }),
  setConstants: (constants) => set({ constants }),
  setPC: (pc) => set({ pc }),
  setCycles: (cycles) => set({ cycles }),
  reset: () => set(initial)
}));
