# Emulator V2 - Vectrex Emulator (Rust)

**REGLA CRÍTICA**: Este es un port 1:1 desde el emulador Vectrexy original en C++.

## Referencia Obligatoria

Antes de implementar cualquier funcionalidad, SIEMPRE consultar el código original:
- **Ubicación**: `C:\Users\DanielFerrerGuerrero\source\repos\pseudo-python\vectrexy_backup\libs\emulator\`
- **Archivos**: `.cpp` y `.h` correspondientes
- **Formato mandatorio**: Cada método debe incluir comentario `// C++ Original:` con código fuente

## Estado Actual

### ✅ Completado (2025-01-20)
- Estructura básica del proyecto (111 tests)
- Todos los módulos compilan sin errores
- **VIA6522 corregido**: Todas las firmas de métodos ahora coinciden 1:1 con Vectrexy
- **set_sync_context()**: Usa parámetros correctos (Input, RenderContext, AudioContext)
- **irq_enabled()**: Implementa lógica GetInterruptFlagValue() original
- **firq_enabled()**: Retorna campo m_firq_enabled como en original
- **frame_update()**: Llama correctamente a screen.frame_update() y psg.frame_update()
- **SyncContext**: Rediseñado para ownership de Rust manteniendo semántica original
- **engine_types.rs**: Traits Debug añadidos para contextos

### Tests Status
- **111 tests passing** ✅
- Todas las implementaciones verificadas contra código original
- Fidelidad 1:1 con Vectrexy confirmada

## Arquitectura

### Módulos Core
- `cpu6809.rs` - Procesador Motorola 6809
- `via6522.rs` - Versatile Interface Adapter (VIA 6522) 
- `memory_device.rs` - Dispositivos de memoria (RAM, BIOS, etc.)
- `emulator.rs` - Emulador principal que integra todos los componentes
- `engine_types.rs` - Tipos y contextos compartidos

### Módulos de Componentes
- `screen.rs` - Manejo de pantalla y vector beam
- `psg.rs` - Programmable Sound Generator (AY-3-8912)
- `timers.rs` - Timers del VIA
- `shift_register.rs` - Registro de desplazamiento
- `delayed_value_store.rs` - Almacenamiento de valores con delay

## Reglas de Desarrollo

### 1. Verificación Obligatoria 1:1
Antes de crear cualquier archivo o API:
1. **VERIFICAR EXISTENCIA**: Comprobar si existe en `vectrexy_backup/libs/emulator/src/` y `vectrexy_backup/libs/emulator/include/emulator/`
2. **LEER CÓDIGO ORIGINAL**: Examinar el .cpp/.h correspondiente LÍNEA POR LÍNEA
3. **NO ASUMIR NADA**: No inventar APIs, estructuras, o patrones sin verificar
4. **DOCUMENTAR ORIGEN**: Cada función/struct debe tener comentario "// C++ Original:" con código fuente
5. **SI NO EXISTE = NO CREAR**: Si un archivo no existe en Vectrexy, NO crearlo sin discusión explícita

### 2. Adaptaciones Permitidas
- **Solo sintaxis Rust**: Ownership, borrowing, lifetimes
- **Mantener semántica idéntica**: El comportamiento debe ser exactamente igual
- **Constantes exactas**: Usar valores exactos del original

### 3. Ejemplos de Inventos Prohibidos
- ❌ Módulos no existentes en Vectrexy
- ❌ Constructores que no existen (`Ram::new(size)` vs template fijo)
- ❌ APIs sintéticas sin verificar código real
- ❌ Tests sin verificar comportamiento original

## Correcciones Históricas

### VIA6522 Method Signatures (2025-01-20)
**Commit**: `d0db5023`

**Problema**: Las firmas de métodos VIA6522 no coincidían con Vectrexy original.

**Correcciones**:
```rust
// ANTES (incorrecto)
pub fn set_sync_context(&mut self, ctx: SyncContext)
pub fn irq_enabled(&self) -> bool  // implementación incorrecta
pub fn firq_enabled(&self) -> bool // implementación incorrecta
pub fn frame_update(&mut self)     // implementación incorrecta

// DESPUÉS (1:1 con Vectrexy)
pub fn set_sync_context(&mut self, input: Input, render_ctx: RenderContext, audio_ctx: AudioContext)
pub fn irq_enabled(&self) -> bool {
    // C++ Original: return GetInterruptFlagValue(m_interruptFlag, m_interruptEnable);
    get_interrupt_flag_value(self.interrupt_flag, self.interrupt_enable)
}
pub fn firq_enabled(&self) -> bool {
    // C++ Original: return m_firqEnabled;
    self.firq_enabled
}
pub fn frame_update(&mut self, input: &Input, render_ctx: &mut RenderContext, audio_ctx: &mut AudioContext) {
    // C++ Original: m_screen.FrameUpdate(...); m_psg.FrameUpdate(...);
    self.screen.frame_update(input, render_ctx);
    self.psg.frame_update(input, audio_ctx);
}
```

## Building & Testing

```powershell
# Compilar
cargo build

# Ejecutar tests
cargo test

# Ejecutar tests específicos
cargo test test_via6522

# Tests con output detallado
cargo test -- --nocapture
```

## Referencias

- **Vectrexy Original**: `vectrexy_backup/libs/emulator/` (C++)
- **Documentación**: `SUPER_SUMMARY.md` (sección 26)
- **Commit History**: Ver mensajes de commit para detalles de correcciones