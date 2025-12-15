# Análisis del ASM Generado por VIDE (gcc6809)

## Compilación VIDE
- **Compilador**: gcc6809 4.3.6
- **Optimización**: -O0 (sin optimizaciones)
- **Opciones**: `-fomit-frame-pointer -mint8`
- **Resultado**: ✅ ROM funcional, dibuja cuadrado estable

## Estructura del main() Generado

```asm
_main:
    ldb     #127                    ; Intensity = 0x7F
    jsr     __Intensity_a           ; Llamar BIOS Intensity_a
L17:
    jsr     ___Wait_Recal           ; Llamar BIOS Wait_Recal
    ldb     #80                     ; scaleList = 0x50
    stb     ,-s                     ; Push a stack
    ldb     #127                    ; scaleMove = 0x7F
    stb     ,-s                     ; Push a stack
    clr     ,-s                     ; y = 0, push a stack
    clrb                            ; x = 0 en B
    ldx     #_vector_list           ; Puntero a datos en X
    jsr     _draw_synced_list_c     ; Llamar función
    leas    3,s                     ; Limpiar stack
    bra     L17                     ; Loop infinito
```

**Observaciones**:
- Usa BIOS `Wait_Recal` (no hace frame init en main, lo hace dentro de draw_synced_list_c)
- Stack-based calling convention (parámetros en stack)
- Loop infinito estable

## Frame Initialization (draw_synced_list_c inicio)

```asm
_draw_synced_list_c:
    pshs    u                       ; Guardar U
    leas    -4,s                    ; Reservar 4 bytes stack (vars locales)
    stx     1,s                     ; Guardar u (puntero lista)
    stb     ,s                      ; Guardar y
L14:                                ; ← DO-WHILE empieza aquí
    ; FRAME INIT SEQUENCE
    clr     _VIA_shift_reg          ; Blank beam (0xD05A = 0)
    ldb     #-52                    ; 0xCC (zero integrators)
    stb     _VIA_cntl               ; VIA_cntl = 0xCC
    clr     _VIA_port_a             ; Reset offset (DAC = 0)
    ldb     #-126                   ; 0x82
    stb     _VIA_port_b             ; VIA_port_b = 0x82
    ldb     9,s                     ; Cargar scaleMove desde stack
    stb     _VIA_t1_cnt_lo          ; Timer scale
    
    ; DELAY LOOP (5 iterations)
    ldb     #5                      ; ZERO_DELAY = 5
    stb     3,s                     ; Guardar contador en stack
    bra     L2
L3:
    ldb     3,s                     ; Cargar contador
    decb                            ; Decrementar
    stb     3,s                     ; Guardar
L2:
    ldb     3,s                     ; Cargar contador
    tstb                            ; Test if > 0
    bgt     L3                      ; Loop si > 0
    
    ldb     #-125                   ; 0x83
    stb     _VIA_port_b             ; VIA_port_b = 0x83
```

**Clave**: Frame init se ejecuta **DENTRO del do-while**, en cada iteración de la lista de vectores.

## Move to Location (después de delay)

```asm
    ldb     ,s                      ; Cargar y desde stack
    stb     _VIA_port_a             ; Y → DAC
    ldb     #-50                    ; 0xCE (integrator mode)
    stb     _VIA_cntl               ; VIA_cntl = 0xCE
    clr     _VIA_port_b             ; Mux enable (0)
    ldb     #1
    stb     _VIA_port_b             ; Mux disable (1)
    ldb     8,s                     ; Cargar x desde stack
    stb     _VIA_port_a             ; X → DAC
    clr     _VIA_t1_cnt_hi          ; Start timer
    
    ldb     10,s                    ; Cargar scaleList
    stb     _VIA_t1_cnt_lo          ; Set scale para draws
    
    ldd     1,s                     ; Cargar puntero u
    addd    #3                      ; u += 3 (skip header)
    std     1,s                     ; Guardar u actualizado
```

**Clave**: Mueve beam a posición (y, x) inicial antes de procesar la lista.

## Vector List Processing Loop

```asm
L7:                                 ; ← WHILE(1) empieza aquí
    ldb     [1,s]                   ; Cargar *u (flag byte)
    tstb
    bge     L8                      ; Si >= 0, no es draw line
    
    ; DRAW LINE (*u < 0)
    ldu     1,s
    leax    1,u                     ; u+1
    ldb     ,x                      ; dy
    stb     _VIA_port_a             ; Y delta → DAC
    clr     _VIA_port_b             ; Mux enable
    ldb     #1
    stb     _VIA_port_b             ; Mux disable
    ldu     1,s
    leax    2,u                     ; u+2
    ldb     ,x                      ; dx
    stb     _VIA_port_a             ; X delta → DAC
    clr     _VIA_t1_cnt_hi          ; Start timer
    ldb     #-1                     ; 0xFF
    stb     _VIA_shift_reg          ; Beam ON
L9:
    ldb     _VIA_int_flags          ; Poll timer
    andb    #64                     ; Bit 6 (Timer1 done)
    tstb
    beq     L9                      ; Wait loop
    clr     _VIA_shift_reg          ; Beam OFF
    bra     L10                     ; Continuar
    
L8:
    ldb     [1,s]                   ; *u
    tstb
    bne     L11                     ; Si != 0, break
    
    ; MOVE TO (*u == 0)
    ; [código moveTo similar a draw line pero sin beam ON]
    
L10:
    ldd     1,s                     ; u
    addd    #3                      ; u += 3 (siguiente entrada)
    std     1,s
    lbra    L7                      ; WHILE(1) continua
    
L11:                                ; ← BREAK del while(1)
    ldb     [1,s]                   ; *u
    cmpb    #2                      ; ¿Es end marker (2)?
    lbne    L14                     ; Si no, repetir DO-WHILE
    
    ; Cleanup y return
    leas    4,s
    puls    u,pc                    ; Return
```

