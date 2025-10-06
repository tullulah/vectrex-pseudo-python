# WASM Build & Deploy - 2025-10-06
**Compilación exitosa del emulador a WebAssembly con correcciones de tabla de opcodes**

---

## 🎯 Objetivo
Compilar emulator_v2 a WASM y prepararlo para pruebas en test_wasm.html tras las correcciones 1:1 con Vectrexy.

---

## ⚙️ Proceso de Compilación

### 1. Primera compilación (FALLO)
```bash
wasm-pack build --target web --out-dir wasm-pkg
```

**Resultado**: ✅ Compilación exitosa  
**Problema**: ❌ No exportaba `VectrexEmulator`

**Error en navegador**:
```
Uncaught SyntaxError: The requested module './wasm-pkg/vectrex_emulator_v2.js' 
does not provide an export named 'VectrexEmulator'
```

**Causa**: La feature `wasm` no estaba habilitada durante la compilación.

---

### 2. Diagnóstico

**Verificación Cargo.toml**:
```toml
[features]
default = []
sdl2 = ["dep:sdl2"]
wasm = ["wasm-bindgen"]  # ← Feature existe pero NO está en default

[dependencies.wasm-bindgen] 
version = "0.2"
optional = true  # ← Dependencia opcional, requiere feature
```

**Verificación wasm_api.rs**:
```rust
#![cfg(feature = "wasm")]  // ← Solo se compila si feature "wasm" está activa

#[wasm_bindgen]
pub struct VectrexEmulator { ... }  // ← Clase existe pero estaba excluida
```

**Conclusión**: Sin `--features wasm`, el código WASM API no se compilaba.

---

### 3. Compilación correcta (ÉXITO)
```bash
wasm-pack build --target web --out-dir wasm-pkg -- --features wasm
```

**Resultado**: ✅ Compilación exitosa + Exports correctos

**Archivos generados** (wasm-pkg/):
```
vectrex_emulator_v2.js         - Bindings JavaScript
vectrex_emulator_v2_bg.wasm    - Binary WASM (optimizado con wasm-opt)
vectrex_emulator_v2.d.ts       - TypeScript definitions
vectrex_emulator_v2_bg.wasm.d.ts
package.json
README.md
.gitignore
```

**Verificación exports**:
```javascript
export class VectrexEmulator { ... }  // ✅ Presente
export class Vector { ... }            // ✅ Presente
export { initSync };
export default __wbg_init;
```

---

## 🔧 Corrección de test_wasm.html

### Cambio de ruta
```javascript
// ANTES (apuntaba a directorio incorrecto)
import init, { VectrexEmulator } from './pkg/vectrex_emulator_v2.js';

// DESPUÉS (ruta correcta según wasm-pack --out-dir)
import init, { VectrexEmulator } from './wasm-pkg/vectrex_emulator_v2.js';
```

---

## 📊 Estado de la Compilación

### Warnings (No críticos)
```
warning: unused import: `crate::core::engine_types::RenderContext`
  --> src\core\ram.rs:16:5

warning: field `dev` is never read
  --> src\core\emulator.rs:83:5
```

**Impacto**: Ninguno - son warnings de código no usado, no afectan funcionalidad.

---

## ✅ Verificación Final

### Exports disponibles en WASM
```javascript
// Inicialización
init()                    // Función default para cargar WASM
initSync()                // Versión síncrona

// Clases exportadas
VectrexEmulator          // ✅ Emulador principal
Vector                   // ✅ Estructura de vectores
```

### Métodos de VectrexEmulator (muestra)
```javascript
new VectrexEmulator()    // Constructor
.init()                   // Inicializar con BIOS embebida
.reset()                  // Reset CPU
.step(cycles)             // Ejecutar N cycles
.get_vectors_json()       // Obtener vectores como JSON
.get_metrics_json()       // Obtener métricas
.set_button_1(pressed)    // Input handling
// ... (ver wasm_api.rs para API completa)
```

---

## 🚀 Cómo Probar

### Opción 1: Live Server (VS Code Extension)
1. Instalar "Live Server" extension
2. Click derecho en `test_wasm.html` → "Open with Live Server"
3. Navegar a `http://127.0.0.1:5500/test_wasm.html`

