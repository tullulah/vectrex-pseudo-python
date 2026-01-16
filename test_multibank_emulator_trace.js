#!/usr/bin/env node

// Test multibank ROM in JSVecx emulator
const fs = require('fs');

// Import JSVecx
const jsVecx = require('./jsvecx_comparison.js');

const romPath = './test_multibank_simple.bin';
if (!fs.existsSync(romPath)) {
    console.error(`❌ ROM not found: ${romPath}`);
    process.exit(1);
}

const romData = fs.readFileSync(romPath);
console.log(`📖 Loaded multibank ROM: ${romData.length} bytes`);

// Setup emulator
const e6809 = require('./node_modules/jsvecx/src/deploy/js/e6809.js').e6809;
const vecx = {cpu: new e6809()};

// Custom memory hooks
const origWrite = vecx.cpu.memory.write8.bind(vecx.cpu.memory);
const origRead = vecx.cpu.memory.read8.bind(vecx.cpu.memory);

let bankSwitches = 0;
let lastPC = 0;

vecx.cpu.memory.write8 = function(addr, val) {
    if (addr === 0xDF00) {
        console.log(`  Bank switch to #${val}`);
        bankSwitches++;
    }
    origWrite(addr, val);
};

vecx.cpu.memory.read8 = function(addr) {
    const val = origRead(addr);
    
    // Log interesting reads
    if (addr >= 0x70 && addr <= 0x85 && lastPC !== addr) {
        console.log(`  Read 0x${addr.toString(16).padStart(4,'0')}: 0x${val.toString(16).padStart(2,'0')}`);
    }
    return val;
};

// Load ROM
console.log('\n🚀 Loading ROM into emulator...');
for (let i = 0; i < romData.length; i++) {
    vecx.cpu.memory.write8(i, romData[i]);
}

// Run for short time
console.log('\n▶️  Running emulator (max 2000 steps)...');
let stepCount = 0;
let halted = false;

while (stepCount < 2000 && !halted) {
    const pc = vecx.cpu.getPC();
    
    if (pc === 0xF33D) {
        console.log(`\n⚠️  HALT at PC=0xF33D after ${stepCount} steps`);
        halted = true;
    } else if (stepCount % 200 === 0) {
        console.log(`  Step ${stepCount}: PC=0x${pc.toString(16).padStart(4,'0')}`);
    }
    
    try {
        vecx.cpu.sstep();
        lastPC = pc;
        stepCount++;
    } catch (e) {
        console.log(`\n❌ Error at step ${stepCount}: ${e.message}`);
        console.log(`  PC=0x${vecx.cpu.getPC().toString(16).padStart(4,'0')}`);
        break;
    }
}

console.log(`\n📊 Summary:`);
console.log(`  Total steps: ${stepCount}`);
console.log(`  Bank switches: ${bankSwitches}`);
console.log(`  Final PC: 0x${vecx.cpu.getPC().toString(16).padStart(4,'0')}`);
console.log(`  Status: ${halted ? '⏸️ HALTED' : '⏱️ TIMEOUT'}`);
