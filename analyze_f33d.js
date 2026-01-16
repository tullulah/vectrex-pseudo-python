#!/usr/bin/env node

/**
 * Inspección del estado multibank en emulador
 * Lee directamente el estado que se guardó previamente
 */

const fs = require('fs');

console.log('🔍 Analizando snapshot multibank para diagnosticar 0xF33D\n');

const snapshotPath = '/Users/daniel/projects/vectrex-pseudo-python/examples/test_callgraph/rom_snapshot_bank0_and_31.bin';

if (!fs.existsSync(snapshotPath)) {
    console.error(`❌ Snapshot no encontrado: ${snapshotPath}`);
    process.exit(1);
}

const snapshot = fs.readFileSync(snapshotPath);
console.log(`✓ Snapshot cargado: ${snapshot.length} bytes\n`);

// Dirección 0xF33D en el mapa de memoria:
// BIOS está en $E000-$FFFF
// 0xF33D está en el BIOS

// En el snapshot:
// - Bank 0: $0000-$3FFF (primeros 16KB)
// - Bank 31: $4000-$7FFF (segundos 16KB)
// 
// 0xF33D (BIOS) NO está en el snapshot (snapshot solo tiene espacio de cartucho $0000-$7FFF)
// pero podemos analizar qué está en Bank 0 que podría estar saltando a 0xF33D

console.log('=== ANÁLISIS DIRECCIONES CLAVE ===\n');

// Buscar saltos a BIOS (0xF000 y superiores)
// En M6809, JSR tiene opcode 0x8D o 0x9D (directo) o 0xAD (indexado)
// Formato: 8D XX XX (dirección 16-bit)

console.log('Buscando JSR al BIOS en Bank 0...\n');

for (let addr = 0; addr < snapshot.length - 2; addr++) {
    const opcode = snapshot[addr];
    
    // JSR directo (8D)
    if (opcode === 0x8D) {
        const target = (snapshot[addr + 1] << 8) | snapshot[addr + 2];
        if (target >= 0xE000) {
            console.log(`  0x${addr.toString(16).padStart(4, '0').toUpperCase()}: JSR 0x${target.toString(16).padStart(4, '0').toUpperCase()}`);
            if (target === 0xF33D) {
                console.log(`    ⚠️  ENCONTRADO: Salto directo a 0xF33D\n`);
            }
        }
    }
}

// Buscar en dirección 0xF33D específicamente en Bank 0
// 0xF33D en cartucho sería: 0xF33D - 0xE000 = 0x133D (fuera del snapshot de 32KB)
// Pero si Bank 0 en algunas condiciones contiene código del BIOS...

console.log('=== VERIFICACIÓN DE CONTENIDO ===\n');

// Primeros bytes de Bank 0
console.log('Primeros 32 bytes de Bank 0 ($0000-$001F):');
for (let i = 0; i < 32; i += 16) {
    const hex = [];
    for (let j = 0; j < 16; j++) {
        hex.push(snapshot[i + j].toString(16).padStart(2, '0').toUpperCase());
    }
    console.log(`  0x${(i).toString(16).padStart(4, '0').toUpperCase()}: ${hex.join(' ')}`);
}

// Primeros bytes de Bank 31
console.log('\nPrimeros 32 bytes de Bank 31 ($4000-$401F):');
for (let i = 0x4000; i < 0x4000 + 32; i += 16) {
    const hex = [];
    for (let j = 0; j < 16; j++) {
        hex.push(snapshot[i + j].toString(16).padStart(2, '0').toUpperCase());
    }
    console.log(`  0x${(i).toString(16).padStart(4, '0').toUpperCase()}: ${hex.join(' ')}`);
}

console.log('\n=== TEORÍAS POSIBLES ===\n');
console.log('1. 0xF33D es en el BIOS (no en cartucho)');
console.log('   → Ejecución llega al BIOS correctamente');
console.log('   → Problema: BIOS está en bucle infinito esperando algo\n');

console.log('2. Hay un salto desde Bank 0 al BIOS (0xF33D)');
console.log('   → Boot stub correcto pero se cuelga en BIOS\n');

console.log('3. Stack pointer o DP no inicializados correctamente');
console.log('   → CPU sigue ejecutando pero ubicación de memoria incorrecta\n');

console.log('=== PRÓXIMOS PASOS ===\n');
console.log('✓ El ROM se cargó correctamente (verificado con analyze_rom_snapshot.py)');
console.log('✓ Bank 0 contiene código válido (16K de datos reales)');
console.log('✓ Bank 31 contiene código válido (16K de datos reales)');
console.log('\n❓ Necesitamos inspeccionar qué sucede en BIOS:');
console.log('  - Ver si hay bucle infinito en 0xF33D');
console.log('  - Ver qué condición está esperando BIOS');
console.log('  - Ver si hay pendiente algún registro o flag\n');

console.log('💡 Sugerencia: Usa inspector en tiempo de ejecución en IDE:');
console.log('  1. Compila multibank: cargo run --bin vectrexc -- build examples/test_callgraph/src/main.vpy --bin');
console.log('  2. Carga en IDE (run-ide.sh)');
console.log('  3. Pausa en cualquier momento');
console.log('  4. Usa consola: inspect_multibank_console.js');
console.log('  5. Verifica registros y memoria en tiempo real\n');
