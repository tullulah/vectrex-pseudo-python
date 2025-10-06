# Comparative Testing Framework - Implementation Summary

## Status: FRAMEWORK STRUCTURE COMPLETE ✅ | API INTEGRATION PENDING ⏸️

Fecha: 2025-10-06

## Componentes Implementados

### 1. Vectrexy Runner (C++ Wrapper) ✅
**Ubicación**: `emulator_v2/tests/comparative/vectrexy_runner/`

**Archivos**:
- `main.cpp` - Wrapper que ejecuta Vectrexy y serializa estado a JSON
- `CMakeLists.txt` - Configuración de build con linkeo a librerías Vectrexy

**Funcionalidad**:
```cpp
vectrexy_runner test.bin 500
→ Ejecuta test.bin por 500 ciclos
→ Output: JSON con estado CPU, VIA, vectores, audio
```

**Características**:
- Serializa registros CPU (PC, A, B, X, Y, U, S, DP, CC)
- Serializa estado VIA (IFR, IER, Timer1/2, Port A/B, ShiftReg)
- Serializa vectores generados (p0, p1, brightness)
- Usa nlohmann/json para formato estándar

**Estado**: PENDIENTE CMAKE BUILD (requiere Vectrexy compilado)

---

### 2. Rust Runner ⏸️
**Ubicación**: `emulator_v2/tests/comparative/rust_runner/`

**Archivos**:
- `Cargo.toml` - Configuración con workspace vacío para evitar conflicts
- `src/main.rs` - Runner que ejecuta emulator_v2 y serializa estado

**Problemas detectados**:
1. ❌ API mismatch - emulator_v2 no expone `cpu()`, `cpu_mut()` públicamente
2. ❌ `emulator_types` no existe - usa `engine_types` en core
3. ❌ `load_bios()` espera `&str` (ruta archivo), no `&[u8]` (datos)
4. ❌ `Cycles` type mismatch - `.0` no accesible directamente

**Solución requerida**:
- Opción A: Agregar getters públicos a `Emulator` para estado CPU/VIA
- Opción B: Crear módulo `test_utils` con helpers de introspección
- Opción C: Usar WASM API como referencia (ya expone métodos de lectura)

**Estado**: COMPILACIÓN FALLIDA - API refactor needed

---

### 3. Comparison Tool (Python) ✅
**Ubicación**: `emulator_v2/tests/comparative/compare.py`

**Funcionalidad**:
```python
python compare.py expected.json vectrexy.json rust.json
→ Compara recursivamente todos los campos
→ Reporta diferencias con colores ANSI
→ Exit code 0=PASS, 1=FAIL
```

**Características**:
- Tolerancia configurable para floats (default 0.01)
- Comparación profunda de dicts/arrays anidados
- Clasificación de severidad (RUST_DIFF, VECTREXY_DIFF, BOTH_DIFF)
- Output colorido con symbols (✅❌⚠️)
- Cálculo de deltas para valores numéricos

**Estado**: COMPLETO ✅

---

### 4. Test Case: IRQ Timer1 ✅
**Ubicación**: `emulator_v2/tests/comparative/test_cases/irq_timer1/`

**Archivos**:
- `test.asm` - Assembly source con test de interrupt Timer1
- `test.bin` - Binary ensamblado (generado por lwasm)
- `expected.json` - Estado esperado tras 500 ciclos

**Test scenario**:
```asm
1. ORCC #$10        - Disable IRQ (set I flag)
2. LDA #$40         - Enable Timer1 interrupt (IER bit 6)
   STA $D00E
3. LDA #$64         - Set Timer1 counter = 100
   STA $D004
4. ANDCC #$EF       - Enable IRQ (clear I flag)
5. NOP loop         - Wait for interrupt
→ After ~100 cycles: Timer1 expires → IRQ fires
→ Expected: IFR bit 6 set, PC jumps to BIOS handler
```

**Estado**: ENSAMBLADO OK ✅ (lwasm funcional)

---

### 5. Automation Script (PowerShell) ✅
**Ubicación**: `emulator_v2/tests/comparative/run_test.ps1`

**Funcionalidad**:
```powershell
.\run_test.ps1 -TestCase irq_timer1 -Cycles 500
```

**Pasos automatizados**:
1. ✅ Ensambla test.asm → test.bin (usando lwasm.exe)
2. ⏸️ Build Rust runner (API mismatch pending)
3. ⏸️ Build Vectrexy runner (CMake setup needed)
4. ⏸️ Run Rust emulator → rust_output.json
5. ⏸️ Run Vectrexy → vectrexy_output.json
6. ⏸️ Compare outputs con compare.py

**Configuración**:
- lwasm.exe path: `C:\Users\...\ide\frontend\dist\tools\lwasm.exe` ✅
- Workspace detection: Cargo.toml con `[workspace]` vacío ✅
- Error handling: Try-catch con fallbacks ✅

**Estado**: PARCIALMENTE FUNCIONAL (assembly OK, Rust runner falla)

---

## Próximos Pasos (Prioridad)

### INMEDIATO - Fix Rust Runner API
**Archivo**: `rust_runner/src/main.rs`

**Cambios requeridos**:
1. Usar API pública correcta de `Emulator`:
   ```rust
   // ANTES (no existe):
   use vectrex_emulator_v2::emulator_types::{...};
   
   // DESPUÉS (correcto):
   use vectrex_emulator_v2::core::engine_types::{AudioContext, Input, RenderContext};
   use vectrex_emulator_v2::core::emulator::Emulator;
   ```

