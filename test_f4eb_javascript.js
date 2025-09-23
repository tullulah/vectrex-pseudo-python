// Test específico para analizar el bucle F4EB en el emulador JavaScript
// Basado en test_vecx_node.js pero enfocado en detectar el comportamiento en F4EB

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
console.log('🔧 Cargando emulador JavaScript desde:', baseDir);

try {
  loadJSGlobal(path.join(baseDir, 'globals.js'), ['Globals']);
  loadJSGlobal(path.join(baseDir, 'utils.js'), ['utils']);
  loadJSGlobal(path.join(baseDir, 'e6809.js'), ['e6809']);
  loadJSGlobal(path.join(baseDir, 'e8910.js'), ['e8910']);
  loadJSGlobal(path.join(baseDir, 'osint.js'), ['osint']);
  loadJSGlobal(path.join(baseDir, 'vecx.js'), ['VecX']);
} catch (e) {
  console.error('❌ Error cargando archivos JavaScript:', e.message);
  process.exit(1);
}

// Cargar BIOS real (misma que usa Rust)
const biosPath = 'C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\src\\assets\\bios.bin';
console.log('🎯 Cargando BIOS desde:', biosPath);

let biosData;
try {
  biosData = fs.readFileSync(biosPath);
  console.log('✅ BIOS cargada,', biosData.length, 'bytes');
} catch (e) {
  console.error('❌ Error cargando BIOS:', e.message);
  process.exit(1);
}

// Instanciar el emulador
const emu = new VecX();
emu.osint.osint_render = function() { /* No-op en Node.js */ };

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

// Cargar BIOS (sin cartucho para que arranque Minestorm automáticamente)
for (let i = 0; i < biosData.length && i < emu.rom.length; ++i) {
  emu.rom[i] = biosData[i];
}

console.log('🚀 Iniciando emulación hasta llegar a F4EB...');

// Variables de control
let stepCount = 0;
let f4ebDetected = false;
let f4ebState = null;
const maxSteps = 10000;

// Hook para capturar llegada a F4EB
const orig_sstep = emu.e6809.e6809_sstep.bind(emu.e6809);
emu.e6809.e6809_sstep = function(irq_i, irq_f) {
  stepCount++;
  
  if (this.reg_pc === 0xF4EB && !f4ebDetected) {
    f4ebDetected = true;
    console.log('🎯 ¡LLEGAMOS A F4EB! Step:', stepCount);
    
    // Capturar estado completo
    f4ebState = {
      step: stepCount,
      pc: this.reg_pc,
      dp: this.reg_dp,
      a: this.reg_a,
      b: this.reg_b,
      x: this.reg_x.value,
      y: this.reg_y.value,
      cc: this.reg_cc,
      // Leer valor en 0xD05A
      d05a_value: typeof this.vecx.read8 === 'function' ? this.vecx.read8(0xD05A) : undefined,
      // Leer registros VIA
      via_registers: {}
    };
    
    // Capturar registros VIA
    for (let reg = 0x00; reg <= 0x0F; reg++) {
      const addr = 0xD000 + reg;
      f4ebState.via_registers[`0x${addr.toString(16).toUpperCase()}`] = 
        typeof this.vecx.read8 === 'function' ? this.vecx.read8(addr) : undefined;
    }
    
    console.log('📊 JAVASCRIPT EMULATOR - Estado en F4EB:');
    console.log('   PC:', f4ebState.pc.toString(16).toUpperCase());
    console.log('   DP:', f4ebState.dp.toString(16).toUpperCase());
    console.log('   A:', f4ebState.a.toString(16).toUpperCase(), 'B:', f4ebState.b.toString(16).toUpperCase());
    console.log('   X:', f4ebState.x.toString(16).toUpperCase(), 'Y:', f4ebState.y.toString(16).toUpperCase());
    console.log('   CC:', f4ebState.cc.toString(16).toUpperCase());
    console.log('   Steps para llegar:', f4ebState.step);
    console.log('   Valor en 0xD05A:', f4ebState.d05a_value !== undefined ? '0x' + f4ebState.d05a_value.toString(16).toUpperCase() : 'undefined');
    
    console.log('   Registros VIA:');
    for (const [addr, value] of Object.entries(f4ebState.via_registers)) {
      const regNum = parseInt(addr.slice(3), 16) - 0xD000;
      console.log(`     ${addr} (reg ${regNum.toString(16).toUpperCase().padStart(2, '0')}): 0x${value !== undefined ? value.toString(16).toUpperCase() : 'undefined'}`);
    }
  }
  
  // Después de F4EB, analizar comportamiento del bucle
  if (f4ebDetected && stepCount <= f4ebState.step + 10) {
    const instruction_analysis = stepCount - f4ebState.step;
    console.log(`🔄 JS - Bucle iter ${instruction_analysis}:`);
    
    if (this.reg_pc === 0xF4EB) {
      console.log(`   LDA #$81 → A será 0x81`);
    } else if (this.reg_pc === 0xF4ED) {
      console.log(`   STX <$5A → X=0x${this.reg_x.value.toString(16).toUpperCase()} escribiendo en 0xD05A`);
    } else if (this.reg_pc === 0xF4EF) {
      const d05a_after = typeof this.vecx.read8 === 'function' ? this.vecx.read8(0xD05A) : undefined;
      console.log(`   BNE → leyendo 0xD05A=0x${d05a_after !== undefined ? d05a_after.toString(16).toUpperCase() : 'undefined'}`);
      
      if (d05a_after === 0x81) {
        console.log(`   ✅ JS: Bucle TERMINARÁ (valor correcto 0x81)`);
      } else {
        console.log(`   ⚠️  JS: Bucle CONTINUARÁ (valor incorrecto, esperaba 0x81)`);
      }
    }
  }
  
  if (stepCount > maxSteps) {
    throw new Error(`Máximo de steps alcanzado (${maxSteps})`);
  }
  
  return orig_sstep(irq_i, irq_f);
};

