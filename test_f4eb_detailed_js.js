// Test más detallado para ver dónde se queda el emulador JavaScript
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

// Direcciones importantes de Minestorm en BIOS
const minestormAddresses = [
    0xF000, 0xF100, 0xF200, 0xF300, 0xF400, 0xF500, 0xF600, 0xF700, 0xF800, 0xF900, 0xFA00,
    0xE000, 0xE100, 0xE200, 0xE300, 0xE400, 0xE500, 0xE600, 0xE700, 0xE800, 0xE900,
];

let f19eCount = 0;
let step = 0;

while (step < maxSteps) {
    const pc = emu.e6809.e6809_get_reg_pc();
    
    // Detectar entrada a posibles rutinas de Minestorm
    const pcRange = pc & 0xFF00;
    if (minestormAddresses.includes(pcRange) && step > 100000) {
        if (minestormEntries.length < 10) {
            minestormEntries.push({step, pc});
            console.log(`🎯 Posible entrada Minestorm: PC=0x${pc.toString(16).toUpperCase().padStart(4, '0')} en step ${step}`);
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
    
    // Detectar bucle F19E (común en delay loops BIOS)
    if (pc === 0xF19E) {
        f19eCount++;
        if (f19eCount % 1000 === 0 && f19eCount < 10000) {
            console.log(`🎯 F19E detectado en step ${step} (count: ${f19eCount})`);
        }
    }
    
    // Progress reporting
    if (step % 5_000_000 === 0 && step > 0) {
        console.log(`   Progress: ${step / 1_000_000} millones de steps, PC actual: 0x${pc.toString(16).toUpperCase().padStart(4, '0')}`);
    }
    
    emu.e6809.e6809_sstep();
    step++;
    
    // Si encontramos evidencia clara de Minestorm, podemos parar antes
    if (minestormEntries.length >= 5 && printStrCalls.length >= 2) {
        console.log('✅ JAVASCRIPT: Evidencia suficiente de actividad Minestorm encontrada');
        break;
    }
}

console.log('📊 RESULTADOS JAVASCRIPT:');
console.log(`   Posibles entradas Minestorm: ${minestormEntries.length}`);
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

// Cargar BIOS
for (let i = 0; i < biosData.length && i < emu.rom.length; ++i) {
  emu.rom[i] = biosData[i];
}

console.log('🚀 Comenzando análisis...');

let stepCount = 0;
let lastPC = -1;
let pcChanges = 0;
const pcHistory = [];
const maxSteps = 25000000; // 25 millones de steps igual que Rust

// Hook detallado
const orig_sstep = emu.e6809.e6809_sstep.bind(emu.e6809);
emu.e6809.e6809_sstep = function(irq_i, irq_f) {
  stepCount++;
  
  // Detectar cambios de PC
  if (this.reg_pc !== lastPC) {
    pcChanges++;
    lastPC = this.reg_pc;
    pcHistory.push({step: stepCount, pc: this.reg_pc});
  }
  
  // Log periódico
  if (stepCount % 1000000 === 0) {
    console.log(`📍 Step ${stepCount/1000000}M: PC=0x${this.reg_pc.toString(16).toUpperCase()} (${pcChanges} cambios PC)`);
  }
  
  // Detectar bucles importantes
  if (this.reg_pc === 0xF19E) {
    console.log(`🎯 F19E detectado en step ${stepCount}`);
  }
  if (this.reg_pc === 0xF4EB) {
    console.log(`🔴 F4EB detectado en step ${stepCount}`);
    
    // Ejecutar test extendido igual que Rust
    console.log('🚀 EJECUTANDO TEST EXTENDIDO (25M cycles):');
    console.log('   Verificando si JavaScript eventualmente sale del bucle F4EB...');
    
    let extendedSteps = 0;
    const maxExtendedSteps = 25000000;
    
    // Capturar estado inicial
    console.log('📊 JAVASCRIPT - Estado en F4EB:');
    console.log(`   PC: 0x${this.reg_pc.toString(16).toUpperCase()}`);
    console.log(`   DP: 0x${this.reg_dp.toString(16).toUpperCase()}`);
    console.log(`   A: 0x${this.reg_a.toString(16).toUpperCase()}, B: 0x${this.reg_b.toString(16).toUpperCase()}`);
    console.log(`   X: 0x${this.reg_x.value.toString(16).toUpperCase()}, Y: 0x${this.reg_y.value.toString(16).toUpperCase()}`);
    
    const d05a_value = typeof this.vecx.read8 === 'function' ? this.vecx.read8(0xD05A) : undefined;
    console.log(`   Valor en 0xD05A: 0x${d05a_value !== undefined ? d05a_value.toString(16).toUpperCase() : 'undefined'}`);
    
    // Test extendido
    while (this.reg_pc === 0xF4EB && extendedSteps < maxExtendedSteps) {
      orig_sstep(irq_i, irq_f);
      extendedSteps++;
      
      // Progress report cada millón
      if (extendedSteps % 1000000 === 0) {
        const timer2_val = typeof this.vecx.read8 === 'function' ? this.vecx.read8(0xD05A) : undefined;
        console.log(`   Progress: ${extendedSteps/1000000}M steps, PC=0x${this.reg_pc.toString(16).toUpperCase()}, Timer2=0x${timer2_val !== undefined ? timer2_val.toString(16).toUpperCase() : 'undefined'}`);
      }
    }
    
    if (this.reg_pc !== 0xF4EB) {
      console.log(`   🎉 ¡JAVASCRIPT SALIÓ DEL BUCLE! PC=0x${this.reg_pc.toString(16).toUpperCase()} después de ${extendedSteps} steps adicionales`);
      const final_timer2 = typeof this.vecx.read8 === 'function' ? this.vecx.read8(0xD05A) : undefined;
      console.log(`   Timer2 final: 0x${final_timer2 !== undefined ? final_timer2.toString(16).toUpperCase() : 'undefined'}`);
    } else {
      console.log(`   ⚠️ JavaScript se queda en bucle F4EB después de ${extendedSteps} steps adicionales`);
    }
    
    throw new Error('F4EB alcanzado - terminando análisis');
  }
  
  if (stepCount > maxSteps) {
    throw new Error(`Máximo de steps alcanzado (${maxSteps/1000000}M)`);
  }
  
  return orig_sstep(irq_i, irq_f);
};

// Ejecutar emulación
try {
  while (stepCount < maxSteps) {
    emu.vecx_emu(100, 0);
  }
} catch (e) {
  if (e.message === 'F4EB alcanzado - terminando análisis') {
    console.log('✅ Análisis completado en F4EB');
  } else {
    console.log('🛑 Emulación detenida:', e.message);
  }
}

console.log('\n📈 ESTADÍSTICAS:');
console.log(`Steps totales: ${stepCount}`);
console.log(`Cambios de PC: ${pcChanges}`);
console.log(`Últimos 10 PCs visitados:`);

const recentPCs = pcHistory.slice(-10);
for (const {step, pc} of recentPCs) {
  console.log(`  Step ${step}: 0x${pc.toString(16).toUpperCase()}`);
}

// Analizar si se queda en bucle
if (pcHistory.length > 0) {
  const lastPC = pcHistory[pcHistory.length - 1].pc;
  const lastPCCount = pcHistory.filter(entry => entry.pc === lastPC).length;
  
  if (lastPCCount > 100) {
    console.log(`⚠️  POSIBLE BUCLE INFINITO en PC=0x${lastPC.toString(16).toUpperCase()} (${lastPCCount} veces)`);
  }
}

console.log('\n🔍 COMPARACIÓN RUST vs JAVASCRIPT:');
console.log('RUST: Llega a F4EB en step 1525');
console.log(`JAVASCRIPT: ${stepCount >= maxSteps ? 'NO llega a F4EB' : 'Llega a F4EB'} en ${stepCount} steps`);

if (stepCount >= maxSteps) {
  console.log('\n💡 El emulador JavaScript parece comportarse de manera muy diferente.');
  console.log('   Posibles causas:');
  console.log('   - Timer1/Timer2 implementación diferente');
  console.log('   - VIA 6522 comportamiento diferente'); 
  console.log('   - Bucle infinito en lugar diferente');
  console.log('   - Inicialización de hardware diferente');
}