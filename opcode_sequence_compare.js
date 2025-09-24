#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function parseEmulatorFile(filename) {
    const content = fs.readFileSync(filename, 'utf8');
    const lines = content.split('\n');
    const opcodes = [];
    
    for (const line of lines) {
        // Buscar líneas que contengan datos de estado (formato tabla)
        const match = line.match(/^│\s*(\d+)\s*│\s*([A-F0-9]{4})\s*│\s*(?:0x)?([A-F0-9]+)\s*│/);
        if (match) {
            opcodes.push({
                step: parseInt(match[1]),
                pc: match[2],
                opcode: match[3].padStart(2, '0').toUpperCase()
            });
        }
    }
    return opcodes;
}

function compareOpcodeSequences() {
    console.log('🔍 COMPARACIÓN DE SECUENCIAS DE OPCODES');
    console.log('=====================================');
    
    const rustFile = 'emulator_comparison_100_steps.txt';
    const jsvecxFile = 'jsvecx_comparison_100_steps.txt';
    
    if (!fs.existsSync(rustFile) || !fs.existsSync(jsvecxFile)) {
        console.log('⚠️  Archivos no encontrados');
        return;
    }
    
    const rustOpcodes = parseEmulatorFile(rustFile);
    const jsvecxOpcodes = parseEmulatorFile(jsvecxFile);
    
    console.log(`📊 Rust: ${rustOpcodes.length} opcodes`);
    console.log(`📊 JSVecx: ${jsvecxOpcodes.length} opcodes`);
    
    const maxSteps = Math.min(rustOpcodes.length, jsvecxOpcodes.length);
    let identicalSequence = true;
    let firstDifference = -1;
    
    console.log('\n┌──────┬─────────────────┬─────────────────┬──────────┐');
    console.log('│ Step │      Rust       │     JSVecx      │  Status  │');
    console.log('├──────┼─────────────────┼─────────────────┼──────────┤');
    
    for (let i = 0; i < Math.min(maxSteps, 50); i++) {
        const rust = rustOpcodes[i];
        const jsvecx = jsvecxOpcodes[i];
        
        const rustStr = `${rust.pc}:0x${rust.opcode}`;
        const jsvecxStr = `${jsvecx.pc}:0x${jsvecx.opcode}`;
        
        const isIdentical = rust.pc === jsvecx.pc && rust.opcode === jsvecx.opcode;
        
        if (!isIdentical && identicalSequence) {
            identicalSequence = false;
            firstDifference = i;
        }
        
        const status = isIdentical ? '✅ IGUAL' : '❌ DIFF';
        
        console.log(
            `│ ${i.toString().padStart(4)} │ ${rustStr.padEnd(15)} │ ${jsvecxStr.padEnd(15)} │ ${status.padEnd(8)} │`
        );
    }
    
    console.log('└──────┴─────────────────┴─────────────────┴──────────┘');
    
    // Análisis completo
    let totalIdentical = 0;
    let totalDifferent = 0;
    
    for (let i = 0; i < maxSteps; i++) {
        const rust = rustOpcodes[i];
        const jsvecx = jsvecxOpcodes[i];
        
        if (rust.pc === jsvecx.pc && rust.opcode === jsvecx.opcode) {
            totalIdentical++;
        } else {
            totalDifferent++;
        }
    }
    
    console.log('\n📈 RESUMEN DE SECUENCIA DE OPCODES:');
    console.log(`Total comparados: ${maxSteps}`);
    console.log(`Idénticos: ${totalIdentical} (${(totalIdentical/maxSteps*100).toFixed(1)}%)`);
    console.log(`Diferentes: ${totalDifferent} (${(totalDifferent/maxSteps*100).toFixed(1)}%)`);
    
    if (identicalSequence) {
        console.log('\n🎉 ¡SECUENCIA IDÉNTICA! Los opcodes ejecutados son exactamente los mismos');
    } else {
        console.log(`\n❌ SECUENCIA DIFERENTE desde el paso ${firstDifference}`);
        
        if (firstDifference >= 0 && firstDifference < maxSteps) {
            const rust = rustOpcodes[firstDifference];
            const jsvecx = jsvecxOpcodes[firstDifference];
            console.log(`Primera diferencia en paso ${firstDifference}:`);
            console.log(`  Rust:   PC=${rust.pc} Opcode=0x${rust.opcode}`);
            console.log(`  JSVecx: PC=${jsvecx.pc} Opcode=0x${jsvecx.opcode}`);
        }
    }
    
    // Análisis de patrones
    console.log('\n🔍 ANÁLISIS DE PATRONES:');
    
    // Crear secuencias de PC:Opcode
    const rustSequence = rustOpcodes.slice(0, maxSteps).map(op => `${op.pc}:${op.opcode}`);
    const jsvecxSequence = jsvecxOpcodes.slice(0, maxSteps).map(op => `${op.pc}:${op.opcode}`);
    
    // Buscar secuencias comunes
    let longestCommonSequence = 0;
    let currentCommonLength = 0;
    
    for (let i = 0; i < Math.min(rustSequence.length, jsvecxSequence.length); i++) {
        if (rustSequence[i] === jsvecxSequence[i]) {
            currentCommonLength++;
            longestCommonSequence = Math.max(longestCommonSequence, currentCommonLength);
        } else {
            currentCommonLength = 0;
        }
    }
    
    console.log(`Secuencia común más larga: ${longestCommonSequence} opcodes consecutivos`);
    
    // Verificar si solo difieren en el comienzo
    let identicalFromStep = -1;
    for (let i = 1; i < maxSteps; i++) {
        let allIdenticalFromHere = true;
        for (let j = i; j < Math.min(maxSteps, i + 20); j++) {
            if (rustSequence[j] !== jsvecxSequence[j]) {
                allIdenticalFromHere = false;
                break;
            }
        }
        if (allIdenticalFromHere) {
            identicalFromStep = i;
            break;
        }
    }
    
    if (identicalFromStep >= 0) {
        console.log(`🎯 Las secuencias se vuelven idénticas desde el paso ${identicalFromStep}`);
    }
    
    return {
        totalIdentical,
        totalDifferent,
        identicalSequence,
        firstDifference,
        longestCommonSequence
    };
}

if (require.main === module) {
    compareOpcodeSequences();
}