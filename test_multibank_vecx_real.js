// Harness: load real BIOS + real binary (test_callgraph.bin) and verify multi-bank mapping.
// Run with: node test_multibank_vecx_real.js

const fs = require('fs');
const path = require('path');
const vm = require('vm');

// --- Load real assets ---
const biosPath = path.join(__dirname, 'ide/frontend/dist/bios.bin');
const binPath = path.join(__dirname, 'examples/test_callgraph/build/test_callgraph.bin');
const pdbPath = path.join(__dirname, 'examples/test_callgraph/build/test_callgraph.pdb');

const biosData = fs.readFileSync(biosPath);
const binData = fs.readFileSync(binPath);
const pdbData = JSON.parse(fs.readFileSync(pdbPath, 'utf8'));

// --- Minimal stubs (only what VecX touches before start()) ---
const Globals = {
  ALG_MAX_X: 256,
  ALG_MAX_Y: 256,
  VECTOR_CNT: 4,
  VECTOR_HASH: 16,
  VECTREX_COLORS: 0,
  FCYCLES_INIT: 0,
  romdata: biosData.toString('latin1'),
  cartdata: binData.toString('latin1'),
};

const utils = {
  initArray(arr, value) {
    for (let i = 0; i < arr.length; i += 1) arr[i] = value;
  },
  // JSVecx calls this on errors; stub to log instead of throwing to keep stepping.
  showError(...msg) {
    console.error('[JSVecx Error]', ...msg);
  },
};

function osint() {
  this.osint_clearscreen = () => {};
  this.osint_render = () => {};
}
// e6809 will be provided by the real JS core (loaded below).
function e8910() {
  this.init = () => {};
  this.e8910_write = () => {};
  this.e8910_reset = () => {};
}
function vector_t() { this.reset = () => {}; }
function fptr(v) { this.v = v; }
function iptr(v) { this.v = v; }

const windowStub = {
  addEventListener() {},
  removeEventListener() {},
  postMessage() {},
};

const context = {
  console,
  Globals,
  utils,
  osint,
  e8910,
  vector_t,
  fptr,
  iptr,
  Uint8Array,
  ArrayBuffer,
  DataView,
  setTimeout,
  clearTimeout,
  setInterval,
  clearInterval,
  window: windowStub,
  self: windowStub,
};

// --- Load vecx.js into VM context ---
const vecxSrc = fs.readFileSync(path.join(__dirname, 'ide/frontend/public/jsvecx_deploy/vecx.js'), 'utf8');
const e6809Src = fs.readFileSync(path.join(__dirname, 'ide/frontend/public/jsvecx_deploy/e6809.js'), 'utf8');
vm.runInNewContext(e6809Src, context);
vm.runInNewContext(vecxSrc, context);

// --- Instantiate emulator with real images ---
const vecx = new context.VecX();
// Manually init CPU context and reset without starting timers.
vecx.e6809.init(vecx);
vecx.vecx_reset();
vecx.debugState = 'running';
vecx.running = true;

const bankSize = 0x4000;
const numBanks = Math.floor(binData.length / bankSize);
const lastBank = numBanks - 1;

function expected(bank, offset) {
  const globalOff = bank * bankSize + offset;
  return binData[globalOff] & 0xff;
}

function assertByte(addr, expectedValue, label, failures) {
  const got = vecx.read8(addr) & 0xff;
  if (got !== expectedValue) {
    failures.push(`${label}: expected 0x${expectedValue.toString(16).padStart(2, '0')} got 0x${got.toString(16).padStart(2, '0')}`);
  }
}

const failures = [];
// Collect wrapper addresses from PDB (symbols ending with _BANK_WRAPPER).
const wrapperAddrs = new Set();
let updatePlayerAddr = null;
for (const [name, addrHex] of Object.entries(pdbData.symbols || {})) {
  if (name.endsWith('_BANK_WRAPPER')) {
    const addr = parseInt(addrHex, 16) & 0xffff;
    wrapperAddrs.add(addr);
    if (name === 'update_player_BANK_WRAPPER') {
      updatePlayerAddr = addr;
    }
  }
}

