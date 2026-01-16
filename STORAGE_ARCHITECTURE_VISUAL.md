# Storage Architecture - Visual Comparison

## At a Glance

```
                SINGLE-BANK              vs              MULTIBANK
                (≤32KB)                                   (>32KB)
═════════════════════════════════════════════════════════════════════════

Globals.cartdata:
  32KB ROM          524288 bytes (512KB)
      ↓                    ↓
   Copied to           Copied to
      ↓                    ↓
 this.cart[]          this.multibankRom[]
 (32KB Array)         (512KB Uint8Array)
      ↓                    ↓
  read8() reads:      read8() reads:
   from cart          from multibankRom
                      (with bank switching)
```

## Side-by-Side Comparison Table

| Aspect | Single-Bank | Multibank |
|--------|-------------|-----------|
| **ROM Size** | ≤32KB | >32KB (up to 512KB) |
| **Storage Container** | `vecx.cart` (Array 0x8000) | `vecx.multibankRom` (Uint8Array) |
| **this.cart State** | ✅ Filled with ROM data | ❌ Empty (0x01 init only) |
| **this.multibankRom** | ❌ null | ✅ Filled with entire ROM |
| **this.isMultibank** | false | true |
| **Address Range** | 0x0000-0x7FFF (32KB) | 0x0000-0x7FFF (16KB window) |
| **read8() Source** | `this.cart[address]` | `this.multibankRom[offset]` with bank calc |
| **Bank Switching** | ❌ No | ✅ Yes (0xDF00 register) |
| **Fixed Bank #31** | ❌ No | ✅ Always at 0x4000-0x7FFF |
| **Snapshot from** | `vecx.cart[0:0x4000]` | `vecx.multibankRom[bank*0x4000:(bank+1)*0x4000]` |
| **Example File** | `game.bin` (24KB) | `game.bin` (512KB) |

## Memory Initialization Flow

### Single-Bank Path
```
┌─ vecx_reset()
│
├─ Initialize cart array (0x8000 bytes)
│  └─ Set all to 0x01
│
├─ Check Globals.cartdata size
│  └─ if (len ≤ 32768) → SINGLE-BANK
│
├─ Loop through cartdata
│  └─ this.cart[i] = Globals.cartdata.charCodeAt(i)
│
├─ Set isMultibank = false
│
└─ read8() → reads from this.cart (fallback)
```

**Result**: this.cart contains ROM data

### Multibank Path
```
┌─ vecx_reset()
│
├─ Initialize cart array (0x8000 bytes)
│  └─ Set all to 0x01
│  └─ ❌ NOT filled during multibank
│
├─ Check Globals.cartdata size
│  └─ if (len > 32768) → MULTIBANK
│
├─ Create multibankRom array
│  └─ this.multibankRom = new Uint8Array(len)
│
├─ Loop through cartdata
│  └─ this.multibankRom[i] = Globals.cartdata.charCodeAt(i)
│
├─ Set isMultibank = true
│  └─ currentBank = 0
│
└─ read8() → reads from this.multibankRom with bank mapping
```

**Result**: this.cart stays at 0x01, multibankRom has the actual ROM

## read8() Decision Tree

```
read8(address)
│
├─ if (this.isMultibank) {
│  │
│  ├─ if (address < 0x4000) {
│  │  └─ return multibankRom[(currentBank * 0x4000) + address]
│  │
│  └─ if (address >= 0x4000 && address < 0x8000) {
│     └─ return multibankRom[(31 * 0x4000) + (address - 0x4000)]
│
└─ else {
   └─ return this.cart[address]  ← Single-bank fallback
```

## Snapshot Function Fix

### BEFORE (❌ WRONG)
```javascript
// Always read from cart
const currentBankData = new Uint8Array(0x4000);
for (let i = 0; i < 0x4000; i++) {
    currentBankData[i] = vecx.cart[currentBankOffset + i] || 0;
    // ❌ In multibank, this is 0x01 (empty cart array)
}
```

**Result**: Downloaded file has 0x01 0x01 0x01... (garbage)

### AFTER (✅ CORRECT)
```javascript
if (vecx.isMultibank && vecx.multibankRom) {
    // Read from multibankRom
    const currentBankOffset = currentBankId * 0x4000;
    currentBankData = vecx.multibankRom.slice(
        currentBankOffset, 
        currentBankOffset + 0x4000
    );
} else {
    // Read from cart
    currentBankData = new Uint8Array(vecx.cart.slice(0, 0x4000));
}
```

**Result**: Downloaded file has actual ROM code

## Why This Design?

1. **Backward Compatibility**: Old code uses `this.cart`
2. **Memory Efficiency**: Don't allocate 512KB for tiny 8KB ROMs
3. **Clear Separation**: Multibank uses different container
4. **Simpler Code**: read8() has one fallback for single-bank

## Debugging Checklist

- [ ] Check `Globals.cartdata.length`
  - ≤32768 → Single-bank
  - >32768 → Multibank

- [ ] Check `vecx.isMultibank`
  - Should be `true` for multibank
  - Should be `false` for single-bank

- [ ] Check which array has data
  - Single-bank: `vecx.cart[0]` ≠ 0x01 ✅
  - Multibank: `vecx.multibankRom[0]` ≠ 0x01 ✅

- [ ] Snapshot function checks `isMultibank` ✅
  - Uses `multibankRom` for multibank
  - Uses `cart` for single-bank

## Key Takeaway

**Never assume which container has the ROM data!**

Always check:
```javascript
if (vecx.isMultibank && vecx.multibankRom) {
    // Read from multibankRom
} else {
    // Read from cart
}
```

---

Created: 2026-01-15  
Purpose: Visual explanation of storage architecture  
Status: Reference guide for debugging ROM issues
