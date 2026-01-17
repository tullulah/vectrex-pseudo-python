// Test test1.bin in JSVecx emulator
const fs = require('fs');
const path = require('path');
const vm = require('vm');

// Load BIOS and binary
const biosPath = path.join(__dirname, 'ide/frontend/dist/bios.bin');
const binPath = path.join(__dirname, 'examples/test1/build/test1.bin');

console.log('Loading BIOS from:', biosPath);
console.log('Loading binary from:', binPath);

const biosData = fs.readFileSync(biosPath);
const binData = fs.readFileSync(binPath);

console.log('BIOS size:', biosData.length, 'bytes');
console.log('Binary size:', binData.length, 'bytes');

// Load JSVecx (remove ES6 export syntax for vm compatibility)
let vecxCode = fs.readFileSync(path.join(__dirname, 'ide/frontend/src/generated/jsvecx/vecx_full.js'), 'utf8');
vecxCode = vecxCode.replace(/export\s*\{\s*VecX\s*,\s*Globals\s*\}\s*;/g, '// export removed');

// Minimal stubs
const Globals = {
  ALG_MAX_X: 256,
  ALG_MAX_Y: 256,
  snd_regs: new Uint8Array(16),
};

const context = {
  Globals,
  console,
  Uint8Array,
  Int8Array,
  Uint16Array,
  Int16Array,
  Uint32Array,
  Int32Array,
  Float32Array,
  Float64Array,
  Array,
  Math,
  Date,
};

vm.createContext(context);

// Execute JSVecx
vm.runInContext(vecxCode, context);
const VecX = context.VecX;

console.log('\n=== Initializing JSVecx ===');
const vecx = new VecX();

// Load BIOS
console.log('Loading BIOS into emulator...');
for (let i = 0; i < biosData.length; i++) {
  vecx.write8(0xE000 + i, biosData[i]);
}

// Load cartridge
console.log('Loading cartridge into emulator...');
for (let i = 0; i < binData.length && i < 0x8000; i++) {
  vecx.write8(i, binData[i]);
}

console.log('\n=== Emulator State Before Reset ===');
console.log('PC:', '0x' + vecx.e6809_read16(vecx.REG_PC).toString(16).toUpperCase());

// Reset emulator (should jump to BIOS which verifies and jumps to $0000)
console.log('\n=== Calling vecx_reset() ===');
vecx.vecx_reset();

console.log('\n=== Emulator State After Reset ===');
const pc = vecx.e6809_read16(vecx.REG_PC);
const a = vecx.e6809_read8(vecx.REG_A);
const b = vecx.e6809_read8(vecx.REG_B);
const x = vecx.e6809_read16(vecx.REG_X);
const y = vecx.e6809_read16(vecx.REG_Y);
const s = vecx.e6809_read16(vecx.REG_S);
const u = vecx.e6809_read16(vecx.REG_U);
const dp = vecx.e6809_read8(vecx.REG_DP);

console.log('PC:', '0x' + pc.toString(16).toUpperCase().padStart(4, '0'));
console.log('A:', '0x' + a.toString(16).toUpperCase().padStart(2, '0'));
console.log('B:', '0x' + b.toString(16).toUpperCase().padStart(2, '0'));
console.log('X:', '0x' + x.toString(16).toUpperCase().padStart(4, '0'));
console.log('Y:', '0x' + y.toString(16).toUpperCase().padStart(4, '0'));
console.log('S:', '0x' + s.toString(16).toUpperCase().padStart(4, '0'));
console.log('U:', '0x' + u.toString(16).toUpperCase().padStart(4, '0'));
console.log('DP:', '0x' + dp.toString(16).toUpperCase().padStart(2, '0'));

// Execute 100 steps and show PC progression
console.log('\n=== Executing 100 steps ===');
for (let i = 0; i < 100; i++) {
  const prevPc = vecx.e6809_read16(vecx.REG_PC);
  vecx.e6809_sstep();
  const newPc = vecx.e6809_read16(vecx.REG_PC);
  
  if (i < 20) {
    console.log(`Step ${i}: PC 0x${prevPc.toString(16).toUpperCase().padStart(4, '0')} -> 0x${newPc.toString(16).toUpperCase().padStart(4, '0')}`);
  }
  
  // Check for error conditions
  if (newPc === 0x0050) {
    console.log('\n⚠️  REACHED 0x0050 (ERROR CONDITION)');
    console.log('Reading opcode at 0x0050:', '0x' + vecx.read8(0x0050).toString(16).toUpperCase());
    break;
  }
  
  if (newPc === 0x0019) {
    console.log('\n✅ REACHED START label at 0x0019!');
    break;
  }
}

console.log('\n=== Final State ===');
const finalPc = vecx.e6809_read16(vecx.REG_PC);
console.log('Final PC:', '0x' + finalPc.toString(16).toUpperCase().padStart(4, '0'));

// Disassemble around PC
console.log('\n=== Memory around PC ===');
for (let i = -5; i <= 5; i++) {
  const addr = finalPc + i;
  const byte = vecx.read8(addr);
  console.log(`0x${addr.toString(16).toUpperCase().padStart(4, '0')}: 0x${byte.toString(16).toUpperCase().padStart(2, '0')}`);
}
