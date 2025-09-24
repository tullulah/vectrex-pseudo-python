const fs=require('fs');
const path=require('path');
const p=path.resolve(__dirname,'..','core','src','bios','vectrex.bin');
if(!fs.existsSync(p)){ console.error('bios not found',p); process.exit(1);} 
const b=fs.readFileSync(p);
console.log('size',b.length);
function dumpHalf(h){
  const off=h*4096;
  if (off+0x0FFF >= b.length){ console.log('half',h,'out-of-range'); return; }
  const hi=b[off+0x0FFE]; const lo=b[off+0x0FFF];
  const addr=((hi<<8)|lo)&0xFFFF;
  console.log('half',h,'resetVecBytes',hi.toString(16).padStart(2,'0'),lo.toString(16).padStart(2,'0'),'addr',addr.toString(16).padStart(4,'0'));
}
if (b.length===4096){ dumpHalf(0); }
else if (b.length===8192){ dumpHalf(0); dumpHalf(1); }
else { console.log('unexpected size'); }
