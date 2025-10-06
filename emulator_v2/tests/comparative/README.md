# Comparative Testing: Rust vs Vectrexy

## Objetivo

Validar que nuestro emulador Rust es **1:1 con Vectrexy** ejecutando exactamente el mismo código máquina y comparando:

1. **Estado CPU** después de N instrucciones
2. **Estado VIA** (timers, interrupts, ports)
3. **Vectores generados** (cantidad, coordenadas, brightness)
4. **Audio samples** generados

## Uso del Framework

### Ejecución de Tests

```powershell
# Test completo (ensamblado + vectrexy + rust + comparación)
.\run_comparative_test_v2.ps1 -TestName cpu_arithmetic -Cycles 50

# Skip rebuild de runners (más rápido para tests repetidos)
.\run_comparative_test_v2.ps1 -TestName cpu_arithmetic -Cycles 50 -SkipBuild
```

### Output del Test

El script genera SIEMPRE en cada ejecución:
- `vectrexy_output.json` - Output de Vectrexy C++ (CPU + VIA parcial*)
- `rust_output.json` - Output de Rust (CPU + VIA + vectors + audio)
- `vectrexy_debug.log` - Debug log de Vectrexy (stderr)

**NOTA CRÍTICA**: `vectrexy_output.json` es la **referencia generada dinámicamente** en cada test run. NO usar `expected.json` estático.

### Interpretación de Resultados

✅ **TEST PASSED**: Rust matchea perfectamente con Vectrexy
❌ **TEST FAILED**: Diferencias detectadas (ver sección VIA Limitation abajo)

**Diferencias VIA conocidas**:
- `via.ifr`, `via.ier`, `via.timerX_counter`: Vectrexy returna 0 (SEH crash), Rust returna valores reales
- **Solución**: Verificar valores Rust contra código fuente Vectrexy (`Timers.h`, `Via.cpp`)

---

## ⚠️ LIMITACIÓN CRÍTICA: VIA Reads en Vectrexy

**vectrexy_runner NO puede leer registros VIA después de ejecutar instrucciones** debido a assertions internas en el código C++ de Vectrexy que disparan SEH exceptions.

### Impacto

- ✅ **CPU registers**: Validación completa contra Vectrexy (PC, A, B, X, Y, U, S, DP, CC)
- ❌ **VIA registers**: Vectrexy crashea al leer IFR/IER/Timers post-ejecución
- ✅ **Solución**: Validar VIA contra **código fuente de Vectrexy**, no valores runtime

### Estrategia de Testing

1. **Tests CPU-only** (actual):
   - `expected.json` generado por vectrexy_runner (CPU fields)
   - VIA fields generados por `rust_runner` (verificados contra código fuente Vectrexy)
   
2. **Tests VIA** (futuro):
   - Unit tests que replican lógica de `Timers.h`, `Via.cpp`
   - Verificación 1:1 contra código fuente C++
   - NO comparación runtime contra vectrexy_runner (imposible)

3. **Tests de integración** (futuro):
   - Screen, PSG, ShiftRegister
   - Validación contra comportamiento documentado

Ver [VECTREXY_VIA_LIMITATION.md](VECTREXY_VIA_LIMITATION.md) para detalles completos.

---

## Estructura de Proyecto

```
comparative/
├── vectrexy_runner/        # Ejecutable C++ que carga Vectrexy
│   ├── main.cpp
│   ├── CMakeLists.txt
│   └── output_state.json   # Estado serializado
├── rust_runner/            # Ejecutable Rust con nuestro emulador
│   └── src/main.rs
└── test_cases/             # Casos de prueba (.bin + expected.json)
    ├── irq_timer1/
    ├── firq_trigger/
    ├── port_a_dac/
    ├── mux_brightness/
    └── vector_draw/
```

## Tests Críticos (Prioridad)

### 1. **IRQ Timer1** - CRÍTICO para BIOS
- Timer1 expira → genera IRQ
- CPU salta a vector IRQ (0xFFF8)
- Verifica: IFR, IER, PC, Stack

### 2. **FIRQ Trigger** - CRÍTICO para timing
- FIRQ se dispara
- CPU salta a vector FIRQ (0xFFF6)
- Verifica: CC.F flag, Stack frame

### 3. **Port A → DAC → Integrator X** - CRÍTICO para dibujado
- Escribe valor a Port A (0xD000)
- Verifica: Integrator X actualizado
- Timing: DelayedValueStore con VelocityXDelay=6

### 4. **MUX Select → Brightness/Y/Offset** - CRÍTICO para vectores
- Port B MUX=0 → Port A controla Y
- Port B MUX=1 → Port A controla XY offset
- Port B MUX=2 → Port A controla brightness
- Verifica: Screen internals

### 5. **Vector Draw Complete** - INTEGRACIÓN COMPLETA
- Secuencia completa: Reset0Ref → Move → Draw → Blank
- Verifica: Cantidad de líneas generadas
- Verifica: Coordenadas p0/p1
- Verifica: Brightness de cada línea

## Formato de Test Case

Cada test case contiene:

**`test.bin`** - Código máquina a ejecutar:
```asm
; Ejemplo: IRQ Timer1
LDA #$C0        ; Enable Timer1 interrupt
STA $D00E       ; IER register
LDA #$FF
STA $D004       ; Timer1 Low counter
STA $D005       ; Timer1 High counter
; Wait for interrupt...
```

