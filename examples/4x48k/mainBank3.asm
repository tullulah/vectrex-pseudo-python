CURRENT_BANK        EQU      3                            ; 
                    Bank     3 
                    include  "commonGround.i"
; following is needed for VIDE
; to replace "vars" in this bank with values from the other bank
; #genVarlist# varFromBank3
;
;***************************************************************************
; CODE SECTION
;***************************************************************************
; NOTE!
; the PrintStr_d in the other banks subroutines
; use BIOS routines, which (inherently) also switch banks!
; (since they use SHIFT and T1 timer of VIA, and thus also change the Interrupt flag)
;
; in this example this is "ok", since the interrupt flags upon
; entering and exiting the BIOS functions are equal
; and so they "return" to the correct banks!
;
                    JSR      Wait_Recal           ; Vectrex BIOS recalibration 
                    JSR      Intensity_5F         ; Sets the intensity of the 
                                                  ; vector beam to $5f
                    jsr      playSound
                    LDA      #148                 ; scalefactor
                    STA      VIA_t1_cnt_lo
                    LDA      #40                  ; position relative Y 
                    LDB      #-10                 ; position relative X
                    JSR      Moveto_d             ; sets up VIA control register after a wait recal

                    LDX      #EnemyThree           ; address of string 
                    JSR      Draw_VLc             ; Vectrex BIOS print routine 
                    
                    jsr      Reset0Ref
                    jsr      printBankString
                    jsr      Read_Btns
                    tsta
                    beq      outMain
                    clr      BackGndCtr

                    ; this jumps to the main label in bank #0
REPLACE_1_2_main_varFromBank3_0
                    ldx      #0 
                    jmp      jmpBank0_Shift       ; jmpBank0_T1
            
outMain
                    BRA      main                 ; and repeat forever 

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
bank_string1  DB  "BANK 3!",$80
bank_string2  DB  "PRESS ANY BUTTON.",$80

EnemyThree:
 DB +10 ; number of lines to draw
 DB +9, +8 ; draw to y, x
 DB -1, +7 ; draw to y, x
 DB -13, -8 ; draw to y, x
 DB -12, +11 ; draw to y, x
 DB +25, -3 ; draw to y, x
 DB +0, +7 ; draw to y, x
 DB -25, -4 ; draw to y, x
 DB +12, +12 ; draw to y, x
 DB +13, -8 ; draw to y, x
 DB +1, +7 ; draw to y, x
 DB -9, +8 ; draw to y, x

                  ;  INCLUDE  "ymPlayer.i"
                  ;  INCLUDE  "warp_start_2.asm"
                     INCLUDE "sounds.asm"