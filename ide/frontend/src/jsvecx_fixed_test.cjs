#!/usr/bin/env node
/**
 * jsvecx_fixed_test.cjs
 * Test de jsvecx con correcciones para Node.js
 */

const fs = require('fs');
const path = require('path');

// Crear entorno mínimo de navegador
global.window = {
  AudioContext: null,
  webkitAudioContext: null
};

global.document = {
  getElementById: () => ({ 
    getContext: () => ({
      getImageData: () => ({ data: new Array(1000) }),
      putImageData: () => {}
    })
  }),
  documentElement: {
    addEventListener: () => {}
  }
};

global.console = console;

try {
  console.log('[JSVecX Fixed Test] Loading and fixing jsvecx bundle...');
  
  // Leer el bundle y aplicar correcciones
  const bundlePath = path.join(__dirname, 'generated', 'jsvecx', 'vecx_full.js');
  let bundleCode = fs.readFileSync(bundlePath, 'utf8');
  
  // Corregir la línea problemática con length undefined
  bundleCode = bundleCode.replace(
    'const STEP2        = length; // (igual que macro original)',
    'const STEP2        = 2; // Fixed: length was undefined, using default value'
  );
  
  // Reemplazar export con CommonJS
  bundleCode = bundleCode.replace(
    'export { VecX, Globals };',
    'if (typeof module !== "undefined" && module.exports) { module.exports = { VecX, Globals }; }'
  );
  
  console.log('[JSVecX Fixed Test] Bundle corrections applied');
  
  // Ejecutar el bundle corregido
  eval(bundleCode);
  
  console.log('[JSVecX Fixed Test] Bundle executed successfully');
  
  // Verificar disponibilidad
  if (typeof VecX !== 'undefined' && typeof Globals !== 'undefined') {
    console.log('[JSVecX Fixed Test] ✓ VecX and Globals available');
    
    // Cargar BIOS
    console.log('[JSVecX Fixed Test] Loading BIOS...');
    const biosPath = path.join(__dirname, '..', 'public', 'bios.bin');
    const biosData = fs.readFileSync(biosPath);
    
    console.log(`[JSVecX Fixed Test] BIOS loaded: ${biosData.length} bytes`);
    
    // Convertir BIOS a string
    let biosString = '';
    for (let i = 0; i < biosData.length; i++) {
      biosString += String.fromCharCode(biosData[i]);
    }
    
    // Configurar globals
    Globals.romdata = biosString;
    Globals.cartdata = null;
    console.log('[JSVecX Fixed Test] ✓ BIOS configured in Globals');
    
    // Crear y configurar VecX
    const vecx = new VecX();
    console.log('[JSVecX Fixed Test] ✓ VecX instance created');
    
    // Inicializar componentes manualmente
    vecx.e6809.init(vecx);
    vecx.osint.init(vecx);
    console.log('[JSVecX Fixed Test] ✓ Components initialized');
    
    // Reset completo
    vecx.vecx_reset();
    console.log('[JSVecX Fixed Test] ✓ VecX reset completed');
    
    // Verificar estado inicial
    const initialPC = vecx.e6809.reg_pc;
    console.log(`[JSVecX Fixed Test] Initial PC: 0x${initialPC.toString(16).toUpperCase().padStart(4, '0')}`);
    
    // Verificar reset vector
    const resetVectorLo = vecx.read8(0xFFFE);
    const resetVectorHi = vecx.read8(0xFFFF);
    const resetVector = (resetVectorHi << 8) | resetVectorLo;
    console.log(`[JSVecX Fixed Test] Reset vector: 0x${resetVector.toString(16).toUpperCase().padStart(4, '0')}`);
    
    if (initialPC === resetVector) {
      console.log('[JSVecX Fixed Test] ✓ PC correctly matches reset vector');
    } else {
      console.log(`[JSVecX Fixed Test] ⚠ PC mismatch: expected 0x${resetVector.toString(16)}, got 0x${initialPC.toString(16)}`);
    }
    
    // Mostrar algunos bytes de BIOS para verificar carga
    console.log('[JSVecX Fixed Test] First 8 bytes of ROM:');
    for (let i = 0; i < 8; i++) {
      const addr = 0xE000 + i;
      const byte = vecx.read8(addr);
      console.log(`  0x${addr.toString(16).toUpperCase()}: 0x${byte.toString(16).toUpperCase().padStart(2, '0')}`);
    }
    
    // Capturar secuencia de opcodes
    console.log('\n[JSVecX Fixed Test] === OPCODE CAPTURE ===');
    console.log('┌──────┬──────┬────────┬────┬────┬──────┬──────┬──────┬──────┬────┬────┐');
    console.log('│ Step │  PC  │ Opcode │ A  │ B  │  X   │  Y   │  S   │  U   │ DP │ CC │');
    console.log('├──────┼──────┼────────┼────┼────┼──────┼──────┼──────┼──────┼────┼────┤');
    
    let successfulSteps = 0;
    for (let step = 0; step < 20; step++) {
      const pc = vecx.e6809.reg_pc;
      
      // Verificar PC válido
      if (pc < 0 || pc > 0xFFFF) {
        console.log(`│ ${step.toString().padStart(4)} │ ---- │ INVALID PC: 0x${pc.toString(16)} │`);
        break;
      }
      
      const opcode = vecx.read8(pc);
      const regA = vecx.e6809.reg_a & 0xFF;
      const regB = vecx.e6809.reg_b & 0xFF;
      const regX = vecx.e6809.reg_x.value & 0xFFFF;
      const regY = vecx.e6809.reg_y.value & 0xFFFF;
      const regS = vecx.e6809.reg_s.value & 0xFFFF;
      const regU = vecx.e6809.reg_u.value & 0xFFFF;
      const regDP = vecx.e6809.reg_dp & 0xFF;
      const regCC = vecx.e6809.reg_cc & 0xFF;
      
      console.log(
        `│ ${step.toString().padStart(4)} │ ` +
        `${pc.toString(16).toUpperCase().padStart(4, '0')} │ ` +
        `  0x${opcode.toString(16).toUpperCase().padStart(2, '0')}   │ ` +
        `${regA.toString(16).toUpperCase().padStart(2, '0')} │ ` +
        `${regB.toString(16).toUpperCase().padStart(2, '0')} │ ` +
        `${regX.toString(16).toUpperCase().padStart(4, '0')} │ ` +
        `${regY.toString(16).toUpperCase().padStart(4, '0')} │ ` +
        `${regS.toString(16).toUpperCase().padStart(4, '0')} │ ` +
        `${regU.toString(16).toUpperCase().padStart(4, '0')} │ ` +
        `${regDP.toString(16).toUpperCase().padStart(2, '0')} │ ` +
        `${regCC.toString(16).toUpperCase().padStart(2, '0')} │`
      );
      
      // Ejecutar paso de CPU
      try {
        const cycles = vecx.e6809.e6809_sstep(0, 0);
        if (cycles <= 0) {
          console.log(`│      │      │ ★ CPU returned ${cycles} cycles, stopping │`);
          break;
        }
        successfulSteps++;
      } catch (err) {
        console.log(`│      │      │ ✗ ERROR: ${err.message.substring(0, 30)}... │`);
        break;
      }
      
      // Detectar bucles infinitos simples
      if (step > 2 && vecx.e6809.reg_pc === pc) {
        console.log(`│      │      │ ⚠ PC unchanged, possible infinite loop │`);
        break;
      }
    }
    
    console.log('└──────┴──────┴────────┴────┴────┴──────┴──────┴──────┴──────┴────┴────┘');
    console.log(`\n[JSVecX Fixed Test] ✓ Captured ${successfulSteps} successful CPU steps`);
    
    if (successfulSteps > 0) {
      console.log('[JSVecX Fixed Test] 🎉 SUCCESS: jsvecx emulator running and capturing opcodes!');
    } else {
      console.log('[JSVecX Fixed Test] ⚠ WARNING: No successful CPU steps captured');
    }
    
  } else {
    console.log('[JSVecX Fixed Test] ✗ VecX or Globals not available after bundle execution');
  }
  
} catch (error) {
  console.error(`[JSVecX Fixed Test] ✗ FAILED: ${error.message}`);
  if (error.stack) {
    // Mostrar solo las líneas más relevantes del stack
    const relevantLines = error.stack.split('\n')
      .filter(line => line.includes('jsvecx') || line.includes('eval') || line.includes('anonymous'))
      .slice(0, 4);
    console.error('[JSVecX Fixed Test] Stack trace:', relevantLines.join('\n'));
  }
  process.exit(1);
}