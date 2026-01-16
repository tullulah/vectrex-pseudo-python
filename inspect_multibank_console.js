/**
 * Browser Console Script - Run in Emulator DevTools Console
 * 
 * Purpose: Inspect multibank cartridge loading and memory state
 * 
 * Usage (paste into browser console):
 *   1. copy(document.querySelector('pre#debugging').textContent || '') to get debug output
 *   2. Run the inspection functions below
 * 
 * Expected output: Confirm that Globals.cartdata is properly sized and vecx.cart is initialized
 */

// ============================================================================
// 1. Check if cartdata is fully loaded
// ============================================================================
console.log('%c=== MULTIBANK CARTRIDGE LOADING CHECK ===', 'font-size:14px; font-weight:bold; color:#0f0');

// Check Globals.cartdata
const Globals = window.Globals || globalThis.Globals;
const cartdataSize = Globals?.cartdata?.length || 0;
const expectedSize = 524288; // 512KB multibank

console.log(`Globals.cartdata size: ${cartdataSize} bytes (${(cartdataSize/1024).toFixed(1)}KB)`);
if (cartdataSize === expectedSize) {
  console.log('%c✅ CORRECT: Cartdata is 512KB (multibank)', 'color:#0f0; font-weight:bold');
} else if (cartdataSize === 32768) {
  console.log('%c⚠️  WRONG: Cartdata is 32KB (single bank)', 'color:#ff0; font-weight:bold');
} else {
  console.log(`%c❌ ERROR: Cartdata is ${cartdataSize} bytes (unexpected)`, 'color:#f00; font-weight:bold');
}

// ============================================================================
// 2. Check if cart array is initialized in JSVecx
// ============================================================================
console.log('%c=== EMULATOR CART ARRAY CHECK ===', 'font-size:14px; font-weight:bold; color:#0f0');

const vecx = window.vecx;
const cartSize = vecx?.cart?.length || 0;
const cartCapacity = 0x400000; // 4MB max

console.log(`vecx.cart array size: ${cartSize} bytes (${(cartSize/1024).toFixed(1)}KB)`);
console.log(`vecx.current_bank: ${vecx?.current_bank || 0}`);

if (cartSize >= expectedSize) {
  console.log(`%c✅ CORRECT: Cart array is >= ${expectedSize} bytes`, 'color:#0f0; font-weight:bold');
} else {
  console.log(`%c❌ ERROR: Cart array is ${cartSize} bytes (too small)`, 'color:#f00; font-weight:bold');
}

// ============================================================================
// 3. Check Bank #0 contents via read8() (current bank)
// ============================================================================
console.log('%c=== BANK #0 CONTENTS CHECK (via read8) ===', 'font-size:14px; font-weight:bold; color:#0f0');

const bank0Sample = [];

// Use vecx.read8() which handles all complexity automatically
for (let addr = 0x0000; addr < 0x0040; addr++) {
  bank0Sample.push(vecx.read8(addr));
}

console.log(`Reading from addresses 0x0000-0x003F via read8() (tests CPU address space)`);

// Format as hex
let hexStr = '';
for (let i = 0; i < bank0Sample.length; i++) {
  hexStr += bank0Sample[i].toString(16).padStart(2, '0').toUpperCase();
  if ((i + 1) % 16 === 0) hexStr += '\n';
  else hexStr += ' ';
}

console.log('Bank #0 first 64 bytes (hex):');
console.log(hexStr);

// Check for GCE header or common opcodes
if (bank0Sample[0] === 0x00 || bank0Sample[0] === 0x0C || bank0Sample[0] === 0x86) {
  console.log('%c✅ Bank #0 starts with code (not garbage)', 'color:#0f0; font-weight:bold');
} else {
  console.log(`%c⚠️  Bank #0 starts with 0x${bank0Sample[0].toString(16).toUpperCase().padStart(2,'0')} (verify if expected)`, 'color:#ff0');
}

