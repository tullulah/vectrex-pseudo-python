# DEBUG: SHOW_LEVEL Ghost Vectors Investigation

## Status: FIXES APPLIED, TESTING PENDING

### Fixes Implemented (Commit 2d7b21d0)

#### BUG #1: DECB+BMI Loop Off-by-One ✅ FIXED
**Problem**: 
- Loop executed 1 extra iteration
- Example: count=3 → executed 4 times → read garbage as 4th object

**Root Cause**:
```asm
; BROKEN (old):
SLR_OBJ_LOOP:
    DECB             ; Decrement FIRST
    BMI SLR_OBJ_DONE ; Check negative (only catches $FF)
    ; B=0 → DECB → B=$FF → exits (but already drew garbage!)
```

**Fix Applied**:
```asm
; FIXED (new):
SLR_OBJ_LOOP:
    TSTB             ; Test if zero FIRST
    BEQ SLR_OBJ_DONE ; Exit immediately if zero
    DECB             ; Then decrement
    ; B=0 → exits immediately (no garbage draw)
```

#### BUG #2: Count Corruption (CLRB Missing) ✅ FIXED
**Problem**:
- LEVEL_GP_COUNT read as 769 (0x0301) instead of 3
- High byte had garbage value

**Root Cause**:
```asm
; BROKEN (old):
LDA ,X+          ; A = gameplayCount (but B had garbage)
STA >LEVEL_GP_COUNT ; Stores full 16-bit D register!
```

**Fix Applied**:
```asm
; FIXED (new):
CLRB             ; Clear B register FIRST
LDA ,X+          ; A = gameplayCount (B=0 guaranteed)
STA >LEVEL_GP_COUNT ; Now stores clean 8-bit value
```

---

## New MCP Tools for Verification

### 1. debugger/get_registers
Read all CPU registers in real-time:
```javascript
// Usage from MCP client:
{
  "method": "debugger/get_registers",
  "params": {}
}

// Returns:
{
  "A": { "value": 42, "hex": "0x2A", "decimal": 42 },
  "B": { "value": 0, "hex": "0x00", "decimal": 0 },
  "D": { "value": 42, "hex": "0x002A", "decimal": 42 },
  "X": { "value": 50000, "hex": "0xC350", "decimal": 50000 },
  "Y": { "value": 0, "hex": "0x0000", "decimal": 0 },
  "U": { "value": 0, "hex": "0x0000", "decimal": 0 },
  "S": { "value": 52223, "hex": "0xCBFF", "decimal": 52223 },
  "PC": { "value": 49152, "hex": "0xC000", "decimal": 49152 },
  "DP": { "value": 200, "hex": "0xC8", "decimal": 200 },
  "CC": {
    "value": 4,
    "hex": "0x04",
    "decimal": 4,
    "flags": {
      "C": 0,  // Carry
      "V": 0,  // Overflow
      "Z": 1,  // Zero
      "N": 0,  // Negative
      "I": 0,  // IRQ mask
      "H": 0,  // Half-carry
      "F": 0,  // FIRQ mask
      "E": 0   // Entire flag
    }
  }
}
```

### 2. memory/dump
Hex dump of memory region:
```javascript
// Usage:
{
  "method": "memory/dump",
  "params": {
    "address": 51344,  // 0xC890 (LEVEL_BG_COUNT area)
    "size": 256
  }
}

// Returns:
{
  "address": 51344,
  "size": 256,
  "bytes": [0, 3, 1, ...],
  "dump": "0xC890: 00 03 01 00 00 00 00 00 00 00 00 00 00 00 00 00 | ............\n..."
}
```

### 3. memory/list_variables
List all variables from PDB:
```javascript
// Usage:
{
  "method": "memory/list_variables",
  "params": {}
}

// Returns:
{
  "count": 42,
  "variables": [
    {
      "name": "LEVEL_PTR",
      "address": 51200,
      "addressHex": "0xC800",
      "size": 2,
      "type": "pointer"
    },
    {
      "name": "LEVEL_GP_COUNT",
      "address": 51348,
      "addressHex": "0xC894",
      "size": 1,
      "type": "byte"
    },
    ...
  ]
}
```

### 4. memory/read_variable
Read specific variable value:
```javascript
// Usage:
{
  "method": "memory/read_variable",
  "params": {
    "name": "LEVEL_GP_COUNT"
  }
}

// Returns:
{
  "name": "LEVEL_GP_COUNT",
  "address": 51348,
  "addressHex": "0xC894",
  "size": 1,
  "value": 3,
  "valueHex": "0x03",
  "valueDec": 3,
  "valueBin": "0b00000011"
}
```

---

## Verification Plan

### Step 1: Compile and Load
1. Rebuild vectrexc with fixes: `cargo build --release`
2. Open IDE and load level_test project
3. Build and run level_test in emulator
4. Pause execution (Shift+F5)

