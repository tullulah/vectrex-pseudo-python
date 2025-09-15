const { globalCpu } = require('../ide/electron/dist/emu6809.js');
const fs = require('fs');
const path = require('path');
const p = path.resolve(__dirname,'..','core','src','bios','vectrex.bin');
const data = fs.readFileSync(p);
globalCpu.loadBios(new Uint8Array(data));
function hex(b){return b.toString(16).padStart(2,'0');}
function dump(){
  const addrs=[0xFFF0,0xFFF2,0xFFF4,0xFFF6,0xFFF8,0xFFFA,0xFFFC,0xFFFE];
  for(const a of addrs){
    const hi=globalCpu.mem[a]; const lo=globalCpu.mem[a+1];
    console.log(a.toString(16), hex(hi), hex(lo), '->', ((hi<<8)|lo).toString(16));
  }
}
dump();