// Ejecutar emulación
try {
  while (!f4ebDetected && stepCount < maxSteps) {
    emu.vecx_emu(100, 0); // Bloques pequeños para control fino
  }
  
  if (f4ebDetected) {
    // Ejecutar algunas iteraciones más para analizar el bucle
    console.log('🔍 Analizando comportamiento del bucle...');
    emu.vecx_emu(50, 0); // Unas pocas iteraciones más
  }
  
} catch (e) {
  console.error('🛑 Emulación detenida:', e.message);
}

console.log('\n📋 RESUMEN COMPARACIÓN:');
console.log('='.repeat(50));

if (f4ebDetected) {
  console.log('✅ JavaScript emulator llegó a F4EB');
  console.log(`📈 Steps necesarios: ${f4ebState.step}`);
  console.log(`🎯 Valor en 0xD05A: 0x${f4ebState.d05a_value !== undefined ? f4ebState.d05a_value.toString(16).toUpperCase() : 'undefined'}`);
  
  // Comparar con resultados de Rust
  console.log('\n🔬 COMPARACIÓN RUST vs JAVASCRIPT:');
  console.log('RUST Results (from previous test):');
  console.log('   PC: 0xF4EB');  
  console.log('   DP: 0xD0');
  console.log('   A: 0x0F, B: 0xDC');
  console.log('   X: 0xCBE6, Y: 0x0000');
  console.log('   IFR: 0x60, IER: 0x00');
  console.log('   Steps: 1524');
  console.log('   Valor en 0xD05A: 0xFF');
  console.log('   Resultado: BUCLE INFINITO');
  
  console.log('\nJAVASCRIPT Results:');
  console.log(`   PC: 0x${f4ebState.pc.toString(16).toUpperCase()}`);
  console.log(`   DP: 0x${f4ebState.dp.toString(16).toUpperCase()}`);
  console.log(`   A: 0x${f4ebState.a.toString(16).toUpperCase()}, B: 0x${f4ebState.b.toString(16).toUpperCase()}`);
  console.log(`   X: 0x${f4ebState.x.toString(16).toUpperCase()}, Y: 0x${f4ebState.y.toString(16).toUpperCase()}`);
  console.log(`   Steps: ${f4ebState.step}`);
  console.log(`   Valor en 0xD05A: 0x${f4ebState.d05a_value !== undefined ? f4ebState.d05a_value.toString(16).toUpperCase() : 'undefined'}`);
  
  // Conclusión
  if (f4ebState.d05a_value === 0xFF) {
    console.log('\n🔴 CONCLUSIÓN: JavaScript también lee 0xFF desde 0xD05A');
    console.log('   Esto confirma que es comportamiento esperado del sistema');
    console.log('   La dirección 0xD05A no está mapeada correctamente en AMBOS emuladores');
  } else if (f4ebState.d05a_value === 0x81) {
    console.log('\n🟡 CONCLUSIÓN: JavaScript lee 0x81 desde 0xD05A');
    console.log('   Esto indica que Rust tiene un bug en el mapeo de memoria');
  } else {
    console.log('\n🟠 CONCLUSIÓN: JavaScript lee valor diferente');
    console.log('   Ambos emuladores pueden tener comportamientos diferentes');
  }
  
} else {
  console.log('❌ JavaScript emulator NO llegó a F4EB en', maxSteps, 'steps');
  console.log('   Esto indica comportamiento muy diferente entre emuladores');
}

console.log('\n💡 RECOMENDACIÓN:');
if (f4ebDetected && f4ebState.d05a_value === 0xFF) {
  console.log('   - Investigar mapeo de memoria en rango 0xD050-0xD05F');
  console.log('   - Verificar si 0xD05A debería mapear a RAM específica');
  console.log('   - Comprobar documentación de hardware Vectrex');
} else if (f4ebDetected && f4ebState.d05a_value !== 0xFF) {
  console.log('   - Corregir mapeo de memoria en emulador Rust');
  console.log('   - Implementar comportamiento correcto para 0xD05A');
} else {
  console.log('   - Verificar diferencias en emulación inicial de BIOS');
  console.log('   - Comprobar Timer1/Timer2 y VIA en JavaScript');
}