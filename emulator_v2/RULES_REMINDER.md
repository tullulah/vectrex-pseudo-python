🚨 REGLAS CRÍTICAS EMULATOR_V2 - COPILOT NO OLVIDES NUNCA

## 0.1. REGLA CRÍTICA: VERIFICACIÓN 1:1 OBLIGATORIA
**ANTES DE CREAR CUALQUIER ARCHIVO O API**:
1. **VERIFICAR EXISTENCIA**: Comprobar si existe en `vectrexy_backup/libs/emulator/src/` y `vectrexy_backup/libs/emulator/include/emulator/`
2. **LEER CÓDIGO ORIGINAL**: Examinar el .cpp/.h correspondiente LÍNEA POR LÍNEA
3. **NO ASUMIR NADA**: No inventar APIs, estructuras, o patrones sin verificar
4. **DOCUMENTAR ORIGEN**: Cada función/struct debe tener comentario "// C++ Original:" con código fuente
5. **SI NO EXISTE = NO CREAR**: Si un archivo no existe en Vectrexy, NO crearlo sin discusión explícita

## ESTRUCTURA 1:1 OBLIGATORIA

### ARCHIVOS PRINCIPALES (.cpp + .h)
| Vectrexy Original | Rust Port | Status |
|------------------|-----------|---------|
| `Cpu.h`/`Cpu.cpp` | `cpu6809.rs` | ✅ COMPLETO (solo LD/ST) |
| `Via.h`/`Via.cpp` | `via6522.rs` | ✅ COMPLETO |
| `BiosRom.cpp` | `bios_rom.rs` | ✅ COMPLETO |
| `Cartridge.cpp` | `cartridge.rs` | ✅ COMPLETO |
| `Emulator.cpp` | `emulator.rs` | ✅ COMPLETO |
| `DevMemoryDevice.cpp` | `dev_memory_device.rs` | ✅ COMPLETO |
| `UnmappedMemoryDevice.cpp` | `unmapped_memory_device.rs` | ✅ COMPLETO |
| **`Psg.cpp`** | **`psg.rs`** | ✅ **COMPLETO** |
| **`Screen.cpp`** | **`screen.rs`** | ❌ **FALTA** |
| **`ShiftRegister.cpp`** | **`shift_register.rs`** | ❌ **FALTA** |

### ARCHIVOS HEADERS-ONLY (.h)
| Vectrexy Original | Rust Port | Status |
|------------------|-----------|---------|
| `CpuHelpers.h` | `cpu_helpers.rs` | ✅ COMPLETO |
| `CpuOpCodes.h` | `cpu_op_codes.rs` | ✅ COMPLETO |
| `MemoryBus.h` | `memory_bus.rs` | ✅ COMPLETO |
| `MemoryMap.h` | `memory_map.rs` | ✅ COMPLETO |
| `Ram.h` | `ram.rs` | ✅ COMPLETO |
| `IllegalMemoryDevice.h` | `illegal_memory_device.rs` | ✅ COMPLETO |
| **`DelayedValueStore.h`** | **`delayed_value_store.rs`** | ✅ **COMPLETO** |
| **`EngineTypes.h`** | **`engine_types.rs`** | ✅ **COMPLETO** |
| **`Timers.h`** | **`timers.rs`** | ✅ **COMPLETO** |

## EJEMPLOS DE INVENTOS PROHIBIDOS (ya detectados):
- ❌ Módulo `devices/` (no existe en Vectrexy - dispositivos están directos en src/)
- ❌ `Ram::new(size)` - En Vectrexy es template fijo 1024 bytes
- ❌ `BiosRom::new(data)` - En Vectrexy es `LoadBiosRom(const char* file)`
- ❌ `MemoryMap` como enums - En Vectrexy es namespace con struct `Mapping`
- ❌ Tests sintéticos sin verificar APIs reales
- ❌ Meter todo en `cpu6809.rs` cuando existen headers separados

## PROCESO OBLIGATORIO ANTES DE IMPLEMENTAR:
1. `ls vectrexy_backup/libs/emulator/src/`
2. `ls vectrexy_backup/libs/emulator/include/emulator/`
3. `cat ArchivoCorrespondiente.cpp`
4. `cat ArchivoCorrespondiente.h`
5. Implementar EXACTAMENTE lo que dice el código original
6. NUNCA implementar tests/APIs hasta verificar pasos 1-4

## FRASE RECORDATORIO:
"NUNCA inventar implementación propia. TODO debe ser port línea-por-línea desde Vectrexy C++."

## SI TIENES DUDAS:
- PREGUNTA al usuario antes de crear algo nuevo
- VERIFICA en vectrexy_backup/ si existe
- LEE el código C++ original COMPLETO
- NO ASUMAS que algo "debería existir"