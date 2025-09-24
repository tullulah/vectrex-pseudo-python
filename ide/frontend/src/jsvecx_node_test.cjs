#!/usr/bin/env node
/**
 * jsvecx_node_test.cjs
 * Test de jsvecx adaptado para Node.js con bundle modificado
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
  console.log('[JSVecX Node Test] Loading and adapting jsvecx bundle...');
  
  // Leer el bundle y modificarlo para CommonJS
  const bundlePath = path.join(__dirname, 'generated', 'jsvecx', 'vecx_full.js');
  let bundleCode = fs.readFileSync(bundlePath, 'utf8');
  
  // Reemplazar la línea export con asignación a module.exports
  bundleCode = bundleCode.replace(
    'export { VecX, Globals };',
    'if (typeof module !== "undefined" && module.exports) { module.exports = { VecX, Globals }; }'
  );
  
  // Ejecutar el bundle modificado
  eval(bundleCode);
  
  console.log('[JSVecX Node Test] Bundle executed successfully');
  
  // Verificar que las clases están disponibles globalmente
  if (typeof VecX !== 'undefined' && typeof Globals !== 'undefined') {
    console.log('[JSVecX Node Test] VecX and Globals available');
    
    // Cargar BIOS
    console.log('[JSVecX Node Test] Loading BIOS...');
    const biosPath = path.join(__dirname, '..', 'public', 'bios.bin');
    const biosData = fs.readFileSync(biosPath);
    
    console.log(`[JSVecX Node Test] BIOS loaded: ${biosData.length} bytes`);
    
    // Convertir BIOS a string para jsvecx
    let biosString = '';
    for (let i = 0; i < biosData.length; i++) {
      biosString += String.fromCharCode(biosData[i]);
    }
    
    // Configurar globals
    Globals.romdata = biosString;
    Globals.cartdata = null; // Sin cartridge
    console.log('[JSVecX Node Test] BIOS configured in Globals');
    
    // Crear instancia de VecX
    const vecx = new VecX();
    console.log('[JSVecX Node Test] VecX instance created');
    
    // Inicializar componentes
    vecx.e6809.init(vecx);
    vecx.osint.init(vecx);
    console.log('[JSVecX Node Test] Components initialized');
    
    // Reset del sistema
    vecx.vecx_reset();
    console.log('[JSVecX Node Test] VecX reset completed');
    
    // Verificar PC inicial
    const initialPC = vecx.e6809.reg_pc;
    console.log(`[JSVecX Node Test] Initial PC: 0x${initialPC.toString(16).toUpperCase().padStart(4, '0')}`);
    
    // Verificar que el reset vector se leyó correctamente
    const resetVectorLo = vecx.read8(0xFFFE);
    const resetVectorHi = vecx.read8(0xFFFF);
    const resetVector = (resetVectorHi << 8) | resetVectorLo;
    console.log(`[JSVecX Node Test] Reset vector at 0xFFFE-0xFFFF: 0x${resetVector.toString(16).toUpperCase().padStart(4, '0')}`);
    
    if (initialPC === resetVector) {
      console.log('[JSVecX Node Test] ✓ PC correctly set to reset vector');
    } else {
      console.log(`[JSVecX Node Test] ✗ PC mismatch! Expected 0x${resetVector.toString(16)}, got 0x${initialPC.toString(16)}`);
    }
    
    // Capturar primeros opcodes
    console.log('\n[JSVecX Node Test] Capturing first 15 opcodes...');
    console.log('┌──────┬──────┬────────┬────┬────┬──────┬──────┬──────┬──────┬────┬────┐');
    console.log('│ Step │  PC  │ Opcode │ A  │ B  │  X   │  Y   │  S   │  U   │ DP │ CC │');
    console.log('├──────┼──────┼────────┼────┼────┼──────┼──────┼──────┼──────┼────┼────┤');
    
    for (let step = 0; step < 15; step++) {
      const pc = vecx.e6809.reg_pc;
      
      // Verificar que PC está en rango válido
      if (pc < 0 || pc > 0xFFFF) {
        console.log(`│ ${step.toString().padStart(4)} │ INVALID PC: 0x${pc.toString(16)} │`);
        break;
      }
      
      const opcode = vecx.read8(pc);
      const regA = vecx.e6809.reg_a;
      const regB = vecx.e6809.reg_b;
      const regX = vecx.e6809.reg_x.value;
      const regY = vecx.e6809.reg_y.value;
      const regS = vecx.e6809.reg_s.value;
      const regU = vecx.e6809.reg_u.value;
      const regDP = vecx.e6809.reg_dp;
      const regCC = vecx.e6809.reg_cc;
      
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
      
      // Ejecutar un paso de CPU
      try {
        const cycles = vecx.e6809.e6809_sstep(0, 0);
        if (cycles === 0) {
          console.log('│      │      │ ★ CPU returned 0 cycles, stopping');
          break;
        }
      } catch (err) {
        console.log(`│      │      │ ✗ Error: ${err.message}`);
        break;
      }
    }
    
    console.log('└──────┴──────┴────────┴────┴────┴──────┴──────┴──────┴──────┴────┴────┘');
    console.log('\n[JSVecX Node Test] ✓ Opcode capture completed successfully');
    
  } else {
    console.log('[JSVecX Node Test] ✗ VecX or Globals not found after bundle execution');
    console.log('[JSVecX Node Test] Available globals:', Object.keys(global).filter(k => !k.startsWith('_') && k.length < 20));
  }
  
} catch (error) {
  console.error(`[JSVecX Node Test] ✗ Failed: ${error.message}`);
  if (error.stack) {
    const relevantStack = error.stack.split('\n')
      .filter(line => line.includes('jsvecx') || line.includes('eval'))
      .slice(0, 3);
    console.error('[JSVecX Node Test] Relevant stack:', relevantStack.join('\n'));
  }
  process.exit(1);
}