// ============================================================================
// 4. Check Bank #31 contents via read8() (fixed bank)
// ============================================================================
console.log('%c=== BANK #31 CONTENTS CHECK (via read8) ===', 'font-size:14px; font-weight:bold; color:#0f0');

const bank31Sample = [];

// Use vecx.read8() with addresses 0x4000-0x403F (fixed bank #31 window)
for (let addr = 0x4000; addr < 0x4040; addr++) {
  bank31Sample.push(vecx.read8(addr));
}

console.log('Reading from addresses 0x4000-0x403F via read8() (tests fixed bank #31)');

// Format as hex
let hex31Str = '';
for (let i = 0; i < bank31Sample.length; i++) {
  hex31Str += bank31Sample[i].toString(16).padStart(2, '0').toUpperCase();
  if ((i + 1) % 16 === 0) hex31Str += '\n';
  else hex31Str += ' ';
}

console.log('Bank #31 first 64 bytes (hex):');
console.log(hex31Str);

// Check for CUSTOM_RESET pattern (LDA #0 = 86 00)
if (bank31Sample[0] === 0x86 && bank31Sample[1] === 0x00) {
  console.log('%c✅ Bank #31 starts with "LDA #0" (CUSTOM_RESET)', 'color:#0f0; font-weight:bold');
} else {
  console.log(`%c⚠️  Bank #31 starts with 0x${bank31Sample[0].toString(16).toUpperCase().padStart(2,'0')} 0x${bank31Sample[1].toString(16).toUpperCase().padStart(2,'0')} (expected 86 00)`, 'color:#ff0');
}

// ============================================================================
// 5. CPU Register State (if paused in debugger)
// ============================================================================
console.log('%c=== CPU STATE ===', 'font-size:14px; font-weight:bold; color:#0f0');

if (vecx?.regs) {
  console.log(`PC: 0x${vecx.regs.PC.toString(16).toUpperCase().padStart(4, '0')}`);
  console.log(`A: 0x${vecx.regs.A.toString(16).toUpperCase().padStart(2, '0')}`);
  console.log(`B: 0x${vecx.regs.B.toString(16).toUpperCase().padStart(2, '0')}`);
  console.log(`X: 0x${vecx.regs.X.toString(16).toUpperCase().padStart(4, '0')}`);
  console.log(`Y: 0x${vecx.regs.Y.toString(16).toUpperCase().padStart(4, '0')}`);
  console.log(`DP: 0x${vecx.regs.DP.toString(16).toUpperCase().padStart(2, '0')}`);
} else {
  console.log('Registers not available');
}

// ============================================================================
// 6. Download raw diagnostic data
// ============================================================================
console.log('%c=== DIAGNOSTIC EXPORT ===', 'font-size:14px; font-weight:bold; color:#0f0');

const diagnosticData = {
  timestamp: new Date().toISOString(),
  cartdataSize,
  cartSize,
  currentBank: vecx?.current_bank || 0,
  bank0FirstBytes: bank0Sample.map(b => `0x${b.toString(16).padStart(2,'0').toUpperCase()}`).join(' '),
  bank31FirstBytes: bank31Sample.map(b => `0x${b.toString(16).padStart(2,'0').toUpperCase()}`).join(' '),
  cpuPC: vecx?.regs?.PC || null,
  cpuA: vecx?.regs?.A || null,
  cpuDP: vecx?.regs?.DP || null,
  status: vecx?.debugState || 'unknown',
};

console.log('Diagnostic data:', diagnosticData);
console.table(diagnosticData);

// Export as JSON for inspection
const jsonExport = JSON.stringify(diagnosticData, null, 2);
console.log('%cTo copy diagnostic JSON to clipboard, run:', 'font-weight:bold');
console.log("copy(JSON.stringify(" + JSON.stringify(diagnosticData) + ", null, 2))");
