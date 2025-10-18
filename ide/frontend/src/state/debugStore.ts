import { create } from 'zustand';
import type { DebugState as LegacyDebugState } from '../types/models';
import { useEditorStore } from './editorStore';

export type ExecutionState = 'stopped' | 'running' | 'paused';

export interface PdbData {
  version: string;
  source: string;
  binary: string;
  entry_point: string;
  symbols: Record<string, string>;
  lineMap: Record<string, string>;
  functions?: Record<string, {
    startLine: number;
    endLine: number;
    address: string;
    type: 'vpy' | 'native';
  }>;
  nativeCalls?: Record<string, string>;
}

export interface CallFrame {
  function: string;
  line: number | null;
  address: string;
  type: 'vpy' | 'native' | 'bios';
}

interface DebugStore extends LegacyDebugState {
  // Legacy setters (keep for compatibility)
  setRegisters: (r: Record<string,string>) => void;
  setVariables: (vars: Array<{name:string;value:string}>) => void;
  setConstants: (c: Array<{name:string;value:string}>) => void;
  setPC: (pc: number) => void;
  setCycles: (cycles: number) => void;
  reset: () => void;
  
  // New debug state
  state: ExecutionState;
  currentVpyLine: number | null;
  currentAsmAddress: string | null;
  pdbData: PdbData | null;
  callStack: CallFrame[];
  currentFps: number;
  loadingForDebug: boolean; // Flag to indicate loading binary for debug session (don't auto-start)
  
  // New actions
  setState: (state: ExecutionState) => void;
  setCurrentVpyLine: (line: number | null) => void;
  setCurrentAsmAddress: (address: string | null) => void;
  loadPdbData: (pdb: PdbData) => void;
  updateCallStack: (stack: CallFrame[]) => void;
  updateStats: (cycles: number, fps: number) => void;
  setLoadingForDebug: (loading: boolean) => void;
  
  // Debug controls
  run: () => void;
  pause: () => void;
  stop: () => void;
  stepOver: () => void;
  stepInto: () => void;
  stepOut: () => void;
  
  // Breakpoint synchronization
  onBreakpointAdded: (uri: string, line: number) => void;
  onBreakpointRemoved: (uri: string, line: number) => void;
}

const initial: LegacyDebugState = {
  registers: {},
  pc: 0,
  cycles: 0,
  variables: [],
  constants: [],
};

export const useDebugStore = create<DebugStore>((set, get) => ({
  ...initial,
  
  // Legacy setters (keep for compatibility)
  setRegisters: (r) => set({ registers: r }),
  setVariables: (variables) => set({ variables }),
  setConstants: (constants) => set({ constants }),
  setPC: (pc) => set({ pc }),
  setCycles: (cycles) => set({ cycles }),
  reset: () => set(initial),
  
  // New state
  state: 'stopped',
  currentVpyLine: null,
  currentAsmAddress: null,
  pdbData: null,
  callStack: [],
  currentFps: 0,
  loadingForDebug: false,
  
  // New actions
  setState: (state) => set({ state }),
  setCurrentVpyLine: (line) => set({ currentVpyLine: line }),
  setCurrentAsmAddress: (address) => set({ currentAsmAddress: address }),
  setLoadingForDebug: (loading) => set({ loadingForDebug: loading }),
  
  loadPdbData: (pdb) => {
    console.log('[DebugStore] 📋 Loaded .pdb:', pdb);
    set({ pdbData: pdb });
    
    // Re-synchronize existing breakpoints from editorStore
    const allBreakpoints = useEditorStore.getState().breakpoints;
    console.log('[DebugStore] 🔄 Re-synchronizing breakpoints from editorStore:', allBreakpoints);
    
    // For each file's breakpoints, send add-breakpoint messages
    Object.entries(allBreakpoints).forEach(([uri, lines]) => {
      lines.forEach((line) => {
        const address = pdb.lineMap[line.toString()];
        if (address) {
          console.log(`[DebugStore] ♻️  Re-sync breakpoint: ${uri}:${line} → ${address}`);
          window.postMessage({
            type: 'debug-add-breakpoint',
            address,
            line
          }, '*');
        }
      });
    });
  },
  
  updateCallStack: (stack) => set({ callStack: stack }),
  updateStats: (cycles, fps) => set({ cycles, currentFps: fps }),
  
  // Debug controls
  run: () => {
    console.log('[DebugStore] Run');
    set({ state: 'running' });
    window.postMessage({ type: 'debug-continue' }, '*');
  },
  
  pause: () => {
    console.log('[DebugStore] Pause');
    set({ state: 'paused' });
    window.postMessage({ type: 'debug-pause' }, '*');
  },
  
  stop: () => {
    console.log('[DebugStore] Stop');
    set({ 
      state: 'stopped',
      currentVpyLine: null,
      currentAsmAddress: null,
      callStack: [],
      cycles: 0
    });
    window.postMessage({ type: 'debug-stop' }, '*');
  },
  
  stepOver: () => {
    console.log('[DebugStore] Step Over');
    const { pdbData, currentVpyLine } = get();
    
    if (!pdbData || currentVpyLine === null) return;
    
    const nextLine = currentVpyLine + 1;
    const nextAddress = pdbData.lineMap[nextLine.toString()];
    
    if (nextAddress) {
      window.postMessage({ 
        type: 'debug-step-over',
        targetAddress: nextAddress
      }, '*');
    }
  },
  
  stepInto: () => {
    console.log('[DebugStore] Step Into');
    const { pdbData, currentVpyLine } = get();
    
    if (!pdbData || currentVpyLine === null) return;
    
    const nativeCall = pdbData.nativeCalls?.[currentVpyLine.toString()];
    
    window.postMessage({ 
      type: 'debug-step-into',
      isNativeCall: !!nativeCall,
      functionName: nativeCall
    }, '*');
  },
  
  stepOut: () => {
    console.log('[DebugStore] Step Out');
    window.postMessage({ type: 'debug-step-out' }, '*');
  },
  
  // Breakpoint synchronization
  onBreakpointAdded: (uri, line) => {
    const { pdbData } = get();
    
    if (!pdbData) {
      console.warn(`[DebugStore] ⚠️ Cannot add breakpoint: no PDB data loaded yet`);
      return;
    }
    
    const address = pdbData.lineMap[line.toString()];
    
    if (address) {
      console.log(`[DebugStore] ➕ Breakpoint added: line ${line} → ${address}`);
      window.postMessage({
        type: 'debug-add-breakpoint',
        address,
        line
      }, '*');
    } else {
      console.warn(`[DebugStore] ⚠️ No address mapping for VPy line ${line}`);
    }
  },
  
  onBreakpointRemoved: (uri, line) => {
    const { pdbData } = get();
    
    if (!pdbData) return;
    
    const address = pdbData.lineMap[line.toString()];
    
    if (address) {
      console.log(`[DebugStore] ➖ Breakpoint removed: line ${line} → ${address}`);
      window.postMessage({
        type: 'debug-remove-breakpoint',
        address,
        line
      }, '*');
    }
  }
}));
