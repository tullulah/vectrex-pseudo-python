CURRENT_BANK        EQU      2                            ; 
                    Bank     2 
                    include  "commonGround.i"
; following is needed for VIDE
; to replace "vars" in this bank with values from the other bank
; #genVarlist# varFromBank2
;
;***************************************************************************
; CODE SECTION
;***************************************************************************

                    bra     main

main
                    JSR      Wait_Recal           ; Vectrex BIOS recalibration 
                    JSR      Intensity_5F         ; Sets the intensitdddddy of the 
                                                  ; vector beam to $5f
                    jsr      playSound
                    LDA      #148                 ; scalefactor
                    STA      VIA_t1_cnt_lo
                    LDA      #36                  ; position relative Y 
                    LDB      #15                 ; position relative X
                    JSR      Moveto_d             ; sets up VIA control register after a wait recal

                    LDX      #EnemyTwo           ; address of string 
                    JSR      Draw_VLc             ; Vectrex BIOS print routine 
                    
                    jsr      Reset0Ref
                    jsr      printBankString
                    jsr      Read_Btns
                    tsta
                    beq      outMain
                    clr      BackGndCtr
                    ; this jumps to the main label in bank #3
REPLACE_1_2_main_varFromBank3_0
                    ldx      #0
                    jmp      jmpBank3
               

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
bank_string1  DB  "BANK 2!",$80
bank_string2  DB  "PRESS ANY BUTTON.",$80

EnemyTwo:
 DB +7 ; number of lines to draw
 DB +19, +8 ; draw to y, x
 DB -17, -15 ; draw to y, x
 DB +4, -20 ; draw to y, x
 DB -10, +18 ; draw to y, x
 DB -18, -7 ; draw to y, x
 DB +16, +13 ; draw to y, x
 DB -4, +21 ; draw to y, x
 DB +10, -18 ; draw to y, x

                    INCLUDE "sounds.asm"
