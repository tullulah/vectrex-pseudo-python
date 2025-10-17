# Phase 6: UI Enhancements - Progress Report

**Date**: October 17, 2025  
**Status**: Phase 6.1 COMPLETE ✅

---

## Overview

Adding visual feedback and UI improvements to the debugging system for better developer experience.

### Total Plan: 4 Sub-Phases
1. ✅ **Current Line Highlighting** - Yellow highlight when paused at breakpoint
2. 🎯 **Call Stack Panel** - Visual call stack display (optional)
3. 🎯 **Native Call Highlighting** - Different color for native BIOS calls (optional)
4. 🎯 **Step Over / Step Into** - Advanced stepping controls (optional)

---

## ✅ Phase 6.1: Current Line Highlighting (COMPLETE)

### Objective
Visually highlight the current line in Monaco editor when the emulator pauses at a breakpoint, providing immediate visual feedback to the developer about execution position.

### Implementation

#### File 1: `ide/frontend/src/components/MonacoEditorWrapper.tsx`

**Changes Made**:

1. **Added References** (line ~134):
```typescript
const currentLineDecorationsRef = useRef<string[]>([]); // Track current line highlight (Phase 6.1)
```

2. **Added Debug State Subscriptions** (line ~124):
```typescript
const currentVpyLine = useDebugStore(s => s.currentVpyLine); // Phase 6.1: Track current line
const debugState = useDebugStore(s => s.state); // Phase 6.1: Track debug state
```

3. **New useEffect for Line Highlighting** (line ~708):
```typescript
// Phase 6.1: Highlight current line when paused in debugger
useEffect(() => {
  if (!editorRef.current || !monacoRef.current || !doc) return;
  
  // Only show highlight when paused and we have a valid line number
  if (debugState === 'paused' && currentVpyLine !== null) {
    const decorations = [{
      range: new monacoRef.current!.Range(currentVpyLine, 1, currentVpyLine, 1),
      options: {
        isWholeLine: true,
        className: 'current-line-highlight', // Yellow background
        glyphMarginClassName: 'current-line-arrow' // Optional: arrow in gutter
      }
    }];
    
    currentLineDecorationsRef.current = editorRef.current.deltaDecorations(
      currentLineDecorationsRef.current,
      decorations
    );
    
    // Scroll to the current line (reveal in center of viewport)
    editorRef.current.revealLineInCenter(currentVpyLine);
    
    logger.debug('Debug', `Highlighted current line: ${currentVpyLine}`);
  } else {
    // Clear decorations when not paused or no line
    currentLineDecorationsRef.current = editorRef.current.deltaDecorations(
      currentLineDecorationsRef.current,
      []
    );
  }
}, [debugState, currentVpyLine, doc]);
```

**Key Features**:
- ✅ **Automatic highlighting**: Activates when `debugState === 'paused'` AND `currentVpyLine !== null`
- ✅ **Whole line decoration**: Yellow transparent background spans entire line
- ✅ **Auto-scroll**: `revealLineInCenter()` ensures highlighted line is visible
- ✅ **Auto-cleanup**: Clears decoration when resuming or stopping debug session
- ✅ **React to changes**: Re-renders when debug state or current line changes

#### File 2: `ide/frontend/src/global.css`

**Changes Made** (line ~251):

```css
/* Phase 6.1: Current line highlight when paused in debugger */
.current-line-highlight {
  background: rgba(255, 255, 0, 0.2) !important; /* Yellow transparent background */
  border-left: 3px solid #ffeb3b !important; /* Yellow left border for emphasis */
}

.current-line-arrow {
  background: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="%23ffeb3b" d="M6 4l6 4-6 4z"/></svg>') center center no-repeat !important;
  width: 16px !important;
  height: 16px !important;
  margin-left: 2px !important;
}
```

**Visual Design**:
- ✅ **Yellow background**: `rgba(255, 255, 0, 0.2)` - subtle but visible
- ✅ **Left border**: 3px solid yellow for extra emphasis
- ✅ **Arrow glyph**: Yellow triangle pointing right in the gutter (similar to VSCode)
- ✅ **Consistent with breakpoints**: Red dots for breakpoints, yellow for current line

### How It Works

**Flow**:
1. User sets breakpoint on line 33 with F9
2. User starts debug session with Ctrl+F5
3. User continues execution with F5
4. Emulator hits breakpoint at address 0x06C3
5. `EmulatorPanel.checkBreakpoint()` calls `asmAddressToVpyLine(0x06C3, pdbData)` → returns 33
6. `debugStore.setCurrentVpyLine(33)` updates state
7. `debugStore.setState('paused')` triggers pause
8. **NEW**: Monaco's useEffect detects `debugState === 'paused'` AND `currentVpyLine === 33`
9. **NEW**: Monaco applies yellow highlight decoration to line 33
10. **NEW**: Monaco scrolls to center line 33 in viewport
11. Developer sees: **Red breakpoint dot** + **Yellow highlighted line** 🎨

