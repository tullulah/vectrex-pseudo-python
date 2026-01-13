#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Load JSVecx from the frontend deployment
const jsvecxPath = path.join(__dirname, 'ide/frontend/public/jsvecx_deploy');
const utils = require(path.join(jsvecxPath, 'utils.js'));
const globals = require(path.join(jsvecxPath, 'globals.js'));
const e6809 = require(path.join(jsvecxPath, 'e6809.js'));
const vecx = require(path.join(jsvecxPath, 'vecx.js'));

console.log('=== VIA Address Fix Verification ===\n');

// Setup emulator
const emulator = new vecx.Vecx();

// Load BIOS
const biosPath = path.join(__dirname, 'ide/frontend/dist/bios.bin');
const biosData = fs.readFileSync(biosPath);
console.log(`[SETUP] BIOS loaded: ${biosData.length} bytes`);

// Load compiled TestController ROM
const romPath = path.join(__dirname, 'examples/TestController/src/main.bin');
if (!fs.existsSync(romPath)) {
    console.log(`❌ ROM not found: ${romPath}`);
    console.log('   Run: cargo run --bin vectrexc -- build examples/TestController/src/main.vpy --bin');
    process.exit(1);
}

const romData = fs.readFileSync(romPath);
console.log(`[SETUP] ROM loaded: ${romData.length} bytes`);

// Initialize emulator with BIOS + ROM
emulator.load(biosData, romData);
console.log(`[SETUP] Emulator initialized\n`);

// Track VIA writes to check for bank register writes
let viaWrites = [];
let bankSwitches = [];
const originalWrite8 = emulator.write8.bind(emulator);

emulator.write8 = function(addr, data) {
    // Track all writes to the VIA region ($D000-$D00F)
    if (addr >= 0xD000 && addr <= 0xD00F) {
        viaWrites.push({ pc: this.cpu.pc, addr: addr, data: data, label: getVIALabel(addr) });
    }
    
    // Track bank switches specifically
    if (addr === 0xD000) {
        const oldBank = this.currentBank || 0;
        bankSwitches.push({ pc: this.cpu.pc, addr: 0xD000, oldBank: oldBank, newBank: data });
    }
    
    return originalWrite8(addr, data);
};

function getVIALabel(addr) {
    const labels = {
        0xD001: 'VIA_port_a',
        0xD002: 'VIA_DDR_b',
        0xD003: 'VIA_DDR_a',
        0xD004: 'VIA_t1_cnt_lo',
        0xD005: 'VIA_t1_cnt_hi',
        0xD006: 'VIA_t1_lch_lo',
        0xD007: 'VIA_t1_lch_hi',
        0xD008: 'VIA_t2_lo',
        0xD009: 'VIA_t2_hi',
        0xD00A: 'VIA_shift_reg',
        0xD00B: 'VIA_aux_cntl',
        0xD00C: 'VIA_cntl',
        0xD00D: 'VIA_int_flags',
        0xD00E: 'VIA_int_enable',
        0xD00F: 'VIA_port_a_nohs',
        0xD000: '⚠️ BANK_SWITCH_REGISTER'
    };
    return labels[addr] || `UNKNOWN_0x${addr.toString(16).toUpperCase()}`;
}

// Run for a limited number of cycles
console.log('[TEST] Running CPU for 10,000 cycles...\n');
let targetCycles = 10000;
let startCycles = emulator.cpu.cycles;

try {
    while (emulator.cpu.cycles - startCycles < targetCycles) {
        emulator.step();
        
        // Detect if we're stuck in a loop
        if (viaWrites.length > 100) break;
        if (bankSwitches.length > 50) break;
    }
} catch (e) {
    console.log(`[TEST] Execution stopped: ${e.message}`);
}

const cyclesRun = emulator.cpu.cycles - startCycles;
console.log(`[TEST] Ran ${cyclesRun} cycles\n`);

// Analyze results
console.log('=== VIA WRITE ANALYSIS ===\n');

if (bankSwitches.length > 0) {
    console.log(`⚠️ BANK SWITCHES DETECTED: ${bankSwitches.length}\n`);
    
    // Show first and last few
    console.log('First 5 bank switches:');
    bankSwitches.slice(0, 5).forEach((sw, i) => {
        console.log(`  ${i+1}. PC=0x${sw.pc.toString(16).toUpperCase().padStart(4, '0')} Bank: ${sw.oldBank} → ${sw.newBank}`);
    });
    
    if (bankSwitches.length > 5) {
        console.log(`  ... (${bankSwitches.length - 10} more) ...`);
        console.log('Last 5 bank switches:');
        bankSwitches.slice(-5).forEach((sw, i) => {
            console.log(`  ${bankSwitches.length - 4 + i}. PC=0x${sw.pc.toString(16).toUpperCase().padStart(4, '0')} Bank: ${sw.oldBank} → ${sw.newBank}`);
        });
    }
} else {
    console.log('✅ NO BANK SWITCHES during execution\n');
}

// Check for writes to actual VIA registers (should be normal)
const normalVIAWrites = viaWrites.filter(w => w.addr !== 0xD000);

if (normalVIAWrites.length > 0) {
    console.log(`✅ NORMAL VIA REGISTER WRITES: ${normalVIAWrites.length}\n`);
    
    // Group by register
    const byAddr = {};
    normalVIAWrites.forEach(w => {
        if (!byAddr[w.addr]) byAddr[w.addr] = [];
        byAddr[w.addr].push(w);
    });
    
    Object.keys(byAddr).sort().forEach(addr => {
        const writes = byAddr[addr];
        const addrNum = parseInt(addr);
        const label = getVIALabel(addrNum);
        console.log(`  ${label} (0x${addrNum.toString(16).toUpperCase()}): ${writes.length} writes`);
    });
} else {
    console.log('ℹ️  No VIA register writes detected\n');
}

// Final status
console.log('\n=== TEST RESULT ===\n');

if (bankSwitches.length === 0) {
    console.log('✅ PASS: No erratic bank switches during execution');
    console.log('         The VIA address fix is working correctly!');
} else {
    console.log('❌ FAIL: Bank switches still occurring');
    console.log(`         ${bankSwitches.length} switches detected`);
}

console.log(`\nPC at end: 0x${emulator.cpu.pc.toString(16).toUpperCase()}`);
console.log(`Cycles: ${cyclesRun}`);
console.log(`Bank: ${emulator.currentBank}\n`);
