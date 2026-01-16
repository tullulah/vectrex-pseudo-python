#!/usr/bin/env node

/**
 * Debugger para investigar por qué multibank se cuelga en 0xF33D
 * 
 * Estrategia:
 * 1. Cargar ROM multibank 
 * 2. Ejecutar hasta PC=0xF33D
 * 3. Capturar estado completo de registros, stack, memoria
 * 4. Avanzar 1000 ciclos y ver si sale del bucle
 * 5. Verificar si hay patrón repetitivo
 */

const fs = require('fs');
const path = require('path');

// Cargar módulo del emulador (es CommonJS)
const vecxModule = require('./ide/frontend/dist/jsvecx_deploy/vecx.js');
const Vectrex = vecxModule.Vectrex;

console.log('🔍 Inicializando debugger para multibank...\n');

// Crear emulador
const vecx = new Vectrex();

// Cargar ROM multibank de 512KB
const romPath = path.join(__dirname, 'multibank_temp/test_rom.bin');
if (!fs.existsSync(romPath)) {
    console.error(`❌ ROM no encontrada: ${romPath}`);
    process.exit(1);
}

const romData = fs.readFileSync(romPath);
console.log(`✓ ROM cargada: ${romData.length} bytes`);

vecx.reset(romData);

// Ayudante para leer registro
function readReg(name) {
    const regs = {
        'A': () => vecx.cpu.regA,
        'B': () => vecx.cpu.regB,
        'D': () => (vecx.cpu.regA << 8) | vecx.cpu.regB,
        'X': () => vecx.cpu.regX,
        'Y': () => vecx.cpu.regY,
        'U': () => vecx.cpu.regU,
        'S': () => vecx.cpu.regS,
        'PC': () => vecx.cpu.pc,
        'DP': () => vecx.cpu.dp,
        'CC': () => vecx.cpu.cc,
    };
    return regs[name]?.() ?? '?';
}

function printRegs() {
    console.log(`  A=$${readReg('A').toString(16).padStart(2, '0').toUpperCase()}`);
    console.log(`  B=$${readReg('B').toString(16).padStart(2, '0').toUpperCase()}`);
    console.log(`  X=$${readReg('X').toString(16).padStart(4, '0').toUpperCase()}`);
    console.log(`  Y=$${readReg('Y').toString(16).padStart(4, '0').toUpperCase()}`);
    console.log(`  S=$${readReg('S').toString(16).padStart(4, '0').toUpperCase()}`);
    console.log(`  U=$${readReg('U').toString(16).padStart(4, '0').toUpperCase()}`);
    console.log(`  PC=$${readReg('PC').toString(16).padStart(4, '0').toUpperCase()}`);
    console.log(`  DP=$${readReg('DP').toString(16).padStart(2, '0').toUpperCase()}`);
    console.log(`  CC=$${readReg('CC').toString(16).padStart(2, '0').toUpperCase()}`);
}

console.log('\n=== ESTADO INICIAL ===');
printRegs();

// Ejecutar hasta 0xF33D
console.log('\n⏳ Ejecutando hasta PC=0xF33D...');
const TARGET_PC = 0xF33D;
let steps = 0;
const MAX_STEPS = 1000000;

while (vecx.cpu.pc !== TARGET_PC && steps < MAX_STEPS) {
    vecx.cpu_step();
    steps++;
}

if (steps >= MAX_STEPS) {
    console.log(`⏱️ Llegó a ${MAX_STEPS} ciclos sin encontrar 0xF33D`);
    console.log(`Último PC: 0x${vecx.cpu.pc.toString(16).toUpperCase()}`);
} else {
    console.log(`✓ Encontrado 0xF33D después de ${steps} ciclos`);
}

console.log('\n=== ESTADO EN PC=0xF33D ===');
printRegs();

// Leer instrucción en PC
const pcAddr = vecx.cpu.pc;
const instr = vecx.read8(pcAddr);
const instr2 = vecx.read8(pcAddr + 1);
console.log(`\nInstrucción en PC: 0x${instr.toString(16).padStart(2, '0').toUpperCase()} 0x${instr2.toString(16).padStart(2, '0').toUpperCase()}`);

// Leer algunos bytes del stack
console.log('\n=== STACK (top 8 bytes) ===');
const sp = readReg('S');
for (let i = 0; i < 8; i++) {
    const addr = sp + i;
    const val = vecx.read8(addr);
    console.log(`  $${addr.toString(16).padStart(4, '0').toUpperCase()}: 0x${val.toString(16).padStart(2, '0').toUpperCase()}`);
}

// Avanzar 1000 pasos y ver si sale del bucle
console.log('\n⏳ Avanzando 1000 ciclos más...');
for (let i = 0; i < 1000; i++) {
    vecx.cpu_step();
}

console.log(`\n=== ESTADO DESPUÉS DE 1000 CICLOS ===`);
console.log(`PC: 0x${vecx.cpu.pc.toString(16).padStart(4, '0').toUpperCase()}`);

if (vecx.cpu.pc === pcAddr) {
    console.log('⚠️ PC no cambió → Bucle infinito en 0xF33D');
} else {
    console.log(`✓ PC avanzó a 0x${vecx.cpu.pc.toString(16).toUpperCase()}`);
}

printRegs();

// Análisis del Direct Page
console.log('\n=== ANÁLISIS DIRECT PAGE (DP) ===');
const dp = readReg('DP');
console.log(`DP = 0x${dp.toString(16).padStart(2, '0').toUpperCase()} (direct page en 0x${(dp * 0x100).toString(16).padStart(4, '0').toUpperCase()})`);

// Memoria en DP
console.log(`Primeros 16 bytes de DP:`);
for (let i = 0; i < 16; i++) {
    const addr = (dp * 0x100) + i;
    const val = vecx.read8(addr);
    console.log(`  0x${addr.toString(16).padStart(4, '0').toUpperCase()}: 0x${val.toString(16).padStart(2, '0').toUpperCase()}`);
}

// Estado VIA si DP es 0xD0
if (dp === 0xD0) {
    console.log('\n=== ESTADO VIA (accesible con DP=0xD0) ===');
    console.log('PSG Port A (0xD000): 0x' + vecx.read8(0xD000).toString(16).padStart(2, '0').toUpperCase());
    console.log('PSG Port B (0xD002): 0x' + vecx.read8(0xD002).toString(16).padStart(2, '0').toUpperCase());
    console.log('PSG Port Control (0xD00C): 0x' + vecx.read8(0xD00C).toString(16).padStart(2, '0').toUpperCase());
} else if (dp === 0xC8) {
    console.log('\n⚠️ DP=0xC8, no acceso directo a VIA');
}

console.log('\n=== CONCLUSIONES ===');
console.log('1. Si PC no avanzó después de 1000 ciclos: hay bucle infinito en 0xF33D');
console.log('2. Verificar si DP está correctamente inicializado');
console.log('3. Verificar estado del stack (¿está apuntando a RAM válida?)');
console.log('4. Revisar si hay IRQ o excepción pendiente');
