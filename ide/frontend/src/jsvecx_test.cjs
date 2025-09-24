#!/usr/bin/env node

/**
 * Test unitario independiente para jsvecx
 * Arranca la BIOS y captura opcodes ejecutados
 */

const fs = require('fs');
const path = require('path');

// Importar el bundle jsvecx
const jsvecxPath = path.join(__dirname, 'generated/jsvecx/vecx_full.js');
let VecX, Globals;

try {
    // Cargar jsvecx usando require (Node.js)
    const jsvecxModule = require(jsvecxPath);
    VecX = jsvecxModule.VecX;
    Globals = jsvecxModule.Globals;
    
    if (!VecX) {
        throw new Error('VecX constructor not found in module');
    }
    
    console.log('[JSVecX Test] jsvecx module loaded successfully');
} catch (e) {
    console.error('[JSVecX Test] Failed to load jsvecx:', e.message);
    process.exit(1);
}

// Cargar BIOS
const biosPath = path.join(__dirname, '../public/bios.bin');
let biosData;

try {
    biosData = fs.readFileSync(biosPath);
    console.log(`[JSVecX Test] BIOS loaded: ${biosData.length} bytes`);
} catch (e) {
    console.error('[JSVecX Test] Failed to load BIOS:', e.message);
    process.exit(1);
}

// Crear instancia jsvecx
console.log('[JSVecX Test] Creating jsvecx instance...');
const vecx = new VecX();

// Cargar BIOS en ROM
console.log('[JSVecX Test] Loading BIOS into ROM...');
const maxLen = Math.min(biosData.length, 0x2000);
for (let i = 0; i < maxLen; i++) {
    vecx.rom[i] = biosData[i];
}

// Configurar reset vector
const resetVectorLow = vecx.rom[0x1FFE];   // 0xFFFE - 0xE000
const resetVectorHigh = vecx.rom[0x1FFF];  // 0xFFFF - 0xE000  
const resetVector = (resetVectorHigh << 8) | resetVectorLow;

console.log(`[JSVecX Test] Reset vector: 0x${resetVector.toString(16).toUpperCase()}`);

// Configurar PC al reset vector
vecx.e6809.reg_pc = resetVector;
console.log(`[JSVecX Test] PC set to: 0x${resetVector.toString(16).toUpperCase()}`);

// Configurar funciones de memoria
vecx.read8 = function(address) {
    address = address & 0xFFFF;
    
    if (address < 0x8000) {
        // Cartridge space
        return vecx.cart[address] || 0;
    } else if (address >= 0xC800 && address < 0xD000) {
        // RAM
        const ramAddr = (address - 0xC800) & 0x3FF;
        return vecx.ram[ramAddr] || 0;
    } else if (address >= 0xE000) {
        // ROM/BIOS
        const romAddr = address - 0xE000;
        return vecx.rom[romAddr] || 0;
    }
    
    return 0xFF; // Unmapped
};

vecx.write8 = function(address, value) {
    address = address & 0xFFFF;
    value = value & 0xFF;
    
    if (address < 0x8000) {
        vecx.cart[address] = value;
    } else if (address >= 0xC800 && address < 0xD000) {
        const ramAddr = (address - 0xC800) & 0x3FF;
        vecx.ram[ramAddr] = value;
    }
    // ROM es read-only
};

console.log('[JSVecX Test] Memory functions configured');

// Intentar reset
try {
    console.log('[JSVecX Test] Attempting vecx_reset...');
    vecx.vecx_reset();
    console.log('[JSVecX Test] vecx_reset successful');
} catch (e) {
    console.warn('[JSVecX Test] vecx_reset failed:', e.message);
    // Continuar sin reset
}

// Re-configurar PC después del reset (por si el reset lo cambió)
vecx.e6809.reg_pc = resetVector;
console.log(`[JSVecX Test] PC re-set to: 0x${resetVector.toString(16).toUpperCase()}`);

// Ejecutar instrucciones y capturar opcodes
console.log('\n[JSVecX Test] Starting instruction execution...');
console.log('PC       | Opcode | A  | B  | X    | Y    | S    | U    | DP | CC');
console.log('---------|--------|----|----|------|------|------|------|----|----');

try {
    for (let step = 0; step < 50; step++) {
        const pc = vecx.e6809.reg_pc;
        const opcode = vecx.read8(pc);
        
        // Capturar estado antes de ejecutar
        const state = {
            pc: pc,
            opcode: opcode,
            a: vecx.e6809.reg_a || 0,
            b: vecx.e6809.reg_b || 0,
            x: (vecx.e6809.reg_x && vecx.e6809.reg_x.value) || 0,
            y: (vecx.e6809.reg_y && vecx.e6809.reg_y.value) || 0,
            s: (vecx.e6809.reg_s && vecx.e6809.reg_s.value) || 0,
            u: (vecx.e6809.reg_u && vecx.e6809.reg_u.value) || 0,
            dp: vecx.e6809.reg_dp || 0,
            cc: vecx.e6809.reg_cc || 0
        };
        
        console.log(
            `${state.pc.toString(16).padStart(8, '0').toUpperCase()} | ` +
            `${state.opcode.toString(16).padStart(6, '0').toUpperCase()} | ` +
            `${state.a.toString(16).padStart(2, '0').toUpperCase()} | ` +
            `${state.b.toString(16).padStart(2, '0').toUpperCase()} | ` +
            `${state.x.toString(16).padStart(4, '0').toUpperCase()} | ` +
            `${state.y.toString(16).padStart(4, '0').toUpperCase()} | ` +
            `${state.s.toString(16).padStart(4, '0').toUpperCase()} | ` +
            `${state.u.toString(16).padStart(4, '0').toUpperCase()} | ` +
            `${state.dp.toString(16).padStart(2, '0').toUpperCase()} | ` +
            `${state.cc.toString(16).padStart(2, '0').toUpperCase()}`
        );
        
        // Ejecutar un paso de CPU
        vecx.e6809.e6809_sstep();
        
        // Detectar loops infinitos
        if (vecx.e6809.reg_pc === pc) {
            console.log(`[JSVecX Test] Infinite loop detected at PC=0x${pc.toString(16).toUpperCase()}`);
            break;
        }
    }
} catch (e) {
    console.error('[JSVecX Test] Execution failed:', e.message);
    console.error('Stack trace:', e.stack);
}

console.log('\n[JSVecX Test] Test completed');