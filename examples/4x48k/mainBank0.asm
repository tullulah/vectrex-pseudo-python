CURRENT_BANK        EQU      0                            ; 
                    Bank     0 
                    include  "commonGround.i"
; following is needed for VIDE
; to replace "vars" in this bank with values from the other bank
; #genVarlist# varFromBank0
;
;***************************************************************************
; CODE SECTION
;***************************************************************************

                    ;bra     main
main
                    JSR      Wait_Recal           ; Vectrex BIOS recalibration 
                    JSR      Intensity_5F         ; Sets the intensitdddddy of the 
                                                  ; vector beam to $5f
                    jsr      playSound
                    LDA      #148                 ; scalefactor
                    STA      VIA_t1_cnt_lo
                    LDA      #46                  ; position relative Y 
                    LDB      #23                 ; position relative X
                    JSR      Moveto_d             ; sets up VIA control register after a wait recal

                    LDX      #EnemyZero           ; address of string 
                    JSR      Draw_VLc             ; Vectrex BIOS print routine 
                    
                    jsr      Reset0Ref
                    jsr      printBankString
                    jsr      Read_Btns
                    tsta
                    beq      outMain
                    clr      BackGndCtr
        
                    ; this jumps to the main label in bank #1
REPLACE_1_2_main_varFromBank0_1
                    ldx     #1
                    jmp      jmpBank1_Shift
                    ;jmp      jmpBank1_T1
               

outMain             bra      main

;***************************************************************************
printBankString:
                    ldd      #(MAX_TEXT_HEIGHT*256)+$48
                    std      Vec_Text_HW 
                    LDU      #bank_string1
                    LDA      #-30
                    LDB      #-30
                    JSR      Print_Str_d
                    LDU      #bank_string2
                    LDA      #-50
                    LDB      #-81
                    JSR      Print_Str_d
                    rts




;***************************************************************************
bank_string1  DB  "BANK 0!",$80
bank_string2  DB  "PRESS ANY BUTTON.",$80

EnemyZero:
 DB +11 ; number of lines to draw
 DB -11, +0 ; draw to y, x
 DB -12, -10 ; draw to y, x
 DB +12, +6 ; draw to y, x
 DB -2, -11 ; draw to y, x
 DB +2, -11 ; draw to y, x
 DB -12, +5 ; draw to y, x
 DB +12, -9 ; draw to y, x
 DB +11, +0 ; draw to y, x
 DB -11, +4 ; draw to y, x
 DB +6, +11 ; draw to y, x
 DB -6, +11 ; draw to y, x
 DB +11, +4 ; draw to y, x

                     INCLUDE "sounds.asm"