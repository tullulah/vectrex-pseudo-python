// ...existing code...
// Test mínimo para jsvecx en Node.js
// Requiere: node, y que los archivos vecx.js, e6809.js, e8910.js, utils.js, globals.js estén en el mismo directorio

const fs = require('fs');
const path = require('path');
const vm = require('vm');

// Cargar los archivos fuente del emulador en el contexto actual

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

const baseDir = __dirname;
loadJSGlobal(path.join(baseDir, 'globals.js'), ['Globals']);
loadJSGlobal(path.join(baseDir, 'utils.js'), ['utils']);
loadJSGlobal(path.join(baseDir, 'e6809.js'), ['e6809']);
loadJSGlobal(path.join(baseDir, 'e8910.js'), ['e8910']);
loadJSGlobal(path.join(baseDir, 'osint.js'), ['osint']);
loadJSGlobal(path.join(baseDir, 'vecx.js'), ['VecX']);

// Cargar la ROM de Minestorm (como ejemplo)
const romPath = path.join(baseDir, '../../roms/MineStorm.bin');
const biosPath = path.join(baseDir, '../../roms/fastrom.dat');

const romData = fs.readFileSync(romPath);
const biosData = fs.readFileSync(biosPath);


// Instanciar el emulador
const emu = new VecX();
// Parchear osint.osint_render para entorno Node.js (sin canvas)
emu.osint.osint_render = function() {
  // No-op en Node.js
};

