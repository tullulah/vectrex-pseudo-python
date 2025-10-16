# JSVecx Debug Backend Implementation

**Fecha**: 2025-10-16  
**Estado**: Phase 2A - Breakpoint Detection COMPLETE  
**Archivos Modificados**: `ide/frontend/public/jsvecx_deploy/vecx.js`

---

## 1. Overview

Implementación completa del backend de debugging en JSVecx con:
- ✅ Detección de breakpoints en tiempo real
- ✅ Step modes (Over/Into/Out)
- ✅ postMessage bidireccional con IDE
- ✅ Control de estado (stopped/running/paused)
- ✅ Call stack tracking (básico)

---

## 2. Estado del Debugger

```javascript
// Añadido al constructor VecX()
this.debugState = 'stopped'; // 'stopped' | 'running' | 'paused'
this.breakpoints = new Set(); // Set de direcciones con breakpoints
this.stepMode = null; // null | 'over' | 'into' | 'out'
this.stepTargetAddress = null; // Dirección objetivo para step over
this.callStackDepth = 0; // Profundidad de la pila de llamadas (para step out)
```

### Estados del Debugger:
- **stopped**: Emulador detenido, no ejecuta código
- **running**: Emulador ejecutando normalmente (checks de breakpoints activos)
- **paused**: Emulador pausado en breakpoint o step

---

## 3. Detección de Breakpoints (vecx_emu)

### Implementación en `vecx_emu()`:

```javascript
while( cycles > 0 )
{
    // 1. CHECK BREAKPOINT - ANTES de ejecutar instrucción
    var currentPC = e6809.reg_pc;
    if (this.debugState === 'running' && this.breakpoints.has(currentPC)) {
        this.pauseDebugger('breakpoint', currentPC);
        return; // Detener ejecución inmediatamente
    }
    
    // 2. CHECK STEP OVER - Pausar al llegar a targetAddress
    if (this.stepMode === 'over' && currentPC === this.stepTargetAddress) {
        this.pauseDebugger('step', currentPC);
        this.stepMode = null;
        this.stepTargetAddress = null;
        return;
    }
    
    // 3. CHECK STEP INTO - Pausar en CADA instrucción
    if (this.stepMode === 'into') {
        this.pauseDebugger('step', currentPC);
        this.stepMode = null;
        return;
    }
    
    // 4. CHECK STEP OUT - Pausar cuando callStackDepth === 0
    if (this.stepMode === 'out' && this.callStackDepth === 0) {
        this.pauseDebugger('step', currentPC);
        this.stepMode = null;
        return;
    }
    
    // 5. EJECUTAR INSTRUCCIÓN
    icycles = e6809.e6809_sstep(this.via_ifr & 0x80, 0);
    
    // 6. TRACK CALL STACK (para step out)
    if (this.stepMode === 'out') {
        var opcode = this.read8(currentPC);
        if (opcode === 0xBD || opcode === 0x17 || opcode === 0x9D || opcode === 0xAD) { // JSR
            this.callStackDepth++;
        } else if (opcode === 0x39) { // RTS
            this.callStackDepth--;
        }
    }
}
```

### Opcodes Críticos (6809):
- **0xBD**: JSR Extended (absolute address)
- **0x17**: LBSR (long branch to subroutine)
- **0x9D**: JSR Direct
- **0xAD**: JSR Indexed
- **0x39**: RTS (return from subroutine)

---

## 4. Métodos de Control del Debugger

### 4.1 Pausar Ejecución

```javascript
this.pauseDebugger = function(mode, pc) {
    this.debugState = 'paused';
    
    var registers = this.getRegisters();
    var callStack = this.buildCallStack();
    
    // Enviar evento al IDE vía postMessage
    if (window.parent !== window) {
        window.parent.postMessage({
            type: 'debugger-paused',
            pc: '0x' + pc.toString(16).toUpperCase().padStart(4, '0'),
            mode: mode, // 'breakpoint' | 'step' | 'manual'
            registers: registers,
            callStack: callStack,
            cycles: this.totalCycles
        }, '*');
    }
}
```

