# Debug Integration Plan - .pdb + Breakpoints

**Date**: October 17, 2025  
**Goal**: Full F5 debugging experience with .pdb support

## Implementation Plan

### ✅ Phase 1: Backend (Electron) - .pdb Loading

**File**: `ide/backend/main.js` or similar

Add handlers:
```javascript
// Load .pdb file (JSON)
ipcMain.handle('load-pdb-file', async (event, pdbPath) => {
  try {
    const content = await fs.readFile(pdbPath, 'utf-8');
    const pdb = JSON.parse(content);
    return { success: true, pdb };
  } catch (error) {
    return { success: false, error: error.message };
  }
});
```

Modify `runCompile` to auto-load .pdb:
```javascript
ipcMain.handle('run-compile', async (event, args) => {
  // ... existing compilation code ...
  
  // After successful compilation:
  const pdbPath = asmPath.replace('.asm', '.pdb');
  if (fs.existsSync(pdbPath)) {
    const pdbContent = await fs.readFile(pdbPath, 'utf-8');
    result.pdbData = JSON.parse(pdbContent);
  }
  
  return result;
});
```

### 🎯 Phase 2: Emulator - Breakpoint System

**File**: `ide/frontend/src/components/panels/EmulatorPanel.tsx`

Add breakpoint state:
```typescript
const [breakpoints, setBreakpoints] = useState<Set<number>>(new Set());
const [isDebugging, setIsDebugging] = useState(false);

// Hook into emulator step
const checkBreakpoint = useCallback((pc: number) => {
  if (isDebugging && breakpoints.has(pc)) {
    // Pause emulation
    if (animRef.current) {
      cancelAnimationFrame(animRef.current);
      animRef.current = null;
    }
    
    // Update debug state
    useDebugStore.getState().setState('paused');
    useDebugStore.getState().setCurrentAsmAddress(`0x${pc.toString(16).padStart(4, '0')}`);
    
    // Find VPy line from .pdb
    const pdb = useDebugStore.getState().pdbData;
    if (pdb) {
      const vpyLine = findVpyLineForAddress(pc, pdb);
      if (vpyLine !== null) {
        useDebugStore.getState().setCurrentVpyLine(vpyLine);
      }
    }
  }
}, [isDebugging, breakpoints]);

// Modify step() to check breakpoints
const step = useCallback(() => {
  if (!emulator) return;
  
  const pc = emulator.cpu().program_counter();
  checkBreakpoint(pc);
  
  // Execute instruction
  emulator.step();
  
  // ... existing code ...
}, [emulator, checkBreakpoint]);
```

### 🎯 Phase 3: Frontend - Debug Commands

**File**: `ide/frontend/src/main.tsx`

Implement debug.start:
```typescript
case 'debug.start': {
  logger.info('Debug', 'Starting debug session');
  
  // 1. Compile with debug info
  await handleBuild(false); // Don't auto-run
  
  // 2. Load .pdb
  const activeDoc = documents.find(d => d.uri === useEditorStore.getState().active);
  if (!activeDoc) break;
  
  const pdbPath = activeDoc.diskPath.replace('.vpy', '.pdb');
  const pdbResult = await (window as any).electronAPI.loadPdbFile(pdbPath);
  
  if (pdbResult.success) {
    useDebugStore.getState().loadPdbData(pdbResult.pdb);
    logger.info('Debug', '.pdb loaded successfully');
  } else {
    logger.error('Debug', 'Failed to load .pdb:', pdbResult.error);
    break;
  }
  
  // 3. Load ROM into emulator
  const binPath = activeDoc.diskPath.replace('.vpy', '.bin');
  // Call emulator's loadROM function
  // ... (implementation depends on EmulatorPanel API)
  
  // 4. Enter debug mode (paused at entry point)
  useDebugStore.getState().setState('paused');
  useDebugStore.getState().setCurrentVpyLine(null);
  
  logger.info('Debug', 'Debug session started - paused at entry');
  break;
}

case 'debug.stop': {
  logger.info('Debug', 'Stopping debug session');
  useDebugStore.getState().setState('stopped');
  useDebugStore.getState().setCurrentVpyLine(null);
  useDebugStore.getState().loadPdbData(null);
  // Stop emulator
  break;
}

case 'debug.continue': {
  // Resume execution until next breakpoint
  useDebugStore.getState().setState('running');
  // Tell emulator to continue
  break;
}
```

### 🎯 Phase 4: Breakpoint Synchronization

**File**: `ide/frontend/src/main.tsx`

Add breakpoint toggle:
```typescript
case 'debug.toggleBreakpoint': {
  const editor = (window as any).monacoEditor;
  if (!editor) break;
  
  const position = editor.getPosition();
  const model = editor.getModel();
  if (!model || !position) break;
  
  const line = position.lineNumber;
  const uri = model.uri.toString();
  
  // Get current breakpoints
  const decorations = model.getDecorations();
  const hasBreakpoint = decorations.some(d => 
    d.range.startLineNumber === line && d.options.glyphMarginClassName === 'breakpoint-glyph'
  );
  
  if (hasBreakpoint) {
    // Remove breakpoint
    useDebugStore.getState().onBreakpointRemoved(uri, line);
  } else {
    // Add breakpoint
    useDebugStore.getState().onBreakpointAdded(uri, line);
  }
  
  logger.debug('Debug', `Toggled breakpoint at ${uri}:${line}`);
  break;
}
```

