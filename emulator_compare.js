#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function parseEmulatorFile(filename) {
    const content = fs.readFileSync(filename, 'utf8');
    const lines = content.split('\n');
    const states = [];
    
    for (const line of lines) {
        // Buscar líneas que contengan datos de estado (formato tabla)
        // Asegurándonos de que empiece con │ seguido de spaces y número, luego │ con hex de 4 dígitos
        const match = line.match(/^│\s*(\d+)\s*│\s*([A-F0-9]{4})\s*│\s*(?:0x)?([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│\s*([A-F0-9]+)\s*│/);
        if (match) {
            states.push({
                step: parseInt(match[1]),
                pc: match[2],
                opcode: match[3],
                a: match[4],
                b: match[5],
                x: match[6],
                y: match[7],
                s: match[8],
                u: match[9],
                dp: match[10],
                cc: match[11]
            });
        }
    }
    return states;
}

function compareEmulators(steps) {
    console.log(`\n=== COMPARACIÓN DIRECTA (${steps} pasos) ===`);
    
    const rustFile = `emulator_comparison_${steps}_steps.txt`;
    const jsvecxFile = `jsvecx_comparison_${steps}_steps.txt`;
    
    if (!fs.existsSync(rustFile) || !fs.existsSync(jsvecxFile)) {
        console.log(`⚠️  Archivos no encontrados para ${steps} pasos`);
        return;
    }
    
    const rustStates = parseEmulatorFile(rustFile);
    const jsvecxStates = parseEmulatorFile(jsvecxFile);
    
    console.log(`📊 Rust: ${rustStates.length} estados, JSVecx: ${jsvecxStates.length} estados`);
    
    let differences = 0;
    const maxSteps = Math.min(rustStates.length, jsvecxStates.length);
    
    let output = '';
    output += `=== COMPARACIÓN DETALLADA (${steps} pasos) ===\n`;
    output += `Rust vs JSVecx - Diferencias resaltadas\n\n`;
    
    // Tabla header
    output += '┌──────┬──────┬────────┬────────┬────────┬──────────┬──────────┬──────────┬──────────┬────────┬────────┐\n';
    output += '│ Step │  PC  │ Opcode │   A    │   B    │    X     │    S     │    U     │    Y     │   DP   │   CC   │\n';
    output += '├──────┼──────┼────────┼────────┼────────┼──────────┼──────────┼──────────┼──────────┼────────┼────────┤\n';
    
    for (let i = 0; i < Math.min(maxSteps, 50); i++) { // Limitar a 50 para legibilidad
        const rust = rustStates[i];
        const jsvecx = jsvecxStates[i];
        
        if (!rust || !jsvecx) break;
        
        const stepDiff = rust.step !== jsvecx.step;
        const pcDiff = rust.pc !== jsvecx.pc;
        const opcodeDiff = rust.opcode !== jsvecx.opcode;
        const aDiff = rust.a !== jsvecx.a;
        const bDiff = rust.b !== jsvecx.b;
        const xDiff = rust.x !== jsvecx.x;
        const sDiff = rust.s !== jsvecx.s;
        const uDiff = rust.u !== jsvecx.u;
        const yDiff = rust.y !== jsvecx.y;
        const dpDiff = rust.dp !== jsvecx.dp;
        const ccDiff = rust.cc !== jsvecx.cc;
        
        const hasDiff = pcDiff || opcodeDiff || aDiff || bDiff || xDiff || sDiff || uDiff || yDiff || dpDiff || ccDiff;
        
        if (hasDiff) {
            differences++;
            const marker = '❌';
            
            output += `│ ${rust.step.toString().padStart(4)} │ ${marker} ${rust.pc}${pcDiff ? '≠' + jsvecx.pc : '=' + jsvecx.pc} │`;
            output += ` ${rust.opcode}${opcodeDiff ? '≠' + jsvecx.opcode : '=' + jsvecx.opcode} │`;
            output += ` ${rust.a}${aDiff ? '≠' + jsvecx.a : '=' + jsvecx.a} │`;
            output += ` ${rust.b}${bDiff ? '≠' + jsvecx.b : '=' + jsvecx.b} │`;
            output += ` ${rust.x}${xDiff ? '≠' + jsvecx.x : '=' + jsvecx.x} │`;
            output += ` ${rust.s}${sDiff ? '≠' + jsvecx.s : '=' + jsvecx.s} │`;
            output += ` ${rust.u}${uDiff ? '≠' + jsvecx.u : '=' + jsvecx.u} │`;
            output += ` ${rust.y}${yDiff ? '≠' + jsvecx.y : '=' + jsvecx.y} │`;
            output += ` ${rust.dp}${dpDiff ? '≠' + jsvecx.dp : '=' + jsvecx.dp} │`;
            output += ` ${rust.cc}${ccDiff ? '≠' + jsvecx.cc : '=' + jsvecx.cc} │\n`;
        } else {
            output += `│ ${rust.step.toString().padStart(4)} │ ✅ ${rust.pc} │ ${rust.opcode} │ ${rust.a} │ ${rust.b} │ ${rust.x} │ ${rust.s} │ ${rust.u} │ ${rust.y} │ ${rust.dp} │ ${rust.cc} │\n`;
        }
    }
    
    output += '└──────┴──────┴────────┴────────┴────────┴──────────┴──────────┴──────────┴──────────┴────────┴────────┘\n\n';
    
    // Resumen de diferencias
    output += `📈 RESUMEN DE DIFERENCIAS:\n`;
    output += `Total de pasos comparados: ${maxSteps}\n`;
    output += `Pasos con diferencias: ${differences}\n`;
    output += `Coincidencia: ${((maxSteps - differences) / maxSteps * 100).toFixed(1)}%\n\n`;
    
    // Análisis de diferencias específicas
    let initialDifferences = '';
    if (rustStates[0] && jsvecxStates[0]) {
        const r0 = rustStates[0];
        const j0 = jsvecxStates[0];
        
        initialDifferences += '🔍 DIFERENCIAS EN ESTADO INICIAL:\n';
        if (r0.s !== j0.s) initialDifferences += `  - Stack Pointer (S): Rust=${r0.s}, JSVecx=${j0.s}\n`;
        if (r0.dp !== j0.dp) initialDifferences += `  - Direct Page (DP): Rust=${r0.dp}, JSVecx=${j0.dp}\n`;
        if (r0.cc !== j0.cc) initialDifferences += `  - Condition Codes (CC): Rust=${r0.cc}, JSVecx=${j0.cc}\n`;
        if (r0.u !== j0.u) initialDifferences += `  - User Stack (U): Rust=${r0.u}, JSVecx=${j0.u}\n`;
        initialDifferences += '\n';
    }
    
    output += initialDifferences;
    
    // Análisis de tendencias
    let xRegisterAnalysis = '';
    const rustXValues = rustStates.slice(8, 23).map(s => s.x);
    const jsvecxXValues = jsvecxStates.slice(8, 23).map(s => s.x);
    
    const rustXIncreasing = rustXValues.every((val, i, arr) => i === 0 || parseInt(val, 16) > parseInt(arr[i-1], 16));
    const jsvecxXConstant = jsvecxXValues.every(val => val === jsvecxXValues[0]);
    
    if (rustXIncreasing && jsvecxXConstant) {
        xRegisterAnalysis += '🔄 DIFERENCIA EN REGISTRO X:\n';
        xRegisterAnalysis += '  - Rust: X se incrementa en cada iteración del loop\n';
        xRegisterAnalysis += '  - JSVecx: X permanece constante\n';
        xRegisterAnalysis += '  - Esto puede indicar diferencias en cómo se maneja el post-incremento\n\n';
    }
    
    output += xRegisterAnalysis;
    
    // Guardar resultado
    const outputFile = `comparison_${steps}_steps.txt`;
    fs.writeFileSync(outputFile, output);
    console.log(`📄 Comparación guardada en: ${path.resolve(outputFile)}`);
    
    console.log(`❌ ${differences} diferencias encontradas de ${maxSteps} pasos (${(differences/maxSteps*100).toFixed(1)}% diferentes)`);
}

function main() {
    console.log('🔍 ANALIZADOR DE DIFERENCIAS ENTRE EMULADORES');
    console.log('============================================');
    
    const testSteps = [100, 500, 1000, 2000, 5000];
    
    for (const steps of testSteps) {
        compareEmulators(steps);
    }
    
    console.log('\n✅ Análisis completo de diferencias generado');
}

if (require.main === module) {
    main();
}