**`expected.json`** - Estado esperado después de N ciclos:
```json
{
  "cycles": 1000,
  "cpu": {
    "pc": 0xF123,
    "a": 0x42,
    "b": 0x00,
    "x": 0x0000,
    "y": 0x0000,
    "u": 0xCFFF,
    "s": 0xCFF8,
    "dp": 0x00,
    "cc": {
      "c": false,
      "v": false,
      "z": false,
      "n": false,
      "i": true,
      "h": false,
      "f": false,
      "e": false
    }
  },
  "via": {
    "ifr": 0x40,
    "ier": 0xC0,
    "timer1_counter": 0,
    "port_a": 0x80,
    "port_b": 0x00
  },
  "vectors": {
    "count": 0,
    "lines": []
  }
}
```

## Ejecución

```bash
# 1. Compilar runner de Vectrexy
cd vectrexy_runner
cmake . && make
./vectrexy_runner test_cases/irq_timer1/test.bin > vectrexy_output.json

# 2. Compilar runner de Rust
cd ../rust_runner
cargo run --release -- ../test_cases/irq_timer1/test.bin > rust_output.json

# 3. Comparar
cd ..
python compare.py test_cases/irq_timer1/expected.json \
                  vectrexy_runner/vectrexy_output.json \
                  rust_runner/rust_output.json
```

## Output de Comparación

```
✅ PASS: test_cases/irq_timer1
  CPU State: MATCH
  VIA State: MATCH
  Vectors: MATCH (0 generated)

❌ FAIL: test_cases/vector_draw
  CPU State: MATCH
  VIA State: MATCH
  Vectors: MISMATCH
    Expected: 1 line
    Vectrexy: 1 line ✓
    Rust:     3 lines ✗
    
  Difference:
    Rust generated 2 extra lines:
      Line 1: (0, 0) → (10, 10) brightness=0.5  [DUPLICATE]
      Line 2: (0, 0) → (10, 10) brightness=0.5  [DUPLICATE]
```

## Próximos Pasos

1. ✅ Crear estructura de carpetas
2. ✅ Implementar `vectrexy_runner` (C++) - **COMPLETADO 2025-10-06**
3. ⏳ Implementar `rust_runner`
4. ⏳ Implementar `compare.py`
5. ✅ Escribir test cases críticos (3 CPU-only tests)
6. ⏳ Ejecutar y documentar diferencias

---

## UPDATE 2025-10-06: BREAKTHROUGH - Vectrexy Runner Working!

### ✅ Logros Conseguidos

**CRÍTICO**: Hemos logrado compilar y ejecutar **Vectrexy C++ emulador** como referencia:

1. **Compilado vectrexy_runner.exe** que ejecuta tests binarios en Vectrexy
2. **Generadas 3 referencias válidas** (`expected.json`) desde Vectrexy real
3. **Probado que funciona** - output JSON correcto con estado CPU

### 🎯 Tests CPU-Only Funcionando

Debido a limitación VIA (ver abajo), creamos tests que **solo usan CPU**:

| Test | Descripción | A | B | Cycles |
|------|-------------|---|---|--------|
| `cpu_arithmetic` | ADDA/ADDB | 0x30 | 0x55 | 50 |
| `cpu_load_store` | LDA/STA RAM | 0xAA | 0x00 | 101 |
| `cpu_branch` | BEQ/BNE | 0x00 | 0xFF | 101 |

**Cada test tiene**:
- ✅ `test.asm` - Código assembly
- ✅ `test.bin` - Binario ensamblado
- ✅ `expected.json` - **Generado desde Vectrexy C++** (ground truth)

### ⚠️ Limitación Crítica: VIA No Testable

**Problema**: Vectrexy crashea al escribir/leer registros VIA (0xD000-0xD7FF)

**No podemos testear**:
- ❌ Timer1/Timer2 (requieren IER writes)
- ❌ IRQ/FIRQ (requieren configuración VIA)
- ❌ PSG/Audio
- ❌ Hardware I/O ports
- ❌ Los 8 tests originales de arriba (todos usan VIA)

**SÍ podemos testear**:
- ✅ Todas las instrucciones CPU 6809
- ✅ Arithmetic/Logic operations
- ✅ Memory access (RAM)
- ✅ Branches y jumps
- ✅ Stack operations
- ✅ Todos los addressing modes

### 📊 Ejemplo: cpu_arithmetic

**Test Code**:
```asm
LDA #$10    ; A = 0x10
ADDA #$20   ; A = 0x30
LDB #$30    ; B = 0x30
ADDB #$25   ; B = 0x55
BRA loop
```

**Vectrexy Output** (expected.json):
```json
{
  "cpu": {
    "a": 48,     // 0x30 ✅
    "b": 85,     // 0x55 ✅
    "cc": { "c": false, "v": false, "z": false, "n": false },
    "pc": 51208
  },
  "cycles": 50
}
```

### 🛠️ Uso

**Generar referencia Vectrexy**:
```powershell
.\vectrexy_runner\build\Release\vectrexy_runner.exe `
    test_cases\cpu_arithmetic\test.bin 50 `
    2>$null > test_cases\cpu_arithmetic\expected.json
```

### 📝 Próximo Paso INMEDIATO

**Crear `rust_runner`** equivalente que:
1. Cargue `test.bin` a RAM (0xC800)
2. Ejecute N cycles en emulador Rust
3. Serialize estado CPU a JSON (mismo formato)
4. Compare con `expected.json` de Vectrexy

**ENTONCES** tendremos testing comparativo REAL Rust vs Vectrexy (C++).

Ver `VECTREXY_STATUS.md` para detalles técnicos completos.

## Notas

- Los binarios `.bin` pueden ser generados con `lwasm` o manualmente
- El formato JSON permite fácil comparación programática
- Podemos añadir más campos según necesitemos (audio samples, etc)
- **VIA testing**: Requiere enfoque diferente (tests unitarios Rust, documentación 6522)
