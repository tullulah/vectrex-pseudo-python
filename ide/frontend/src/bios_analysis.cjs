#!/usr/bin/env node
/**
 * bios_analysis.cjs
 * Análisis detallado de la BIOS para entender el problema del reset vector
 */

const fs = require('fs');
const path = require('path');

try {
  console.log('=== ANÁLISIS DE BIOS ===');
  
  // Cargar BIOS
  const biosPath = path.join(__dirname, '..', 'public', 'bios.bin');
  const biosData = fs.readFileSync(biosPath);
  
  console.log(`BIOS cargada: ${biosData.length} bytes`);
  
  // Verificar reset vector en 0xFFFE-0xFFFF
  // La BIOS se mapea en 0xE000-0xFFFF (8KB), así que:
  // 0xFFFE = offset 0x1FFE en el archivo
  // 0xFFFF = offset 0x1FFF en el archivo
  
  const resetVectorLo = biosData[0x1FFE];
  const resetVectorHi = biosData[0x1FFF];
  const resetVector = (resetVectorHi << 8) | resetVectorLo;
  
  console.log(`\nReset vector en archivo BIOS:`);
  console.log(`  Offset 0x1FFE (0xFFFE): 0x${resetVectorLo.toString(16).padStart(2, '0').toUpperCase()}`);
  console.log(`  Offset 0x1FFF (0xFFFF): 0x${resetVectorHi.toString(16).padStart(2, '0').toUpperCase()}`);
  console.log(`  Reset vector: 0x${resetVector.toString(16).padStart(4, '0').toUpperCase()}`);
  
  // Buscar direcciones de inicio típicas de Vectrex
  console.log(`\n=== BÚSQUEDA DE PUNTOS DE ENTRADA TÍPICOS ===`);
  
  // Verificar si 0x00F0 debería mapear a algún offset de la BIOS
  if (resetVector >= 0xE000 && resetVector <= 0xFFFF) {
    const biosOffset = resetVector - 0xE000;
    console.log(`\nReset vector 0x${resetVector.toString(16).toUpperCase()} está en BIOS:`);
    console.log(`  Offset en BIOS: 0x${biosOffset.toString(16).padStart(4, '0').toUpperCase()}`);
    
    // Mostrar primeros bytes en esa dirección
    console.log(`  Primeros 16 bytes en reset vector:`);
    for (let i = 0; i < 16 && (biosOffset + i) < biosData.length; i++) {
      const byte = biosData[biosOffset + i];
      const addr = resetVector + i;
      console.log(`    0x${addr.toString(16).padStart(4, '0').toUpperCase()}: 0x${byte.toString(16).padStart(2, '0').toUpperCase()}`);
    }
  } else if (resetVector < 0x8000) {
    console.log(`\nReset vector 0x${resetVector.toString(16).toUpperCase()} está en RAM/Cartridge, no en BIOS`);
    console.log(`  Esto significa que 0x${resetVector.toString(16).padStart(4, '0').toUpperCase()} debería contener código del cartucho o RAM`);
  }
  
  // Buscar patrones típicos de inicio de BIOS
  console.log(`\n=== ANÁLISIS DE PATRONES DE INICIO ===`);
  
  // Buscar instrucciones típicas de inicio (LDS, JSR, etc.)
  const commonStartPatterns = [
    { name: 'LDS #$CBEA', pattern: [0x10, 0x8E, 0xCB, 0xEA] },
    { name: 'JSR extended', pattern: [0xBD] },
    { name: 'LDA immediate', pattern: [0x86] },
    { name: 'CLRA', pattern: [0x4F] },
    { name: 'CLRB', pattern: [0x5F] }
  ];
  
  for (const p of commonStartPatterns) {
    for (let i = 0; i <= biosData.length - p.pattern.length; i++) {
      let matches = true;
      for (let j = 0; j < p.pattern.length; j++) {
        if (biosData[i + j] !== p.pattern[j]) {
          matches = false;
          break;
        }
      }
      if (matches) {
        const biosAddr = 0xE000 + i;
        console.log(`  Found ${p.name} at BIOS 0x${biosAddr.toString(16).padStart(4, '0').toUpperCase()} (offset 0x${i.toString(16).padStart(4, '0').toUpperCase()})`);
        if (i < 10) break; // Solo mostrar primeras coincidencias
      }
    }
  }
  
  // Mostrar entradas de vectores de interrupción conocidas
  console.log(`\n=== TABLA DE VECTORES DE INTERRUPCIÓN ===`);
  const vectors = [
    { name: 'SWI3', addr: 0xFFF2 },
    { name: 'SWI2', addr: 0xFFF4 },
    { name: 'FIRQ', addr: 0xFFF6 },
    { name: 'IRQ', addr: 0xFFF8 },
    { name: 'SWI', addr: 0xFFFA },
    { name: 'NMI', addr: 0xFFFC },
    { name: 'RESET', addr: 0xFFFE }
  ];
  
  for (const v of vectors) {
    const offset = v.addr - 0xE000;
    if (offset >= 0 && offset < biosData.length - 1) {
      const lo = biosData[offset];
      const hi = biosData[offset + 1];
      const vector = (hi << 8) | lo;
      console.log(`  ${v.name.padEnd(6)}: 0x${vector.toString(16).padStart(4, '0').toUpperCase()}`);
    }
  }
  
  // Verificar inicio real de código ejecutable
  console.log(`\n=== ANÁLISIS DEL CÓDIGO EN 0xF000 ===`);
  const f000Offset = 0xF000 - 0xE000; // 0x1000
  if (f000Offset < biosData.length) {
    console.log(`Código en 0xF000 (offset 0x${f000Offset.toString(16).toUpperCase()}):`);
    for (let i = 0; i < 32; i++) {
      const byte = biosData[f000Offset + i];
      const addr = 0xF000 + i;
      let annotation = '';
      if (i === 0 && byte === 0x10) annotation = ' ; Page 2 prefix';
      if (i === 1 && byte === 0x8E) annotation = ' ; LDS immediate';
      if (i === 4 && byte === 0xBD) annotation = ' ; JSR extended';
      
      console.log(`  0x${addr.toString(16).padStart(4, '0').toUpperCase()}: 0x${byte.toString(16).padStart(2, '0').toUpperCase()}${annotation}`);
    }
  }
  
} catch (error) {
  console.error(`Error: ${error.message}`);
  process.exit(1);
}