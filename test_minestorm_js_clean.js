/*
JavaScript Test - Entrada a Minestorm (Extendido)
Compara capacidad de llegar a Minestorm 0xE000 con 50M ciclos
*/

const fs = require('fs');
const path = require('path');

console.log('🔧 Cargando emulador JavaScript...');

// Cargar JSVecx (archivos compilados) usando eval como el script que funciona
const JSVecxPath = path.join(__dirname, 'jsvecx', 'src', 'deploy', 'js');

// Necesitamos cargar todos los archivos en orden
eval(fs.readFileSync(path.join(JSVecxPath, 'utils.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'globals.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'vector_t.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'e6809.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'e8910.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'osint.js'), 'utf8'));
eval(fs.readFileSync(path.join(JSVecxPath, 'vecx.js'), 'utf8'));

console.log('✅ Archivos JavaScript cargados');

const biosPath = 'C:\\Users\\DanielFerrerGuerrero\\source\\repos\\pseudo-python\\ide\\frontend\\src\\assets\\bios.bin';
const biosData = fs.readFileSync(biosPath);
console.log('✅ BIOS cargada,', biosData.length, 'bytes');

console.log('=== COMPARACIÓN JSVecx - ENTRADA A MINESTORM (EXTENDIDO) ===');

// Crear instancia de JSVecx
const emu = new VecX();

// Cargar BIOS de la forma correcta para JSVecx
const biosArray = new Uint8Array(biosData);

// JSVecx espera que la BIOS esté en Globals.romdata como string
let romString = '';
for (let i = 0; i < biosData.length; i++) {
    romString += String.fromCharCode(biosData[i]);
}

// Asegurarnos de que Globals.romdata esté disponible
if (typeof Globals === 'undefined') {
    global.Globals = {};
}
Globals.romdata = romString;

console.log('🔧 BIOS convertida a string para JSVecx');

// Inicializar sólo el CPU (osint requiere DOM)
try {
    console.log('🔧 Inicializando CPU...');
    emu.e6809.init(emu);
    console.log('🔧 Ejecutando reset...');
    emu.vecx_reset();
    console.log('✅ Emulador inicializado');
} catch (e) {
    console.error('❌ Error inicializando emulador:', e.message);
    throw e;
}

console.log('🟡 JAVASCRIPT EMULATOR:');
console.log('   Ejecutando hasta 500 millones de ciclos buscando entrada a Minestorm...');

const maxSteps = 500_000_000;
let minestormEntries = [];
let printStrCalls = [];

// Rango 0xE000-0xEFFF para Minestorm (confirmado en bios.asm línea 127)
let step = 0;
let lastUpdate = 0;

const startTime = Date.now();

while (step < maxSteps) {
    const pc = emu.e6809.reg_pc & 0xFFFF;
    
    // Detectar entrada al rango de Minestorm 0xE000-0xEFFF
    if (pc >= 0xE000 && pc <= 0xEFFF && step > 100000) {
        if (minestormEntries.length < 20) {
            const cycles = emu.e6809.cycles || 0;
            minestormEntries.push({step, pc, cycles});
            console.log(`🎯 ENTRADA MINESTORM: PC=0x${pc.toString(16).toUpperCase().padStart(4, '0')} en step ${step}, cycles ${cycles}`);
        }
    }
    
    // Detectar llamadas a Print_Str (0xF373)
    if (pc === 0xF373) {
        if (printStrCalls.length < 5) {
            const cycles = emu.e6809.cycles || 0;
            printStrCalls.push({step, pc, cycles});
            console.log(`📝 Print_Str detectado en step ${step}, cycles ${cycles}`);
        }
    }
    
    // Progress report cada 25M pasos
    if (step - lastUpdate >= 25_000_000) {
        const elapsed = (Date.now() - startTime) / 1000;
        console.log(`⏳ Progreso: ${step / 1_000_000}M pasos (${elapsed.toFixed(1)}s), PC=0x${pc.toString(16).toUpperCase().padStart(4, '0')}`);
        lastUpdate = step;
    }
    
    // Single step
    emu.e6809.e6809_sstep();
    step++;
}

const elapsed = (Date.now() - startTime) / 1000;

console.log('\n🏁 RESUMEN JAVASCRIPT:');
console.log(`⏱️  Tiempo de ejecución: ${elapsed.toFixed(2)} segundos`);
console.log(`🔢 Total de pasos: ${step}`);
console.log(`🎯 Entradas a Minestorm: ${minestormEntries.length}`);
console.log(`📝 Llamadas Print_Str: ${printStrCalls.length}`);

if (minestormEntries.length > 0) {
    console.log('\n🎮 ENTRADAS A MINESTORM DETECTADAS:');
    minestormEntries.forEach((entry, i) => {
        console.log(`   ${i+1}. Step ${entry.step}: PC=0x${entry.pc.toString(16).toUpperCase().padStart(4, '0')}, Cycles=${entry.cycles}`);
    });
} else {
    console.log('\n❌ JAVASCRIPT NO llegó a Minestorm en 500M ciclos');
}

if (printStrCalls.length > 0) {
    console.log('\n📝 LLAMADAS PRINT_STR:');
    printStrCalls.forEach((call, i) => {
        console.log(`   ${i+1}. Step ${call.step}: Cycles=${call.cycles}`);
    });
}

console.log('\n===============================================');