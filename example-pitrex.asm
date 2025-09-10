; --- ARM backend (PiTrex) --- title='UNTITLED' origin=$0400 ---
; Entry point
.global _start
_start:
    BL pitrex_init ; engine init placeholder
    BL main
1:  B 1b ; loop

.global add
add:
    LDR r4, =VAR_A
    STR r0 , [r4]
    LDR r4, =VAR_B
    STR r1 , [r4]
    LDR r0, =VAR_A
    LDR r0, [r0]
    MOV r4, r0
    LDR r0, =VAR_B
    LDR r0, [r0]
    MOV r5, r0
    ADD r0, r4, r5
    AND r0, r0, #0xFFFF
    BX LR

.global main
main:
    MOV r0, #10
    MOV r1 , r0
    MOV r0, #16
    BL add
    LDR r1, =VAR_S
    STR r0, [r1]
    LDR r0, =VAR_S
    LDR r0, [r0]
    MVN r0,r0
    AND r0,r0,#0xFFFF
    LDR r1, =VAR_INV
    STR r0, [r1]
    LDR r0, =VAR_S
    LDR r0, [r0]
    MOV r4, r0
    MOV r0, #3
    MOV r5, r0
    MOV r0,r4,LSL r5
    AND r0,r0,#0xFFFF
    MOV r4, r0
    MOV r0, #1
    MOV r5, r0
    MOV r0,r4,LSR r5
    AND r0,r0,#0xFFFF
    LDR r1, =VAR_SH
    STR r0, [r1]
    LDR r0, =VAR_SH
    LDR r0, [r0]
    MOV r4, r0
    MOV r0, #16
    MOV r5, r0
    MOV r0,r4
    MOV r1,r5
    BL __div32
    ; quotient now in r0 -> compute remainder r4 - r0*r5
    MOV r2,r0
    MUL r2,r2,r5
    RSBS r0,r2,r4
    AND r0,r0,#0xFFFF
    MOV r4, r0
    LDR r0, =VAR_INV
    LDR r0, [r0]
    MOV r4, r0
    MOV r0, #255
    MOV r5, r0
    AND r0, r4, r5
    MOV r5, r0
    EOR r0, r4, r5
    LDR r1, =VAR_R
    STR r0, [r1]
    MOV r0, #0
    LDR r1, =VAR_ACC
    STR r0, [r1]
    MOV r0, #0
    LDR r1, =VAR_I
    STR r0, [r1]
FOR_0:
    LDR r1, =VAR_I
    LDR r1, [r1]
    MOV r0, #16
    CMP r1, r0
    BGE FOR_END_1
    MOV r0, #0
    MOV r4, r0
    LDR r0, =VAR_I
    LDR r0, [r0]
    MOV r4, r0
    MOV r0, #3
    MOV r5, r0
    AND r0, r4, r5
    MOV r5, r0
    ADD r0, r4, r5
    AND r0, r0, #0xFFFF
    LDR r1, =VAR_ACC
    STR r0, [r1]
    MOV r0, #2
    LDR r2, =VAR_I
    LDR r3, [r2]
    ADD r3, r3, r0
    STR r3, [r2]
    B FOR_0
FOR_END_1:
    MOV r0, #0
    MOV r4, r0
    LDR r0, =VAR_R
    LDR r0, [r0]
    MOV r5, r0
    ADD r0, r4, r5
    AND r0, r0, #0xFFFF
    LDR r1, =VAR_ACC
    STR r0, [r1]
    LDR r0, =VAR_ACC
    LDR r0, [r0]
    MOV r4, r0
    LDR r0, =VAR_S
    LDR r0, [r0]
    MOV r5, r0
    ADD r0, r4, r5
    AND r0, r0, #0xFFFF
    MOV r4, r0
    LDR r0, =VAR_R
    LDR r0, [r0]
    MOV r5, r0
    ADD r0, r4, r5
    AND r0, r0, #0xFFFF
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

; Data segment (prototype)
.data
VAR_A: .word 0
VAR_ACC: .word 0
VAR_B: .word 0
VAR_I: .word 0
VAR_INV: .word 0
VAR_R: .word 0
VAR_S: .word 0
VAR_SH: .word 0
; Call arg scratch (if needed by future ABI changes)
VAR_ARG0: .word 0
VAR_ARG1: .word 0
VAR_ARG2: .word 0
VAR_ARG3: .word 0
