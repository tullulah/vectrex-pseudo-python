# Breakpoint Fix - debugStore State Synchronization

**Issue**: When starting the emulator, breakpoints were not being hit even though they were registered. The debugStore.state remained `'stopped'` while JSVecx was `'running'`, causing the breakpoint check to be skipped.

**Root Cause**: 
```typescript
// In EmulatorPanel.tsx, when user clicks Play:
const onPlay = () => {
  const vecx = (window as any).vecx;
  if (vecx) {
    vecx.start();  // ✓ Starts JSVecx
    setStatus('running');  // ✓ Updates emulatorStore.status
    // ✗ BUT: debugStore.state was NOT updated!
  }
};

// In checkBreakpointHit():
if (debugState !== 'running') return;  // ✗ SKIPPED because debugState='stopped'!
```

**Error State Observed**:
```
Debug State: STOPPED | JSVecx: RUNNING ⚠️ MISMATCH
```

**Solution**: Synchronize debugStore.state with emulatorStore.status whenever user controls playback.

## Changes Made

### File: `ide/frontend/src/components/panels/EmulatorPanel.tsx`

**Updated Functions**:

1. **onPlay()** - Now updates debugStore
```typescript
const onPlay = () => {
  const vecx = (window as any).vecx;
  if (vecx) {
    vecx.start();
    setStatus('running');
    useDebugStore.getState().setState('running');  // ✓ NEW
    console.log('[EmulatorPanel] JSVecX started, debugStore.state set to running');
  }
};
```

2. **onPause()** - Now updates debugStore
```typescript
const onPause = () => {
  const vecx = (window as any).vecx;
  if (vecx) {
    vecx.stop();
    setStatus('paused');
    useDebugStore.getState().setState('paused');  // ✓ NEW
    console.log('[EmulatorPanel] JSVecX paused, debugStore.state set to paused');
  }
};
```

3. **onStop()** - Now updates debugStore
```typescript
const onStop = () => {
  const vecx = (window as any).vecx;
  if (vecx) {
    vecx.stop();
    setStatus('stopped');
    useDebugStore.getState().setState('stopped');  // ✓ NEW
    console.log('[EmulatorPanel] JSVecX stopped, debugStore.state set to stopped');
  }
};
```

## How Breakpoints Work Now

### State Flow
```
1. User clicks Play
   ↓
2. onPlay() called
   ↓
3. vecx.start() starts emulation
   ↓
4. setStatus('running') updates emulatorStore
   ↓
5. useDebugStore.getState().setState('running') updates debugStore ✓ NEW
   ↓
6. checkBreakpointHit() runs every 50ms (was skipped before!)
   ↓
7. Checks if vecx.isPausedByBreakpoint()
   ↓
8. If true:
   - vecx.stop() pauses emulation
   - debugStore.setState('paused')
   - Maps address → VPy line
   - Updates currentVpyLine for editor highlighting ✓
```

## Testing Breakpoints

### Test File
```vpy
# test_breakpoint.vpy
def main():
    WAIT_RECAL()        # Line 6 → Address 0x0028
    SET_INTENSITY(80)   # Line 7 → Address 0x0030

def loop():
    MOVE(10, 10)        # Line 11 → Address 0x0051
    DRAW_TO(50, 50)     # Line 13 → Address 0x0071
    DRAW_TO(-50, 0)     # Line 14 → Address 0x0091
```

### Test Steps
1. Compile: `.\target\release\vectrexc.exe build test_breakpoint.vpy`
2. Open IDE
3. Load test_breakpoint.vpy
4. Click on gutter (left margin) of line 6 to set breakpoint
5. Load ROM or default BIOS
6. Click Play button ▶️
7. Expected: Execution pauses at line 6, yellow arrow appears in editor

### Expected Console Output
```
[EmulatorPanel] JSVecX started, debugStore.state set to running
[EmulatorPanel] 🔴 Breakpoint hit detected at PC: 0x0028
[EmulatorPanel] ✓ Emulator paused by breakpoint
[EmulatorPanel] ✓ Mapped to VPy line: 6
[EmulatorPanel] 🛑 Execution paused at breakpoint
```

## Breakpoint Check Interval

The `checkBreakpointHit()` function runs every 50ms via `setInterval`:

```typescript
// In useEffect, around line 544:
breakpointCheckIntervalRef.current = window.setInterval(checkBreakpointHit, 50);
```

This checks:
1. If `debugState === 'running'` (now fixed!)
2. If `vecx.isPausedByBreakpoint()` returns true
3. If true, pause emulation and update UI

## Files Modified

- `ide/frontend/src/components/panels/EmulatorPanel.tsx` (lines 1265-1291)
  - Updated onPlay()
  - Updated onPause()
  - Updated onStop()

## Why This Fixes the Issue

Before: `debugState='stopped'` while `vecxRunning=true`
- `checkBreakpointHit()` exited early (line 492: `if (debugState !== 'running') return;`)
- Breakpoint detection was completely disabled

After: `debugState='running'` when `vecxRunning=true`
- `checkBreakpointHit()` continues to check
- `isPausedByBreakpoint()` can detect and handle breakpoint hits
- Execution pauses and editor is updated with current line

## Verification

The mismatch warning shown before:
```
Debug State: STOPPED | JSVecx: RUNNING ⚠️ MISMATCH
```

Should now show:
```
Debug State: RUNNING | JSVecx: RUNNING ✓ SYNCED
```

This indicates that breakpoint detection is active and breakpoints will be respected when hit.
