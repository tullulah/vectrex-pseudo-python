# VIA Memory Bus Crash Analysis

**Fecha**: 2025-10-06  
**Test**: cpu_arithmetic (50 ciclos)  
**Problema**: Crash de SEH al leer registros VIA via MemoryBus después de ejecutar instrucciones

## Hallazgos Críticos

### 1. Direct Via Object Access - ✅ FUNCIONA
```cpp
auto& via = emulator.GetVia();
bool irq = via.IrqEnabled();   // ✅ SUCCESS
bool firq = via.FirqEnabled(); // ✅ SUCCESS
```

**Log Output**:
```
[DEBUG] Testing direct Via object access...
[DEBUG] Got Via reference successfully
[DEBUG] Testing Via::IrqEnabled()...
[DEBUG] Via::IrqEnabled() = 0 SUCCESS
[DEBUG] Testing Via::FirqEnabled()...
[DEBUG] Via::FirqEnabled() = 0 SUCCESS
[DEBUG] Direct Via access SUCCEEDED
```

### 2. Memory Bus Read - ❌ CRASH
```cpp
auto& bus = emulator.GetMemoryBus();
uint8_t ifr = bus.Read(0xD00D);  // ❌ SEH EXCEPTION
```

**Log Output**:
```
[ERROR] SEH Exception reading IFR at 0xd00d!
[ERROR] SEH Exception reading IER at 0xd00e!
[ERROR] SEH Exception reading Timer1_Low at 0xd004!
[ERROR] SEH Exception reading Timer1_High at 0xd005!
[ERROR] SEH Exception reading Timer2_Low at 0xd008!
[ERROR] SEH Exception reading Port_A at 0xd001!
[ERROR] SEH Exception reading Port_B at 0xd000!
[ERROR] SEH Exception reading Shift_Register at 0xd00a!
```

### 3. Timing del Problema
- **ANTES de ejecutar instrucciones**: Memory bus lee VIA correctamente ✅
  ```
  [DEBUG] VIA state BEFORE execution: IFR=0x0 IER=0x0
  ```

- **DESPUÉS de ejecutar 50 ciclos**: Memory bus crashea ❌
  - 18 instrucciones ejecutadas
  - 50 ciclos totales
  - CPU state OK (PC=0xC808)
  - Via object OK (IrqEnabled/FirqEnabled accesibles)
  - **PERO**: Memory bus read → SEH crash

## Hipótesis

### Hipótesis #1: Estado Corrupto del MemoryBus
- Ejecutar instrucciones podría corromper punteros internos del MemoryBus
- Via device podría estar desmapeado o reubicado

### Hipótesis #2: SyncContext Inválido
Via requiere `m_syncContext` para funcionar:
```cpp
struct SyncContext {
    const Input* input{};
    RenderContext* renderContext{};
    AudioContext* audioContext{};
} m_syncContext;
```

- En nuestro wrapper NO seteamos `SetSyncContext()`
- Via::Read() podría desreferenciar punteros nulos
- **PERO**: ¿Por qué funciona ANTES de ejecutar?

### Hipótesis #3: Ciclo de Update Faltante
Via necesita:
```cpp
void Via::Sync(cycles_t cycles) {
    DoSync(cycles, *m_syncContext.input, *m_syncContext.renderContext, ...);
}
```

- Cuando CPU ejecuta instrucciones, llama `Via::Sync()`
- Si `m_syncContext` tiene punteros nulos → CRASH
- **Explicación**: ANTES no se ha llamado a Sync(), DESPUÉS sí

## Comparación Rust vs Vectrexy C++

### Vectrexy C++ (Crashea)
```
via.ifr = 0 (fallback por SEH crash)
via.ier = 0
via.timer1_counter = 0
via.timer2_counter = 0
via.port_a = -1 (crash)
via.port_b = -1 (crash)
via.shift_register = -1 (crash)
```

### Rust (Funciona)
```
via.ifr = 96 (0x60 = Timer1 + Timer2 flags)
via.ier = 0
via.timer1_counter = 0 (TODO)
via.timer2_counter = 0 (TODO)
via.port_a = 0
via.port_b = 0
via.shift_register = 0
```

**Conclusión**: Rust puede leer VIA correctamente porque:
1. No usa MemoryBus para leer (lee directo: `memory.read(0xD00D)`)
2. No tiene el problema de SyncContext

