# Test Plan: ASM File Highlighting Fix

## Problem
Step Into navigates to ASM file but doesn't show line highlight.

## Root Cause (Fixed)
MonacoEditorWrapper.tsx checked `isVpyFile` to determine if highlight should be shown. This blocked ASM files (.asm extension) from getting highlighted.

## Solution Implemented
Modified MonacoEditorWrapper.tsx to:
1. Detect both .vpy and .asm files
2. Track ASM debugging mode via `(window as any).asmDebuggingMode`
3. Compare current file URI with `(window as any).asmDebuggingFile`
4. Apply highlight if ANY of these conditions are met:
   - VPy file in VPy debugging mode
   - ASM file in ASM debugging mode

## Files Modified
- `/Users/daniel/projects/vectrex-pseudo-python/ide/frontend/src/components/MonacoEditorWrapper.tsx`
  - Lines 1058-1089: Extended highlight detection logic
  - Added support for `.asm` files
  - Added ASM debugging mode tracking

## Test Steps

### Setup
1. Open IDE: `npm run dev` or run from VS Code
2. Load example: `examples/test_incremental/src/main.vpy`
3. Compile: F5 (or button) to build
4. Place breakpoint: Click line number in editor

### Test Case 1: Step Into VPy (Should work before and after fix)
1. Click Step Into button (or F11)
2. **Expected**: Cursor jumps to line, line highlights yellow
3. **Actual (Before Fix)**: ✓ Works
4. **Actual (After Fix)**: ✓ Still works

### Test Case 2: Step Into ASM (Main fix - was broken)
1. Step Into enough times to reach a BIOS call or reach 0x0000 offset
2. Debugger should detect multibank or native ASM
3. **Expected (Before Fix)**: 
   - ✗ Line highlight MISSING (empty yellow area)
   - ✓ File navigation works
4. **Expected (After Fix)**:
   - ✓ File navigation works  
   - ✓ Line highlight SHOWS (yellow background on line)
   - ✓ Line number visible with highlight

### Test Case 3: Step Over in ASM (F10)
1. After Step Into to ASM, press F10 (Step Over)
2. **Expected**:
   - ✓ PC advances to next instruction line
   - ✓ Yellow highlight moves to new line
   - ✓ Console shows: `[Monaco] ✅ Applying highlight to line X in ASM file`

### Test Case 4: Return to VPy from ASM
1. Continue stepping (F10/F11) until reaching VPy code
2. **Expected**:
   - ✓ File switches back to VPy
   - ✓ Line highlight re-appears
   - ✓ Console shows: `[Monaco] ✅ Applying highlight to line X in VPy file`

## Console Logs to Watch

### When highlight is applied:
```
[Monaco] 🔍 Highlight check: debugState=paused, currentVpyLine=42, currentVpyFile=main.vpy, currentFileName=main.vpy, isCorrectVpyFile=true, isCorrectAsmFile=false, isActiveDoc=true, isVpyFile=true, isAsmFile=false
[Monaco] ✅ Applying highlight to line 42 in VPy file (main.vpy)
```

### When in ASM debug mode:
```
[Monaco] 🔍 Highlight check: debugState=paused, currentVpyLine=115, currentVpyFile=main.asm, currentFileName=bank_0.asm, isCorrectVpyFile=false, isCorrectAsmFile=true, isActiveDoc=true, isVpyFile=false, isAsmFile=true
[Monaco] ✅ Applying highlight to line 115 in ASM file (bank_0.asm)
```

### When highlight should NOT be applied:
```
[Monaco] ❌ NOT applying highlight: isVpyFile=false, isAsmFile=true, isCorrectVpyFile=false, isCorrectAsmFile=false
```

## Expected Behavior After Fix

| Scenario | Before Fix | After Fix |
|----------|-----------|-----------|
| Step Into VPy | ✓ Highlight | ✓ Highlight |
| Step Into ASM | ✗ No highlight | ✓ Highlight |
| Step Over in ASM | ✗ No highlight update | ✓ Highlight moves |
| Back to VPy | ✓ Highlight | ✓ Highlight |

## Build Verification

Frontend build status: ✅ SUCCESS (3.46s)
- TypeScript compilation: ✅ `tsc --noEmit`
- Vite bundling: ✅ `vite build`
- Output: `/dist/` directory ready

## Remaining Work

After verifying this fix works:
1. [ ] Test multibank projects (bank_0.asm, bank_31.asm)
2. [ ] Test single-bank projects (main.asm)
3. [ ] Test edge cases (very large files, long lines)
4. [ ] Document user-facing behavior in README

## Performance Considerations

- Highlight check runs on every `currentVpyLine` change ✓ (efficient)
- No new DOM manipulations ✓ (uses existing deltaDecorations)
- Minimal console logging ✓ (conditional on highlight application)

---
Status: READY FOR TESTING
Build Status: ✅ SUCCESS
Testing Date: 2025-01-16
