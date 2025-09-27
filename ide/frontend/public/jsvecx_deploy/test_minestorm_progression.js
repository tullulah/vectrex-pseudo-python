// Test mínimo para jsvecx en Node.js
const fs = require('fs');
const path = require('path');
const vm = require('vm');

// Cargar los archivos fuente del emulador
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

// Cargar BIOS
const biosPath = path.join(baseDir, '../../roms/fastrom.dat');
const biosData = fs.readFileSync(biosPath);

console.log('=== TEST PROGRESIÓN JAVASCRIPT HACIA MINESTORM ===');

// Instanciar el emulador
const emu = new VecX();
emu.osint.osint_render = function() {}; // No-op en Node.js

// Inicializar arrays de vectores
for (let i = 0; i < emu.vectors_draw.length; ++i) {
  emu.vectors_draw[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}
for (let i = 0; i < emu.vectors_erse.length; ++i) {
  emu.vectors_erse[i] = {x0:0, y0:0, x1:0, y1:0, color:0};
}

// Asignar referencias cruzadas
emu.e6809.vecx = emu;
emu.osint.vecx = emu;

// Cargar BIOS (sin cartucho para arranque automático de Minestorm)
for (let i = 0; i < biosData.length && i < emu.rom.length; ++i) {
  emu.rom[i] = biosData[i];
}

// Resetear emulador
emu.e6809.e6809_reset();

let steps = 0;
const maxSteps = 25_000_000; // 25M steps to see all BIOS loops
let copyright_reached = false;
let f4eb_reached = false;
let f4eb_exits = 0;
let wait_recal_reached = false;
let minestorm_reached = false;
let bios_init_complete = false;
let f4eb_last_b = -1;

let last_pc = 0;
let stuck_count = 0;

// Patch para capturar ejecución
const orig_sstep = emu.e6809.e6809_sstep.bind(emu.e6809);
emu.e6809.e6809_sstep = function(irq_i, irq_f) {
    const pc = this.reg_pc;
    
    // Detectar si está atascado (solo advertir, no interrumpir)
    if (pc === last_pc) {
        stuck_count++;
        if (stuck_count === 1000000) { // Solo mostrar mensaje una vez por loop
            console.log(`⚠️  LOOP LARGO detectado en $${pc.toString(16).toUpperCase().padStart(4, '0')} (paso ${steps})`);
        }
    } else {
        if (stuck_count > 1000000) {
            console.log(`   ✅ Salió del loop largo después de ${stuck_count} iteraciones`);
        }
        last_pc = pc;
        stuck_count = 0;
    }
    
    // Detectar puntos críticos
    switch (pc) {
        case 0xF151:
            if (!copyright_reached) {
                console.log(`📄 COPYRIGHT alcanzado (paso ${steps})`);
                copyright_reached = true;
            }
            break;
            
        case 0xF4EB:
            if (!f4eb_reached) {
                console.log(`🔄 LOOP F4EB alcanzado (paso ${steps}) - B=${this.reg_b.toString(16).toUpperCase()}`);
                f4eb_reached = true;
                f4eb_last_b = this.reg_b;
            }
            // Detectar salida del loop
            if (this.reg_b === 0 && f4eb_last_b !== 0) {
                f4eb_exits++;
                console.log(`   🚪 Salida #${f4eb_exits} del loop F4EB (B=0)`);
            }
            f4eb_last_b = this.reg_b;
            break;
            
        case 0xF19E:
            if (!wait_recal_reached) {
                console.log(`⏳ WAIT_RECAL loop alcanzado (paso ${steps})`);
                wait_recal_reached = true;
            }
            break;
            
        case 0xF18B:
            if (!bios_init_complete) {
                console.log(`🎯 BIOS Init_OS completo (paso ${steps})`);
                bios_init_complete = true;
            }
            break;
    }
    
    // Detectar Minestorm (dirección en RAM)
    if (pc >= 0x0000 && pc <= 0x7FFF && pc !== 0x0000 && !minestorm_reached) {
        console.log(`🎮 MINESTORM ENTRY en $${pc.toString(16).toUpperCase().padStart(4, '0')} (paso ${steps})`);
        minestorm_reached = true;
        throw new Error('Minestorm alcanzado'); // Parar aquí
    }
    
    // Progress report cada 1M steps
    if (steps % 1000000 === 0 && steps > 0) {
        console.log(`📊 Progreso: ${steps} steps, PC=$${pc.toString(16).toUpperCase().padStart(4, '0')}`);
    }
    
    steps++;
    if (steps >= maxSteps) {
        throw new Error('Máximo de steps alcanzado');
    }
    
    return orig_sstep(irq_i, irq_f);
};

try {
    // Ejecutar hasta que se alcance alguna condición de parada
    while (true) {
        emu.vecx_emu(1, 0); // Un frame
    }
} catch (e) {
    console.log(`\nEjecución detenida: ${e.message}`);
}

console.log('\n=== RESUMEN PROGRESIÓN JAVASCRIPT ===');
console.log(`Copyright alcanzado: ${copyright_reached}`);
console.log(`Loop F4EB alcanzado: ${f4eb_reached}`);
console.log(`Salidas del loop F4EB: ${f4eb_exits}`);
console.log(`Wait_Recal alcanzado: ${wait_recal_reached}`);
console.log(`BIOS Init completo: ${bios_init_complete}`);
console.log(`Minestorm alcanzado: ${minestorm_reached}`);
console.log(`PC final: $${emu.e6809.reg_pc.toString(16).toUpperCase().padStart(4, '0')}`);
console.log(`Total pasos: ${steps}`);