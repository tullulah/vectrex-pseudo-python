; lib.asm - Library module with helper function
; This will be compiled to lib.vo

; Section markers for linker
        ORG $C880

; .SECTION .text.lib
; .GLOBAL helper_draw_square

; Helper function that draws a square
helper_draw_square:
        LDA #127           ; Intensity
        JSR $F2AB          ; Intensity_a (BIOS)
        
        ; Draw square: 4 lines
        LDA #-10           ; y1
        LDB #-10           ; x1
        JSR $F312          ; Moveto_d
        
        LDA #0             ; dy = 10 - (-10) = 20
        LDB #20            ; dx = 10 - (-10) = 20
        JSR $F3DF          ; Draw_Line_d
        
        LDA #20            ; dy = 20
        LDB #0             ; dx = 0
        JSR $F3DF          ; Draw_Line_d
        
        LDA #0             ; dy = 0
        LDB #-20           ; dx = -20
        JSR $F3DF          ; Draw_Line_d
        
        LDA #-20           ; dy = -20
        LDB #0             ; dx = 0
        JSR $F3DF          ; Draw_Line_d
        
        RTS

; .END_SECTION

        END
