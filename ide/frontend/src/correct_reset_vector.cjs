#!/usr/bin/env node
/**
 * correct_reset_vector.cjs
 * Leer el reset vector correctamente en big-endian
 */

const fs = require('fs');
const path = require('path');

console.log('=== CORRECCIÓN: LECTURA RESET VECTOR BIG-ENDIAN ===\n');

const biosPath = path.join(__dirname, 'assets', 'bios.bin');
const biosData = fs.readFileSync(biosPath);
console.log(`BIOS cargada: ${biosData.length} bytes`);

// Reset vector en 0xFFFE-0xFFFF (offset 0x1FFE-0x1FFF en archivo)
const resetVectorOffset = 0x1FFE;
const resetVectorHi = biosData[resetVectorOffset];     // 0xFFFE = byte alto
const resetVectorLo = biosData[resetVectorOffset + 1]; // 0xFFFF = byte bajo

console.log(`Reset vector en archivo (BIG-ENDIAN):`);
console.log(`  Offset 0x${resetVectorOffset.toString(16).toUpperCase()} (0xFFFE): 0x${resetVectorHi.toString(16).toUpperCase().padStart(2, '0')} [ALTO]`);
console.log(`  Offset 0x${(resetVectorOffset + 1).toString(16).toUpperCase()} (0xFFFF): 0x${resetVectorLo.toString(16).toUpperCase().padStart(2, '0')} [BAJO]`);

// Lectura correcta big-endian (como jsvecx)
const resetVector = (resetVectorHi << 8) | resetVectorLo;
console.log(`  Reset vector (big-endian): 0x${resetVector.toString(16).toUpperCase().padStart(4, '0')}`);

console.log(`\nSegún bios.asm, el reset vector debería ser 0xF000`);

if (resetVector === 0xF000) {
    console.log('✓ Reset vector CORRECTO: 0xF000');
} else {
    console.log(`✗ Reset vector INCORRECTO: esperado 0xF000, encontrado 0x${resetVector.toString(16).toUpperCase()}`);
}

// Verificar otros vectores en big-endian
console.log('\n=== TABLA COMPLETA DE VECTORES (BIG-ENDIAN) ===');
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
    const hi = biosData[vec.offset];
    const lo = biosData[vec.offset + 1];
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

console.log('\n=== VERIFICACIÓN CÓDIGO EN 0xF000 ===');
// Verificar que 0xF000 contiene código válido
// En el archivo, 0xF000 corresponde a offset 0x1000
const codeOffset = 0x1000; // 0xF000 - 0xE000
console.log(`Código en 0xF000 (offset 0x${codeOffset.toString(16).toUpperCase()}):`);
for (let i = 0; i < 16; i++) {
    const addr = 0xF000 + i;
    const byte = biosData[codeOffset + i];
    console.log(`  0x${addr.toString(16).toUpperCase()}: 0x${byte.toString(16).toUpperCase().padStart(2, '0')}`);
}