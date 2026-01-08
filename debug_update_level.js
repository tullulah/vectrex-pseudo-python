// ============================================
// COMANDOS SIMPLES - Copia y pega en consola
// ============================================

// 1. Primero obtén el emulador (prueba estos en orden):
vecx = window.vecx;

// 2. Leer variables de debug (RESULT base = 0xCF10):
counter = (vecx.read8(0xCF2E) << 8) | vecx.read8(0xCF2F);
uSaved = (vecx.read8(0xCF30) << 8) | vecx.read8(0xCF31);
uAfter = (vecx.read8(0xCF32) << 8) | vecx.read8(0xCF33);
flags = vecx.read8(0xCF34);

// 3. Mostrar resultados:
console.log("Counter:", counter, "| U_saved:", "0x" + uSaved.toString(16), "| U_after:", "0x" + uAfter.toString(16), "| Flags:", flags);

// 4. Verificar si hay problema:
if (uSaved !== uAfter) console.error("❌ STACK CORRUPT"); else console.log("✅ Stack OK");

// 5. Leer registros CPU actuales:
console.log("CPU A:", vecx.e6809_a, "| B:", vecx.e6809_b, "| X:", "0x" + vecx.e6809_x.toString(16), "| Y:", "0x" + vecx.e6809_y.toString(16), "| U:", "0x" + vecx.e6809_u.toString(16), "| S:", "0x" + vecx.e6809_s.toString(16));