**Parámetros**:
- `mode`: Razón de la pausa ('breakpoint', 'step', 'manual')
- `pc`: Program Counter actual (dirección donde se pausó)

**Evento enviado al IDE**:
```javascript
{
    type: 'debugger-paused',
    pc: '0x0050',
    mode: 'breakpoint',
    registers: { PC: 80, A: 128, B: 0, ... },
    callStack: [{ function: 'MAIN', line: 10, address: '0x0050', type: 'vpy' }],
    cycles: 5342
}
```

---

### 4.2 Breakpoint Management

#### Añadir Breakpoint
```javascript
this.addBreakpoint = function(address) {
    if (typeof address === 'string') {
        address = parseInt(address, 16); // '0x0050' → 80
    }
    this.breakpoints.add(address);
    console.log('[JSVecx Debug] Breakpoint added at 0x' + address.toString(16));
}
```

#### Eliminar Breakpoint
```javascript
this.removeBreakpoint = function(address) {
    if (typeof address === 'string') {
        address = parseInt(address, 16);
    }
    this.breakpoints.delete(address);
    console.log('[JSVecx Debug] Breakpoint removed from 0x' + address.toString(16));
}
```

#### Limpiar Todos
```javascript
this.clearBreakpoints = function() {
    this.breakpoints.clear();
    console.log('[JSVecx Debug] All breakpoints cleared');
}
```

**Uso desde IDE**:
```javascript
// Añadir breakpoint en línea 10 (dirección 0x0050)
window.frames[0].postMessage({
    type: 'debug-add-breakpoint',
    address: '0x0050',
    line: 10
}, '*');
```

---

### 4.3 Continue / Pause / Stop

#### Continue (F5)
```javascript
this.debugContinue = function() {
    if (this.debugState === 'paused') {
        this.debugState = 'running';
        console.log('[JSVecx Debug] Continuing execution');
        
        // Reiniciar el loop de emulación si está detenido
        if (!this.running) {
            this.vecx_emuloop();
        }
    }
}
```

#### Pause
```javascript
this.debugPause = function() {
    if (this.debugState === 'running') {
        this.pauseDebugger('manual', this.e6809.reg_pc);
    }
}
```

#### Stop
```javascript
this.debugStop = function() {
    this.debugState = 'stopped';
    this.running = false;
    this.stepMode = null;
    this.stepTargetAddress = null;
    this.callStackDepth = 0;
    console.log('[JSVecx Debug] Execution stopped');
}
```

---

### 4.4 Step Modes

#### Step Over (F10)
Ejecuta hasta la **siguiente línea VPy**.

```javascript
this.debugStepOver = function(targetAddress) {
    if (typeof targetAddress === 'string') {
        targetAddress = parseInt(targetAddress, 16);
    }
    
    this.stepMode = 'over';
    this.stepTargetAddress = targetAddress;
    this.debugState = 'running';
    
    console.log('[JSVecx Debug] Step Over to 0x' + targetAddress.toString(16));
    
    if (!this.running) {
        this.vecx_emuloop();
    }
}
```

**Lógica**:
1. IDE calcula la dirección de la siguiente línea VPy usando `.pdb` lineMap
2. Envía `debug-step-over` con `targetAddress: '0x0060'`
3. JSVecx ejecuta hasta `PC === targetAddress`, luego pausa

**Ejemplo**:
```vpy
func main() {
    WAIT_RECAL();        # Línea 1 → 0x0000
    SET_INTENSITY(128);  # Línea 2 → 0x0050 ← Breakpoint actual
    MOVE(0, 0);          # Línea 3 → 0x0060 ← Target para Step Over
}
```

IDE envía: `{ type: 'debug-step-over', targetAddress: '0x0060' }`  
JSVecx ejecuta hasta PC=0x0060, pausa, envía `debugger-paused`.

---

#### Step Into (F11)
Entra en **cada instrucción** (VPy o nativa).