### Step 2: Inspect Counts (Should Be 3, Not 769)
```javascript
// Read LEVEL_GP_COUNT
memory/read_variable({ "name": "LEVEL_GP_COUNT" })
// Expected: value=3, hex="0x03"
// Old bug: value=769, hex="0x0301"
```

### Step 3: Inspect Level Data Structure
```javascript
// Dump LEVEL_PTR area
memory/dump({ "address": 0xC800, "size": 128 })
// Should show:
// +0: LEVEL_PTR (2 bytes) → points to .vplay data
// +12: counts (3 bytes): BG=0, GP=3, FG=1
// +15: pointers (6 bytes): bg_ptr, gp_ptr, fg_ptr
```

### Step 4: Set Breakpoint at SLR_OBJ_LOOP
1. Find SLR_OBJ_LOOP address in level_test.asm
2. Set breakpoint: `debugger/add_breakpoint({ "line": X })`
3. Continue execution (F12)
4. When hit, inspect B register:
   - 1st iteration: B=3 → TSTB detects 3, DECB → B=2
   - 2nd iteration: B=2 → TSTB detects 2, DECB → B=1
   - 3rd iteration: B=1 → TSTB detects 1, DECB → B=0
   - 4th iteration: B=0 → TSTB detects 0, BEQ exits ✅

### Step 5: Verify Vector Count
- Expected: 4 vectors drawn (3 GP + 1 FG)
- Old bug: 13 vectors (4 real + 9 ghosts)
- After fix: Should see exactly 4 vectors

---

## Expected Test Results

### ✅ SUCCESS Criteria:
1. LEVEL_GP_COUNT = 3 (not 769)
2. Loop executes exactly 3 iterations for GP layer
3. Loop executes exactly 1 iteration for FG layer
4. Total vectors rendered: 4 (no ghosts)
5. No diagonal sawtooth pattern
6. No disappearing real vectors

### ❌ FAILURE Indicators:
1. LEVEL_GP_COUNT still reads 769
2. Loop executes 4+ iterations
3. More than 4 vectors visible
4. Ghost vectors appear
5. Memory corruption visible in dumps

---

## Investigation Tools (If Bugs Persist)

### Check X Register During Loop
```javascript
// At each loop iteration:
debugger/get_registers()
// Verify X advances by 20 bytes per iteration:
// X = gp_ptr + 0   (1st object)
// X = gp_ptr + 20  (2nd object)
// X = gp_ptr + 40  (3rd object)
// X = gp_ptr + 60  (should NOT reach here)
```

### Dump Object Data
```javascript
// Dump 3 gameplay objects (3 * 20 = 60 bytes)
memory/dump({ "address": gp_ptr, "size": 60 })
// Verify structure:
// +0: type, +1-2: x, +3-4: y, +8: intensity, +16-17: vector_ptr
```

### Verify Vector Pointers
```javascript
// Each object has vector_ptr at offset +16
// Read all 3 vector pointers:
memory/dump({ "address": gp_ptr + 16, "size": 2 })
memory/dump({ "address": gp_ptr + 36, "size": 2 })
memory/dump({ "address": gp_ptr + 56, "size": 2 })
// Should point to: COIN_VECTORS, BUBBLE_HUGE_VECTORS, BUBBLE_LARGE_VECTORS
```

---

## Next Steps for User

1. **Restart IDE** to pick up new vectrexc binary
2. **Compile level_test** (Build → Build or Ctrl+F7)
3. **Run in emulator** (Build → Run or Ctrl+F5)
4. **Report results**:
   - How many vectors appear?
   - Do ghosts still appear?
   - Does LEVEL_GP_COUNT read correctly?
   - Do vectors disappear over time?

If bugs persist, use new MCP tools to inspect state and report findings.

---

## Technical Notes

### Why CLRB Is Critical
- M6809 `STA` stores only accumulator A (8-bit)
- But RAM addresses are 16-bit, so STA uses full D register (A:B)
- If B has garbage (e.g., B=0x03 from previous operation):
  - LDA loads A=3 (correct)
  - STA writes D=0x0303 (incorrect! high byte corrupts memory)
- CLRB ensures B=0 before LDA, so D=0x0003 (correct)

### Why TSTB+BEQ Is Critical
- DECB decrements BEFORE testing
- BMI only catches B=$FF (negative), NOT B=0
- TSTB tests B BEFORE decrementing
- BEQ catches B=0 immediately, preventing extra iteration

### Object Structure (20 bytes)
```
+0:  FCB type
+1:  FDB x (position)
+3:  FDB y (position)
+5:  FDB scale (8.8 fixed point)
+7:  FCB rotation
+8:  FCB intensity
+9:  FCB velocity_x
+10: FCB velocity_y
+11: FCB physics_flags
+12: FCB collision_flags
+13: FCB collision_size
+14: FDB spawn_delay
+16: FDB vector_ptr  ← Points to vector data
+18: FDB properties_ptr
```

---

Last Updated: 2026-01-07
Status: FIXES APPLIED, AWAITING USER TEST
