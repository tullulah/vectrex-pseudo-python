#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function parseEmulatorFile(filename) {
    const content = fs.readFileSync(filename, 'utf8');
    const lines = content.split('\n');
    const states = [];
    
    for (const line of lines) {
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

function analyzeRegisterDifferences() {
    console.log('🔍 ANÁLISIS DETALLADO DE DIFERENCIAS EN REGISTROS');
    console.log('================================================');
    
    const rustFile = 'emulator_comparison_1000_steps.txt';
    const jsvecxFile = 'jsvecx_comparison_1000_steps.txt';
    
    if (!fs.existsSync(rustFile) || !fs.existsSync(jsvecxFile)) {
        console.log('⚠️  Archivos no encontrados');
        return;
    }
    
    const rustStates = parseEmulatorFile(rustFile);
    const jsvecxStates = parseEmulatorFile(jsvecxFile);
    
    console.log(`📊 Analizando ${Math.min(rustStates.length, jsvecxStates.length)} estados\n`);
    
    // Análisis del estado inicial
    console.log('🚀 DIFERENCIAS EN ESTADO INICIAL:');
    console.log('=================================');
    const rust0 = rustStates[0];
    const jsvecx0 = jsvecxStates[0];
    
    console.log('│ Registro │    Rust    │   JSVecx   │ Diferencia │ Impacto Posible en Vectores │');
    console.log('├──────────┼────────────┼────────────┼────────────┼──────────────────────────────┤');
    console.log(`│    S     │    ${rust0.s}    │    ${jsvecx0.s}    │ ${parseInt(rust0.s, 16) - parseInt(jsvecx0.s, 16)} bytes │ Stack inicial diferente      │`);
    console.log(`│    DP    │     ${rust0.dp}     │     ${jsvecx0.dp}     │ ${parseInt(rust0.dp, 16) - parseInt(jsvecx0.dp, 16)} pages │ Página directa diferente     │`);
    console.log(`│    CC    │     ${rust0.cc}     │     ${jsvecx0.cc}     │ ${parseInt(rust0.cc, 16) - parseInt(jsvecx0.cc, 16)} flags │ Flags de condición diferentes│`);
    console.log('└──────────┴────────────┴────────────┴────────────┴──────────────────────────────┘\n');
    
    // Análisis del comportamiento del registro X
    console.log('🎯 ANÁLISIS DEL REGISTRO X (CRÍTICO PARA VECTORES):');
    console.log('===================================================');
    
    // Buscar el patrón del loop donde X debería cambiar
    const loopStart = 13; // Aproximadamente donde empieza el loop en F548-F54D
    const loopEnd = Math.min(50, rustStates.length, jsvecxStates.length);
    
    console.log('│ Step │  PC  │ Opcode │     Rust X     │    JSVecx X    │ Diferencia │');
    console.log('├──────┼──────┼────────┼────────────────┼────────────────┼────────────┤');
    
    let xDifferences = [];
    for (let i = loopStart; i < loopEnd; i += 3) { // Cada 3 pasos para ver el patrón
        const rust = rustStates[i];
        const jsvecx = jsvecxStates[i];
        
        const rustXInt = parseInt(rust.x, 16);
        const jsvecxXInt = parseInt(jsvecx.x, 16);
        const diff = rustXInt - jsvecxXInt;
        
        xDifferences.push(diff);
        
        console.log(
            `│ ${i.toString().padStart(4)} │ ${rust.pc} │  0x${rust.opcode}  │ ${rust.x} (${rustXInt.toString().padStart(5)}) │ ${jsvecx.x} (${jsvecxXInt.toString().padStart(5)}) │ ${diff.toString().padStart(10)} │`
        );
    }
    console.log('└──────┴──────┴────────┴────────────────┴────────────────┴────────────────┘\n');
    
    // Analizar el patrón de incremento
    console.log('📈 PATRÓN DE INCREMENTO DEL REGISTRO X:');
    console.log('======================================');
    
    const uniqueDiffs = [...new Set(xDifferences)];
    console.log(`Diferencias únicas encontradas: ${uniqueDiffs.join(', ')}`);
    
    if (uniqueDiffs.length === 1 && uniqueDiffs[0] > 0) {
        console.log(`🔍 Rust incrementa X consistentemente en ${uniqueDiffs[0]} por iteración`);
        console.log('🔍 JSVecx mantiene X constante');
        console.log('\n⚠️  IMPACTO POTENCIAL EN VECTORES:');
        console.log('   - El registro X a menudo se usa como puntero de datos');
        console.log('   - Si X apunta a una tabla de vectores, el incremento afecta qué vector se lee');
        console.log('   - Esto podría explicar vectores mal generados o corrompidos');
    }
    
    // Análisis de flags CC
    console.log('\n🏁 ANÁLISIS DE FLAGS DE CONDICIÓN (CC):');
    console.log('======================================');
    
    // Buscar instrucciones que deberían afectar flags
    const flagChangingOpcodes = ['83', '2A', '6F']; // SUBD, BPL, CLR
    
    for (let i = 10; i < Math.min(30, rustStates.length); i++) {
        const rust = rustStates[i];
        const jsvecx = jsvecxStates[i];
        
        if (flagChangingOpcodes.includes(rust.opcode) && rust.cc !== jsvecx.cc) {
            console.log(`Step ${i}: Opcode 0x${rust.opcode} at ${rust.pc}`);
            console.log(`  Rust CC:   0x${rust.cc} (${parseInt(rust.cc, 16).toString(2).padStart(8, '0')})`);
            console.log(`  JSVecx CC: 0x${jsvecx.cc} (${parseInt(jsvecx.cc, 16).toString(2).padStart(8, '0')})`);
            
            // Decodificar flags
            const rustFlags = parseInt(rust.cc, 16);
            const jsvecxFlags = parseInt(jsvecx.cc, 16);
            
            console.log('  Flags differences:');
            console.log(`    Carry (C):    Rust=${(rustFlags & 1) ? '1' : '0'}, JSVecx=${(jsvecxFlags & 1) ? '1' : '0'}`);
            console.log(`    Overflow (V): Rust=${(rustFlags & 2) ? '1' : '0'}, JSVecx=${(jsvecxFlags & 2) ? '1' : '0'}`);
            console.log(`    Zero (Z):     Rust=${(rustFlags & 4) ? '1' : '0'}, JSVecx=${(jsvecxFlags & 4) ? '1' : '0'}`);
            console.log(`    Negative (N): Rust=${(rustFlags & 8) ? '1' : '0'}, JSVecx=${(jsvecxFlags & 8) ? '1' : '0'}`);
            console.log('');
            break;
        }
    }
    
    // Verificar si estamos en código relacionado con vectores
    console.log('🎨 ANÁLISIS DE UBICACIONES DE CÓDIGO:');
    console.log('====================================');
    
    const vectorRelatedAddresses = ['F53F', 'F540', 'F548', 'F54A', 'F54D']; // Loop en F548-F54D
    const inVectorCode = rustStates.slice(10, 50).some(state => 
        vectorRelatedAddresses.includes(state.pc)
    );
    
    if (inVectorCode) {
        console.log('⚠️  CÓDIGO EJECUTADO EN ÁREA DE GENERACIÓN DE VECTORES!');
        console.log('   Las diferencias en registros podrían estar afectando:');
        console.log('   - Cálculo de coordenadas de vectores');
        console.log('   - Punteros a tablas de datos de vectores'); 
        console.log('   - Lógica de scaling o transformación');
        console.log('   - Timing de generación de vectores');
    }
    
    return {
        initialStateDifferences: {
            stackPointer: parseInt(rust0.s, 16) - parseInt(jsvecx0.s, 16),
            directPage: parseInt(rust0.dp, 16) - parseInt(jsvecx0.dp, 16),
            conditionCodes: parseInt(rust0.cc, 16) - parseInt(jsvecx0.cc, 16)
        },
        xRegisterPattern: {
            rustIncrements: uniqueDiffs.length === 1 && uniqueDiffs[0] > 0,
            jsvecxConstant: uniqueDiffs.includes(uniqueDiffs[0]),
            incrementValue: uniqueDiffs[0] || 0
        },
        inVectorCode
    };
}

function generateRecommendations(analysis) {
    console.log('\n💡 RECOMENDACIONES PARA CORREGIR VECTORES:');
    console.log('==========================================');
    
    if (analysis.xRegisterPattern.rustIncrements) {
        console.log('1. 🎯 REGISTRO X - CRÍTICO:');
        console.log('   - Verificar implementación de post-incremento en instrucciones');
        console.log('   - El registro X probablemente se usa como puntero de datos');
        console.log('   - Verificar instrucciones: LEAX, STX, LDX con post-incremento');
        console.log('   - Prioridad: ALTA - puede causar vectores incorrectos\n');
    }
    
    if (Math.abs(analysis.initialStateDifferences.conditionCodes) > 0) {
        console.log('2. 🏁 FLAGS DE CONDICIÓN:');
        console.log('   - Verificar implementación de flags Z, N, V, C');
        console.log('   - Pueden afectar branches condicionales en generación de vectores');
        console.log('   - Verificar instrucciones: CMP, TST, SUB, ADD');
        console.log('   - Prioridad: MEDIA\n');
    }
    
    if (Math.abs(analysis.initialStateDifferences.stackPointer) > 0) {
        console.log('3. 📚 STACK POINTER:');
        console.log('   - Verificar inicialización del stack pointer');
        console.log('   - Puede afectar llamadas a funciones y retornos');
        console.log('   - Prioridad: BAJA para vectores directamente\n');
    }
    
    console.log('🔧 ACCIONES RECOMENDADAS:');
    console.log('1. Revisar implementación de post-incremento en emulador Rust');
    console.log('2. Comparar con implementación de referencia en JSVecx');
    console.log('3. Ejecutar tests específicos de generación de vectores');
    console.log('4. Verificar si los vectores mal generados correlacionan con uso del registro X');
}

if (require.main === module) {
    const analysis = analyzeRegisterDifferences();
    generateRecommendations(analysis);
}