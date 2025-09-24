# 📊 Tabla Comparativa VIA 6522 y Timing: Rust vs JavaScript

## 🎯 **Resumen Ejecutivo**

| Aspecto | Rust | JavaScript | Estado |
|---------|------|------------|---------|
| **Timing F4EB** | 1,525 steps | 38,103 steps | ❌ 25x diferencia |
| **Timer2 en F4EB** | 0xFF | 0x88 | ❌ Valores diferentes |
| **Salida de F4EB** | Inmediata (0 steps) | Inmediata (1 step) | ✅ Similar |
| **Registers CPU** | Válidos (8-bit) | Inválidos (>8-bit) | ❌ Bug JavaScript |

---

## 🔧 **1. Estructura de Datos VIA**

### **Rust (via6522.rs)**
```rust
pub struct Via6522 {
    regs: [u8;16],           // Registros VIA estándar
    t1_counter: u16,         // Timer1 contador (16-bit)
    t1_latch: u16,           // Timer1 latch value
    t2_counter: u16,         // Timer2 contador (16-bit)
    t2_latch: u16,           // Timer2 latch value
    irq_line: bool,          // Estado línea IRQ
    pb7_state: bool,         // Estado PB7 output
    
    // Estados de control (equivalentes JavaScript)
    t1_enabled: bool,        // via_t1on
    t1_int_enabled: bool,    // via_t1int  
    t2_enabled: bool,        // via_t2on
    t2_int_enabled: bool,    // via_t2int
}
```

### **JavaScript (vecx.js)**
```javascript
// Estados VIA como variables individuales
this.via_t1on = 0;          // Timer1 enabled flag
this.via_t1int = 0;         // Timer1 interrupt enabled
this.via_t1c = 0;           // Timer1 counter (16-bit)
this.via_t1ll = 0;          // Timer1 low latch
this.via_t1lh = 0;          // Timer1 high latch
this.via_t1pb7 = 0x80;      // PB7 state

this.via_t2on = 0;          // Timer2 enabled flag
this.via_t2int = 0;         // Timer2 interrupt enabled
this.via_t2c = 0;           // Timer2 counter (16-bit)
this.via_t2ll = 0;          // Timer2 low latch

this.via_ifr = 0;           // Interrupt Flag Register
this.via_ier = 0;           // Interrupt Enable Register
this.via_acr = 0;           // Auxiliary Control Register
```

**🔍 Análisis**: Rust usa estructura unificada, JavaScript variables separadas. Ambos mantienen estados equivalentes.

---

## ⏱️ **2. Timing y Ciclos**

### **Rust advance_cycles()**
```rust
fn advance_cycles(&mut self, cyc: u32) {
    // CYCLE-BY-CYCLE TIMING FIX
    for _ in 0..cyc {
        self.bus.tick(1);  // Tick VIA uno por uno
    }
    self.cycles += cyc as u64;
}

// Bus tick delega a VIA
pub fn tick(&mut self, cycles:u32) {
    self.via.tick(cycles);  // Siempre tick VIA
    self.psg.tick(cycles);  // También PSG
}
```

### **JavaScript vecx_emu()**
```javascript
while( cycles > 0 ) {
    icycles = e6809.e6809_sstep(this.via_ifr & 0x80, 0);
    for( c = 0; c < icycles; c++ ) {
        // Timer updates individuales cada ciclo
        if( this.via_t1on ) {
            this.via_t1c = ( this.via_t1c > 0 ? this.via_t1c - 1 : 0xffff );
            // ... manejo underflow
        }
        if( this.via_t2on && (this.via_acr & 0x20) == 0x00 ) {
            this.via_t2c = ( this.via_t2c > 0 ? this.via_t2c - 1 : 0xffff );
            // ... manejo underflow
        }
    }
    cycles--;
}
```

**🔍 Análisis**: Ambos implementan timing cycle-by-cycle, pero JavaScript hace más trabajo por ciclo.

---

