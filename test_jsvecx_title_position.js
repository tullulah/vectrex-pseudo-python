// Comparación Rust vs JSVecx: Análisis de posición del título "MINE STORM"
// Objetivo: Verificar si el offset de -11.6 también aparece en JSVecx

const fs = require('fs');
const path = require('path');

// Cargar JSVecx
const jsvecxPath = 'ide/frontend/public/jsvecx_deploy';
global.vecx = {};  // Necesario para JSVecx
require(`./${jsvecxPath}/utils.js`);
require(`./${jsvecxPath}/globals.js`);
require(`./${jsvecxPath}/e6809.js`);
require(`./${jsvecxPath}/e8910.js`);
require(`./${jsvecxPath}/vecx.js`);
require(`./${jsvecxPath}/osint.js`);

// Cargar BIOS
const biosPath = 'ide/frontend/dist/bios.bin';
const biosData = fs.readFileSync(biosPath);
console.log(`BIOS loaded: ${biosData.length} bytes\n`);

// Inicializar JSVecx
vecx_reset();
for (let i = 0; i < biosData.length; i++) {
    vecx.rom[0xE000 - 0x8000 + i] = biosData[i];
}

console.log("=== JSVECX TEXT POSITION ANALYSIS ===\n");

// Ejecutar suficientes ciclos para que aparezca el título
const targetCycles = 300000;
let totalCycles = 0;

while (totalCycles < targetCycles) {
    vecx_emu(1000);
    totalCycles += 1000;
}

console.log(`Executed ${totalCycles} cycles\n`);

// Analizar vectores generados (en JSVecx están en vecx.vectors)
// Nota: JSVecx puede tener estructura diferente para vectores
console.log("JSVecx vectors structure:");
console.log("  vecx object keys:", Object.keys(vecx).filter(k => k.includes('vec') || k.includes('line')).slice(0, 10));

// Intentar acceder a los vectores
if (vecx.vectors && Array.isArray(vecx.vectors)) {
    console.log(`\nTotal vectors: ${vecx.vectors.length}`);
    
    // Filtrar líneas horizontales en región del título (Y entre 15 y 40)
    const titleLines = vecx.vectors.filter(v => {
        if (!v) return false;
        const y_center = (v.y0 + v.y1) / 2.0;
        const dx = Math.abs(v.x1 - v.x0);
        const dy = Math.abs(v.y1 - v.y0);
        return y_center > 15 && y_center < 40 && dy < 2.0 && dx > 0.5;
    });
    
    if (titleLines.length > 0) {
        const avgX = titleLines.reduce((sum, v) => sum + (v.x0 + v.x1) / 2.0, 0) / titleLines.length;
        
        console.log(`\nTitle region (Y: 15-40):`);
        console.log(`  Horizontal lines: ${titleLines.length}`);
        console.log(`  Average X center: ${avgX.toFixed(2)}`);
        console.log(`  Offset from origin: ${avgX.toFixed(2)}`);
        
        if (avgX < -10) {
            console.log(`  ⚠️  Title shifted LEFT by ${(-avgX).toFixed(1)} units`);
        } else if (avgX > 10) {
            console.log(`  ⚠️  Title shifted RIGHT by ${avgX.toFixed(1)} units`);
        } else {
            console.log(`  ✅ Title properly centered`);
        }
    } else {
        console.log("\n⚠️  No title lines found in expected region");
    }
} else {
    console.log("\n⚠️  JSVecx vectors not found or incompatible structure");
    console.log("Available vecx properties:", Object.keys(vecx).slice(0, 20));
}
