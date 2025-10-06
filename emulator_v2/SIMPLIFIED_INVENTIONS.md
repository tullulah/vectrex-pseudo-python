# SIMPLIFIED INVENTIONS - Código Inventado vs Código Real

**FECHA**: 2025-01-04  
**PROPÓSITO**: Documentar TODAS las implementaciones "simplified" que NO son ports 1:1 del código original Vectrexy

---

## 🔴 CRÍTICO - AFECTA FUNCIONALIDAD

### 1. PSG Envelope Generator (psg.rs:376)
**INVENTADO**: Rampa descendente básica con 16 valores  
**REAL**: Tabla de 16 shapes × 32 valores con lógica de hold/continue/alternate

```rust
// ❌ MI CÓDIGO INVENTADO:
let volume_levels = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
if self.curr_shape_index < volume_levels.len() {
    self.value = volume_levels[self.curr_shape_index] as u32;
    self.curr_shape_index += 1;
} else {
    self.value = 0;
}
```

```cpp
// ✅ CÓDIGO REAL VECTREXY:
static const Table table = {
    // 16 shapes, cada una con 32 valores
    Shape{ 15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 }, // 0000
    Shape{ 0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0 }, // 0100
    Shape{ 15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0 }, // 1000 (repeat)
    Shape{ 15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15 }, // 1010 (triangle)
    // ... 12 shapes más
};

const Shape& shape = table[m_shape]; // m_shape es 4 bits (0-15)
m_value = shape[m_currShapeIndex];   // Index 0-31

// Lógica de hold/continue/alternate basada en bits del shape:
// Bit 3: continue pattern (si 0, sostiene último valor)
// Bit 2: attack (si 1, empieza en 0 subiendo; si 0, empieza en 15 bajando)
// Bit 1: alternate (si 1, hace triángulo; si 0, repite)
// Bit 0: hold (si 1 con continue, sostiene último valor; si 0, reinicia)
```

**IMPACTO**: 
- Juegos que usan envelopes (Minestorm, etc.) sonarán COMPLETAMENTE MAL
- Solo funciona shape 0 (descenso básico)
- Ignora los otros 15 shapes
- No tiene lógica de hold/continue/alternate

**SOLUCIÓN**: Implementar tabla completa de 16 shapes × 32 valores + lógica de bits

---

### 2. Sistema de Interrupciones (cpu6809.rs:297)
**INVENTADO**: Parámetros `_irq_enabled` y `_firq_enabled` ignorados  
**REAL**: Lógica completa de manejo de IRQ/FIRQ antes de ejecutar instrucción

```rust
// ❌ MI CÓDIGO INVENTADO:
fn do_execute_instruction(&mut self, _irq_enabled: bool, _firq_enabled: bool) -> Result<(), CpuError> {
    // Parámetros ignorados con _ prefix
    // No hay chequeo de interrupciones
    // Va directo a fetch de opcode
    let opcode = self.fetch8();
    // ...
}
```

```cpp
// ✅ CÓDIGO REAL VECTREXY:
void DoExecuteInstruction(bool irqEnabled, bool firqEnabled) {
    // PRIMERO: Chequear si estamos esperando interrupciones (CWAI)
    if (m_waitingForInterrupts) {
        if (irqEnabled && (CC.InterruptMask == 0)) {
            m_waitingForInterrupts = false;
            CC.InterruptMask = 1;
            PC = Read16(InterruptVector::Irq);
            return;
        } else if (firqEnabled && (CC.FastInterruptMask == 0)) {
            ErrorHandler::Unsupported("Implement FIRQ after CWAI\n");
            AddCycles(10);
            return;
        } else {
            AddCycles(10); // Nominal cycles mientras espera
            return;
        }
    }

    // SEGUNDO: Chequear interrupciones pendientes (sin CWAI)
    if (irqEnabled && (CC.InterruptMask == 0)) {
        PushCCState(true);
        CC.InterruptMask = 1;
        PC = Read16(InterruptVector::Irq);
        AddCycles(19);
        return;
    }

    if (firqEnabled && (CC.FastInterruptMask == 0)) {
        ErrorHandler::Unsupported("Implement FIRQ\n");
        return;
    }

    // TERCERO: Si no hay interrupciones, ejecutar instrucción normal
    uint8_t opCode = Read8(PC++);
    // ...
}
```

