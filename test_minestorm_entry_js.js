/*
JavaScript Test - Entrada a Minestorm (Extendido)
Compara capacidad de llegar a Minestorm 0xE000 con 50M ciclos
*/

const fs = require('fs');
const path = require('path');

console.log('🔧 Cargando emulador JavaScript...');

// Cargar JSVecx (archivos compilados) usando eval como el script que funciona
const JSVecxPath = path.join(__dirname, 'jsvecx', 'src', 'deploy', 'js');

// Necesitamos cargar todos los archivos en orden
eval(fs.readFileSync(path.join(JSVecxPath, 'utils.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'globals.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'vector_t.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'e6809.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'e8910.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'osint.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'vecx.js'), 'utf8'));

console.log('✅ Archivos JavaScript cargados');
const fs = require('fs');
const path = require('path');
const vm = require('vm');

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

const baseDir = path.join(__dirname, 'jsvecx', 'src', 'deploy', 'js');
console.log('🔧 Cargando emulador JavaScript...');

try {
  loadJSGlobal(path.join(baseDir, 'globals.js'), ['Globals']);
  loadJSGlobal(path.join(baseDir, 'utils.js'), ['utils']);
  loadJSGlobal(path.join(baseDir, 'e6809.js'), ['e6809']);
  loadJSGlobal(path.join(baseDir, 'e8910.js'), ['e8910']); 
  loadJSGlobal(path.join(baseDir, 'osint.js'), ['osint']);
  loadJSGlobal(path.join(baseDir, 'vecx.js'), ['VecX']);
  console.log('✅ Archivos JavaScript cargados');
} catch (e) {
  console.error('❌ Error:', e.message);
  process.exit(1);
}

const biosPath = 'C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\src\\assets\\bios.bin';
const biosData = fs.readFileSync(biosPath);
console.log('✅ BIOS cargada,', biosData.length, 'bytes');

const emu = new VecX();
emu.osint.osint_render = function() {};

// Inicializar arrays
for (let i = 0; i < emu.vectors_draw.length; ++i) {
  emu.vectors_draw[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}
for (let i = 0; i < emu.vectors_erse.length; ++i) {
  emu.vectors_erse[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}

emu.e6809.vecx = emu;
emu.osint.vecx = emu;

console.log('=== COMPARACIÓN JSVecx - ENTRADA A MINESTORM (EXTENDIDO) ===');

// Cargar ROM
emu.vecx_emu_init();
const biosArray = new Uint8Array(biosData);
emu.mmu.rom = biosArray.slice();
emu.e6809.e6809_reset();

console.log('🟡 JAVASCRIPT EMULATOR:');
console.log('   Ejecutando hasta 50 millones de ciclos buscando entrada a Minestorm...');

const maxSteps = 50_000_000;
let minestormEntries = [];
let printStrCalls = [];

// Direcciones importantes de Minestorm en BIOS (confirmado en bios.asm línea 127)
const minestormAddresses = [
    0xE000, // Minestorm start address (confirmed in bios.asm)
    0xE100, 0xE200, 0xE300, 0xE400, 0xE500, 0xE600, 0xE700, 0xE800, 0xE900, 0xEA00,
    0xEB00, 0xEC00, 0xED00, 0xEE00, 0xEF00, // Minestorm address range
];

let f19eCount = 0;
let step = 0;

while (step < maxSteps) {
    const pc = emu.e6809.e6809_get_reg_pc();
    
    // Detectar entrada a Minestorm (0xE000-0xEFFF)
    const pcRange = pc & 0xFF00;
    if (minestormAddresses.includes(pcRange) && step > 100000) {
        if (minestormEntries.length < 10) {
            minestormEntries.push({step, pc});
            console.log(`🎯 ¡ENTRADA A MINESTORM! PC=0x${pc.toString(16).toUpperCase().padStart(4, '0')} en step ${step}`);
        }
    }
    
    // Detectar llamadas a Print_Str (0xF373)
    if (pc === 0xF373) {
        if (printStrCalls.length < 5) {
            printStrCalls.push({step, pc});
            console.log(`📝 Print_Str detectado en step ${step}`);
        }
    }
    
    // Detectar si llegamos a 0x0000 (cartucho externo)
    if (pc === 0x0000 && step > 1000) {
        console.log(`❌ JAVASCRIPT llegó a 0x0000 (cartucho externo) en step ${step}`);
        break;
    }
    
    // Detectar entrada específica a Minestorm (0xE000)
    if (pc === 0xE000 && step > 1000) {
        console.log(`🎮 ¡MINESTORM INICIADO! PC=0xE000 en step ${step}`);
        // Ejecutar algunas instrucciones más para confirmar
        for (let i = 0; i < 10; i++) {
            emu.e6809.e6809_sstep();
            step++;
            const newPc = emu.e6809.e6809_get_reg_pc();
            console.log(`   Step ${step}: PC=0x${newPc.toString(16).toUpperCase().padStart(4, '0')}`);
        }
        break;
    }
    
    // Detectar bucle F19E (común en delay loops BIOS)
    if (pc === 0xF19E) {
        f19eCount++;
        if (f19eCount % 2000 === 0 && f19eCount < 20000) {
            console.log(`🔄 F19E delay loop: step ${step} (count: ${f19eCount})`);
        }
    }
    
    // Progress reporting
    if (step % 5_000_000 === 0 && step > 0) {
        console.log(`   Progress: ${step / 1_000_000} millones de steps, PC actual: 0x${pc.toString(16).toUpperCase().padStart(4, '0')}`);
    }
    
    emu.e6809.e6809_sstep();
    step++;
    
    // Si encontramos evidencia clara de Minestorm, podemos parar antes
    if (minestormEntries.length >= 3 && printStrCalls.length >= 2) {
        console.log('✅ JAVASCRIPT: Evidencia suficiente de actividad Minestorm encontrada');
        break;
    }
}

console.log('📊 RESULTADOS JAVASCRIPT:');
console.log(`   Entradas a Minestorm (0xE000-0xEFFF): ${minestormEntries.length}`);
for (const entry of minestormEntries) {
    console.log(`     Step ${entry.step}: PC=0x${entry.pc.toString(16).toUpperCase().padStart(4, '0')}`);
}
console.log(`   Llamadas Print_Str: ${printStrCalls.length}`);
for (const call of printStrCalls) {
    console.log(`     Step ${call.step}: PC=0x${call.pc.toString(16).toUpperCase().padStart(4, '0')}`);
}

console.log('');
console.log('🔬 ANÁLISIS TIMING VIA/CPU JAVASCRIPT:');
console.log(`   VIA Timer1: 0x${emu.via.via_t1_counter.toString(16).toUpperCase().padStart(4, '0')}`);
console.log(`   VIA Timer2: 0x${emu.via.via_t2_counter.toString(16).toUpperCase().padStart(4, '0')}`);
console.log(`   VIA IFR: 0x${emu.via.via_ifr.toString(16).toUpperCase().padStart(2, '0')}`);
console.log(`   VIA IER: 0x${emu.via.via_ier.toString(16).toUpperCase().padStart(2, '0')}`);
console.log(`   Steps totales ejecutados: ${step}`);
console.log(`   Bucles F19E detectados: ${f19eCount}`);

// Información de timing para comparación con Rust
console.log('');
console.log('🔍 COMPARACIÓN RUST vs JAVASCRIPT:');
console.log('RUST: Busca entrada a 0xE000 en step ????');
console.log(`JAVASCRIPT: Llegó a step ${step} buscando entrada a 0xE000`);