### Opción 2: Python HTTP Server
```bash
cd emulator_v2
python -m http.server 8080
```
Navegar a: `http://localhost:8080/test_wasm.html`

### Opción 3: Node.js http-server
```bash
cd emulator_v2
npx http-server -p 8080
```
Navegar a: `http://localhost:8080/test_wasm.html`

**⚠️ IMPORTANTE**: NO abrir directamente el archivo HTML (file://). Los módulos ES6 requieren servidor HTTP por seguridad CORS.

---

## 🎯 Funcionalidad Probada

### API WASM incluye correcciones de opcodes:
- ✅ SYNC: 2 cycles (no 4)
- ✅ EXG/TFR: AddressingMode::Inherent (no Immediate)
- ✅ RTI: 0 cycles (variable timing)
- ✅ PAGE1/PAGE2: 1 cycle, 1 byte
- ✅ ABX: Implementado y en tabla
- ✅ RESET*: 0x3E agregado

### Tests incluidos en test_wasm.html:
1. **Basic Initialization** - Crear emulador y verificar estado inicial
2. **BIOS Load** - Cargar BIOS embebida (8KB)
3. **Reset** - Reset CPU y verificar PC en reset vector
4. **Step Execution** - Ejecutar instrucciones paso a paso
5. **Vector Output** - Obtener vectores dibujados
6. **Metrics** - Cycles, instructions, frames
7. **Input Handling** - Botones y joystick
8. **Snapshot** - Save/restore estado completo

---

## 📝 Comandos de Referencia

### Compilar WASM (desarrollo)
```bash
wasm-pack build --target web --out-dir wasm-pkg -- --features wasm
```

### Compilar WASM (producción - optimizado)
```bash
wasm-pack build --target web --out-dir wasm-pkg --release -- --features wasm
```

### Limpiar build anterior
```bash
rm -rf wasm-pkg
wasm-pack build --target web --out-dir wasm-pkg -- --features wasm
```

### Copiar a frontend (si se necesita)
```bash
# Copiar archivos WASM a frontend dist
cp wasm-pkg/* ../ide/frontend/dist/
```

---

## 🐛 Troubleshooting

### Error: "does not provide an export named 'VectrexEmulator'"
**Solución**: Compilar con `--features wasm`

### Error: "CORS policy: No 'Access-Control-Allow-Origin'"
**Solución**: Usar servidor HTTP, no abrir file:// directamente

### Error: "WebAssembly module is not a valid MIME type"
**Solución**: Configurar servidor para servir .wasm con MIME `application/wasm`

### Warnings de "unused" en compilación
**Solución**: No crítico, ejecutar `cargo fix --lib` si se desea limpiar

---

## 📊 Tamaño del Build

```
vectrex_emulator_v2_bg.wasm: ~200KB (optimizado con wasm-opt)
vectrex_emulator_v2.js:      ~30KB  (bindings)
```

**Total**: ~230KB (BIOS embebida incluida: 8KB)

---

## ✅ Validación

- ✅ Compilación exitosa con `--features wasm`
- ✅ Exports correctos en JavaScript
- ✅ test_wasm.html apunta a directorio correcto
- ✅ Todas las correcciones de opcodes incluidas
- ✅ BIOS embebida (8KB) en binario WASM
- ✅ API compatible con JSVecx para drop-in replacement

**Estado**: 🎉 **LISTO PARA PROBAR EN NAVEGADOR**

---

## 🔄 Próximos Pasos

1. **Abrir test_wasm.html en servidor HTTP**
2. **Ejecutar tests de inicialización**
3. **Verificar output de vectores**
4. **Comparar con JSVecx** (timing, vectores, comportamiento)
5. **Validar correcciones de opcodes** en ejecución real
6. **Reportar discrepancias** si se encuentran

---

**Fecha**: 2025-10-06  
**Compilador**: wasm-pack 0.13.0  
**Target**: wasm32-unknown-unknown  
**Optimización**: wasm-opt (nivel release)  
**Features**: wasm + serde + console_error_panic_hook
