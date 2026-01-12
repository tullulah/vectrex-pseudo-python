; ========================================
; BANK 31 - Fixed Bank (Main Program)
; ========================================
; Minimal program with WAIT_RECAL and infinite loop

    INCLUDE "VECTREX.I"

    ORG $4000

START:
    LDA #$D0
    TFR A,DP            ; Set Direct Page for BIOS
    CLR $C80E           ; Clear button debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF          ; Initialize stack
    
    ; No Init_Music_Buf here - test if audio is the problem

MAIN:
    JSR Wait_Recal      ; Sync with CRT
    
    ; Set a simple intensity
    LDA #100
    JSR Intensity_a
    
    ; Loop forever
    BRA MAIN

; ========================================
; Interrupt Vectors (at end of ROM)
; ========================================
    ORG $5FF0
    FDB $0000    ; Reserved
    FDB $0000    ; SWI3
    FDB $0000    ; SWI2
    FDB $0000    ; FIRQ
    FDB $0000    ; IRQ
    FDB $0000    ; SWI
    FDB $0000    ; NMI
    FDB $4000    ; RESET vector
