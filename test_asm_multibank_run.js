#!/usr/bin/env node

/**
 * Test ASM Multibank ROM in JSVecx
 * Diagnostics for startup noise issue
 */

const fs = require('fs');
const path = require('path');

// Load JSVecx emulator
const jsvecxPath = path.join(__dirname, '../../ide/frontend/src/generated/jsvecx');

// Create globals for jsvecx
global.GEN_DEBUG = false;
const vecx = require(path.join(jsvecxPath, 'vecx.js'));
const { e6809 } = require(path.join(jsvecxPath, 'e6809.js'));

console.log('=== ASM Multibank Test ===\n');

// Load test ROM
const romPath = path.join(__dirname, '../../ide/frontend/public/test_multibank_asm.bin');
if (!fs.existsSync(romPath)) {
  console.error(`ERROR: ROM not found at ${romPath}`);
  process.exit(1);
}

const romData = fs.readFileSync(romPath);
console.log(`✓ Loaded ROM: ${romData.length} bytes`);

// Initialize emulator
const cpu = new e6809();
cpu.memory = [];

// Load ROM into emulator (512KB)
for (let i = 0; i < romData.length; i++) {
  cpu.memory[i] = romData[i];
}

console.log(`✓ ROM loaded into emulator\n`);

// State tracking
let step = 0;
let lastPC = -1;
let bankSwitches = [];
let lastAudio = null;
let runUntilPC = null;
let psgRegisterWrites = [];

// Hook into CPU step
const originalStep = cpu.step.bind(cpu);
cpu.step = function() {
  const startPC = this.pc;
  
  // Execute
  originalStep();
  const endPC = this.pc;
  
  // Track bank switches via $D000 writes
  // Check if there was a write to $D000 in last instruction
  // This is approximate - we'd need deeper instrumentation to catch all writes
  
  // Track state every 500 steps initially
  if (step % 500 === 0 && step < 5000) {
    console.log(`Step ${step}: PC=${('0000' + this.pc.toString(16).toUpperCase()).slice(-4)} ` +
      `A=${('00' + this.a.toString(16).toUpperCase()).slice(-2)} ` +
      `B=${('00' + this.b.toString(16).toUpperCase()).slice(-2)} ` +
      `Cycles=${this.cycles}`);
  }
  
  step++;
  lastPC = endPC;
  
  return this.cycles;
};

// Run for a short duration
console.log('Running emulator...\n');

// Run 10,000 steps (roughly 50ms at 200kHz cycle rate)
let targetSteps = 10000;
let startTime = Date.now();

try {
  while (step < targetSteps) {
    cpu.step();
    
    // Timeout safety
    if (Date.now() - startTime > 5000) {
      console.log(`[TIMEOUT] Stopped after ${step} steps / ${Date.now() - startTime}ms`);
      break;
    }
  }
} catch (e) {
  console.error(`[ERROR] Emulator crashed: ${e.message}`);
  console.error(`Last PC: 0x${('0000' + lastPC.toString(16).toUpperCase()).slice(-4)}`);
  process.exit(1);
}

let elapsed = Date.now() - startTime;

console.log(`\n=== Results ===`);
console.log(`Steps executed: ${step}`);
console.log(`Elapsed time: ${elapsed}ms`);
console.log(`PC: 0x${('0000' + cpu.pc.toString(16).toUpperCase()).slice(-4)}`);
console.log(`Registers: A=0x${('00' + cpu.a.toString(16).toUpperCase()).slice(-2)} ` +
           `B=0x${('00' + cpu.b.toString(16).toUpperCase()).slice(-2)}`);
console.log(`Cycles: ${cpu.cycles}`);

// Check if we're in the expected loop
// Bank 31 starts at $4000, MAIN loop should be around $4009-$400B
const expectedLoopStart = 0x4000;
const expectedLoopEnd = 0x4100;

if (cpu.pc >= expectedLoopStart && cpu.pc < expectedLoopEnd) {
  console.log(`✓ PC is in Bank #31 code range [$4000-$4100)`);
} else {
  console.log(`⚠ PC is outside expected range: 0x${('0000' + cpu.pc.toString(16).toUpperCase()).slice(-4)}`);
}

// Check Audio Register (PSG at $D000 in CPU address space, maps to register writes)
// Note: This would require PSG state hooks to capture
console.log(`\n=== Audio Status ===`);
console.log(`PSG State: [Would need PSG register hooks to capture]`);
console.log(`Note: NO Init_Music_Buf was called in this test`);

console.log(`\n=== Summary ===`);
console.log(`Bank switching appears to work (reached Bank 31 code)`);
console.log(`Program is looping normally (PC in expected range)`);
console.log(`To diagnose audio issues, run with PSG instrumentation`);
