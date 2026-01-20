const fs = require('fs');
const path = require('path');
const vm = require('vm');

// Load BIOS
const biosPath = path.join(__dirname, 'ide/frontend/dist/bios.bin');
const biosData = fs.readFileSync(biosPath);

// Load pang_multi binary
const binPath = path.join(__dirname, 'examples/pang_multi/src/main.bin');
const binData = fs.readFileSync(binPath);

console.log('[SETUP] BIOS size:', biosData.length);
console.log('[SETUP] Binary size:', binData.length);

// Minimal stubs
const Globals = { ALG_MAX_X: 256, ALG_MAX_Y: 256, VECTOR_CNT: 4, VECTOR_HASH: 16, VECTREX_COLORS: 0, FCYCLES_INIT: 0, romdata: Buffer.alloc(0x2000, 0).toString('latin1'), cartdata: null };
const utils = { initArray(arr, value) { for (let i = 0; i < arr.length; i++) arr[i] = value; } };
function osint() { this.osint_clearscreen = () => {}; this.osint_render = () => {}; }
function e6809() { this.e6809_reset = () => {}; this.e6809_sstep = () => {}; }
function e8910() { this.init = () => {}; this.e8910_write = () => {}; this.e8910_reset = () => {}; this.start = () => {}; }
function vector_t() { this.reset = () => {}; }

// Load JSVecx in isolated context
const vecxCode = fs.readFileSync(path.join(__dirname, 'ide/frontend/public/jsvecx_deploy/vecx.js'), 'utf8');
const windowStub = { addEventListener() {}, removeEventListener() {}, postMessage() {} };
const sandbox = { Globals, utils, osint, e6809, e8910, vector_t, window: windowStub, console };
vm.createContext(sandbox);
vm.runInContext(vecxCode, sandbox);

const vecx = new sandbox.VecX();

// Install multibank binary
vecx.install_rom(binData);

console.log('[VECX] Current bank:', vecx.current_bank);
console.log('[VECX] Cart size:', vecx.cart ? vecx.cart.length : 0);

// Trace first 50 instructions
for (let i = 0; i < 50; i++) {
    const pc_before = vecx.e6809.pc;
    const bank_before = vecx.current_bank;
    
    vecx.e6809_sstep();
    
    const pc_after = vecx.e6809.pc;
    const bank_after = vecx.current_bank;
    
    if (bank_after !== bank_before) {
        console.log(`[BANK SWITCH] Step ${i}: PC=${pc_before.toString(16)} -> ${pc_after.toString(16)}, Bank ${bank_before} -> ${bank_after}`);
    }
}

console.log('\n[FINAL] Bank:', vecx.current_bank);
