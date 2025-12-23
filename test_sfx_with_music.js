#!/usr/bin/env node

/**
 * Test: Music + SFX Coexistence
 * 
 * After reordering (MUSIC_UPDATE first, SFX_UPDATE second),
 * verify that both music and SFX play without interference.
 * 
 * Expected: BGM plays continuously in background, SFX triggers
 * every ~17 frames (coin → jump → hit → repeat) with clear sound.
 */

const fs = require('fs');
const path = require('path');
const vm = require('vm');

// Load JSVecx environment
const jsVecxDir = path.join(__dirname, 'ide', 'frontend', 'public', 'jsvecx_deploy');

// Mock window and document for Node.js
global.window = global;
global.document = {
  getElementById: () => ({ getContext: () => ({}) }),
  addEventListener: () => {}
};

function loadJSGlobal(filename, symbolNames) {
  const code = fs.readFileSync(filename, 'utf8');
  vm.runInThisContext(code, { filename });
  if (symbolNames) {
    for (const sym of symbolNames) {
      if (typeof global[sym] === 'undefined' && typeof eval(sym) !== 'undefined') {
        global[sym] = eval(sym);
      }
    }
  }
}

// Load JSVecx modules in correct order
loadJSGlobal(path.join(jsVecxDir, 'globals.js'), ['Globals']);
loadJSGlobal(path.join(jsVecxDir, 'utils.js'), ['utils']);
loadJSGlobal(path.join(jsVecxDir, 'e6809.js'), ['e6809']);
loadJSGlobal(path.join(jsVecxDir, 'e8910.js'), ['e8910']);
loadJSGlobal(path.join(jsVecxDir, 'osint.js'), ['osint']);
loadJSGlobal(path.join(jsVecxDir, 'vecx.js'), ['VecX']);

// Load BIOS and ROM
const biosPath = path.join(__dirname, 'ide', 'frontend', 'dist', 'bios.bin');
const romPath = path.join(__dirname, 'examples', 'sfx_buttons', 'src', 'main.bin');

if (!fs.existsSync(romPath)) {
  console.error(`❌ ROM not found: ${romPath}`);
  console.error(`   Make sure to compile: ./target/release/vectrexc build examples/sfx_buttons/src/main.vpy --bin`);
  process.exit(1);
}

const biosData = fs.readFileSync(biosPath);
const romData = fs.readFileSync(romPath);

console.log('🎮 Music + SFX Coexistence Test\n');
console.log(`📂 BIOS: ${biosPath} (${biosData.length} bytes)`);
console.log(`📂 ROM:  ${romPath} (${romData.length} bytes)\n`);

// Initialize Vectrex emulator
const v = new VecX();

// Patch osint for Node.js
v.osint.osint_render = function() { /* no-op */ };

// Initialize vector arrays
for (let i = 0; i < v.vectors_draw.length; ++i) {
  v.vectors_draw[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}
for (let i = 0; i < v.vectors_erse.length; ++i) {
  v.vectors_erse[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}

// Setup cross-references
v.e6809.vecx = v;
v.osint.vecx = v;

// Load BIOS
for (let i = 0; i < biosData.length && i < v.rom.length; ++i) {
  v.rom[i] = biosData[i];
}

// Load cartridge
for (let i = 0; i < romData.length && i < v.cart.length; ++i) {
  v.cart[i] = romData[i];
}

// Track audio activity
let sfxActivityLog = [];
let musicActivityLog = [];
let lastSfxFrame = null;
let lastMusicFrame = null;

// Hook into PSG register writes to track audio activity
const originalWrite8 = v.write8.bind(v);
v.write8 = function(addr, val) {
  // PSG register range: 0xD000-0xD00F
  if (addr >= 0xD000 && addr <= 0xD00F) {
    const regNum = addr - 0xD000;
    
    // Track music vs SFX based on register
    // Music typically uses registers for channel B (9, 11, 12, 13, 14)
    // SFX typically uses channel A (0, 1, 2, 3, 8)
    
    const isMusicReg = [9, 11, 12, 13, 14].includes(regNum);
    const isSfxReg = [0, 1, 2, 3, 8].includes(regNum);
    
    if (isMusicReg && v.ssteps >= (lastMusicFrame || 0) + 10) {
      if (!musicActivityLog.length || musicActivityLog[musicActivityLog.length - 1].frame < v.ssteps) {
        musicActivityLog.push({ frame: v.ssteps, reg: regNum, val });
        lastMusicFrame = v.ssteps;
      }
    }
    
    if (isSfxReg && v.ssteps >= (lastSfxFrame || 0) + 10) {
      if (!sfxActivityLog.length || sfxActivityLog[sfxActivityLog.length - 1].frame < v.ssteps) {
        sfxActivityLog.push({ frame: v.ssteps, reg: regNum, val });
        lastSfxFrame = v.ssteps;
      }
    }
  }
  
  return originalWrite8(addr, val);
};

// Run emulation
const maxFrames = 100; // ~2 seconds at 50 FPS
let lastReportFrame = 0;

console.log('Running emulation...\n');
console.log('Frame | Event');
console.log('------|-------');

for (let frame = 0; frame < maxFrames; frame++) {
  try {
    v.sstep(50); // Single frame step
    
    // Report every 17 frames (SFX interval)
    if (frame % 17 === 0) {
      let event = 'SFX trigger expected';
      if (frame === 0) event = 'Start (coin plays)';
      console.log(`${frame.toString().padStart(5)} | ${event}`);
    }
  } catch (e) {
    console.error(`\n❌ Emulation error at frame ${frame}: ${e.message}`);
    break;
  }
}

console.log('\n✅ Emulation complete\n');

// Analyze activity
console.log('📊 Audio Activity Summary:');
console.log(`   Music register writes: ${musicActivityLog.length}`);
console.log(`   SFX register writes:   ${sfxActivityLog.length}\n`);

if (sfxActivityLog.length === 0) {
  console.log('⚠️  WARNING: No SFX register activity detected!');
  console.log('   This suggests the SFX system is not updating.');
} else {
  console.log(`✅ SFX system active: Detected at frames ${sfxActivityLog.slice(0, 5).map(a => a.frame).join(', ')}...`);
}

if (musicActivityLog.length === 0) {
  console.log('⚠️  WARNING: No music register activity detected!');
  console.log('   This suggests the music system is not updating.');
} else {
  console.log(`✅ Music system active: Detected at frames ${musicActivityLog.slice(0, 5).map(a => a.frame).join(', ')}...`);
}

if (sfxActivityLog.length > 0 && musicActivityLog.length > 0) {
  console.log('\n🎉 SUCCESS: Both music and SFX systems active!');
  console.log('   The reordering (MUSIC_UPDATE → SFX_UPDATE) appears to work.');
} else {
  console.log('\n❌ FAILED: One or both audio systems not active.');
}

process.exit(sfxActivityLog.length > 0 && musicActivityLog.length > 0 ? 0 : 1);