// Inicializar arrays de vectores para evitar undefined en Node.js
for (let i = 0; i < emu.vectors_draw.length; ++i) {
  emu.vectors_draw[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}
for (let i = 0; i < emu.vectors_erse.length; ++i) {
  emu.vectors_erse[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}
// Asignar referencias cruzadas para acceso a memoria y render
emu.e6809.vecx = emu;
emu.osint.vecx = emu;

// Cargar BIOS y cartucho
for (let i = 0; i < biosData.length && i < emu.rom.length; ++i) {
  emu.rom[i] = biosData[i];
}
for (let i = 0; i < romData.length && i < emu.cart.length; ++i) {
  emu.cart[i] = romData[i];
}


// Ejecutar la emulación en bloques hasta que se dibuje al menos un vector


// --- Per-instruction trace buffer ---
const traceBuf = [];
const maxTrace = 200000; // safety limit

// Ensure emu.e6809 is initialized
if (!emu.e6809) {
  // Try to force initialization by running a minimal step
  if (typeof emu.vecx_emu === 'function') {
    emu.vecx_emu(1, 0);
  }
}
if (!emu.e6809) {
  throw new Error('emu.e6809 is not initialized after minimal step');
}

// Patch e6809_sstep to hook per-instruction execution
const orig_sstep = emu.e6809.e6809_sstep.bind(emu.e6809);
emu.e6809.e6809_sstep = function(irq_i, irq_f) {
  // Capture CPU state before executing instruction
  // Use reg_pc, reg_a, reg_b, reg_x, reg_y, reg_u, reg_s, reg_dp, reg_cc
  
  // ANÁLISIS DE FLAGS Z y BRANCHES críticos para copyright detection
  if (traceBuf.length % 5000 === 0 && traceBuf.length > 5000) {
    const z_flag = (this.reg_cc & 0x04) !== 0;
    console.log(`[DEBUG] PC=${this.reg_pc.toString(16).toUpperCase().padStart(4,'0')} Z=${z_flag} CC=${this.reg_cc.toString(16).toUpperCase().padStart(2,'0')}`);
    
    // Detectar branches críticos de copyright
    if (this.reg_pc >= 0xF180 && this.reg_pc <= 0xF1C0) {
      const opcode = (typeof this.vecx.read8 === 'function') ? this.vecx.read8(this.reg_pc) : undefined;
      if (opcode === 0x26) {
        console.log(`[BRANCH] BNE en ${this.reg_pc.toString(16).toUpperCase()}, Z=${z_flag}, saltará: ${!z_flag}`);
      } else if (opcode === 0x27) {
        console.log(`[BRANCH] BEQ en ${this.reg_pc.toString(16).toUpperCase()}, Z=${z_flag}, saltará: ${z_flag}`);
      }
    }
    
    // ANÁLISIS DE Vec_Str_Ptr y memoria de copyright
    // Vec_Str_Ptr está en $C839/C83A - vamos a leer ese puntero y volcar su contenido
    if (typeof this.vecx.read8 === 'function') {
      const vec_str_ptr_low = this.vecx.read8(0xC83A);
      const vec_str_ptr_high = this.vecx.read8(0xC839);
      const vec_str_ptr = (vec_str_ptr_high << 8) | vec_str_ptr_low;
      
      console.log(`[DEBUG] Vec_Str_Ptr: $${vec_str_ptr.toString(16).toUpperCase().padStart(4,'0')}`);
      
      // Volcar contenido de la dirección apuntada por Vec_Str_Ptr
      if (vec_str_ptr !== 0) {
        let mem_dump = '[DEBUG] Contenido en Vec_Str_Ptr: ';
        let ascii_dump = '[DEBUG] ASCII: ';
        for (let i = 0; i < 32; i++) {
          const byte_val = this.vecx.read8(vec_str_ptr + i);
          mem_dump += byte_val.toString(16).toUpperCase().padStart(2,'0') + ' ';
          ascii_dump += (byte_val >= 32 && byte_val <= 126) ? String.fromCharCode(byte_val) : '.';
        }
        console.log(mem_dump);
        console.log(ascii_dump);
        
        // También volcar memoria alrededor de $C800-$C900 (área típica de datos de cartucho)
        console.log('[DEBUG] Memoria C800-C81F:');
        let cart_mem = '';
        for (let i = 0; i < 32; i++) {
          const addr = 0xC800 + i;
          const byte_val = this.vecx.read8(addr);
          cart_mem += byte_val.toString(16).toUpperCase().padStart(2,'0') + ' ';
          if ((i + 1) % 16 === 0) {
            console.log(`[DEBUG] C${(0x800 + i - 15).toString(16).toUpperCase()}: ${cart_mem}`);
            cart_mem = '';
          }
        }
      } else {
        console.log('[DEBUG] Vec_Str_Ptr es NULL (0x0000) - problema detectado!');
      }
    }
  }
  
  traceBuf.push({
    pc: this.reg_pc,
    a: this.reg_a,
    b: this.reg_b,
    x: this.reg_x.value,
    y: this.reg_y.value,
    u: this.reg_u.value,
    s: this.reg_s.value,
    dp: this.reg_dp,
    cc: this.reg_cc,
    opcode: (typeof this.vecx.read8 === 'function') ? this.vecx.read8(this.reg_pc) : undefined,
    // Optionally, add more fields as needed
  });
  if (traceBuf.length < maxTrace) {
    return orig_sstep(irq_i, irq_f);
  } else {
    throw new Error('Trace buffer overflow');
  }
};

let totalCycles = 0;
let maxBlocks = 1000; // safety limit
let blockCycles = 100000;
let block = 0;
let halted = false;
try {
  while (emu.vector_draw_cnt === 0 && block < maxBlocks) {
    emu.vecx_emu(blockCycles, 0);
    totalCycles += blockCycles;
    block++;
    if (block % 10 === 0) {
      console.log(`Bloque ${block}, ciclos totales: ${totalCycles}`);
    }
  }
} catch (e) {
  console.error('Emulación detenida por excepción:', e.message);
  halted = true;
}

const pc = emu.e6809.reg_pc || (emu.e6809.pc ? emu.e6809.pc : undefined);
const a = emu.e6809.reg_a || (emu.e6809.a ? emu.e6809.a : undefined);
const b = emu.e6809.reg_b || (emu.e6809.b ? emu.e6809.b : undefined);
const ramC880 = emu.ram[0x880];

console.log('--- Resultado al primer vector dibujado ---');
console.log('Bloques ejecutados:', block);
console.log('Ciclos totales:', totalCycles);
console.log('Vectores dibujados:', emu.vector_draw_cnt);
console.log('PC:', pc && pc.value !== undefined ? pc.value.toString(16) : pc);
console.log('A:', a && a.value !== undefined ? a.value.toString(16) : a);
console.log('B:', b && b.value !== undefined ? b.value.toString(16) : b);
console.log('RAM[0xC880]:', ramC880);

// --- Print first 50 and last 20 trace entries ---
function fmtReg(val, len=2) {
  return val !== undefined ? val.toString(16).toUpperCase().padStart(len, '0') : '??';
}

console.log('\n=== PRIMERAS 50 ENTRADAS (INICIO BIOS) ===');
for (let i = 0; i < Math.min(50, traceBuf.length); ++i) {
  const e = traceBuf[i];
  console.log(`${i.toString().padStart(3)}: ${fmtReg(e.pc,4)}: OPC=${fmtReg(e.opcode)} A=${fmtReg(e.a)} B=${fmtReg(e.b)} X=${fmtReg(e.x,4)} Y=${fmtReg(e.y,4)}`);
}

console.log('\n=== ÚLTIMAS 20 ENTRADAS ===');
let startIdx = Math.max(0, traceBuf.length - 20);
for (let i = startIdx; i < traceBuf.length; ++i) {
  const e = traceBuf[i];
  console.log(`${i.toString().padStart(4)}: ${fmtReg(e.pc,4)}: OPC=${fmtReg(e.opcode)} A=${fmtReg(e.a)} B=${fmtReg(e.b)} X=${fmtReg(e.x,4)} Y=${fmtReg(e.y,4)}`);
}

// --- Loop/delay region detection (simple: look for repeated PC) ---
const pcCounts = {};
for (const e of traceBuf) {
  pcCounts[e.pc] = (pcCounts[e.pc] || 0) + 1;
}
const loopPCs = Object.entries(pcCounts).filter(([pc, count]) => count > 10).sort((a,b)=>b[1]-a[1]);

// === ANÁLISIS DE TRANSICIONES CRÍTICAS ===
console.log('\n=== ANÁLISIS DE TRANSICIONES CRÍTICAS ===');

// Análisis detallado de funciones Print_Str
let print_str_functions = {
    0xF373: { name: 'Print_Str_hwyx', count: 0 },
    0xF378: { name: 'Print_Str_yx', count: 0 },
    0xF495: { name: 'Print_Str', count: 0 },
    0xF383: { name: 'Print_Str_d', count: 0 }
};

let f4ebEntries = 0, f19eEntries = 0;
for (const e of traceBuf) {
  if (e.pc === 0xF4EB) f4ebEntries++;
  if (e.pc === 0xF19E) f19eEntries++;
  
  // Detectar visitas a funciones Print_Str
  if (print_str_functions[e.pc]) {
    print_str_functions[e.pc].count++;
  }
}

console.log(`F4EB (bucle DECB/BNE): ${f4ebEntries} veces`);
console.log(`F19E (bucle BITA/BEQ): ${f19eEntries} veces`);

console.log('\n=== FUNCIONES PRINT_STR VISITADAS ===');
for (const [pc, func] of Object.entries(print_str_functions)) {
  console.log(`${func.name} (${pc.toString(16).toUpperCase()}): ${func.count} veces`);
}

// Análisis de transiciones hacia Print_Str
console.log('\n=== TRANSICIONES HACIA PRINT_STR ===');

// Detectar transiciones clave
for (let i = 1; i < traceBuf.length; i++) {
  const prev = traceBuf[i-1];
  const curr = traceBuf[i];
  
  // Detectar entradas a funciones Print_Str
  if (print_str_functions[curr.pc] && print_str_functions[curr.pc].count <= 3) {
    console.log(`ENTRADA a ${print_str_functions[curr.pc].name} desde ${fmtReg(prev.pc,4)} en step ${i}`);
  }
  
  // Transición hacia F19E (bucle bueno)
  if (curr.pc === 0xF19E && prev.pc !== 0xF1A0) {
    console.log(`ENTRADA a F19E desde ${fmtReg(prev.pc,4)} en step ${i} (A=${fmtReg(curr.a)} B=${fmtReg(curr.b)})`);
  }
  
  // Transición hacia F4EB (bucle malo)
  if (curr.pc === 0xF4EB && prev.pc !== 0xF4EF) {
    console.log(`ENTRADA a F4EB desde ${fmtReg(prev.pc,4)} en step ${i} (A=${fmtReg(curr.a)} B=${fmtReg(curr.b)})`);
  }
  
  // Salida de F19E
  if (prev.pc === 0xF19E && curr.pc !== 0xF1A0) {
    console.log(`SALIDA de F19E hacia ${fmtReg(curr.pc,4)} en step ${i} (A=${fmtReg(curr.a)} B=${fmtReg(curr.b)})`);
  }
}

if (loopPCs.length > 0) {
  const [loopPC, loopCount] = loopPCs[0];
  console.log(`\n=== BUCLE/DELAY DETECTADO: PC=${fmtReg(Number(loopPC),4)} (${loopCount} repeticiones) ===`);
  // Print first 20 and last 10 entries with this PC
  const delayEntries = traceBuf.map((e,idx)=>({e,idx})).filter(obj=>obj.e.pc==loopPC);
  for (let i = 0; i < Math.min(20, delayEntries.length); ++i) {
    const {e,idx} = delayEntries[i];
    console.log(`${i}: trace[${idx}] ${fmtReg(e.pc,4)}: OPC=${fmtReg(e.opcode)} Y=${fmtReg(e.y,4)}`);
  }
  if (delayEntries.length > 20) {
    console.log('...');
    for (let i = Math.max(0, delayEntries.length-10); i < delayEntries.length; ++i) {
      const {e,idx} = delayEntries[i];
      console.log(`${i}: trace[${idx}] ${fmtReg(e.pc,4)}: OPC=${fmtReg(e.opcode)} Y=${fmtReg(e.y,4)}`);
    }
  }
}

// --- Vector call detection (BIOS draw routines) ---
const vectorPCs = [0xF2A4, 0xF2A6, 0xF2A8, 0xF2AA];
const vectorCalls = traceBuf.map((e,idx)=>({e,idx})).filter(obj=>vectorPCs.includes(obj.e.pc));
console.log('\n=== LLAMADAS VECTORIALES DETECTADAS ===');
console.log(`Llamadas BIOS de vectores encontradas: ${vectorCalls.length}`);
for (let i = 0; i < Math.min(5, vectorCalls.length); ++i) {
  const {e,idx} = vectorCalls[i];
  console.log(`${i}: trace[${idx}] ${fmtReg(e.pc,4)}: OPC=${fmtReg(e.opcode)} A=${fmtReg(e.a)} B=${fmtReg(e.b)}`);
}

// --- Suspicious coordinate analysis (254 or 30 in A/B/X/Y) ---
const suspiciousVals = [254, 30];
const suspiciousEntries = traceBuf.map((e,idx)=>({e,idx})).filter(obj=>
  suspiciousVals.includes(obj.e.a) || suspiciousVals.includes(obj.e.b) ||
  suspiciousVals.includes(obj.e.x) || suspiciousVals.includes(obj.e.y)
);
console.log('\n=== ENTRADAS CON COORDENADAS SOSPECHOSAS (254 o 30) ===');
console.log(`Entradas encontradas: ${suspiciousEntries.length}`);
for (let i = 0; i < Math.min(10, suspiciousEntries.length); ++i) {
  const {e,idx} = suspiciousEntries[i];
  console.log(`${i}: trace[${idx}] ${fmtReg(e.pc,4)}: OPC=${fmtReg(e.opcode)} A=${fmtReg(e.a)} B=${fmtReg(e.b)} X=${fmtReg(e.x,4)} Y=${fmtReg(e.y,4)}`);
}

// --- Export trace as JSON ---
const traceJson = JSON.stringify(traceBuf, null, 2);
fs.writeFileSync('startup_trace.json', traceJson);
console.log('\n=== TRACE EXPORTADO ===');
console.log('Trace completo guardado en startup_trace.json');
console.log('Usar tools/analyze_trace.py para análisis detallado');
