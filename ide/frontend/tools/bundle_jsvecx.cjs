#!/usr/bin/env node
// Script: bundle_jsvecx.cjs
// Objetivo: concatenar los archivos preprocess de jsvecx en orden correcto en un único módulo ESM/UMD ligero
// Salida: public/jsvecx/vecx_full.js
// NOTA: Mantiene comentarios de licencia iniciales y añade export { VecX, Globals }.

const fs = require('fs');
const path = require('path');

const root = path.resolve(__dirname, '../../..'); // ide/frontend -> repo root
const srcDir = path.join(root, 'jsvecx', 'src', 'preprocess');
const outDirPublic = path.join(root, 'ide', 'frontend', 'public', 'jsvecx');
const outFilePublic = path.join(outDirPublic, 'vecx_full.js');
// Nueva salida adicional dentro de src para permitir import directo sin warning de Vite (no /public):
const outDirSrc = path.join(root, 'ide', 'frontend', 'src', 'generated', 'jsvecx');
const outFileSrc = path.join(outDirSrc, 'vecx_full.js');

const order = [
  'utils.js',      // fptr, utils
  'globals.js',    // Globals
  'vector_t.js',   // vector struct
  'header.js',     // macros / header pieces (si definen algo usado después)
  'e6809.js',      // CPU core
  'e8910.js',      // PSG
  'osint.js',      // OS integration helpers
  'vecx.js'        // main wrapper
];

if(!fs.existsSync(srcDir)){
  console.error('[bundle_jsvecx] Directorio preprocess no encontrado:', srcDir);
  process.exit(1);
}

fs.mkdirSync(outDirPublic, { recursive: true });
fs.mkdirSync(outDirSrc, { recursive: true });
let banner = `/* jsvecx bundle generado automáticamente.\n   Fuente: ${srcDir}\n   Fecha: ${new Date().toISOString()}\n   Licencia: se asume misma que los archivos originales (gnu). */\n`;
let concat = banner;

for(const fname of order){
  const p = path.join(srcDir, fname);
  if(!fs.existsSync(p)){
    console.warn('[bundle_jsvecx] archivo faltante', fname);
    continue;
  }
  let content = fs.readFileSync(p, 'utf8');
  // Quitar potential export statements previos si existieran (no hay en preprocess, pero defensa)
  content = content.replace(/export\s+\{[^}]*\};?/g, '');
  // Transformar macros #define simples a JS válido (evita SyntaxError en runtime):
  // Casos manejados:
  // 1) #define NAME value  => const NAME = value;
  // 2) #define NAME(arglist) expr => se convierte en función inline const NAME = (...args)=> (expr)
  // 3) #define GETCC( flag ) ((this.reg_cc/flag>>0)&1) (espacios permitidos)
  // Limitaciones: no maneja macros multiline ni substituciones complejas con tokens. Suficiente para jsvecx preprocess.
  content = content.replace(/^#define\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s+([^\n\r]+)/gm,
    (m,name,args,body)=>`const ${name} = (${args.trim()}) => (${body.trim()});`);
  content = content.replace(/^#define\s+([A-Za-z_][A-Za-z0-9_]*)\s+([^\n\r]+)/gm,
    (m,name,val)=>`const ${name} = ${val.trim()};`);
  // Eliminar bloques #if 0 ... #endif (código muerto / structs C no usados):
  content = content.replace(/#if\s+0[\s\S]*?#endif/gm, '');
  concat += `\n/* BEGIN ${fname} */\n` + content + `\n/* END ${fname} */\n`;
}

// Asegurar que Globals y VecX estén en ámbito global del bundle para export.
concat += `\n// Export explícito para import dinámico ESM\nexport { VecX, Globals };\n`;

fs.writeFileSync(outFilePublic, concat, 'utf8');
fs.writeFileSync(outFileSrc, concat, 'utf8');
console.log('[bundle_jsvecx] Bundle generado en', outFilePublic, 'y', outFileSrc);
