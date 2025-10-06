# Test Comparativo: bios_print_str_vectors
## Status: ✅ Rust Runner COMPLETE | ⏳ JSVecx Runner PENDING

### Objetivo
Ejecutar BIOS en ambos emuladores (Rust y JSVecx) durante 2.5M cycles para capturar todos los vectores generados durante el rendering del copyright. Comparar vectores pixel-por-pixel para identificar la causa del problema "skewed letters".

---

## Rust Runner - COMPLETED ✅

### Implementación
- **Ubicación**: `tests/comparative/rust_runner_print_str/`
- **Binario**: `target/release/rust_runner_print_str.exe`
- **Lógica**: Ejecuta BIOS por 2.5M cycles (suficiente para copyright completo)
- **Output**: `test_cases/bios_print_str_vectors/rust_output.json`

### Compilación
```powershell
cd emulator_v2
cargo build --release -p rust_runner_print_str
```

### Ejecución
```powershell
.\target\release\rust_runner_print_str.exe `
  tests\comparative\test_cases\bios_print_str_vectors\test.bin `
  2>tests\comparative\test_cases\bios_print_str_vectors\stderr.log `
  | Out-File -Encoding utf8 tests\comparative\test_cases\bios_print_str_vectors\rust_output.json
```

### Resultados
```
✅ Cycles ejecutados: 2,500,000
✅ Vectores capturados: 39,157
✅ Llamadas a Print_Str: 380
✅ PC final: 0xF4EB (BIOS infinite loop - esperado)
✅ Estado CPU capturado: A, B, X, Y, U, S, DP, CC
✅ Estado VIA capturado: IFR, IER, Timer1/2, Port A/B, Shift Reg
```

### Estructura JSON Output
```json
{
  "cycles": 2500000,
  "pc_when_stopped": 62699,
  "cpu": { ... },
  "via": { ... },
  "vectors": {
    "count": 39157,
    "lines": [
      {
        "p0": { "x": -45.123, "y": 32.456 },
        "p1": { "x": -44.789, "y": 32.456 },
        "brightness": 0.9921875
      },
      ...
    ]
  },
  "audio_samples": 0
}
```

### Características Técnicas

**Vector Capture:**
- Captura desde `render_context.lines` (acumula TODOS los vectores)
- Tipos: `p0/p1: Vector2 { x: f32, y: f32 }`, `brightness: f32`
- Coordenadas en sistema Vectrex (-127 a +127, centro en 0,0)

**Print_Str Tracking:**
- Detecta cada llamada a 0xF495 (informacional)
- NO detiene ejecución - continúa hasta TARGET_CYCLES
- Loguea primeras 5 llamadas + totales

**Progress Logging:**
- Cada 500k cycles: `Progress: N M cycles, X vectors, PC=0x....`
- Final: `Execution complete: ... cycles, ... Print_Str calls, ... vectors`

---

## JSVecx Runner - PENDING ⏳

### Plan de Implementación
1. **Opción A: Modificar runner C++ existente** (`vectrexy_runner`)
   - Ejecutar 2.5M cycles
   - Capturar vectores desde Screen/Integrator
   - Serializar a JSON matching Rust format

2. **Opción B: Usar jsvecx_comparison.js** (Node.js)
   - Adaptar test_f4eb_detailed_js.js
   - Ejecutar 2.5M cycles con hook e6809_sstep
   - Capturar vectores desde JSVecx globals
   - Serializar a JSON matching Rust format

### Requerimientos Output JSON
```json
{
  "cycles": 2500000,
  "pc_when_stopped": <u16>,
  "cpu": {
    "pc": <u16>,
    "a": <u8>, "b": <u8>,
    "x": <u16>, "y": <u16>, "u": <u16>, "s": <u16>,
    "dp": <u8>, "cc": <u8>
  },
  "via": {
    "ifr": <u8>, "ier": <u8>,
    "timer1_counter": <u16>, "timer2_counter": <u16>,
    "port_a": <u8>, "port_b": <u8>,
    "shift_register": <u8>
  },
  "vectors": {
    "count": <usize>,
    "lines": [
      {
        "p0": { "x": <f32>, "y": <f32> },
        "p1": { "x": <f32>, "y": <f32> },
        "brightness": <f32>
      },
      ...
    ]
  },
  "audio_samples": 0
}
```

**CRÍTICO**: Coordenadas deben estar en el MISMO sistema de referencia (Vectrex screen space después de transformaciones integrator).

---

## Comparación - PENDING ⏳

### Script de Comparación
Una vez que tengamos `jsvecx_output.json`, usar Python para comparación detallada:

```python
import json