// Bank 0 after reset.
assertByte(0x0000, expected(0, 0x0000), 'bank0 base', failures);
assertByte(0x0123, expected(0, 0x0123), 'bank0 off 0x0123', failures);
assertByte(0x3ffe, expected(0, 0x3ffe), 'bank0 off 0x3ffe', failures);

// Bank 1 via register.
vecx.write8(vecx.bankRegister, 0x01);
assertByte(0x0000, expected(1, 0x0000), 'bank1 base', failures);
assertByte(0x0200, expected(1, 0x0200), 'bank1 off 0x0200', failures);
assertByte(0x3abc, expected(1, 0x3abc), 'bank1 off 0x3abc', failures);

// Bank 5 via register.
vecx.write8(vecx.bankRegister, 0x05);
assertByte(0x0001, expected(5, 0x0001), 'bank5 base+1', failures);
assertByte(0x0101, expected(5, 0x0101), 'bank5 off 0x0101', failures);
assertByte(0x3fff, expected(5, 0x3fff), 'bank5 off 0x3fff', failures);

// Last bank via register.
vecx.write8(vecx.bankRegister, lastBank);
assertByte(0x0002, expected(lastBank, 0x0002), 'last bank base+2', failures);
assertByte(0x0202, expected(lastBank, 0x0202), 'last bank off 0x0202', failures);

// Fixed window always last bank.
assertByte(0x4000, expected(lastBank, 0x0000), 'fixed start', failures);
assertByte(0x7abc, expected(lastBank, 0x3abc), 'fixed off 0x3abc', failures);

if (failures.length) {
  console.error('Multi-bank mapping (real ROM): FAIL');
  failures.forEach((f) => console.error(' - ' + f));
  process.exit(1);
} else {
  console.log('Multi-bank mapping (real ROM): PASS');
  // Integration: run CPU until update_player_BANK_WRAPPER or stop, with trace around the hit.
  const target = updatePlayerAddr;
  const maxSteps = 800000;
  const trace = [];
  let reason = 'max-steps';

  for (let i = 0; i < maxSteps; i++) {
    const pcBefore = vecx.e6809.reg_pc & 0xffff;
    const opBefore = vecx.read8(pcBefore) & 0xff;
    const bankBefore = vecx.currentBank | 0;

    vecx.e6809.e6809_sstep();

    trace.push({ step: i + 1, pc: pcBefore, op: opBefore, bank: bankBefore });
    if (trace.length > 64) trace.shift();

    if (pcBefore === target) {
      reason = 'hit-target';
      console.log(`Hit update_player_BANK_WRAPPER at PC=0x${pcBefore.toString(16).toUpperCase().padStart(4, '0')} after ${i + 1} steps`);
      break;
    }

    const pcAfter = vecx.e6809.reg_pc & 0xffff;
    if (vecx.debugState === 'stopped') {
      reason = 'stopped';
      console.log(`Execution stopped by emulator at PC=0x${pcAfter.toString(16).toUpperCase().padStart(4, '0')} after ${i + 1} steps`);
      break;
    }
  }

  // Dump last 32 entries of trace for inspection.
  const tail = trace.slice(-32);
  console.log('--- Last 32 steps before stop/target ---');
  tail.forEach((t) => {
    console.log(`step ${t.step.toString().padStart(6)} | PC=0x${t.pc.toString(16).toUpperCase().padStart(4, '0')} | OP=0x${t.op.toString(16).toUpperCase().padStart(2, '0')} | BANK=${t.bank}`);
  });

  if (reason === 'max-steps') {
    console.log(`Wrapper not reached within ${maxSteps} steps.`);
  }
  process.exit(0);
}