```javascript
this.debugStepInto = function(isNativeCall) {
    this.stepMode = 'into';
    this.debugState = 'running';
    
    console.log('[JSVecx Debug] Step Into (native=' + isNativeCall + ')');
    
    // Ejecutar UNA instrucción y pausar
    if (!this.running) {
        this.vecx_emuloop();
    }
}
```

**Lógica**:
1. Activa `stepMode = 'into'`
2. En el siguiente check de `vecx_emu()`, pausa inmediatamente
3. Permite debugging instrucción por instrucción

**Uso**:
- Si línea actual es **función VPy**: Navega a su definición
- Si línea actual es **llamada nativa** (WAIT_RECAL): Muestra ASM de la BIOS

---

#### Step Out (Shift+F11)
Sale de la **función actual** hasta el RTS.

```javascript
this.debugStepOut = function() {
    this.stepMode = 'out';
    this.callStackDepth = 0; // Reset depth counter
    this.debugState = 'running';
    
    console.log('[JSVecx Debug] Step Out');
    
    if (!this.running) {
        this.vecx_emuloop();
    }
}
```

**Lógica**:
1. Activa `stepMode = 'out'`
2. Inicia `callStackDepth = 0`
3. Durante ejecución:
   - Detecta **JSR**: `callStackDepth++` (entramos en otra función)
   - Detecta **RTS**: `callStackDepth--` (salimos)
4. Cuando `callStackDepth === 0` → pausamos (volvimos al nivel original)

**Ejemplo**:
```asm
MAIN:
    JSR LOOP_BODY    ; Llamamos a función
    BRA MAIN

LOOP_BODY:          ; ← Estamos aquí, hacemos Step Out
    LDA #$80
    JSR SUB_FUNC     ; callStackDepth++ (entramos)
    RTS              ; callStackDepth-- (salimos de SUB_FUNC)
    RTS              ; callStackDepth === 0 → PAUSA (salimos de LOOP_BODY)
```

---

## 5. postMessage API (IDE ↔ JSVecx)

### 5.1 Mensajes IDE → JSVecx

#### Continue
```javascript
{
    type: 'debug-continue'
}
```

#### Pause
```javascript
{
    type: 'debug-pause'
}
```

#### Stop
```javascript
{
    type: 'debug-stop'
}
```

#### Step Over
```javascript
{
    type: 'debug-step-over',
    targetAddress: '0x0060'  // Dirección de la siguiente línea VPy
}
```

#### Step Into
```javascript
{
    type: 'debug-step-into',
    isNativeCall: false  // true si es WAIT_RECAL, false si es función VPy
}
```

#### Step Out
```javascript
{
    type: 'debug-step-out'
}
```

#### Add Breakpoint
```javascript
{
    type: 'debug-add-breakpoint',
    address: '0x0050',  // Dirección en hexadecimal
    line: 10            // Línea VPy (opcional, para logging)
}
```

#### Remove Breakpoint
```javascript
{
    type: 'debug-remove-breakpoint',
    address: '0x0050',
    line: 10
}
```

#### Clear All Breakpoints
```javascript
{
    type: 'debug-clear-breakpoints'
}
```

---

### 5.2 Mensajes JSVecx → IDE

#### Debugger Paused
```javascript
{
    type: 'debugger-paused',
    pc: '0x0050',                    // Program Counter (hex)
    mode: 'breakpoint',              // 'breakpoint' | 'step' | 'manual'
    registers: {                     // Estado de registros CPU
        PC: 80,
        A: 128,
        B: 0,
        X: 0,
        Y: 0,
        U: 0,
        S: 49151,
        DP: 0,
        CC: 0
    },
    callStack: [                     // Pila de llamadas
        {
            function: 'LOOP_BODY',
            line: 8,
            address: '0x0050',
            type: 'vpy'
        },
        {
            function: 'MAIN',
            line: 3,
            address: '0x0010',
            type: 'vpy'
        }
    ],
    cycles: 5342                     // Total de cycles ejecutados
}
```

---

## 6. Listener Setup (Auto-Inicializado)

