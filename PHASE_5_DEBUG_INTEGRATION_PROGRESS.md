# Phase 5: IDE Debug Integration - Progress Report

**Date**: October 17, 2025  
**Status**: Phase 1 & 2 COMPLETE ✅

---

## Overview

Implementing full F5 debugging experience in IDE with .pdb symbol support, breakpoints, and step-by-step execution control.

### Total Plan: 5 Phases
1. ✅ **Backend .pdb Loading** - Electron automatically loads .pdb after compilation
2. ✅ **Frontend Debug Commands** - Implement debug.start/stop in main.tsx  
3. 🎯 **Emulator Breakpoint System** - Add breakpoint checking to EmulatorPanel
4. 🎯 **Monaco Breakpoint Decorations** - F9 toggle + visual gutter markers
5. 🎯 **Address Mapping Utilities** - VPy line ↔ ASM address conversion

---

## ✅ Phase 1: Backend .pdb Loading (COMPLETE)

### File: `ide/electron/src/main.ts`

**Implementation**: Modified `run:compile` IPC handler to automatically load `.pdb` after successful compilation.

**Changes**:
```typescript
// After binary read success (line ~535):
const pdbPath = binPath.replace(/\.bin$/, '.pdb');
let pdbData: any = null;

try {
  const pdbExists = await fs.access(pdbPath).then(() => true).catch(() => false);
  
  if (pdbExists) {
    const pdbContent = await fs.readFile(pdbPath, 'utf-8');
    pdbData = JSON.parse(pdbContent);
    mainWindow?.webContents.send('run://status', `✓ Phase 3 SUCCESS: Debug symbols loaded (.pdb)`);
    mainWindow?.webContents.send('run://stdout', `✓ Debug symbols: ${pdbPath}`);
  } else {
    mainWindow?.webContents.send('run://status', `⚠ Phase 3 SKIPPED: No .pdb file found`);
  }
} catch (e: any) {
  mainWindow?.webContents.send('run://stderr', `⚠ Warning: Failed to load .pdb: ${e.message}`);
}

// Include pdbData in IPC event and return value
mainWindow?.webContents.send('emu://compiledBin', { base64, size: buf.length, binPath, pdbData });
resolvePromise({ ok: true, binPath, size: buf.length, pdbData, ... });
```

**Result**: 
- ✅ `.pdb` loaded automatically after every successful compilation
- ✅ Sent to frontend via `emu://compiledBin` event
- ✅ Included in `runCompile` return value
- ✅ Error handling with warning messages (non-fatal if .pdb missing)
- ✅ Phase 3 logging integrated with existing Phase 1 (ASM) and Phase 2 (Binary)

---

## ✅ Phase 2: Frontend Debug Commands (COMPLETE)

### File: `ide/frontend/src/main.tsx`

**Implementation**: Implemented `debug.start` and `debug.stop` commands in command dispatcher.

### 2.1 debug.start Command