Add keyboard shortcut (F9):
```typescript
else if (e.key === 'F9') { 
  e.preventDefault(); 
  commandExec('debug.toggleBreakpoint'); 
}
```

### 🎯 Phase 5: Address Mapping Helper

**File**: `ide/frontend/src/utils/debugHelpers.ts` (new)

```typescript
import type { PdbData } from '../state/debugStore';

/**
 * Find VPy source line for a given ASM address
 */
export function findVpyLineForAddress(address: number, pdb: PdbData): number | null {
  const addrHex = `0x${address.toString(16).padStart(4, '0').toUpperCase()}`;
  
  // Check if address is in lineMap
  for (const [line, addr] of Object.entries(pdb.lineMap)) {
    if (addr.toUpperCase() === addrHex) {
      return parseInt(line, 10);
    }
  }
  
  // Check if address is a function entry point
  if (pdb.functions) {
    for (const func of Object.values(pdb.functions)) {
      if (func.address.toUpperCase() === addrHex) {
        return func.startLine;
      }
    }
  }
  
  return null;
}

/**
 * Find ASM address for a VPy source line
 */
export function findAddressForVpyLine(line: number, pdb: PdbData): number | null {
  const lineStr = line.toString();
  
  // Direct lookup in lineMap
  if (pdb.lineMap[lineStr]) {
    return parseInt(pdb.lineMap[lineStr], 16);
  }
  
  // Check if line is within a function's range
  if (pdb.functions) {
    for (const func of Object.values(pdb.functions)) {
      if (line >= func.startLine && line <= func.endLine) {
        // Use function's entry address as fallback
        return parseInt(func.address, 16);
      }
    }
  }
  
  return null;
}

/**
 * Convert VPy line breakpoints to ASM address breakpoints
 */
export function vpyBreakpointsToAddresses(
  vpyLines: number[], 
  pdb: PdbData
): Set<number> {
  const addresses = new Set<number>();
  
  for (const line of vpyLines) {
    const addr = findAddressForVpyLine(line, pdb);
    if (addr !== null) {
      addresses.add(addr);
    }
  }
  
  return addresses;
}
```

## Workflow Example

### User Presses F5:
1. **Compile**: `vectrexc build examples/bouncing_ball.vpy`
   - Generates: `bouncing_ball.asm`, `bouncing_ball.bin`, `bouncing_ball.pdb`

2. **Load .pdb**: Frontend loads JSON with addresses + line mapping

3. **Enter Debug Mode**: Emulator pauses at entry point

4. **User Sets Breakpoints**: Click gutter in editor → Line 42

5. **User Presses F5 (Continue)**: Emulator runs until PC reaches address for line 42

6. **Hit Breakpoint**: Emulator pauses, highlights line 42 in editor

7. **User Presses F10 (Step Over)**: Execute next instruction, update highlight

8. **User Presses Shift+F5 (Stop)**: Exit debug mode

## Integration Points

### Monaco Editor:
- Add glyph margin decorations for breakpoints (red dots)
- Add current line decoration (yellow highlight)
- Sync breakpoints with debug store

### Emulator Panel:
- Add `setBreakpoints(addresses: Set<number>)` method
- Add `isDebugging` prop
- Modify step loop to check breakpoints

### Debug Store:
- Track VPy line breakpoints: `vpyBreakpoints: Set<number>`
- Convert to addresses when .pdb loaded
- Sync with emulator

## Files to Modify

1. **ide/backend/main.js** (+30 lines)
   - Add `load-pdb-file` handler
   - Modify `run-compile` to auto-load .pdb

2. **ide/frontend/src/main.tsx** (+150 lines)
   - Implement `debug.start`
   - Implement `debug.stop`
   - Implement `debug.toggleBreakpoint`
   - Add F9 keyboard shortcut

3. **ide/frontend/src/components/panels/EmulatorPanel.tsx** (+80 lines)
   - Add breakpoint checking in step loop
   - Add `setBreakpoints()` method
   - Add debug mode state

4. **ide/frontend/src/state/debugStore.ts** (+40 lines)
   - Add `vpyBreakpoints` state
   - Add `addBreakpoint()` / `removeBreakpoint()` actions
   - Auto-convert to addresses when .pdb loaded

5. **ide/frontend/src/utils/debugHelpers.ts** (new, +100 lines)
   - Address mapping utilities

6. **ide/frontend/src/components/Editor.tsx** (+60 lines)
   - Monaco breakpoint decorations
   - Current line decoration
   - F9 handling

## Testing Plan

1. **Basic Compilation**: F5 compiles and loads .pdb ✅
2. **Breakpoint Toggle**: F9 adds/removes breakpoints ✅
3. **Hit Breakpoint**: Emulator pauses at correct line ✅
4. **Step Over**: F10 executes one instruction ✅
5. **Continue**: F5 resumes until next breakpoint ✅
6. **Stop**: Shift+F5 exits debug mode ✅

## Timeline Estimate

- Phase 1 (Backend): 30 minutes
- Phase 2 (Emulator): 1 hour
- Phase 3 (Debug Commands): 1 hour
- Phase 4 (Breakpoints): 1 hour
- Phase 5 (Address Mapping): 30 minutes
- Testing & Polish: 1 hour

**Total**: ~5 hours for complete implementation

## Next Steps

Start with Phase 1 (Backend) - simplest and foundational.
