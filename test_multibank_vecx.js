// Quick harness to verify JSVecx multi-bank mapping and offsets.
// Run with: node test_multibank_vecx.js

const fs = require('fs');
const path = require('path');
const vm = require('vm');

// Minimal stubs so vecx.js can run in Node.
const Globals = {
  ALG_MAX_X: 256,
  ALG_MAX_Y: 256,
  VECTOR_CNT: 4,
  VECTOR_HASH: 16,
  VECTREX_COLORS: 0,
  FCYCLES_INIT: 0,
  romdata: Buffer.alloc(0x2000, 0).toString('latin1'),
  cartdata: null,
};

const utils = {
  initArray(arr, value) {
    for (let i = 0; i < arr.length; i += 1) arr[i] = value;
  },
};

function osint() {
  this.osint_clearscreen = () => {};
  this.osint_render = () => {};
}
function e6809() {
  this.e6809_reset = () => {};
  this.e6809_sstep = () => {};
}
function e8910() {
  this.init = () => {};
  this.e8910_write = () => {};
  this.e8910_reset = () => {};
  this.start = () => {};
}
function vector_t() { this.reset = () => {}; }

// Load vecx.js in an isolated context with our stubs.
const windowStub = {
  addEventListener() {},
  removeEventListener() {},
  postMessage() {},
};

const dollarStub = () => ({ text() {}, append() {}, empty() {} });

const context = {
  console,
  Globals,
  utils,
  osint,
  e6809,
  e8910,
  vector_t,
  Uint8Array,
  ArrayBuffer,
  DataView,
  setTimeout,
  clearTimeout,
  setInterval,
  clearInterval,
  $: dollarStub,
  window: windowStub,
  self: windowStub,
};
const vecxSrc = fs.readFileSync(path.join(__dirname, 'ide/frontend/public/jsvecx_deploy/vecx.js'), 'utf8');
vm.runInNewContext(vecxSrc, context);

// Build a 32-bank (512KB) ROM like test_callgraph (32 * 16KB).
const bankSize = 0x4000;
const numBanks = 32;
const romBytes = new Uint8Array(bankSize * numBanks);
for (let bank = 0; bank < numBanks; bank += 1) {
  for (let off = 0; off < bankSize; off += 1) {
    // Pattern encodes bank and offset so we can detect bad offsets.
    romBytes[bank * bankSize + off] = ((bank << 4) ^ (off & 0xff)) & 0xff;
  }
}
Globals.cartdata = Buffer.from(romBytes).toString('latin1');

const vecx = new context.VecX();
vecx.reset();

function expected(bank, offset) {
  return ((bank << 4) ^ (offset & 0xff)) & 0xff;
}

function assertByte(addr, expectedValue, label, failures) {
  const got = vecx.read8(addr) & 0xff;
  if (got !== expectedValue) {
    failures.push(`${label}: expected 0x${expectedValue.toString(16).padStart(2, '0')} got 0x${got.toString(16).padStart(2, '0')}`);
  }
}

const failures = [];

// Bank 0 (default) via register state after reset.
assertByte(0x0000, expected(0, 0x0000), 'bank0 base', failures);
assertByte(0x0123, expected(0, 0x0123), 'bank0 offset 0x0123', failures);
assertByte(0x3ffe, expected(0, 0x3ffe), 'bank0 offset 0x3ffe', failures);

// Switch to bank 1 via bank register.
vecx.write8(vecx.bankRegister, 0x01);
assertByte(0x0000, expected(1, 0x0000), 'bank1 base (via register)', failures);
assertByte(0x0123, expected(1, 0x0123), 'bank1 offset 0x0123 (via register)', failures);
assertByte(0x3ffe, expected(1, 0x3ffe), 'bank1 offset 0x3ffe (via register)', failures);

// Switch to bank 5 via bank register.
vecx.write8(vecx.bankRegister, 0x05);
assertByte(0x0000, expected(5, 0x0000), 'bank5 base (via register)', failures);
assertByte(0x0200, expected(5, 0x0200), 'bank5 offset 0x0200 (via register)', failures);
assertByte(0x3abc, expected(5, 0x3abc), 'bank5 offset 0x3abc (via register)', failures);

// Switch to last bank (31) via bank register and test window + fixed region.
vecx.write8(vecx.bankRegister, 0x1f);
assertByte(0x0000, expected(31, 0x0000), 'bank31 base (via register)', failures);
assertByte(0x0101, expected(31, 0x0101), 'bank31 offset 0x0101 (via register)', failures);

// Fixed bank region should always read the last bank (31) regardless of currentBank.
assertByte(0x4000, expected(31, 0x0000), 'fixed bank start', failures);
assertByte(0x7abc, expected(31, 0x3abc), 'fixed bank offset 0x3abc', failures);

// Masking: write an out-of-range bank number (e.g., 0x40) and ensure it wraps to maxBankId (31) via bitmask.
vecx.write8(vecx.bankRegister, 0x40);
assertByte(0x0001, expected(0, 0x0001), 'masked bank wrap to 0 (via register)', failures);
assertByte(0x4000, expected(31, 0x0000), 'fixed bank after wrap (should stay last bank)', failures);

if (failures.length) {
  console.error('Multi-bank mapping test: FAIL');
  failures.forEach((f) => console.error(' - ' + f));
  process.exit(1);
} else {
  console.log('Multi-bank mapping test: PASS');
  // Avoid hanging due to VecX timers (emuloop setInterval, start() setTimeout).
  process.exit(0);
}
