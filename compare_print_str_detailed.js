// Comparación detallada Print_Str: Rust vs JSVecx
// Este script traza paso a paso la ejecución hasta Print_Str para
// identificar dónde divergen las coordenadas X

const fs = require('fs');
const path = require('path');
const vm = require('vm');

// 1. Cargar JSVecx usando el mismo patrón que test_vecx_node.js
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

const baseDir = path.join(__dirname, 'ide/frontend/public/jsvecx_deploy');
loadJSGlobal(path.join(baseDir, 'globals.js'), ['Globals']);
loadJSGlobal(path.join(baseDir, 'utils.js'), ['utils']);
loadJSGlobal(path.join(baseDir, 'e6809.js'), ['e6809']);
loadJSGlobal(path.join(baseDir, 'e8910.js'), ['e8910']);
loadJSGlobal(path.join(baseDir, 'vector_t.js'), ['vector_t']);
loadJSGlobal(path.join(baseDir, 'osint.js'), ['osint']);
loadJSGlobal(path.join(baseDir, 'vecx.js'), ['VecX']);

// 2. Inicializar emulador
const vecx = new VecX();

// Mock osint_render para Node.js
vecx.osint.osint_render = function() { /* No-op en Node.js */ };
vecx.osint.osint_clearscreen = function() { /* No-op en Node.js */ };

// Inicializar arrays de vectores (necesario para Node.js)
for (let i = 0; i < vecx.vectors_draw.length; ++i) {
  vecx.vectors_draw[i] = new vector_t();
}
for (let i = 0; i < vecx.vectors_erse.length; ++i) {
  vecx.vectors_erse[i] = new vector_t();
}

// Asignar referencias cruzadas
vecx.e6809.vecx = vecx;
vecx.osint.vecx = vecx;

// 3. Cargar BIOS
const biosPath = path.join(__dirname, 'ide/frontend/public/bios.bin');
const biosData = fs.readFileSync(biosPath);

// Cargar BIOS en rom array
for (let i = 0; i < biosData.length && i < vecx.rom.length; ++i) {
  vecx.rom[i] = biosData[i];
}

// Inicializar Globals.romdata para que vecx_reset funcione
Globals.romdata = new Uint8Array(0); // Sin cartucho, solo BIOS (ejecutará Minestorm)

console.log(`BIOS loaded: ${biosData.length} bytes`);

// 4. Reset y comenzar
vecx.reset();

// 5. Rastrear valores de VIA Port A (DAC X) hasta Print_Str
const TARGET_PC = 0xF495; // Print_Str entry point
const PRINT_STR_NEG = 0xF4D4; // NEG <VIA_port_a en Print_Str
const VIA_PORT_A = 0xD000;

let step = 0;
let reachedPrintStr = false;
let portAValues = [];
let lastPC = 0;

console.log("\n=== Tracing to Print_Str ===");
console.log("Step | PC     | VIA_portA | Instruction");
console.log("-----|--------|-----------|------------");

while (step < 10000 && !reachedPrintStr) {
    const pc = vecx.e6809.reg_pc;
    const portA = vecx.ram[VIA_PORT_A];
    
    // Detectar cambio en PC
    if (pc !== lastPC) {
        const opcode = vecx.ram[pc] || vecx.rom[pc - 0xE000] || 0;
        
        // Log si estamos cerca de Print_Str o si portA cambia
        if (pc >= 0xF490 && pc <= 0xF510) {
            console.log(`${step.toString().padStart(4)} | ${pc.toString(16).toUpperCase().padStart(6)} | 0x${portA.toString(16).toUpperCase().padStart(2, '0')} (${ portA > 127 ? portA - 256 : portA}) | [0x${opcode.toString(16).toUpperCase().padStart(2, '0')}]`);
        }
        
        if (pc === TARGET_PC) {
            console.log(`\n*** REACHED Print_Str at step ${step} ***`);
            console.log(`    VIA_portA = 0x${portA.toString(16).toUpperCase()} (signed: ${portA > 127 ? portA - 256 : portA})\n`);
            reachedPrintStr = true;
        }
        
        if (pc === PRINT_STR_NEG) {
            console.log(`\n*** BEFORE NEG <VIA_portA> at step ${step} ***`);
            console.log(`    VIA_portA = 0x${portA.toString(16).toUpperCase()} (signed: ${portA > 127 ? portA - 256 : portA})\n`);
        }
        
        lastPC = pc;
    }
    
    // Ejecutar un ciclo
    vecx.e6809.e6809_sstep(0, 0); // Execute one instruction
    step++;
}

if (reachedPrintStr) {
    console.log("\n=== Execution reached Print_Str successfully ===");
    console.log(`Total steps: ${step}`);
} else {
    console.log(`\n=== Did not reach Print_Str after ${step} steps ===`);
    console.log(`Last PC: 0x${lastPC.toString(16).toUpperCase()}`);
}

// Instrucciones para usuario
console.log("\n=== NEXT STEPS ===");
console.log("1. Run Rust emulator with same trace points");
console.log("2. Compare VIA_portA values at:");
console.log("   - Print_Str entry (0xF495)");
console.log("   - Before NEG instruction (0xF4D4)");
console.log("3. Identify first divergence point");
console.log("\nExpected Rust VIA_portA at F495: Compare with JSVecx output above");
