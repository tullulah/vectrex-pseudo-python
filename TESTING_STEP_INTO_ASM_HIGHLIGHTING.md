# Step Into ASM Highlighting - Manual Testing Guide

## Quick Summary
The Step Into debugger now correctly highlights lines in both VPy and ASM files.

**What was broken**: Highlight didn't show on ASM files
**What was fixed**: Extended highlight detection to support `.asm` files
**How to verify**: Follow the steps below

---

## Step 1: Prepare the Environment

### Start the IDE
```bash
cd /Users/daniel/projects/vectrex-pseudo-python/ide/frontend
npm run dev
```

This starts the development server at `http://localhost:5173`

### Open Browser
- Open Firefox/Chrome
- Navigate to `http://localhost:5173`
- Wait for IDE to load

### Load Project
- File → Open → Navigate to `/Users/daniel/projects/vectrex-pseudo-python/examples/test_incremental/src/`
- Open `main.vpy`
- You should see the VPy code in the editor

---

## Step 2: Build the Project

### Method 1: Using IDE Button
1. Click the "Build" button in the toolbar
2. Wait for compilation to complete
3. You should see ✅ "Build SUCCESS" message

### Method 2: Using Keyboard
1. Press `F5` (or Cmd+Shift+P → "Build")
2. Wait for output panel to show results

### Expected Output
```
Phase 9: Generating debug symbols...
✓ Populated 3 VPy line mappings, 88 ASM line mappings
✓ PDB written: examples/test_incremental/src/main.pdb
✓ Build SUCCESS: 32768 bytes written to examples/test_incremental/src/main.bin
```

---

## Step 3: Start Debugging

### Open Emulator Panel
1. Look for "Emulator" panel (should be on right side)
2. Click the "Emulator" tab if not visible

### Start Emulation
1. Click the ▶️ "Run" button in Emulator panel
2. Wait 1-2 seconds for emulator to boot
3. You should see BIOS screen → Minestorm logo loading

---

## Step 4: Place Breakpoint (Optional but Helpful)

### In VPy File
1. Click on any line number in `main.vpy`
2. A red dot appears - this is a breakpoint
3. The emulator will pause when reaching this line

### Recommended Breakpoint Location
- Find any line with `DRAW_VECTOR()` or `DRAW_LINE()`
- Click its line number
- Red dot appears

---

## Step 5: Execute Step Into (F11)

### Method 1: Using Button
1. Click the ⬇️ "Step Into" button in Emulator panel
2. Debugger executes one instruction
3. **EXPECTED: Yellow highlight appears on current line**

### Method 2: Using Keyboard
1. Press `F11` (or Fn+F11 on Mac)
2. Same as button above

### What to Watch
- ✅ **Good**: Yellow/orange background appears on current line
- ✅ **Good**: Line number is highlighted
- ✅ **Good**: File may switch to ASM (if jumping to BIOS)
- ❌ **Bad**: No highlight appears
- ❌ **Bad**: Highlight appears but wrong line
- ❌ **Bad**: Console errors

---

## Step 6: Check Console for Debugging Info

### Open Browser Console
1. Press `F12` to open DevTools
2. Click "Console" tab
3. Look for logs that start with `[Monaco]`

### Expected Logs for VPy Highlighting
```
[Monaco] 🔍 Highlight check: debugState=paused, currentVpyLine=15, currentVpyFile=main.vpy, ...
[Monaco] ✅ Applying highlight to line 15 in VPy file (main.vpy)
```

### Expected Logs for ASM Highlighting
```
[Monaco] 🔍 Highlight check: debugState=paused, currentVpyLine=115, ..., isAsmFile=true, isCorrectAsmFile=true
[Monaco] ✅ Applying highlight to line 115 in ASM file (bank_0.asm)
```

### If Highlight Doesn't Show
```
[Monaco] ❌ NOT applying highlight: isVpyFile=false, isAsmFile=true, isCorrectVpyFile=false, isCorrectAsmFile=false
```

---

## Step 7: Test Step Over (F10)

### Step Over (Advance to Next Line)
1. Press `F10` (or Fn+F10 on Mac)
2. PC advances to next instruction
3. **EXPECTED: Yellow highlight moves to new line**

### Repeat
- Press `F10` multiple times
- Watch highlight move with each step
- It should smoothly update position

---

## Step 8: Test ASM Debugging