```javascript
this.setupDebugListeners = function() {
    var vecx = this;
    
    window.addEventListener('message', function(event) {
        var msg = event.data;
        if (!msg || !msg.type) return;
        
        console.log('[JSVecx Debug] Received message:', msg.type);
        
        switch (msg.type) {
            case 'debug-continue':
                vecx.debugContinue();
                break;
            case 'debug-pause':
                vecx.debugPause();
                break;
            case 'debug-step-over':
                if (msg.targetAddress) {
                    vecx.debugStepOver(msg.targetAddress);
                }
                break;
            // ... resto de casos ...
        }
    });
    
    console.log('[JSVecx Debug] Listeners setup complete');
}

// Auto-setup al crear el emulador
this.setupDebugListeners();
```

**IMPORTANTE**: Los listeners se configuran automáticamente al instanciar `new VecX()`.

---

## 7. Integration con debugStore.ts

### Ejemplo: Step Over desde el IDE

**debugStore.ts**:
```typescript
stepOver: () => {
    const { currentVpyLine, pdbData } = get();
    
    // 1. Buscar la siguiente línea en lineMap
    const nextLine = currentVpyLine + 1;
    const targetAddress = pdbData.lineMap[nextLine];
    
    if (!targetAddress) {
        console.error('No address found for line', nextLine);
        return;
    }
    
    // 2. Enviar mensaje a JSVecx
    window.frames[0].postMessage({
        type: 'debug-step-over',
        targetAddress: targetAddress
    }, '*');
    
    // 3. Actualizar estado local
    set({ state: 'running' });
}
```

### Ejemplo: Recibir debugger-paused

**debugStore.ts**:
```typescript
// Listener para eventos de JSVecx
window.addEventListener('message', (event) => {
    const msg = event.data;
    
    if (msg.type === 'debugger-paused') {
        const { pc, mode, registers, callStack, cycles } = msg;
        
        // Buscar línea VPy correspondiente al PC
        const vpyLine = findLineByAddress(pc, get().pdbData.lineMap);
        
        // Actualizar estado
        set({
            state: 'paused',
            currentVpyLine: vpyLine,
            currentAsmAddress: pc,
            registers: registers,
            callStack: callStack,
            totalCycles: cycles
        });
        
        console.log(`[Debug] Paused at line ${vpyLine}, PC=${pc}, mode=${mode}`);
    }
});
```

---

## 8. Call Stack Tracking (TODO: Enhanced)

### Implementación Actual (Básica):

```javascript
this.buildCallStack = function() {
    // TODO: Implementar tracking real de JSR/RTS
    return [{
        function: 'MAIN',
        line: 0,
        address: '0x' + this.e6809.reg_pc.toString(16).toUpperCase().padStart(4, '0'),
        type: 'vpy'
    }];
}
```

### Implementación Futura (Enhanced):

1. **Mantener stack de llamadas**:
   ```javascript
   this.callStackFrames = []; // Array de { function, address, returnAddress }
   ```

2. **Detectar JSR y pushear frame**:
   ```javascript
   if (opcode === 0xBD) { // JSR Extended
       var targetAddr = this.read16(this.e6809.reg_pc);
       this.callStackFrames.push({
           function: this.lookupSymbol(targetAddr), // Buscar en .pdb symbols
           address: targetAddr,
           returnAddress: this.e6809.reg_pc + 3
       });
   }
   ```

3. **Detectar RTS y popear frame**:
   ```javascript
   if (opcode === 0x39) { // RTS
       this.callStackFrames.pop();
   }
   ```

4. **buildCallStack retorna frames reales**:
   ```javascript
   this.buildCallStack = function() {
       return this.callStackFrames.map(frame => ({
           function: frame.function,
           line: this.lookupLine(frame.address), // Buscar en lineMap
           address: '0x' + frame.address.toString(16).toUpperCase().padStart(4, '0'),
           type: this.isNativeAddress(frame.address) ? 'native' : 'vpy'
       }));
   }
   ```

---

## 9. Testing Checklist

