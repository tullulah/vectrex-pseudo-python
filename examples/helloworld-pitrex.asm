; --- ARM backend (PiTrex) --- title='Hello Demo' origin=$0400 ---
;***************************************************************************
; DEFINE SECTION (ARM)
;***************************************************************************
; (No BIOS header required for PiTrex)

;***************************************************************************
; CODE SECTION
;***************************************************************************
; Entry point
.global _start
_start:
    BL pitrex_init ; engine init placeholder
    BL main
1:  B 1b ; loop

.global main
main:
    BL set_origin
    MOV r0, #95
    BL set_intensity
    LDR r0, =STR_0
    MOV r2 , r0
    MOV r0, #0
    MOV r1 , r0
    MOV r0, #0
    BL print_text
    MOV r0, #0
    MOV r1 , r0
    MOV r0, #0
    BL move_to
    MOV r0, #15
    MOV r2 , r0
    MOV r0, #0
    MOV r1 , r0
    MOV r0, #50
    BL draw_to
    MOV r0, #15
    MOV r2 , r0
    MOV r0, #40
    MOV r1 , r0
    MOV r0, #25
    BL draw_to
    MOV r0, #15
    MOV r2 , r0
    MOV r0, #0
    MOV r1 , r0
    MOV r0, #0
    BL draw_to
    MOV r0, #0
    BX LR

;***************************************************************************
; RUNTIME SECTION
;***************************************************************************
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

;***************************************************************************
; DATA SECTION
;***************************************************************************
; Data segment (prototype)
.data
; String literals (null-terminated)
STR_0: .ascii "HELLO VECTREX"
    .byte 0
; Call arg scratch (if needed by future ABI changes)
VAR_ARG0: .word 0
VAR_ARG1: .word 0
VAR_ARG2: .word 0
VAR_ARG3: .word 0
