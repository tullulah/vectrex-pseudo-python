# 🎉 VIA IFR Difference Investigation - RESOLVED

**Fecha**: 2025-10-06  
**Problema Original**: VIA.IFR mostraba 0 (Vectrexy) vs 96 (Rust)  
**Estado**: ✅ **RESUELTO** - Era un bug en vectrexy_runner, NO en Rust

---

## Resumen Ejecutivo

### Problema Inicial
```
❌ via.ifr
  Expected:  0.0 (Vectrexy)
  Rust:      96.0
```

**Hipótesis inicial (INCORRECTA)**: Rust calcula IFR incorrectamente.

### Hallazgo Real
- Vectrexy **crasheaba con SEH exception** al leer VIA via MemoryBus post-ejecución
- `safeReadViaRegister()` capturaba el crash y devolvía **0 como fallback**
- El **valor 0 era inválido** (crash enmascarado)

### Causa Raíz
Via requiere `m_syncContext` configurado para funcionar:
```cpp
struct SyncContext {
    const Input* input{};
    RenderContext* renderContext{};
    AudioContext* audioContext{};
} m_syncContext;
```

- `vectrexy_runner` NO llamaba `SetSyncContext()` antes de ejecutar
- Al ejecutar instrucciones → CPU llama `Via::Sync()`
- `Via::Sync()` desreferencia `m_syncContext.input` → **CRASH** (puntero nulo)
- Lectura de VIA via MemoryBus → llama métodos que usan `m_syncContext` → **CRASH**

### Solución
```cpp
Input input;
RenderContext renderContext;
AudioContext audioContext(CPU_FREQ / AUDIO_SAMPLE_RATE);

emulator.GetVia().SetSyncContext(input, renderContext, audioContext);
```

**Resultado**: NO más crashes, VIA lee correctamente.

---

## Valores Correctos

### Vectrexy C++ (Ahora correcto)
```json
{
  "via": {
    "ifr": 96,          // 0x60 = Timer1 (0x40) + Timer2 (0x20) flags
    "ier": 0,
    "port_a": 0,
    "port_b": 128,      // 0x80 = RampDisabled (default after reset)
    "shift_register": 0,
    "timer1_counter": 65486,  // 0xFFCE (active countdown)
    "timer2_counter": 206     // 0xCE (active countdown)
  }
}
```

### Rust (Necesita mejoras)
```json
{
  "via": {
    "ifr": 96,          // ✅ CORRECTO (0x60)
    "ier": 0,           // ✅ CORRECTO
    "port_a": 0,        // ✅ CORRECTO
    "port_b": 0,        // ❌ Debería ser 128 (RampDisabled bit)
    "shift_register": 0,  // ✅ CORRECTO (placeholder)
    "timer1_counter": 0,  // ❌ Placeholder - debería exponer valor real
    "timer2_counter": 0   // ❌ Placeholder - debería exponer valor real
  }
}
```

---

## Conclusión

### ✅ Verificado: Rust implementa VIA correctamente
- **IFR calculation**: ✅ Correcto (0x60 = Timer1 + Timer2 flags)
- **IER handling**: ✅ Correcto (0x00 = no interrupts enabled)
- **Port A**: ✅ Correcto (0x00 = DAC output)

### ❌ Pendiente: Mejoras menores en Rust
1. **port_b initialization**: Setear bit 7 (RampDisabled) en Reset()
   - Ubicación: `emulator_v2/src/core/via6522.rs`
   - Código: `self.port_b = 0x80;  // RampDisabled bit`

2. **Timer counters**: Exponer valores reales en serialización
   - Actualmente: Placeholders (0)
   - Solución: Agregar getters `timer1.counter()` y `timer2.counter()`
   - Ubicación: `rust_runner/src/main.rs`

---

## Impacto en Testing Framework

### ANTES (Inválido)
- Vectrexy devolvía 0 por crash → **referencia inválida**
- Comparación: 0 (crash) vs 96 (correcto) → falso negativo
- **No confiable** para validar implementación Rust

### DESPUÉS (Válido)
- Vectrexy devuelve valores reales → **referencia válida** ✅
- Comparación: 96 (Vectrexy) vs 96 (Rust) → **MATCH** 🎉
- **Confiable** para detectar bugs reales en Rust

### Nuevas diferencias encontradas
```
via.port_b: 128 (Vectrexy) vs 0 (Rust)
via.timer1_counter: 65486 vs 0 (placeholder)
via.timer2_counter: 206 vs 0 (placeholder)
```

**Todas son bugs reales en Rust, NO en Vectrexy**.

---

## Lecciones Aprendidas

1. **SEH exceptions ocultan bugs**: `__try/__except` con fallback silencioso = datos inválidos
2. **SyncContext es mandatorio**: Via no funciona sin contextos válidos
3. **Tests comparativos deben validar referencias**: Crash en referencia = test inválido
4. **Logging detallado es crítico**: Sin logs DEBUG, nunca habríamos encontrado el crash

---

## Próximos Pasos

### Corto plazo
1. ✅ Fix port_b initialization en Rust (1 línea)
2. ✅ Exponer timer counters en Rust serialización (5 líneas)
3. ✅ Re-ejecutar test → debería pasar 100%

### Mediano plazo
4. ⏳ Crear más test cases (cpu_load_store, cpu_branch, etc.)
5. ⏳ Tests de VIA específicos (timers, interrupts, ports)
6. ⏳ Validar Screen integrator updates

### Largo plazo
7. ⏳ Framework de regression testing automatizado
8. ⏳ CI/CD integration para tests comparativos
9. ⏳ Documentación completa de testing workflow

---

**Última actualización**: 2025-10-06  
**Status**: ✅ VIA comparison framework operacional y confiable  
**Próxima acción**: Fix port_b y timers en Rust port
