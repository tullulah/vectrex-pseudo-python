# Snapshot Feature - Using read8() - FINAL VERSION

## The Elegant Solution ✨

Instead of reimplementing address mapping logic, **use `vecx.read8()` which already handles everything**:

### What read8() Handles Automatically
- ✅ Detects single-bank vs multibank
- ✅ Reads from correct container (cart or multibankRom)
- ✅ Applies bank switching (currentBank)
- ✅ Maps fixed bank #31 to 0x4000-0x7FFF
- ✅ Returns the correct byte

### Benefits
1. **Simpler code** - No manual address calculation
2. **Tests the right function** - We're testing what CPU sees, not raw containers
3. **Reuses proven logic** - `read8()` is already used by CPU emulation
4. **Easier to maintain** - Changes to `read8()` automatically affect snapshot

## Implementation

### onSnapshotROM() in EmulatorPanel.tsx

```typescript
const onSnapshotROM = () => {
  const vecx = (window as any).vecx;
  if (!vecx || status !== 'paused') {
    console.error('[EmulatorPanel] Snapshot failed - emulator not paused');
    return;
  }

  try {
    // Create 32KB snapshot using vecx.read8()
    const snapshot = new Uint8Array(0x8000);
    
    // Read current bank via CPU addresses 0x0000-0x3FFF
    for (let addr = 0x0000; addr < 0x4000; addr++) {
      snapshot[addr] = vecx.read8(addr);  // ← Handles everything!
    }
    
    // Read fixed bank #31 via CPU addresses 0x4000-0x7FFF
    for (let addr = 0x4000; addr < 0x8000; addr++) {
      snapshot[addr] = vecx.read8(addr);  // ← Handles everything!
    }

    // Download...
  }
};
```

### Console Script (inspect_multibank_console.js)

```javascript
// Read via read8() - what CPU actually sees
const bank0Sample = [];
for (let addr = 0x0000; addr < 0x0040; addr++) {
  bank0Sample.push(vecx.read8(addr));  // ← CPU address space
}

const bank31Sample = [];
for (let addr = 0x4000; addr < 0x4040; addr++) {
  bank31Sample.push(vecx.read8(addr));  // ← Fixed bank #31 window
}
```

## What This Tests

### Single-Bank
```
vecx.read8(0x0000)
  → reads from this.cart[0]
  → returns first byte of cartridge

vecx.read8(0x4000)  
  → reads from this.cart[0x4000]
  → returns byte at 0x4000 of 32KB cartridge
```

### Multibank
```
vecx.read8(0x0000)       [currentBank=0]
  → reads from multibankRom[(0 * 0x4000) + 0x0000]
  → returns first byte of Bank 0

vecx.read8(0x4000)       [always bank #31]
  → reads from multibankRom[(31 * 0x4000) + 0x0000]
  → returns first byte of Bank 31

vecx.read8(0x3FFF)       [currentBank=0]
  → reads from multibankRom[(0 * 0x4000) + 0x3FFF]
  → returns last byte of Bank 0

vecx.read8(0x7FFF)       [always bank #31]
  → reads from multibankRom[(31 * 0x4000) + 0x3FFF]
  → returns last byte of Bank 31
```

## Why This Approach is Better

| Aspect | Manual Container Access | Using read8() |
|--------|-------------------------|---------------|
| **Code Complexity** | High - handle multibank/single-bank | Low - one loop |
| **Tests** | Raw container logic | CPU address space (real) |
| **Maintenance** | Must update if read8() changes | Auto-inherits changes |
| **Risk** | Duplicate logic = bugs | Uses proven function |
| **Clarity** | "Read from multibankRom at offset..." | "Read CPU address 0x0000" |

## Verification Flow

```
1. Load multibank ROM (512KB)
   ↓
2. Snapshot calls read8(0x0000) to read8(0x7FFF)
   ↓
3. read8() determines:
   - Current bank from currentBank variable
   - Uses multibankRom array
   - Maps addresses correctly
   ↓
4. Returns 32KB of actual ROM data
   ↓
5. Download file = what CPU sees
   ↓
6. Analyze in Python
   ✅ If data looks good → read8() works!
   ❌ If data is garbage → read8() bug exposed!
```

## Testing Checklist

- [x] Snapshot function uses `vecx.read8()` ✅
- [x] Reads addresses 0x0000-0x3FFF (current bank) ✅
- [x] Reads addresses 0x4000-0x7FFF (fixed bank #31) ✅
- [x] Console script also uses `vecx.read8()` ✅
- [x] No manual container access ✅
- [x] No duplicate address mapping logic ✅

## Key Insight

**The snapshot isn't just about debugging the ROM.**

**It's about testing that `read8()` correctly implements:**
1. Single-bank addressing (0x0000-0x7FFF)
2. Multibank addressing (0x0000-0x3FFF with bank switch, 0x4000-0x7FFF fixed)
3. Container selection (cart vs multibankRom)

If the snapshot shows garbage → read8() has a bug, not the arrays!

---

Created: 2026-01-15  
Status: ✅ FINAL - Using read8() for elegance and correctness  
Credit: User suggestion to use proven function instead of reimplementing logic
