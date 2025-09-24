#!/usr/bin/env node
// bundle_jsvecx.cjs (verbose + AY sanitizer + canonical single output)
// Salida única: ide/frontend/src/generated/jsvecx/vecx_full.js
// Uso:
//   node ide/frontend/tools/bundle_jsvecx.cjs --verbose
//   BUNDLE_VERBOSE=1 node ide/frontend/tools/bundle_jsvecx.cjs
//
// Objetivo:
//   1. Concatenar preprocess en orden definido.
//   2. Convertir #define simples y con argumentos a const / arrow minimal.
//   3. Eliminar bloques #if 0 ... #endif.
//   4. Forzar bloque de registros AY_* a constantes canónicas (sanitizer).
//   5. Exportar { VecX, Globals }.
//
// NOTA: Si más adelante migramos a un parser determinista de macros, este archivo
// se simplificará y se anotará en SUPER_SUMMARY.md.

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const verbose = process.env.BUNDLE_VERBOSE === '1' || process.argv.includes('--verbose');
const startTs = Date.now();
console.log(`[bundle_jsvecx] START file=${__filename} cwd=${process.cwd()} verbose=${verbose}`);

const root   = path.resolve(__dirname, '../../..');
const srcDir = path.join(root, 'jsvecx', 'src', 'preprocess');
const outDir = path.join(root, 'ide', 'frontend', 'src', 'generated', 'jsvecx');
const outFile = path.join(outDir, 'vecx_full.js');

// Orden estricto (dependencias implícitas)
const order = [
  'utils.js',
  'globals.js',
  'vector_t.js',
  'header.js',
  'e6809.js',
  'e8910.js',
  'osint.js',
  'vecx.js'
];

if (!fs.existsSync(srcDir)) {
  console.error('[bundle_jsvecx] ERROR: No existe el directorio source:', srcDir);
  process.exit(1);
}
fs.mkdirSync(outDir, { recursive: true });

function transform(fname, code) {
  const macroTotal = (code.match(/^#define\s+/gm) || []).length;

  // Quitar exports residuales
  code = code.replace(/export\s+\{[^}]*\};?/g, '');

  // Macros con argumentos (#define NAME(arg1,arg2) body)
  let fnCount = 0;
  code = code.replace(
    /^#define\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s+([^\n\r]+)/gm,
    (m, n, args, body) => {
      fnCount++;
      return `const ${n} = (${args.trim()}) => (${body.trim()});`;
    }
  );

  // Macros simples (#define NAME value)
  let simpleCount = 0;
  code = code.replace(
    /^#define\s+([A-Za-z_][A-Za-z0-9_]*)\s+([^\n\r]+)/gm,
    (m, n, v) => {
      simpleCount++;
      return `const ${n} = ${v.trim()};`;
    }
  );

  const beforeStrip = code.length;
  code = code.replace(/#if\s+0[\s\S]*?#endif/gm, '');
  if (verbose) {
    console.log(
      `[bundle_jsvecx] ${fname} macros total=${macroTotal} fn=${fnCount} simple=${simpleCount} stripDelta=${beforeStrip - code.length}`
    );
  }
  return code;
}

// Sanitizer AY: fuerza bloque limpio y elimina corrupción tipo
// const AY_AFINE = (0) => (#define AY_ACOARSE (1));
function sanitizeAY(bundleText) {
  const corruption = /AY_AFINE[^\n]*=>\s*\(#define\s+AY_ACOARSE/;
  const alreadyClean =
    /const\s+AY_AFINE\s*=\s*0;/.test(bundleText) &&
    !corruption.test(bundleText);

  if (alreadyClean) {
    if (verbose) console.log('[bundle_jsvecx] AY ya estaba limpio');
    return bundleText;
  }
  if (corruption.test(bundleText)) {
    console.warn('[bundle_jsvecx] Detectada corrupción AY; se aplica reparación canónica');
  } else {
    if (verbose) console.log('[bundle_jsvecx] Reescribiendo bloque AY por consistencia');
  }

  const canon = [
    'const AY_AFINE = 0;',
    'const AY_ACOARSE = 1;',
    'const AY_BFINE = 2;',
    'const AY_BCOARSE = 3;',
    'const AY_CFINE = 4;',
    'const AY_CCOARSE = 5;',
    'const AY_NOISEPER = 6;',
    'const AY_ENABLE = 7;',
    'const AY_AVOL = 8;',
    'const AY_BVOL = 9;',
    'const AY_CVOL = 10;',
    'const AY_EFINE = 11;',
    'const AY_ECOARSE = 12;',
    'const AY_ESHAPE = 13;',
    'const AY_PORTA = 14;',
    'const AY_PORTB = 15;'
  ].join('\n');

  // Elimina cualquier línea previa const/macros AY_*
  let cleaned = bundleText
    .replace(/const\s+AY_[A-Z0-9_]+\s*=.*\n/g, '')
    .replace(/#define\s+AY_[A-Z0-9_]+.*\n/g, '');

  // Insertar bloque justo tras comentario BEGIN e8910.js si existe
  cleaned = cleaned.replace(
    /\/\* BEGIN e8910\.js \*\//,
    `/* BEGIN e8910.js */\n// [sanitized AY canonical]\n${canon}`
  );

  // Fallback si no encontró el bloque
  if (!/const\s+AY_AFINE\s*=\s*0;/.test(cleaned)) {
    cleaned = `// [sanitized AY canonical]\n${canon}\n` + cleaned;
  }
  return cleaned;
}

// Construcción
let bundle = `/* jsvecx bundle ${new Date().toISOString()} */\n`;
for (const f of order) {
  const p = path.join(srcDir, f);
  if (!fs.existsSync(p)) {
    console.warn('[bundle_jsvecx] Falta archivo', f);
    continue;
  }
  if (verbose) console.log('[bundle_jsvecx] leyendo', f);
  const raw = fs.readFileSync(p, 'utf8');
  const tr = transform(f, raw);
  bundle += `\n/* BEGIN ${f} */\n${tr}\n/* END ${f} */\n`;
}

bundle += `\nexport { VecX, Globals };\n`;

// Hash antes
const hashPre = crypto.createHash('md5').update(bundle).digest('hex');
bundle = sanitizeAY(bundle);
const hashPost = crypto.createHash('md5').update(bundle).digest('hex');
if (verbose) {
  console.log(`[bundle_jsvecx] hash pre=${hashPre} post=${hashPost} changed=${hashPre !== hashPost}`);
}

// Escribir
fs.writeFileSync(outFile, bundle, 'utf8');
const stat = fs.statSync(outFile);
console.log(
  `[bundle_jsvecx] DONE -> ${outFile} size=${stat.size}B elapsed=${Date.now() - startTs}ms`
);

// Validaciones finales
const finalContent = fs.readFileSync(outFile, 'utf8');
if (/=>\s*\(#define\s+AY_ACOARSE/.test(finalContent)) {
  console.error('[bundle_jsvecx] ERROR: Persisten patrones corruptos AY');
  process.exitCode = 2;
} else if (!/const\s+AY_AFINE\s*=\s*0;/.test(finalContent)) {
  console.warn('[bundle_jsvecx] WARN: Bloque canónico AY no encontrado (verificar sanitize)');
} else if (verbose) {
  console.log('[bundle_jsvecx] AY OK (bloque canónico presente)');
}