# Análisis de Opcodes en Print_Str (F495)

## Secuencia Crítica de la BIOS

### Inicio de línea de texto (F4A5)
```asm
LF4A5:  STB     <VIA_port_b     ; $D7 VIA_port_b = B
        DEC     <VIA_port_b     ; $0A VIA_port_b (decrement)
        LDD     #$8081          ; $CC $80 $81 - Load D immediate
        NOP                     ; $12 - No operation
        INC     <VIA_port_b     ; $0C VIA_port_b (increment)
        STB     <VIA_port_b     ; $D7 VIA_port_b = B
        STA     <VIA_port_b     ; $97 VIA_port_b = A
        TST     $C800           ; $7D $C8 $00 - Test memory
        INC     <VIA_port_b     ; $0C VIA_port_b
        LDA     Vec_Text_Width  ; $96 $2B - Load A from $C82B
        STA     <VIA_port_a     ; $97 VIA_port_a = A  ← VELOCITY X
        LDD     #$0100          ; $CC $01 $00
        LDU     Vec_Str_Ptr     ; $FE $C8 $2A
        STA     <VIA_port_b     ; $97 VIA_port_b = A (disable RAMP)
        BRA     LF4CB           ; $20 - Branch
```

### Dibuja caracteres (F4C7)
```asm
LF4C7:  LDA     A,X             ; $A6 $86 - Indexed addressing
        STA     <VIA_shift_reg  ; $97 $0A - Store to shift register
LF4CB:  LDA     ,U+             ; $A6 $C0 - Load with post-increment
        BPL     LF4C7           ; $2A - Branch if positive
```

### ❌ SECCIÓN CRÍTICA: Retorno a izquierda (F4D3)
```asm
LF4D3:  LDA     #$81            ; $86 $81
        STA     <VIA_port_b     ; $97 $01 - Enable RAMP
        NEG     <VIA_port_a     ; $00 $00 - ❌ NEGATE DAC value
        LDA     #$01            ; $86 $01
        STA     <VIA_port_b     ; $97 $01 - Disable RAMP
```

**Opcodes a verificar aquí**:
- **NEG direct**: opcode $00 (addressing mode direct)
- **STA direct**: opcode $97

### Preparación para siguiente línea (F4E0)
```asm
        CMPX    #Char_Table_End-$20  ; $8C - Compare X immediate
        BEQ     LF50A           ; $27 - Branch if equal
        LEAX    $50,X           ; $30 $88 $50 - Load Effective Address
        TFR     U,D             ; $1F $03 - Transfer U to D
        SUBD    Vec_Str_Ptr     ; $B3 $C8 $2A - Subtract
        SUBB    #$02            ; $C0 $02 - Subtract immediate
        ASLB                    ; $58 - Arithmetic shift left
        BRN     LF4EB           ; $21 - Branch never (delay)
```

### ❌ DELAY LOOP con RAMP disabled (F4EB)
```asm
LF4EB:  LDA     #$81            ; $86 $81
        NOP                     ; $12
        DECB                    ; $5A - Decrement B
        BNE     LF4EB           ; $26 - Branch if not equal
```

**Opcode crítico**:
- **DECB**: opcode $5A
- **BNE**: opcode $26

### ❌ Movimiento vertical + limpieza (F4F0)
```asm
        STA     <VIA_port_b     ; $97 $01 - Enable RAMP
        LDB     Vec_Text_Height ; $D6 $2A - Load B from $C82A
        STB     <VIA_port_a     ; $D7 $00 - ❌ VELOCITY Y
        DEC     <VIA_port_b     ; $0A $01 - Enable mux
        LDD     #$8101          ; $CC $81 $01
        NOP                     ; $12 - ❌ Delay
        STA     <VIA_port_b     ; $97 $01 - Enable RAMP
        CLR     <VIA_port_a     ; $0F $00 - ❌ Clear DAC
        STB     <VIA_port_b     ; $D7 $01 - Disable RAMP
        STA     <VIA_port_b     ; $97 $01 - Enable RAMP
        LDB     #$03            ; $C6 $03
        BRA     LF4A5           ; $20 - Back to next line
```

**Opcodes críticos**:
- **CLR direct**: opcode $0F
- **DEC direct**: opcode $0A
- **STB direct**: opcode $D7

## Opcodes Sospechosos a Verificar

### 1. NEG <$00 (opcode $00)
**Función**: Negar valor en memoria (complemento a 2)
**Uso**: `NEG <VIA_port_a` para invertir velocidad X
**Verificar**: 
- Timing: ¿6 cycles?
- Side effects: ¿Afecta flags correctamente?
- Direct mode addressing

### 2. CLR <$00 (opcode $0F)  
**Función**: Limpiar valor en memoria (poner a 0)
**Uso**: `CLR <VIA_port_a` después de movimiento
**Verificar**:
- ¿Timing correcto?
- ¿Escribe realmente 0 al VIA?
- ¿Direct page correcta ($D000)?

### 3. DEC <$01 (opcode $0A)
**Función**: Decrementar memoria
**Uso**: `DEC <VIA_port_b` para cambiar bits de control
**Verificar**:
- ¿Decrementa correctamente?
- ¿Read-modify-write cycle?

### 4. STB/STA direct (opcodes $D7/$97)
**Función**: Store accumulator a memoria
**Verificar**:
- ¿Timing correcto con VIA?
- ¿Direct page = $D0?

### 5. DECB (opcode $5A)
**Función**: Decrementar registro B
**Uso**: Contador del delay loop
**Verificar**:
- ¿Afecta flags Z correctamente?
- ¿Timing 2 cycles?

### 6. BNE (opcode $26)
**Función**: Branch if Not Equal (Z=0)
**Uso**: Loop hasta B=0
**Verificar**:
- ¿Evalúa flag Z correctamente?
- ¿Branch taken = 3 cycles?

## Hipótesis de Bug

### Posibilidad 1: NEG no sincroniza con VIA
Si `NEG <VIA_port_a` no espera el ciclo correcto del VIA, la velocidad negativa puede no aplicarse inmediatamente, causando deriva.

### Posibilidad 2: CLR tiene timing incorrecto
Si `CLR <VIA_port_a` tarda menos cycles de lo esperado, el integrador puede no resetear completamente antes del siguiente enable RAMP.

### Posibilidad 3: Delay loop cuenta mal
Si `DECB + BNE` tienen timing incorrecto, el delay puede ser más corto/largo, afectando cuánto deriva el beam.

### Posibilidad 4: DEC en VIA_port_b no hace read-modify-write
`DEC <VIA_port_b` debe:
1. Leer VIA_port_b
2. Decrementar valor
3. Escribir de vuelta

Si falta algún paso, los bits de control RAMP pueden quedar incorrectos.

## Próximos Pasos

1. Verificar implementación de cada opcode en `emulator_v2/src/core/cpu6809.rs`
2. Comparar con Vectrexy C++ `Cpu.cpp`
3. Verificar timing exacto de cada instrucción
4. Verificar flags afectados (especialmente Z para BNE)
5. Instrumentar ejecución de Print_Str para ver valores reales

## Test a Crear

```rust
#[test]
fn test_print_str_opcodes_timing() {
    // Simular secuencia exacta de Print_Str línea por línea
    // Verificar:
    // 1. NEG <$00 niega correctamente
    // 2. CLR <$00 limpia a 0
    // 3. DEC <$01 decrementa
    // 4. DECB + BNE loop correcto
    // 5. Timing total coincide con BIOS
}
```
