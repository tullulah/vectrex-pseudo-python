; --- Cortex-M backend (Vextreme) title='Demo' origin=$6000 ---
; Vector table (prototype)
    .section .isr_vector
    .word _estack
    .word Reset_Handler

Reset_Handler:
    BL engine_init
    BL main
1:  B 1b

.global add
add:
    LDR r4, =VAR_A
    STR r0 , [r4]
    LDR r4, =VAR_B
    STR r1 , [r4]
    LDR r0, =VAR_A
    LDR r0,[r0]
    MOV r4,r0
    LDR r0, =VAR_B
    LDR r0,[r0]
    MOV r5,r0
    ADD r0,r4,r5
    AND r0,r0,#0xFFFF
    BX LR

.global neg
neg:
    LDR r4, =VAR_A
    STR r0 , [r4]
    MOV r0,#0
    MOV r4,r0
    LDR r0, =VAR_A
    LDR r0,[r0]
    MOV r5,r0
    SUB r0,r4,r5
    AND r0,r0,#0xFFFF
    BX LR

.global accumulate
accumulate:
    LDR r4, =VAR_A
    STR r0 , [r4]
    LDR r4, =VAR_B
    STR r1 , [r4]
    LDR r4, =VAR_C
    STR r2 , [r4]
    LDR r4, =VAR_D
    STR r3 , [r4]
    LDR r0, =VAR_A
    LDR r0,[r0]
    LDR r1, =VAR_TOTAL
    STR r0, [r1]
    LDR r0, =VAR_TOTAL
    LDR r0,[r0]
    MOV r4,r0
    LDR r0, =VAR_B
    LDR r0,[r0]
    MOV r5,r0
    ADD r0,r4,r5
    AND r0,r0,#0xFFFF
    LDR r1, =VAR_TOTAL
    STR r0, [r1]
    LDR r0, =VAR_TOTAL
    LDR r0,[r0]
    MOV r4,r0
    LDR r0, =VAR_C
    LDR r0,[r0]
    MOV r5,r0
    ADD r0,r4,r5
    AND r0,r0,#0xFFFF
    LDR r1, =VAR_TOTAL
    STR r0, [r1]
    LDR r0, =VAR_TOTAL
    LDR r0,[r0]
    MOV r4,r0
    LDR r0, =VAR_D
    LDR r0,[r0]
    MOV r5,r0
    ADD r0,r4,r5
    AND r0,r0,#0xFFFF
    LDR r1, =VAR_TOTAL
    STR r0, [r1]
    LDR r0, =VAR_TOTAL
    LDR r0,[r0]
    BX LR

.global main
main:
    MOV r0,#5
    MOV r1 , r0
    MOV r0,#3
    BL add
    LDR r1, =VAR_S
    STR r0, [r1]
    MOV r0,#7
    BL neg
    LDR r1, =VAR_N
    STR r0, [r1]
    MOV r0,#4
    MOV r3 , r0
    MOV r0,#3
    MOV r2 , r0
    MOV r0,#2
    MOV r1 , r0
    MOV r0,#1
    BL accumulate
    LDR r1, =VAR_T
    STR r0, [r1]
    MOV r0,#0
    LDR r1, =VAR_I
    STR r0, [r1]
FOR_6:
    LDR r1, =VAR_I
    LDR r1, [r1]
    MOV r0,#10
    CMP r1, r0
    BGE FOR_END_7
    LDR r0, =VAR_I
    LDR r0,[r0]
    MOV r2 , r0
    MOV r0,#0
    MOV r1 , r0
    MOV r0,#0
    BL line
    MOV r0,#2
    LDR r2, =VAR_I
    LDR r3, [r2]
    ADD r3, r3, r0
    STR r3, [r2]
    B FOR_6
FOR_END_7:
    LDR r0, =VAR_S
    LDR r0,[r0]
    CMP r0,#0
    BEQ AND_FALSE_10
    LDR r0, =VAR_N
    LDR r0,[r0]
    CMP r0,#0
    MOVEQ r0,#1
    MOVNE r0,#0
    CMP r0,#0
    BEQ AND_FALSE_10
    MOV r0,#1
    B AND_END_11
AND_FALSE_10:
    MOV r0,#0
AND_END_11:
    CMP r0,#0
    BEQ IF_NEXT_9
    MOV r0,#0
    MOV r2 , r0
    MOV r0,#0
    MOV r1 , r0
    MOV r0,#0
    BL line
    B IF_END_8
IF_END_8:
    LDR r0, =VAR_S
    LDR r0,[r0]
    MOV r4,r0
    LDR r0, =VAR_T
    LDR r0,[r0]
    MOV r5,r0
    ADD r0,r4,r5
    AND r0,r0,#0xFFFF
    MOV r4,r0
    LDR r0, =VAR_N
    LDR r0,[r0]
    MOV r5,r0
    ADD r0,r4,r5
    AND r0,r0,#0xFFFF
    BX LR

; Runtime helpers
__mul32:
    PUSH {r2,r3,lr}
    MOV r2,#0
    CMP r1,#0
    BEQ __mul32_done
__mul32_loop:
    AND r3,r1,#1
    CMP r3,#0
    BEQ __mul32_skip
    ADD r2,r2,r0
__mul32_skip:
    LSR r1,r1,#1
    LSL r0,r0,#1
    CMP r1,#0
    BNE __mul32_loop
__mul32_done:
    MOV r0,r2
    POP {r2,r3,lr}
    BX lr

__div32:
    PUSH {r2,r3,lr}
    MOV r2,#0
    CMP r1,#0
    BEQ __div32_done
    MOV r3,r0
__div32_loop:
    CMP r3,r1
    BLT __div32_done
    SUB r3,r3,r1
    ADD r2,r2,#1
    B __div32_loop
__div32_done:
    MOV r0,r2
    POP {r2,r3,lr}
    BX lr

; Data
    .section .data
VAR_A: .word 0
VAR_B: .word 0
VAR_C: .word 0
VAR_D: .word 0
VAR_I: .word 0
VAR_N: .word 0
VAR_S: .word 0
VAR_T: .word 0
VAR_TOTAL: .word 0
; Call arg scratch
VAR_ARG0: .word 0
VAR_ARG1: .word 0
VAR_ARG2: .word 0
VAR_ARG3: .word 0