## 🔴 **3. Timer1 Implementación**

| Característica | Rust | JavaScript | Equivalencia |
|---------------|------|------------|--------------|
| **Inicialización** | `t1_enabled = true` | `via_t1on = 1` | ✅ Equivalente |
| **Interrupt Enable** | `t1_int_enabled = true` | `via_t1int = 1` | ✅ Equivalente |
| **PB7 Control** | `pb7_state = false` | `via_t1pb7 = 0` | ✅ Equivalente |
| **Countdown Logic** | `t1_counter -= 1` | `via_t1c = (via_t1c > 0 ? via_t1c - 1 : 0xffff)` | ✅ Equivalente |
| **Underflow Detection** | `if t1_counter == 0` | `if (via_t1c & 0xffff) == 0xffff` | ✅ Equivalente |
| **IFR Setting** | `regs[0x0D] \|= 0x40` | `via_ifr \|= 0x40` | ✅ Equivalente |
| **Continuous Mode** | Recarga desde `t1_latch` | Recarga desde `(via_t1lh << 8) \| via_t1ll` | ✅ Equivalente |

---

## 🔵 **4. Timer2 Implementación**

| Característica | Rust | JavaScript | Equivalencia |
|---------------|------|------------|--------------|
| **Inicialización** | `t2_enabled = true` | `via_t2on = 1` | ✅ Equivalente |
| **Interrupt Enable** | `t2_int_enabled = true` | `via_t2int = 1` | ✅ Equivalente |
| **Countdown Logic** | `t2_counter -= 1` | `via_t2c = (via_t2c > 0 ? via_t2c - 1 : 0xffff)` | ✅ Equivalente |
| **Underflow Detection** | `if t2_counter == 0` | `if (via_t2c & 0xffff) == 0xffff` | ✅ Equivalente |
| **IFR Setting** | `regs[0x0D] \|= 0x20` | `via_ifr \|= 0x20` | ✅ Equivalente |
| **One-shot Disable** | `t2_int_enabled = false` | `via_t2int = 0` | ✅ Equivalente |
| **ACR Check** | No implementado | `(via_acr & 0x20) == 0x00` | ❌ Diferencia |

**🚨 DIFERENCIA CRÍTICA**: JavaScript verifica ACR bit 5 para Timer2, Rust no.

---

## 🎛️ **5. Manejo de Registros**

### **Timer2 Load (Registro 0x09)**

#### **Rust**
```rust
0x09 => { // T2 high latch/load
    let lo = self.regs[0x08] as u16; 
    let hi = val as u16; 
    let full = (hi << 8) | lo; 
    self.t2_latch = full; 
    
    // AJUSTE ESPECIAL para 0x7530 (30000) -> 600 cycles
    let adjusted_value = if full == 0x7530 { 600 } else { full };
    self.t2_counter = adjusted_value;
    
    self.t2_enabled = true;
    self.t2_int_enabled = true;
    // IFR clearing logic...
}
```

#### **JavaScript**
```javascript
case 0x09: // T2 high
    this.via_t2c = (data << 8) | this.via_t2ll;
    this.via_ifr &= 0xdf;  // Clear IFR bit 5
    this.via_t2on = 1;
    this.via_t2int = 1;
    break;
```

**🔍 Análisis**: Rust tiene ajuste especial para valor 0x7530, JavaScript no.

---

## 📡 **6. IRQ Management**

### **Rust recompute_irq()**
```rust
fn recompute_irq(&mut self){ 
    let ifr_flags = self.ifr() & 0x7F; 
    let ier_mask = self.ier() & 0x7F; 
    let pending = (ifr_flags & ier_mask) != 0; 
    
    // Update IFR bit 7
    if pending {
        self.regs[0x0D] |= 0x80;  
    } else {
        self.regs[0x0D] &= 0x7F;  
    }
    
    if pending != self.irq_line { 
        self.irq_line = pending; 
        if let Some(cb) = &self.on_irq_change { 
            cb(pending); 
        } 
    } 
}
```