rust_data = json.load(open('rust_output.json'))
jsvecx_data = json.load(open('jsvecx_output.json'))

# Comparar CPU state
print(f"PC: Rust=0x{rust_data['cpu']['pc']:04X} vs JSVecx=0x{jsvecx_data['cpu']['pc']:04X}")

# Comparar count de vectores
print(f"Vector count: Rust={rust_data['vectors']['count']} vs JSVecx={jsvecx_data['vectors']['count']}")

# Comparar primeros N vectores
for i in range(min(10, rust_data['vectors']['count'], jsvecx_data['vectors']['count'])):
    rv = rust_data['vectors']['lines'][i]
    jv = jsvecx_data['vectors']['lines'][i]
    
    p0_diff = abs(rv['p0']['x'] - jv['p0']['x']) + abs(rv['p0']['y'] - jv['p0']['y'])
    p1_diff = abs(rv['p1']['x'] - jv['p1']['x']) + abs(rv['p1']['y'] - jv['p1']['y'])
    
    print(f"Vector {i}: p0_diff={p0_diff:.4f}, p1_diff={p1_diff:.4f}")
```

### Métricas de Análisis
- **Count mismatch**: Si JSVecx genera más/menos vectores → problema de integrator sync
- **Coordinate offset**: Si TODOS los vectores tienen offset constante → problema de transformación
- **Random scatter**: Si vectores individuales difieren aleatoriamente → problema de timing/rounding
- **Systematic skew**: Si X o Y tienen bias consistente → problema de coordinate mapping

---

## Archivos Generados

```
tests/comparative/test_cases/bios_print_str_vectors/
├── test.asm                  # Test source (placeholder - ejecuta BIOS)
├── test.bin                  # Assembled binary (vacío)
├── rust_output.json          # ✅ Rust emulator output (39,157 vectors)
├── stderr.log                # ✅ Rust execution log
├── jsvecx_output.json        # ⏳ PENDING - JSVecx emulator output
└── comparison_report.txt     # ⏳ PENDING - Detailed vector comparison

tests/comparative/rust_runner_print_str/
├── Cargo.toml                # ✅ Package manifest
├── src/
│   └── main.rs               # ✅ Runner implementation (232 lines)
└── target/release/
    └── rust_runner_print_str.exe  # ✅ Compiled binary (316KB)
```

---

## Next Steps

1. **IMMEDIATE**: Implementar JSVecx runner (Opción A o B)
2. **CRITICAL**: Asegurar que JSVecx captura vectores en MISMO formato
3. **PRIORITY**: Ejecutar comparación y analizar diferencias
4. **GOAL**: Identificar causa exacta de "skewed letters"

---

## Technical Notes

### Rust Runner Architecture
- **NO ejecuta código custom** - solo BIOS desde RESET vector (0xF000)
- **test.bin es ignorado** - warning es esperado y no-crítico
- **RenderContext acumula vectores** - NO se limpia entre frames
- **39,157 vectores** = ~103 vectores/letra × ~380 llamadas Print_Str

### Coordinate System
- **Vectrex hardware**: DAC 8-bit → -127 a +127 (signed)
- **Integrator output**: f32 coordinates post-transformación
- **Screen space**: Centro (0,0), X→derecha, Y→arriba
- **Brightness**: 0.0 (invisible) a 1.0 (máximo brillo)

### Timing Correlation
- **2.5M cycles** @ 1.5MHz = ~1.67 segundos real-time
- **380 Print_Str calls** = copyright completo
- **Progress logs** cada 500k = 4 checkpoints
- **PC=0xF4EB** = infinite loop waiting for input (normal post-copyright)

---

**Última actualización**: 2025-10-06 16:30  
**Status**: Rust runner implementado y validado ✅ | JSVecx runner pendiente ⏳