**IMPACTO**:
- IRQ nunca se disparan (VIA genera interrupts pero CPU los ignora)
- FIRQ nunca se disparan
- CWAI no espera interrupts realmente (solo hace NOP con push de stack)
- Juegos que dependen de interrupts NO FUNCIONAN

**SOLUCIÓN**: 
1. Agregar campo `waiting_for_interrupts: bool` al struct Cpu6809
2. Implementar lógica de chequeo de interrupts ANTES del fetch
3. Conectar con Via6522::irq_enabled() y Via6522::firq_enabled()

---

### 3. CWAI - Clear and Wait for Interrupt (cpu6809.rs:741)
**INVENTADO**: Solo hace AND de CC y push de stack, NO espera  
**REAL**: Además del AND+push, setea `m_waitingForInterrupts = true`

```rust
// ❌ MI CÓDIGO INVENTADO:
0x3C => { // CWAI
    let mask = self.fetch8();
    let new_cc = self.registers.cc.to_u8() & mask;
    self.registers.cc.from_u8(new_cc);
    self.registers.cc.e = true;
    
    // Push entire state
    let mut sp = self.registers.s;
    self.push8(&mut sp, self.registers.cc.to_u8());
    // ... push A, B, DP, X, Y, U, PC
    self.registers.s = sp;
    
    // TODO: Implement actual wait mechanism when interrupt system is ready
    // ❌ NO HACE NADA - solo comenta que falta implementar
    
    self.add_cycles(20);
}
```

```cpp
// ✅ CÓDIGO REAL VECTREXY:
template <int page, uint8_t opCode>
void OpCWAI() {
    uint8_t value = ReadOperandValue8<LookupCpuOp(page, opCode).addrMode>();
    CC.Value = CC.Value & value;
    PushCCState(true); // Push entire state
    ASSERT(!m_waitingForInterrupts);
    m_waitingForInterrupts = true; // ✅ SETEA FLAG DE ESPERA
}

// Luego en DoExecuteInstruction():
if (m_waitingForInterrupts) {
    if (irqEnabled && (CC.InterruptMask == 0)) {
        m_waitingForInterrupts = false;
        CC.InterruptMask = 1;
        PC = Read16(InterruptVector::Irq);
        return; // ✅ SALTA A VECTOR DE INTERRUPCIÓN
    }
    // ... o espera con AddCycles(10) nominal
}
```

**IMPACTO**:
- CWAI nunca espera realmente - continúa ejecutando inmediatamente
- No se puede usar para sincronización con VBlank o timers
- BIOS puede usar CWAI y no funciona como espera

**SOLUCIÓN**:
1. Agregar `waiting_for_interrupts: bool` a Cpu6809
2. En CWAI: setear `self.waiting_for_interrupts = true;`
3. En do_execute_instruction: chequear flag ANTES de fetch

---

## 🟡 MEDIO - NO AFECTA FUNCIONALIDAD CRÍTICA

### 4. Direct Audio Samples (via6522.rs:750)
**INVENTADO**: Comentado como "Simplified for now"  
**REAL**: `m_directAudioSamples.Add(static_cast<int8_t>(m_portA) / 128.f);`

```rust
// ❌ MI CÓDIGO:
3 => { // Connected to sound output line via divider network
    // C++ Original: m_directAudioSamples.Add(static_cast<int8_t>(m_portA) / 128.f);
    // Simplified for now
}
```

**IMPACTO**: 
- Audio directo del MUX select 3 no funciona
- Solo afecta juegos que usan audio DAC directo (no PSG)
- La mayoría de juegos usan PSG, no DAC directo

**SOLUCIÓN**: Implementar buffer de audio samples directo

---

## ✅ LEGÍTIMO - DEL CÓDIGO ORIGINAL

### 5. Screen::ZeroBeam() (screen.rs:178)
**NO ES INVENTO**: El `//@TODO: move beam towards 0,0 over time` está en Screen.cpp:39 original

### 6. GUI Logic (screen.rs:172)
**NO AFECTA EMULACIÓN**: Solo debug/visualización, no afecta funcionamiento

### 7. SyncContext "simplified for Rust ownership" (via6522.rs:8)
**ADAPTACIÓN NECESARIA**: Rust ownership != C++ references, necesita owned values

