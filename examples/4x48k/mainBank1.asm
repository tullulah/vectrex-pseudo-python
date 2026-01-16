CURRENT_BANK        EQU      1                            ; 
                    Bank     1 
                    include  "commonGround.i"
; following is needed for VIDE
; to replace "vars" in this bank with values from the other bank
; #genVarlist# varFromBank1
;
;***************************************************************************
; CODE SECTION
;***************************************************************************

main
                    JSR      Wait_Recal           ; Vectrex BIOS recalibration 
                    JSR      Intensity_5F         ; Sets the beam intensity
                                                  ; vector beam to $5f
                    jsr      playSound
                    LDA      #148                 ; scalefactor
                    STA      VIA_t1_cnt_lo
                    LDA      #45                  ; position relative Y 
                    LDB      #19                  ; position relative X
                    JSR      Moveto_d             ; sets up VIA control register after a wait recal

                    LDX      #EnemyOne            ; address of string 
                    JSR      Draw_VLc             ; Vectrex BIOS print routine 

                    jsr      Reset0Ref
                    jsr      printBankString
                    jsr      Read_Btns
                    tsta
                    beq      outMain
                    clr      BackGndCtr

                    ; this jumps to the main label in bank #2
REPLACE_1_2_main_varFromBank2_2
                    ldx      #0
                    jmp      jmpBank2
               

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
bank_string1  DB  "BANK 1!",$80
bank_string2  DB  "PRESS ANY BUTTON.",$80

EnemyOne:
 DB +8 ; number of lines to draw
 DB -21, -11 ; draw to y, x
 DB +21, -11 ; draw to y, x
 DB -25, +11 ; draw to y, x
 DB +16, +15 ; draw to y, x
 DB +9, -4 ; draw to y, x
 DB +2, -11 ; draw to y, x
 DB -2, -11 ; draw to y, x
 DB -9, -4 ; draw to y, x
 DB -16, +15 ; draw to y, x

                    INCLUDE "sounds.asm"