2. Agregar getters públicos a `Emulator` (en `emulator.rs`):
   ```rust
   impl Emulator {
       pub fn get_cpu(&self) -> &Cpu6809 { &self.cpu }
       pub fn get_via(&self) -> &Via6522 { unsafe { &*self.via.get() } }
       // ... o usar WASM API como referencia
   }
   ```

3. Ajustar método de carga BIOS:
   ```rust
   // Opción A: Pasar ruta archivo
   emulator.load_bios(bios_path).unwrap();
   
   // Opción B: Agregar load_bios_data(&[u8]) method
   emulator.load_bios_data(&bios_data).unwrap();
   ```

4. Fix Cycles type access:
   ```rust
   // ANTES:
   total_cycles += cycles.0 as u64;
   
   // DESPUÉS:
   total_cycles += cycles.to_u64(); // O implementar method
   ```

---

### CORTO PLAZO - Build Vectrexy Runner
**Requisitos**:
1. Vectrexy compilado: `vectrexy/build/libs/emulator/libvectrexy_emulator.a`
2. CMake setup:
   ```bash
   cd vectrexy_runner
   cmake -B build
   cmake --build build
   ```

3. nlohmann/json library:
   - Verificar en `vectrexy/external/nlohmann/include/`
   - Si no existe, descargar json.hpp

**Resultado esperado**:
```
vectrexy_runner/build/vectrexy_runner.exe
→ Ejecutable listo para usar en run_test.ps1
```

---

### MEDIANO PLAZO - Más Test Cases
**Test cases prioritarios**:

1. **FIRQ Trigger**:
   - Test FIRQ interrupt handling
   - Verify stack frame (PC, CC only)
   - Validate CC.F flag behavior

2. **Port A → DAC → Integrator X**:
   - Write value to Port A (0xD001)
   - Verify delayed propagation (VELOCITY_X_DELAY=6)
   - Check integrator_x accumulation

3. **MUX Select → Brightness**:
   - Port B MUX control
   - Port A value routing
   - Brightness/Y/Offset selection

4. **Vector Draw Complete**:
   - Reset0Ref sequence
   - Move to position
   - Draw line
   - Blank
   - Verify RenderContext output

**Formato estandarizado**:
```
test_cases/
├── firq_trigger/
│   ├── test.asm
│   ├── test.bin
│   └── expected.json
├── port_a_dac/
│   ├── test.asm
│   ├── test.bin
│   └── expected.json
└── ... (3 more)
```

---

## Arquitectura del Framework

```
┌─────────────────────────────────────────────────────────────┐
│                    run_test.ps1                             │
│                   (Orchestrator)                            │
└────────┬─────────────────────────────────────┬──────────────┘
         │                                     │
         ▼                                     ▼
┌─────────────────┐                   ┌─────────────────┐
│  lwasm.exe      │                   │ Rust Runner     │
│  test.asm       │                   │ (emulator_v2)   │
│  → test.bin     │                   │                 │
└─────────────────┘                   └────────┬────────┘
                                               │
         ┌─────────────────────────────────────┴────────┐
         │                                              │
         ▼                                              ▼
┌─────────────────┐                           ┌─────────────────┐
│ Vectrexy Runner │                           │ Test Binary     │
│ (C++ reference) │                           │ test.bin        │
│                 │                           │ (500 cycles)    │
└────────┬────────┘                           └─────────────────┘
         │                                              │
         │                                              │
         ▼                                              ▼
┌─────────────────┐                           ┌─────────────────┐
│ vectrexy.json   │                           │ rust.json       │
│ {cpu,via,vec}   │                           │ {cpu,via,vec}   │
└────────┬────────┘                           └────────┬────────┘
         │                                              │
         └──────────────────┬───────────────────────────┘
                            ▼
                   ┌─────────────────┐
                   │  compare.py     │
                   │  Diff Analysis  │
                   │  → PASS/FAIL    │
                   └─────────────────┘
```

---

## Beneficios del Framework

### 1. Validación Empírica 1:1
- Comparación bit-a-bit con implementación de referencia (Vectrexy)
- No más "claims" sin pruebas - todo verificable
- Detecta discrepancias sutiles (timing, side effects, flags)

### 2. Regression Testing
- Cada fix preservado como test case
- Re-run automático tras cambios
- Previene reintroducción de bugs

### 3. Debugging Estructurado
- Estado completo capturado (CPU, VIA, vectors)
- Formato JSON legible y diff-able
- Identifica exactamente QUÉ difiere y POR CUÁNTO

### 4. Documentación Viva
- Test cases son ejemplos ejecutables
- JSON outputs son especificación de comportamiento
- Framework es tutorial de emulación correcta

---

## Limitaciones Actuales

1. **WASM Runner Missing**: No hay wrapper WASM comparable (solo native Rust)
   - Solución: Agregar modo "test" a wasm_api.rs que exponga get_state_json()

2. **Timer State Opaque**: Timer1/2 counters no exportados desde Via6522
   - Solución: Agregar getters `get_timer1_counter()`, `get_timer2_counter()`

3. **Screen State Hidden**: Integrator X/Y no accesibles
   - Solución: Agregar `get_integrator_state()` en Via6522

4. **Manual Binary Creation**: Assembly manual para cada test
   - Solución: Script Python generador de tests sintéticos

---

## Conclusión

**Framework Status**: 70% complete
- ✅ Estructura completa
- ✅ Tooling funcional (lwasm, compare.py)
- ✅ Primer test case definido
- ⏸️ API refactor needed para rust_runner
- ⏸️ Vectrexy runner pending CMake build

**Next Action**: Fix API públicas en `Emulator` para permitir introspección de estado

**Valor**: Una vez completo, este framework será la **fuente de la verdad** para validar que emulator_v2 es 1:1 compatible con Vectrexy.