## Vector List Data

```asm
_vector_list:
    .byte   0, 0, 0                 ; Header (y=0, x=0, next_y=0)
    .byte   -1, -40, -40            ; Line 1: draw dy=-40, dx=-40
    .byte   -1, 0, 80               ; Line 2: draw dy=0, dx=80
    .byte   -1, 80, 0               ; Line 3: draw dy=80, dx=0
    .byte   -1, 0, -80              ; Line 4: draw dy=0, dx=-80
    .byte   2                       ; End marker
```

**Formato**:
- Header: 3 bytes (y inicial, x inicial, next_y - no usado en este ejemplo)
- Cada entrada: 3 bytes
  - Byte 1: Flag (`-1` = draw, `0` = moveTo, `2` = end)
  - Byte 2: dy (delta Y)
  - Byte 3: dx (delta X)

## Comparación con Nuestro Backend

### VIDE (Malban Algorithm)
```asm
; Frame init DENTRO del do-while
clr     _VIA_shift_reg
ldb     #0xCC
stb     _VIA_cntl
; ... delay loop ...
ldb     #0x83
stb     _VIA_port_b

; Move to initial position
ldb     y
stb     _VIA_port_a
; ... mux sequence ...

; Process vector list
while(1) {
    if (*u < 0) {
        ; Draw line con beam ON
    } else if (*u == 0) {
        ; MoveTo sin beam
    } else break;
    u += 3;
}
while (*u != 2);  // Repeat do-while
```

### Nuestro Backend (test_simple_line)
```asm
LOOP_BODY:
    JSR     Wait_Recal          ; BIOS hace setup
    
    ; Inline VIA (7 instrucciones)
    LDA     #dy
    STA     $D000               ; Y delta
    CLR     $D002               ; Mux enable
    LDA     #1
    STA     $D002               ; Mux disable
    LDA     #dx
    STA     $D000               ; X delta
    CLR     $D005               ; Start timer
    LDA     #$FF
    STA     $D05A               ; Beam ON
DRAW_WAIT:
    LDA     $D00D               ; Poll timer
    ANDA    #$40
    BEQ     DRAW_WAIT
    CLR     $D05A               ; Beam OFF
    
    BRA     LOOP_BODY           ; Loop directo
```

## Diferencias Clave

| Aspecto | VIDE/Malban | Nuestro Backend |
|---------|-------------|-----------------|
| **Frame Init** | Sí, en cada iteración do-while | No, Wait_Recal lo maneja |
| **Delay Loop** | Explícito (5 iterations) | No necesario |
| **VIA_port_b** | 0x82 → delay → 0x83 | No se toca |
| **Integrator Zero** | VIA_cntl = 0xCC antes | Wait_Recal lo hace |
| **Data Structure** | Vector list en memoria | Parámetros directos |
| **Uso de caso** | Múltiples vectores/frame | Una línea por llamada |
| **Complejidad** | ~150 instrucciones | ~15 instrucciones |

## Conclusiones

1. **Frame Init es para Vector Lists**: El algoritmo de Malban está diseñado para procesar listas complejas de vectores desde memoria. Frame init se ejecuta una vez por lista, no por línea.

2. **Wait_Recal es suficiente para líneas simples**: Nuestro backend que usa `JSR Wait_Recal` + inline VIA funciona perfectamente porque:
   - Wait_Recal ya hace el reset necesario
   - Solo dibujamos una línea por llamada
   - No necesitamos el overhead de frame init

3. **Diferentes paradigmas**:
   - **Malban**: Procesar listas completas de vectores (juegos complejos como Minestorm)
   - **Nuestro**: Dibujar líneas individuales on-demand (programación procedural)

4. **Ambos enfoques son válidos**:
   - VIDE/Malban: Óptimo para sprites complejos predefinidos
   - Nuestro backend: Óptimo para gráficos procedurales/dinámicos

5. **Por qué test_malban_exact.vpy falló**: Intentamos usar frame init (diseñado para listas) con llamadas individuales DRAW_LINE. El frame init conflictuaba con el estado que Wait_Recal ya había configurado.

## Referencias

- `malban_vide_reference.bin`: ROM de referencia que funciona ✅
- `test_simple_line.bin`: Nuestro inline VIA que funciona ✅
- `test_malban_exact.bin`: Intento de mezclar ambos enfoques ❌
- `/vide project/test/build/lib/main.s`: ASM generado por gcc6809