## Solución Propuesta

### Opción A: Setear SyncContext en vectrexy_runner
```cpp
Input input;
RenderContext renderContext;
AudioContext audioContext;

emulator.GetVia().SetSyncContext(input, renderContext, audioContext);
```

**Riesgo**: Necesitamos crear estos objetos correctamente

### Opción B: No ejecutar instrucciones en Vectrexy runner
- Solo cargar estado CPU y VIA manualmente
- **Problema**: No podríamos verificar ejecución de instrucciones

### Opción C: Aceptar limitación y usar valores Rust como referencia
- Documentar que Vectrexy C++ tiene limitación post-ejecución
- Usar Rust como fuente de verdad para VIA state
- **Justificación**: Rust port está basado en código fuente de Vectrexy

### Opción D: Leer Via directamente (sin MemoryBus)
```cpp
auto& via = emulator.GetVia();
// Agregar getters públicos en Via.h:
// uint8_t GetIFR() const;
// uint8_t GetIER() const;
```

**Problema**: Requiere modificar código original de Vectrexy (violación de reglas)

## Recomendación

**Usar Opción A primero**: Setear SyncContext para ver si resuelve el crash.

Si falla, **Opción C**: Aceptar que:
1. Vectrexy wrapper tiene limitación conocida (crash post-ejecución via MemoryBus)
2. Rust implementa correctamente la semántica de Vectrexy (basado en código fuente)
3. Usar Rust como referencia para comparación de VIA state

## Próximos Pasos

1. ✅ Implementar Opción A (SetSyncContext) - **COMPLETADO 2025-10-06**
2. ✅ Re-ejecutar test y verificar si resuelve crashes - **ÉXITO**
3. ⏳ Actualizar Rust para exponer timer counters reales
4. ⏳ Investigar diferencia en port_b (128 vs 0)

---

## 🎉 RESULTADO FINAL - PROBLEMA RESUELTO

**Fecha resolución**: 2025-10-06  
**Solución**: SetSyncContext() antes de ejecutar instrucciones

### Código de la Solución
```cpp
// CRITICAL: Crear contextos para Via::SetSyncContext
Input input;  // Default input state (no buttons pressed)
RenderContext renderContext;  // Empty render context
constexpr float CPU_FREQ = 1500000.0f;  // 1.5 MHz
constexpr float AUDIO_SAMPLE_RATE = 44100.0f;
AudioContext audioContext(CPU_FREQ / AUDIO_SAMPLE_RATE);

emulator.GetVia().SetSyncContext(input, renderContext, audioContext);
```

### Resultado Vectrexy DESPUÉS de la solución
```
[DEBUG] IFR (0xd00d) = 0x60 SUCCESS
[DEBUG] IER (0xd00e) = 0x0 SUCCESS
[DEBUG] Timer1_Low (0xd004) = 0xce SUCCESS
[DEBUG] Timer1_High (0xd005) = 0xff SUCCESS
[DEBUG] Timer2_Low (0xd008) = 0xce SUCCESS
```

**NO MÁS SEH EXCEPTIONS** ✅

### Output JSON Vectrexy (Correcto)
```json
{
  "via": {
    "ifr": 96,          // 0x60 = Timer1 + Timer2 flags ✅
    "ier": 0,
    "port_a": 0,
    "port_b": 128,      // 0x80 = RampDisabled bit
    "shift_register": 0,
    "timer1_counter": 65486,  // 0xFFCE
    "timer2_counter": 206     // 0xCE
  }
}
```

### Diferencias Rust vs Vectrexy (Nuevas)

**Ahora solo quedan diferencias de implementación, NO crashes**:

1. **port_b**: 128 (Vectrexy) vs 0 (Rust)
   - Bit 7 = RampDisabled (set en Reset())
   - Rust: Necesita implementar inicialización correcta

2. **timer1_counter**: 65486 (Vectrexy) vs 0 (Rust)
   - Rust: Placeholder, necesita exponer timer counter real

3. **timer2_counter**: 206 (Vectrexy) vs 0 (Rust)
   - Rust: Placeholder, necesita exponer timer counter real

**Conclusión**: Framework comparativo ahora funciona correctamente. Próximo paso: mejorar serialización Rust.

---

**Actualización**: Próxima acción → Exponer timer counters en Rust port y verificar port_b initialization