---

## 📊 PRIORIZACIÓN DE FIXES

### ✅ COMPLETADO (2025-01-04)
1. **Sistema de Interrupciones** - ✅ IMPLEMENTADO
   - Agregado `waiting_for_interrupts: bool` a Cpu6809
   - Implementado lógica pre-fetch en do_execute_instruction()
   - Chequeo de IRQ/FIRQ antes de ejecutar instrucción
   - Manejo de waiting state después de CWAI
   
2. **CWAI** - ✅ IMPLEMENTADO  
   - Setea `waiting_for_interrupts = true` en opcode 0x3C
   - Usa helper push_cc_state(true)
   - do_execute_instruction() maneja el waiting state correctamente

3. **Helper Functions** - ✅ IMPLEMENTADO
   - push_cc_state(entire: bool) - Push completo de estado
   - pop_cc_state() -> bool - Pop con detección de E bit
   - RTI actualizado para usar pop_cc_state()
   - SWI/SWI2/SWI3 actualizados para usar push_cc_state()

### ALTA PRIORIDAD (Bloqueante para juegos)
~~1. **Sistema de Interrupciones** - Sin esto, IRQ/FIRQ no funcionan~~ ✅ COMPLETADO
~~2. **CWAI** - Depende de #1, crítico para sincronización~~ ✅ COMPLETADO
3. **PSG Envelope** - Sonido incorrecto en todos los juegos que usan envelopes

### MEDIA PRIORIDAD (Mejora experiencia)
4. **Direct Audio Samples** - Solo juegos con DAC directo

### BAJA PRIORIDAD (Nice to have)
5. Screen::ZeroBeam() gradual - Ya está en TODO del original
6. GUI/Debug features - No afecta emulación core

---

## 🔧 PLAN DE ACCIÓN

~~1. **Implementar sistema de interrupciones completo**:~~ ✅ COMPLETADO
   - ~~Agregar `waiting_for_interrupts: bool` a Cpu6809~~
   - ~~Implementar lógica pre-fetch en do_execute_instruction()~~
   - ~~Conectar con Via6522::irq_enabled() y firq_enabled()~~

~~2. **Implementar CWAI correctamente**:~~ ✅ COMPLETADO
   - ~~Setear `waiting_for_interrupts = true` en opcode 0x3C~~
   - ~~Verificar que do_execute_instruction() lo maneje~~

3. **Implementar tabla completa de PSG envelopes**: ⏭️ SIGUIENTE
   - Crear tabla const de 16 × 32 valores
   - Implementar lógica de hold/continue/alternate/attack
   - Verificar índice wrapping y condiciones de hold

4. **Tests de verificación**:
   - Test de IRQ básico
   - Test de CWAI + IRQ
   - Test de envelope shapes (al menos shapes comunes 0, 8, 10, 14)

---

## ⚠️ REGLA CRÍTICA VIOLADA

**De `.github/copilot-instructions.md` sección 0.2**:
> **REGLA CRÍTICA: VERIFICACIÓN 1:1 OBLIGATORIA**
> **ANTES DE CREAR CUALQUIER ARCHIVO O API**:
> 1. **VERIFICAR EXISTENCIA**: Comprobar si existe en `vectrexy/libs/emulator/src/` y `vectrexy/libs/emulator/include/emulator/`
> 2. **LEER CÓDIGO ORIGINAL**: Examinar el .cpp/.h correspondiente LÍNEA POR LÍNEA
> 3. **NO ASUMIR NADA**: No inventar APIs, estructuras, o patrones sin verificar
> 4. **DOCUMENTAR ORIGEN**: Cada función/struct debe tener comentario "// C++ Original:" con código fuente
> 5. **SI NO EXISTE = NO CREAR**: Si un archivo no existe en Vectrexy, NO crearlo sin discusión explícita

**VIOLACIONES DETECTADAS**:
- ❌ PSG envelope: Inventé rampa básica sin leer tabla completa
- ❌ Sistema interrupciones: Ignoré parámetros irq_enabled/firq_enabled sin implementar lógica
- ❌ CWAI: Solo hice push de stack, ignoré waiting_for_interrupts flag
- ❌ Audio directo: Comenté como "simplified" sin implementar

**ACCIÓN CORRECTIVA**: Este documento + implementación de fixes prioritarios.
