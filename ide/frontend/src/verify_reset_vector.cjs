#!/usr/bin/env node
/**
 * verify_reset_vector.cjs
 * Verificar el reset vector en la BIOS binaria
 */

const fs = require('fs');
const path = require('path');

console.log('=== VERIFICACIÓN DEL RESET VECTOR ===\n');

// Cargar BIOS binaria
const biosPath = path.join(__dirname, 'assets', 'bios.bin');
const biosData = fs.readFileSync(biosPath);
console.log(`BIOS cargada: ${biosData.length} bytes`);

// Los vectores están al final de la ROM (8K)
// Para una ROM de 8K (0x2000 bytes) mapeada en 0xE000-0xFFFF:
// - Reset vector está en 0xFFFE-0xFFFF
// - En el archivo, esto corresponde a offset 0x1FFE-0x1FFF

const resetVectorOffset = 0x1FFE;
const resetVectorLo = biosData[resetVectorOffset];     // 0xFFFE
const resetVectorHi = biosData[resetVectorOffset + 1]; // 0xFFFF

console.log(`Reset vector en archivo:`);
console.log(`  Offset 0x${resetVectorOffset.toString(16).toUpperCase()} (0xFFFE): 0x${resetVectorLo.toString(16).toUpperCase().padStart(2, '0')}`);
console.log(`  Offset 0x${(resetVectorOffset + 1).toString(16).toUpperCase()} (0xFFFF): 0x${resetVectorHi.toString(16).toUpperCase().padStart(2, '0')}`);

// El 6809 es big-endian, así que:
// Reset vector = (Hi << 8) | Lo
const resetVector = (resetVectorHi << 8) | resetVectorLo;
console.log(`  Reset vector calculado: 0x${resetVector.toString(16).toUpperCase().padStart(4, '0')}`);

console.log(`\nSegún bios.asm, el reset vector debería ser 0xF000 (Start)`);

if (resetVector === 0xF000) {
    console.log('✓ Reset vector CORRECTO: 0xF000');
} else {
    console.log(`✗ Reset vector INCORRECTO: esperado 0xF000, encontrado 0x${resetVector.toString(16).toUpperCase()}`);
}

// Verificar también otros vectores para comparar
console.log('\n=== TABLA COMPLETA DE VECTORES ===');
const vectors = [
    { name: 'SWI3', offset: 0x1FF2 },
    { name: 'SWI2', offset: 0x1FF4 },
    { name: 'FIRQ', offset: 0x1FF6 },
    { name: 'IRQ',  offset: 0x1FF8 },
    { name: 'SWI',  offset: 0x1FFA },
    { name: 'NMI',  offset: 0x1FFC },
    { name: 'RESET', offset: 0x1FFE }
];

vectors.forEach(vec => {
    const lo = biosData[vec.offset];
    const hi = biosData[vec.offset + 1];
    const addr = (hi << 8) | lo;
    console.log(`${vec.name.padEnd(6)}: 0x${addr.toString(16).toUpperCase().padStart(4, '0')}`);
});

console.log('\nSegún bios.asm, los vectores deberían ser:');
console.log('SWI3  : 0xCBF2');
console.log('SWI2  : 0xCBF2'); 
console.log('FIRQ  : 0xCBF5');
console.log('IRQ   : 0xCBF8');
console.log('SWI   : 0xCBFB');
console.log('NMI   : 0xCBFB');
console.log('RESET : 0xF000');