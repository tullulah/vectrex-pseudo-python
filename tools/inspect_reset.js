const { globalCpu, hardResetCpu } = require('../ide/electron/dist/emu6809.js');
const fs = require('fs');
const path = require('path');
const p = path.resolve(__dirname,'..','core','src','bios','vectrex.bin');
const data = fs.readFileSync(p);
console.log('loading BIOS size', data.length);
const ok = globalCpu.loadBios(new Uint8Array(data));
console.log('loadBios result', ok);
console.log('bytes at FFFE/FFFF before reset', globalCpu.mem[0xFFFE].toString(16).padStart(2,'0'), globalCpu.mem[0xFFFF].toString(16).padStart(2,'0'));

hardResetCpu();
console.log('after hardReset pc=', globalCpu.pc.toString(16));
console.log('bytes at FFFE/FFFF after reset', globalCpu.mem[0xFFFE].toString(16).padStart(2,'0'), globalCpu.mem[0xFFFF].toString(16).padStart(2,'0'));