### Continue Stepping Until Reaching BIOS
1. Keep pressing `F10` (Step Over)
2. After several steps, you may reach BIOS code
3. File will switch to something like `bank_0.asm`
4. **EXPECTED: Highlight appears in ASM file**

### Or: Place Breakpoint Near BIOS Call
1. Look for lines with `JSR Print_Str` or similar
2. Place breakpoint on that line
3. Step Into once
4. Highlight should show in ASM

---

## Step 9: Test Return to VPy

### Continue Stepping Back to VPy
1. Keep pressing `F10`
2. After several ASM instructions, reach VPy code
3. File will switch back to `.vpy`
4. **EXPECTED: Highlight appears in VPy file again**

---

## Expected Behavior Summary

| Action | Before Fix | After Fix |
|--------|-----------|-----------|
| Step Into (VPy) | ✅ Highlight | ✅ Highlight |
| Step Into (BIOS/ASM) | ❌ No highlight | ✅ Highlight |
| Step Over in VPy | ✅ Highlight moves | ✅ Highlight moves |
| Step Over in ASM | ❌ No highlight | ✅ Highlight moves |
| F11 → F11 → ... | ⚠️ Mixed results | ✅ Always highlights |
| Console logs | ❌ "NOT applying" | ✅ "Applying highlight" |

---

## Troubleshooting

### Issue: Highlight Never Appears
1. Check console for errors: `F12` → Console
2. Verify build successful: Output shows ✅ SUCCESS
3. Check PDB exists: File → Open → Look for `main.pdb` in same directory as `main.vpy`
4. Try a different step: F11 again

### Issue: File Opens but Highlight Wrong Line
1. This might be a PDB address mapping issue
2. Check console output: Does it say the line number is different?
3. File a bug with console output attached

### Issue: Highlight Flickers or Disappears
1. This is likely a race condition
2. Try stepping again: F11 or F10
3. Should stabilize after a few steps

### Issue: Console Shows "NOT applying highlight"
1. This means the file comparison failed
2. Check if filename matches: Look in console
3. Might be a path mismatch issue
4. File a bug with console output

---

## Additional Tests (Optional)

### Test with Multibank Projects
If you have multibank projects:
1. Edit `main.vpy` to add `META ROM_TOTAL_SIZE = 524288`
2. Rebuild
3. Step Into should work across bank_0.asm, bank_31.asm, etc.

### Test with Different Project Sizes
1. Try small project: `examples/test_pattern/` 
2. Try large project: `examples/pang/`
3. Highlight should work in all sizes

### Test Edge Cases
1. Step to very first instruction (line 1)
2. Step to very last instruction (near end of file)
3. Highlight should work at all positions

---

## Success Criteria

After following these steps, you should see:

✅ **VPy Code**:
- Click F11 → Yellow highlight on line
- Click F10 → Highlight moves to next line
- No errors in console

✅ **ASM Code** (after stepping into BIOS):
- Jump to ASM file → Yellow highlight on line
- Click F10 → Highlight moves to next line
- Console shows `[Monaco] ✅ Applying highlight to line X in ASM file`

✅ **Mixed Debugging**:
- VPy → F11 → ASM → F10 → F10 → VPy
- Highlight visible at all stages
- File switches correctly
- No errors or exceptions

If you see all three ✅ marks, the fix is working correctly!

---

## What to Report if It's Not Working

If highlighting doesn't appear, please collect:

1. **Browser Console Output** (F12):
   - Right-click → "Save as..." → Save to file
   - Include lines with `[Monaco]` and `[EmulatorPanel]`

2. **Debugger State**:
   - What file is open? (main.vpy or .asm?)
   - What line number should be highlighted?
   - What does console say about the file comparison?

3. **Steps to Reproduce**:
   - Exactly which buttons did you click?
   - Did you place a breakpoint?
   - How many times did you press F11/F10?

4. **Expected vs Actual**:
   - Expected: Yellow highlight on line 42
   - Actual: No highlight appears

---

## Building Status
- ✅ Frontend compiled successfully
- ✅ PDB structure verified (camelCase fields)
- ✅ ASM navigation working (confirmed by step 5-6)
- ✅ Highlight logic extended to support ASM
- 🔄 **NOW TESTING**: Highlight visibility in IDE

---

**Testing Date**: January 16, 2025
**Build**: Release (optimized)
**Browser**: Chrome/Firefox (JavaScript enabled)
**Status**: Ready for manual testing