### ✅ Phase 2A Complete:
- [x] Detección de breakpoints en `vecx_emu()`
- [x] Método `pauseDebugger(mode, pc)`
- [x] `addBreakpoint()` / `removeBreakpoint()` / `clearBreakpoints()`
- [x] `debugContinue()` / `debugPause()` / `debugStop()`
- [x] `debugStepOver(targetAddress)`
- [x] `debugStepInto(isNativeCall)`
- [x] `debugStepOut()`
- [x] postMessage listener setup
- [x] Evento `debugger-paused` enviado al IDE

### 📋 Phase 2B Pending:
- [ ] Poblar .pdb con direcciones reales (actualmente 0x0000)
- [ ] Implementar lineMap (línea VPy → dirección ASM)
- [ ] Test: Añadir breakpoint línea 10, verificar pausa en PC correcto
- [ ] Test: F10 Step Over avanza a siguiente línea
- [ ] Test: F11 Step Into entra en función VPy
- [ ] Test: Shift+F11 Step Out sale de función actual

### 📋 Phase 3 Pending:
- [ ] Enhanced call stack tracking (JSR/RTS monitoring)
- [ ] Symbol lookup desde .pdb
- [ ] Diferenciar funciones VPy vs nativas en call stack
- [ ] Stack viewer UI component

### 📋 Phase 4 Pending:
- [ ] ASM disassembler (6809 opcode parser)
- [ ] Mostrar ASM dinámico en panel derecho
- [ ] Highlight current instruction en ASM
- [ ] Sincronizar scroll VPy ↔ ASM

---

## 10. Known Issues & Limitations

### ⚠️ Issues:
1. **Call Stack Placeholder**: `buildCallStack()` retorna array estático, no tracking real
2. **Step Out Simplificado**: Solo cuenta JSR/RTS, no maneja interrupts ni CWAI
3. **No Symbol Resolution**: Direcciones no se resuelven a nombres de función (pending .pdb)
4. **Step Into sin distinción**: No diferencia entre función VPy vs llamada nativa

### 🔧 Limitations:
1. **Breakpoints solo en PC**: No soporta breakpoints condicionales (ej: "parar si A === 0x80")
2. **No Data Breakpoints**: No detecta writes a memoria específica
3. **No Watchpoints**: No observa cambios en variables
4. **Sin memoria de breakpoints**: Se pierden al recargar página (pending persistencia)

---

## 11. Next Steps

### Immediate (Phase 2B):
1. **Modificar `m6809.rs`**: Track address durante `emit_with_debug()`
2. **Poblar lineMap**: Map línea VPy → dirección ASM
3. **Test workflow completo**: Breakpoint → Pause → Step Over → Continue

### Short Term (Phase 3):
1. **Enhanced Call Stack**: Implementar push/pop real en JSR/RTS
2. **Symbol Resolution**: Buscar nombres de función en .pdb symbols
3. **Integrate DebugSplitView**: Mostrar VPy + ASM sincronizado

### Long Term (Phase 4):
1. **ASM Disassembler**: Parser de opcodes 6809 → mnemonics
2. **Dynamic ASM View**: Generar ASM desde binary en tiempo real
3. **Call Stack Viewer**: Component dedicado para visualizar stack

---

## 12. Console Logging

Todos los métodos logean al console para debugging:

```javascript
[JSVecx Debug] Listeners setup complete
[JSVecx Debug] Breakpoint added at 0x50
[JSVecx Debug] Received message: debug-continue
[JSVecx Debug] Continuing execution
[JSVecx Debug] Paused at PC=50, mode=breakpoint
[JSVecx Debug] Step Over to 0x60
[JSVecx Debug] Step Into (native=false)
[JSVecx Debug] Step Out
[JSVecx Debug] Breakpoint removed from 0x50
[JSVecx Debug] All breakpoints cleared
[JSVecx Debug] Execution stopped
```

Usar Developer Tools Console para monitorear eventos en tiempo real.

---

**Última actualización**: 2025-10-16  
**Próxima fase**: Phase 2B - Populate .pdb with real addresses
