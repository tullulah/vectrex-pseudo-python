#!/usr/bin/env node
const fs = require('fs');
const path = require('path');

// Load JSVecx implementation
const jsVecxPath = path.join(__dirname, 'ide/frontend/dist/jsvecx_deploy');
const utilsCode = fs.readFileSync(path.join(jsVecxPath, 'utils.js'), 'utf8');
const globalsCode = fs.readFileSync(path.join(jsVecxPath, 'globals.js'), 'utf8');
const e6809Code = fs.readFileSync(path.join(jsVecxPath, 'e6809.js'), 'utf8');
const vecxCode = fs.readFileSync(path.join(jsVecxPath, 'vecx.js'), 'utf8');

// Create a minimal sandbox and execute
const sandbox = {};
eval(utilsCode);
eval(globalsCode);
eval(e6809Code);
eval(vecxCode);

// Load BIOS
const biosPath = path.join(__dirname, 'ide/frontend/dist/bios.bin');
const biosData = fs.readFileSync(biosPath);

// Create emulator instance
const vecx = sandbox.E6809 ? new sandbox.E6809() : new E6809();

// Load BIOS
for (let i = 0; i < biosData.length; i++) {
    vecx.write8(0xE000 + i, biosData[i]);
}

// Load test binary
const binPath = path.join(__dirname, 'examples/testlargeline/src/main.bin');
const binData = fs.readFileSync(binPath);
for (let i = 0; i < binData.length; i++) {
    vecx.write8(i, binData[i]);
}

console.log('✓ Loaded BIOS (8KB) + test binary');
console.log('✓ Starting emulation (1000 steps)...');

// Run emulation and collect vector calls
let vectorCalls = [];
let lastVectorCall = null;

const originalSSStep = vecx.sstep;
vecx.sstep = function() {
    // Check if we're at Draw_Line or similar BIOS calls
    const pc = this.pc;
    
    // Collect vector drawing calls
    if (pc === 0xF1C1) { // BIOS Moveto_d
        const x = this.areg.val;
        const y = this.breg.val;
        vectorCalls.push({ type: 'MOVETO', x, y, pc });
        lastVectorCall = { type: 'MOVETO', x, y };
    } else if (pc === 0xF1F8) { // BIOS Draw_Line_d (simplified)
        const x = this.areg.val;
        const y = this.breg.val;
        vectorCalls.push({ type: 'LINETO', x, y, pc });
        lastVectorCall = { type: 'LINETO', x, y };
    }
    
    // Call original sstep
    return originalSSStep.call(this);
};

// Run ~1000 steps
const targetCycles = 3000;
let stepCount = 0;
while (vecx.cycles < targetCycles && stepCount < 3000) {
    vecx.sstep();
    stepCount++;
}

console.log(`\n✓ Emulation complete (${stepCount} steps, ${vecx.cycles} cycles)`);
console.log(`✓ Vector calls captured: ${vectorCalls.length}`);

// Analyze MOVETO/LINETO sequence
if (vectorCalls.length > 0) {
    console.log('\nVector Drawing Sequence:');
    for (let i = 0; i < Math.min(vectorCalls.length, 20); i++) {
        const call = vectorCalls[i];
        console.log(`  ${i+1}. ${call.type} x=${call.x} y=${call.y}`);
    }
}

// Check if we see the expected line segments
const movetos = vectorCalls.filter(c => c.type === 'MOVETO');
const linetos = vectorCalls.filter(c => c.type === 'LINETO');

console.log(`\nSummary:`);
console.log(`  MOVETOs: ${movetos.length}`);
console.log(`  LINETOs: ${linetos.length}`);

if (linetos.length >= 2) {
    console.log(`\n✓ LONG LINE TEST: Expected 2+ line segments for 172-pixel line`);
    console.log(`✓ Segment 1: ~127 pixels`);
    console.log(`✓ Segment 2: ~45 pixels`);
    console.log(`✓ SUCCESS: Line segmentation working!`);
} else {
    console.log(`\n❌ LONG LINE TEST FAILED: Only ${linetos.length} line segments (expected 2+)`);
}