### **JavaScript IRQ Logic**
```javascript
// Timer1 underflow
this.via_ifr |= 0x40;
if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) ) {
    this.via_ifr |= 0x80;  // Set master IRQ bit
} else {
    this.via_ifr &= 0x7f;  // Clear master IRQ bit
}

// Timer2 underflow
this.via_ifr |= 0x20;
if( (this.via_ifr & 0x7f) & (this.via_ier & 0x7f) ) {
    this.via_ifr |= 0x80;  
} else {
    this.via_ifr &= 0x7f;  
}
```

**🔍 Análisis**: Lógica IRQ equivalente en ambos, bit 7 de IFR como master interrupt.

---

## 🕐 **7. Timing Execution Flow**

### **Rust**
1. CPU ejecuta instrucción → `advance_cycles(n)`
2. `advance_cycles()` → loop `for _ in 0..cyc { bus.tick(1) }`
3. `bus.tick(1)` → `via.tick(1)` + `psg.tick(1)`
4. `via.tick(1)` → procesa Timer1/Timer2 por 1 ciclo
5. Timer underflow → Set IFR → `recompute_irq()` → callback CPU

### **JavaScript**
1. `vecx_emu(cycles)` loop principal
2. `e6809_sstep()` → retorna icycles ejecutados
3. Loop `for(c = 0; c < icycles; c++)` 
4. Cada ciclo: actualiza Timer1, Timer2, shift register
5. Timer underflow → Set IFR bit → recompute master IRQ bit

**🔍 Análisis**: Flujo similar pero JavaScript hace más processing por ciclo.

---

## 🎯 **8. Problemas Identificados**

### **🚨 Críticos**
1. **25x Diferencia de Timing**: JavaScript 38,103 vs Rust 1,525 steps para llegar a F4EB
2. **Timer2 Values**: 0x88 vs 0xFF en mismo punto de ejecución
3. **Registros CPU Inválidos**: JavaScript muestra A=0x100, B=0x4000C (imposible en 6809)

### **⚠️ Importantes**  
4. **ACR Timer2 Check**: JavaScript verifica `(via_acr & 0x20) == 0x00`, Rust no
5. **Special Timer2 Adjustment**: Rust ajusta 0x7530→600, JavaScript no
6. **IRQ Callback Timing**: Rust usa callback, JavaScript directo a CPU

### **✅ Funcionando Correctamente**
- Lógica básica de countdown Timer1/Timer2
- IFR/IER bit manipulation  
- Cycle-by-cycle timing implementation
- One-shot vs continuous mode logic
- Underflow detection y reload

---

## 🔧 **9. Recomendaciones**

### **Inmediatas**
1. **Investigar CPU register overflow** en JavaScript (A>8bit, B>8bit)
2. **Agregar ACR check en Rust Timer2**: `if self.t2_enabled && (acr & 0x20) == 0`
3. **Comparar inicialización VIA** entre emuladores

### **A Medio Plazo**
4. **Profiling detallado**: ¿Por qué JavaScript es 25x más lento?
5. **Validar Timer2 adjustment**: ¿Es correcto 0x7530→600 en Rust?
6. **Sync test específico**: Timer values a intervalos fijos

### **Investigación**
7. **BIOS timing expectations**: ¿Qué espera realmente la BIOS?
8. **Reference implementation**: Comparar con Vectrexy C++ 
9. **Hardware documentation**: VIA 6522 datasheet vs implementation

---

## 📈 **10. Estado Final**

**✅ CONFIRMADO**: Ambos emuladores salen del bucle F4EB inmediatamente  
**❌ PROBLEMA**: Timing masivamente diferente para llegar al mismo punto  
**🔍 INVESTIGACIÓN**: Diferencias fundamentales en CPU/VIA initialization o timing model

La implementación VIA es **funcionalmente equivalente** pero con **timing characteristics completamente diferentes**.