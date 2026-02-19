# ROM Storage Architecture - Clarification

## The Issue You Spotted ✅

**Your question**: "vecx.cart contains the ENTIRE ROM? Why do you use this.cart for single-bank OR this.multibankRom for multibank?"

**Answer**: CORRECT - It's two different storage paths based on ROM size.

## Memory Layout in JSVecX

### Constructor (vecx.js line ~12-14)
```javascript
this.rom = new Array(0x2000);          // 8KB BIOS ROM (0xE000-0xFFFF)
this.cart = new Array(0x8000);         // 32KB max single-bank cartridge (0x0000-0x7FFF)
this.multibankRom = null;              // FULL ROM data (512KB+ for multibank)
```

**Key Point**: `this.cart` is ONLY 32KB - can't hold multibank data!

## Two Different Paths

### SINGLE-BANK Cartridge (≤32KB)
```javascript
// In vecx_reset():
if (len <= 32768) {  // Single-bank
    this.isMultibank = false;
    for (var i = 0; i < len; i++) {
        this.cart[i] = Globals.cartdata.charCodeAt(i);  // ← Stored in this.cart
    }
}

// In read8():
// Default fallback reads from single-bank
return this.cart[address] & 0xff;  // ← Read from this.cart
```

**Result**: ROM data lives in `this.cart` (32KB array)

### MULTIBANK Cartridge (>32KB, up to 512KB)
```javascript
// In vecx_reset():
if (len > 32768) {  // Multibank
    this.isMultibank = true;
    this.multibankRom = new Uint8Array(len);  // Create LARGE array
    for (var i = 0; i < len; i++) {
        this.multibankRom[i] = Globals.cartdata.charCodeAt(i);  // ← Stored in this.multibankRom
    }
    // NOTE: this.cart is NOT filled (stays as 0x01 initialization)
}

// In read8():
if (this.isMultibank) {
    // Address mapping for multibank
    if (address < 0x4000) {
        const bankOffset = this.currentBank * 0x4000;
        return this.multibankRom[bankOffset + address] & 0xff;  // ← Read from this.multibankRom
    }
}
```

**Result**: ROM data lives in `this.multibankRom` (512KB Uint8Array)

## The Architecture

```
Globals.cartdata (file loaded)
    ↓
    Size > 32KB?
    ├─ YES (Multibank):
    │  └─ Create this.multibankRom = new Uint8Array(len)
    │     └─ Copy to this.multibankRom[0...512KB]
    │     └─ read8() reads from this.multibankRom
    │
    └─ NO (Single-bank):
       └─ Copy to this.cart[0...32KB]
       └─ read8() reads from this.cart (fallback)
```

## Why This Design?

1. **Backward compatibility**: Old single-bank code still works (uses this.cart)
2. **Memory efficiency**: Don't allocate huge arrays for small ROMs
3. **Clear separation**: Multibank path doesn't touch this.cart

## The Bug I Fixed ❌→✅

**BEFORE** (WRONG):
```typescript
const currentBankData = new Uint8Array(0x4000);
for (let i = 0; i < 0x4000; i++) {
    currentBankData[i] = vecx.cart[currentBankOffset + i] || 0;  // ❌ WRONG!
    // ↑ In multibank, vecx.cart is empty (never filled)
}
```

**AFTER** (CORRECT):
```typescript
if (vecx.isMultibank && vecx.multibankRom) {
    // ✅ Read from multibankRom
    const currentBankOffset = currentBankId * 0x4000;
    currentBankData = vecx.multibankRom.slice(currentBankOffset, currentBankOffset + 0x4000);
} else {
    // ✅ Read from cart (single-bank)
    currentBankData = new Uint8Array(vecx.cart.slice(0, 0x4000));
}
```

## Summary Table

| Property | Single-Bank | Multibank |
|----------|------------|-----------|
| **this.cart** | ✅ Filled with ROM | ❌ Empty (0x01) |
| **this.multibankRom** | ❌ null | ✅ Filled with ROM |
| **this.isMultibank** | false | true |
| **Max size** | 32KB | 512KB |
| **Read from** | this.cart | this.multibankRom |
| **Address mapping** | Direct | Bank-switched ($0000-$3FFF) + Fixed Bank #31 ($4000-$7FFF) |

## Verification Checklist

When debugging ROM loading:

- [ ] Check `Globals.cartdata.length`
  - If ≤ 32768: Single-bank path
  - If > 32768: Multibank path

- [ ] Check `vecx.isMultibank`
  - false = read from this.cart
  - true = read from this.multibankRom

- [ ] Check which array has data
  - Single-bank: `vecx.cart[0]` != 0x01
  - Multibank: `vecx.multibankRom[0]` != undefined

## Important: Cart Array State

**Single-bank cartridge loaded**:
```
vecx.cart[0]     = 0x00 (or code byte)
vecx.multibankRom = null
```

**Multibank cartridge loaded**:
```
vecx.cart[0]       = 0x01 (NEVER filled)
vecx.multibankRom[0] = 0x00 (or code byte)
```

---

This is why the snapshot function needs to check `vecx.isMultibank` and read from the correct source!