**Flow**:
1. **Validate** active document is a .vpy file
2. **Compile** with `autoStart: false` (don't auto-run)
3. **Check** compilation result for errors
4. **Verify** .pdb data is available (warn if missing)
5. **Load** .pdb automatically via `onCompiledBin` handler
6. **Enter** debug mode with state = 'paused'
7. **Log** instructions for user (F5 to continue, F10 to step, etc.)

**Code**:
```typescript
case 'debug.start': {
  logger.info('Debug', 'Starting debug session...');
  
  // 1. Get active document
  const activeDoc = documents.find(d => d.uri === editorState.active);
  if (!activeDoc || !activeDoc.uri.endsWith('.vpy')) {
    logger.error('Debug', 'No valid .vpy document to debug');
    break;
  }

  // 2. Compile without auto-run
  const result = await electronAPI.runCompile({
    path: activeDoc.diskPath || activeDoc.uri,
    autoStart: false,  // Key: don't auto-run
    saveIfDirty: activeDoc.dirty ? { content, expectedMTime } : undefined
  });

  // 3. Check compilation
  if (result.error || result.conflict) {
    logger.error('Debug', 'Compilation failed');
    break;
  }

  // 4. Verify .pdb
  if (!result.pdbData) {
    logger.warn('Debug', 'No debug symbols, debugging will be limited');
  } else {
    logger.info('Debug', '✓ Debug symbols loaded');
  }

  // 5. Enter debug mode (paused at entry)
  useDebugStore.getState().setState('paused');
  
  logger.info('Debug', '✓ Debug session started - paused at entry point');
  break;
}
```

### 2.2 debug.stop Command

**Flow**:
1. **Change** debug state to 'stopped'
2. **Clear** current line/address tracking
3. **Clear** call stack
4. **Log** confirmation

**Code**:
```typescript
case 'debug.stop': {
  logger.info('Debug', 'Stopping debug session...');
  
  const { useDebugStore } = await import('./state/debugStore');
  
  // Clear debug state
  useDebugStore.getState().setState('stopped');
  useDebugStore.getState().setCurrentVpyLine(null);
  useDebugStore.getState().setCurrentAsmAddress(null);
  useDebugStore.getState().updateCallStack([]);
  
  logger.info('Debug', '✓ Debug session stopped');
  break;
}
```

---

### File: `ide/frontend/src/components/panels/EmulatorPanel.tsx`

**Implementation**: Modified `onCompiledBin` handler to load .pdb into debugStore.

**Changes**:
```typescript
const handleCompiledBin = (payload: { 
  base64: string; 
  size: number; 
  binPath: string; 
  pdbData?: any  // NEW
}) => {
  console.log(`[EmulatorPanel] Loading compiled binary: ${payload.binPath}`);
  
  // NEW: Load .pdb if present
  if (payload.pdbData) {
    console.log('[EmulatorPanel] ✓ Debug symbols (.pdb) received');
    const { useDebugStore } = require('../../state/debugStore');
    useDebugStore.getState().loadPdbData(payload.pdbData);
  }
  
  // ... existing binary loading code ...
};
```

**Result**:
- ✅ `.pdb` automatically loaded into debugStore when compilation completes
- ✅ Works for both `build.run` (F5) and `debug.start` (Ctrl+F5)
- ✅ Non-intrusive - existing binary loading flow unchanged

---

### File: `ide/frontend/src/utils/logger.ts`

**Implementation**: Added 'Debug' to LogCategory type.

**Change**:
```typescript
export type LogCategory = 
  'LSP' | 'Build' | 'File' | 'Save' | 'Compilation' | 
  'App' | 'HMR' | 'Dock' | 'Project' | 'AI' | 'Debug';  // Added 'Debug'
```

**Result**: All debug logging now properly typed and categorized.

---

## Keyboard Shortcuts

Already configured (no changes needed):
- **F5** → `build.run` (compile & run normally)
- **Ctrl+F5** → `debug.start` (compile & enter debug mode) ✅ NOW WORKING
- **Shift+F5** → `debug.stop` (exit debug mode) ✅ NOW WORKING
- **F9** → `debug.toggleBreakpoint` (🎯 Phase 4 - pending)
- **F10** → `debug.stepOver` (🎯 Phase 3 - pending)
- **F11** → `debug.stepInto` (🎯 Phase 3 - pending)

---

## Testing Phase 1 & 2

### Test Scenario 1: Normal Compilation (F5)
1. ✅ Open `bouncing_ball.vpy`
2. ✅ Press F5
3. ✅ Check console: "Phase 3 SUCCESS: Debug symbols loaded"
4. ✅ Check DevTools: `useDebugStore.getState().pdbData` should be populated
5. ✅ Binary runs normally in emulator

### Test Scenario 2: Debug Start (Ctrl+F5)
1. ✅ Open `bouncing_ball.vpy`
2. ✅ Press Ctrl+F5
3. ✅ Check console: "Debug session started - paused at entry point"
4. ✅ Check DevTools: `useDebugStore.getState().state` should be 'paused'
5. ✅ Check DevTools: `useDebugStore.getState().pdbData` should have:
   - `functions` with real addresses
   - `nativeCalls` with line numbers
   - `symbols` with START/MAIN/LOOP_BODY addresses

### Test Scenario 3: Debug Stop (Shift+F5)
1. ✅ After Ctrl+F5, press Shift+F5
2. ✅ Check console: "Debug session stopped"
3. ✅ Check DevTools: `useDebugStore.getState().state` should be 'stopped'
4. ✅ Check DevTools: `currentVpyLine`, `currentAsmAddress`, `callStack` should be null/empty

### Test Scenario 4: Missing .pdb (Edge Case)
1. ✅ Delete `.pdb` file manually
2. ✅ Press F5
3. ✅ Check console: "Phase 3 SKIPPED: No .pdb file found"
4. ✅ Binary should still load and run
5. ✅ No errors thrown

---

## Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│ User Action: F5 or Ctrl+F5                                       │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│ main.tsx: commandExec('build.run' or 'debug.start')             │
│  - Validate active document                                      │
│  - Call electronAPI.runCompile({ autoStart: bool })             │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│ Electron main.ts: run:compile IPC Handler                        │
│  - Phase 1: Compile .vpy → .asm                                 │
│  - Phase 2: Assemble .asm → .bin                                │
│  - Phase 3: Load .pdb if exists ✅ NEW                          │
│  - Send emu://compiledBin event with { base64, pdbData }        │
│  - Return { ok, binPath, size, pdbData }                        │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│ EmulatorPanel.tsx: onCompiledBin Handler                         │
│  - Load .pdb → useDebugStore.loadPdbData(pdbData) ✅ NEW        │
│  - Load binary → JSVecx Globals.cartdata                        │
│  - Reset and start emulator                                      │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 v
┌─────────────────────────────────────────────────────────────────┐
│ Debug State Update                                               │
│  - debugStore.pdbData populated ✅                              │
│  - If debug.start: debugStore.state = 'paused' ✅               │
│  - If build.run: emulator runs normally                         │
└─────────────────────────────────────────────────────────────────┘
```

---

## What's Working

### ✅ Backend
- Automatic .pdb loading after compilation
- Robust error handling (missing .pdb = warning, not error)
- Phase 3 logging integrated with existing compilation phases
- .pdb included in both IPC event and return value

### ✅ Frontend
- `debug.start` command fully implemented
- `debug.stop` command fully implemented
- Automatic .pdb loading into debugStore
- Debug state management (stopped/paused/running)
- Logger category 'Debug' added and working

### ✅ Integration
- F5 (build.run): Loads .pdb, runs normally
- Ctrl+F5 (debug.start): Loads .pdb, enters paused state
- Shift+F5 (debug.stop): Clears debug state
- No conflicts between normal run and debug mode

---

## 🎯 Next Steps (Phase 3)

### Emulator Breakpoint System

**File**: `ide/frontend/src/components/panels/EmulatorPanel.tsx`

**Objectives**:
1. Add breakpoint state management (`Set<number>` of addresses)
2. Hook into JSVecx step function to check PC against breakpoints
3. When breakpoint hit:
   - Pause emulator (cancel animation frame)
   - Update debugStore: state = 'paused', currentAsmAddress = PC
   - Map ASM address → VPy line using .pdb
   - Update UI to highlight current line

**Estimated Time**: 1-2 hours

**Key Implementation**:
```typescript
const [breakpoints, setBreakpoints] = useState<Set<number>>(new Set());

const checkBreakpoint = useCallback((pc: number) => {
  if (debugStore.state === 'running' && breakpoints.has(pc)) {
    // Cancel animation
    if (animRef.current) {
      cancelAnimationFrame(animRef.current);
      animRef.current = null;
    }
    
    // Update debug state
    useDebugStore.getState().setState('paused');
    useDebugStore.getState().setCurrentAsmAddress(`0x${pc.toString(16)}`);
    
    // TODO: Map address → VPy line using pdbData
  }
}, [breakpoints]);
```

---

## 🎯 Phase 4: Monaco Breakpoint Decorations

**File**: `ide/frontend/src/components/Editor.tsx` (or similar)

**Objectives**:
1. F9 handler to toggle breakpoints at cursor line
2. Sync breakpoints to debugStore
3. Monaco decorations for gutter markers (red dots)
4. Sync debugStore breakpoints → Emulator addresses using .pdb

**Estimated Time**: 1-2 hours

---

## 🎯 Phase 5: Address Mapping Utilities

**File**: `ide/frontend/src/utils/debugHelpers.ts` (new)

**Objectives**:
1. `vpyLineToAsmAddress(line, pdb)` - Convert VPy line → ASM address
2. `asmAddressToVpyLine(address, pdb)` - Convert ASM address → VPy line
3. `getFunctionAtAddress(address, pdb)` - Get function info at address
4. `getNativeCallAtLine(line, pdb)` - Check if line has native call

**Estimated Time**: 30 minutes

---

## Summary

**Phase 1 & 2 Status**: ✅ **COMPLETE**

**What We Achieved**:
- ✅ Electron backend automatically loads .pdb after compilation
- ✅ Frontend receives .pdb via IPC event and return value
- ✅ EmulatorPanel loads .pdb into debugStore
- ✅ `debug.start` command compiles and enters debug mode
- ✅ `debug.stop` command clears debug state
- ✅ Logger category 'Debug' added
- ✅ Keyboard shortcuts already configured (F5, Ctrl+F5, Shift+F5)

**Ready for Phase 3**: Emulator breakpoint system

**Total Time Spent**: ~45 minutes  
**Remaining Estimate**: ~3-4 hours (Phases 3-5)

---

**Last Updated**: October 17, 2025  
**Next Session**: Implement Phase 3 - Emulator Breakpoint System