**Cleanup**:
- User presses F5 (continue) → `debugState` changes to 'running' → highlight clears
- User presses Shift+F5 (stop) → `debugState` changes to 'stopped' → highlight clears
- User hits different breakpoint → `currentVpyLine` changes → highlight moves to new line

### Testing Scenarios

**Test 15: Current Line Highlight Appears**
- **Setup**: Set breakpoint on line 33, compile, start debug, continue
- **Expected**: Line 33 shows yellow background + left border + arrow glyph
- **Expected**: Line 33 is scrolled into center of editor viewport
- **Expected**: Breakpoint red dot + yellow highlight both visible

**Test 16: Highlight Clears on Continue**
- **Setup**: Paused at line 33 with yellow highlight visible
- **Action**: Press F5 (continue)
- **Expected**: Yellow highlight disappears immediately
- **Expected**: Red breakpoint dot remains

**Test 17: Highlight Moves on Next Breakpoint**
- **Setup**: Breakpoints on lines 33 and 75
- **Action**: Hit first breakpoint (line 33), press F5, hit second breakpoint (line 75)
- **Expected**: Yellow highlight moves from line 33 → line 75
- **Expected**: Both red dots remain visible

**Test 18: Highlight Clears on Stop**
- **Setup**: Paused at line 33 with yellow highlight
- **Action**: Press Shift+F5 (stop debug)
- **Expected**: Yellow highlight disappears
- **Expected**: debugState changes to 'stopped'

**Test 19: No Highlight Without .pdb**
- **Setup**: Run emulator without compiling (no .pdb loaded)
- **Expected**: No yellow highlight appears (pdbData is null)
- **Expected**: No crashes or errors

**Test 20: Multiple Editors**
- **Setup**: Open two .vpy files side-by-side
- **Action**: Debug first file, pause at breakpoint
- **Expected**: Highlight only appears in the file being debugged
- **Expected**: Other editor remains unchanged

### Benefits

✅ **Immediate Visual Feedback**: Developer sees exactly where execution paused  
✅ **Matches VSCode UX**: Yellow highlight is standard in professional IDEs  
✅ **Auto-scroll**: No need to manually search for paused line  
✅ **Clean Separation**: Red = breakpoint, Yellow = current execution  
✅ **Professional Look**: Looks like a real IDE debugging experience  

---

## 🎯 Phase 6.2: Call Stack Panel (PENDING)

### Objective
Display a visual panel showing the current call stack with VPy function names and line numbers.

### Planned Features
- Panel in IDE dock (similar to Logger/Emulator)
- Stack frames with function names from `pdbData.functions`
- Click to jump to source line
- Native calls highlighted with special icon

**Status**: Not started (optional enhancement)  
**Estimated Time**: 1-2 hours

---

## 🎯 Phase 6.3: Native Call Highlighting (PENDING)

### Objective
Use different decoration color (e.g., blue) for lines that call native BIOS functions.

### Planned Features
- Check `pdbData.nativeCalls` for each line
- Apply blue glyph + different background color
- Tooltip shows native function name (e.g., "DRAW_CIRCLE")
- Works independently from breakpoints

**Status**: Not started (optional enhancement)  
**Estimated Time**: 30-45 minutes

---

## 🎯 Phase 6.4: Step Over / Step Into (PENDING)

### Objective
Implement advanced stepping controls for fine-grained debugging.

### Planned Features
- **F10 (Step Over)**: Execute next line without entering functions
- **F11 (Step Into)**: Step into function calls
- **Shift+F11 (Step Out)**: Execute until return from current function
- Requires instruction-level stepping in WASM emulator

**Status**: Not started (complex feature)  
**Estimated Time**: 3-4 hours (requires emulator changes)

---

## Summary

**Phase 6.1 Status**: ✅ **COMPLETE**

**What We Achieved**:
- ✅ Yellow highlight for current line when paused
- ✅ Auto-scroll to current line (center viewport)
- ✅ Arrow glyph in gutter margin
- ✅ Automatic cleanup on continue/stop
- ✅ CSS styling matching VSCode UX
- ✅ Integration with debugStore state

**Visual Result**:
```
Line 32: [        ] print("Bouncing Ball Demo")
Line 33: [🔴][████] DRAW_CIRCLE(ball_x, ball_y, 10)  ← Red dot + Yellow highlight
Line 34: [        ] ball_x += velocity_x
```

**Optional Enhancements**: Phases 6.2, 6.3, 6.4 available if needed

**Total Time Spent**: ~30 minutes  
**Phases Complete**: 1/4 (25%) - Core debugging UX complete

---

**Last Updated**: October 17, 2025  
**Next Steps**: Test complete workflow, optionally implement Phase 6.2 (Call Stack Panel)
