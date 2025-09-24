#!/usr/bin/env node
/**
 * jsvecx_simple_test.cjs
 * Prueba simple de opcode capture usando approach directo
 */

const fs = require('fs');
const path = require('path');

// Crear entorno mínimo de navegador
global.window = {
  AudioContext: null,
  webkitAudioContext: null
};

global.document = {
  getElementById: () => null,
  documentElement: {
    addEventListener: () => {}
  }
};

global.console = console;

try {
  console.log('[JSVecX Simple Test] Loading jsvecx bundle...');
  
  // Cargar el bundle directamente como string y evaluarlo
  const bundlePath = path.join(__dirname, 'generated', 'jsvecx', 'vecx_full.js');
  const bundleCode = fs.readFileSync(bundlePath, 'utf8');
  
  // Ejecutar el bundle en el contexto actual
  eval(bundleCode);
  
  console.log('[JSVecX Simple Test] Bundle loaded successfully');
  
  // Verificar que las clases principales están disponibles
  if (typeof VecX !== 'undefined') {
    console.log('[JSVecX Simple Test] VecX class available');
    
    // Intentar crear instancia
    const vecx = new VecX();
    console.log('[JSVecX Simple Test] VecX instance created');
    
    // Cargar BIOS
    console.log('[JSVecX Simple Test] Loading BIOS...');
    const biosPath = path.join(__dirname, '..', 'public', 'bios.bin');
    const biosData = fs.readFileSync(biosPath);
    
    console.log(`[JSVecX Simple Test] BIOS loaded: ${biosData.length} bytes`);
    
    // Convertir BIOS a string para compatibilidad
    let biosString = '';
    for (let i = 0; i < biosData.length; i++) {
      biosString += String.fromCharCode(biosData[i]);
    }
    
    // Configurar global para jsvecx
    if (typeof Globals !== 'undefined') {
      Globals.romdata = biosString;
      console.log('[JSVecX Simple Test] BIOS configured in Globals');
    }
    
    // Reset del sistema
    vecx.vecx_reset();
    console.log('[JSVecX Simple Test] VecX reset completed');
    
    // Obtener estado inicial
    const initialPC = vecx.e6809.reg_pc;
    console.log(`[JSVecX Simple Test] Initial PC: 0x${initialPC.toString(16).toUpperCase()}`);
    
    // Capturar primeros opcodes
    console.log('[JSVecX Simple Test] Capturing first 10 opcodes...');
    console.log('Step | PC   | Opcode | A  | B  | X    | Y    | S    | U    | DP | CC');
    console.log('-----|------|--------|----|----|------|------|------|------|----|----|');
    
    for (let step = 0; step < 10; step++) {
      const pc = vecx.e6809.reg_pc;
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
        `${step.toString().padStart(4)} | ` +
        `${pc.toString(16).toUpperCase().padStart(4)} | ` +
        `${opcode.toString(16).toUpperCase().padStart(6)} | ` +
        `${regA.toString(16).toUpperCase().padStart(2)} | ` +
        `${regB.toString(16).toUpperCase().padStart(2)} | ` +
        `${regX.toString(16).toUpperCase().padStart(4)} | ` +
        `${regY.toString(16).toUpperCase().padStart(4)} | ` +
        `${regS.toString(16).toUpperCase().padStart(4)} | ` +
        `${regU.toString(16).toUpperCase().padStart(4)} | ` +
        `${regDP.toString(16).toUpperCase().padStart(2)} | ` +
        `${regCC.toString(16).toUpperCase().padStart(2)}`
      );
      
      // Ejecutar un paso
      try {
        const cycles = vecx.e6809.e6809_sstep(0, 0);
        if (cycles === 0) {
          console.log('[JSVecX Simple Test] CPU returned 0 cycles, stopping');
          break;
        }
      } catch (err) {
        console.log(`[JSVecX Simple Test] Error executing step ${step}: ${err.message}`);
        break;
      }
    }
    
    console.log('[JSVecX Simple Test] Opcode capture completed');
    
  } else {
    console.log('[JSVecX Simple Test] VecX class not found in bundle');
    console.log('[JSVecX Simple Test] Available globals:', Object.keys(global).filter(k => !k.startsWith('_')));
  }
  
} catch (error) {
  console.error('[JSVecX Simple Test] Failed:', error.message);
  if (error.stack) {
    console.error('[JSVecX Simple Test] Stack:', error.stack.split('\n').slice(0, 5).join('\n'));
  }
  process.exit(1);
}