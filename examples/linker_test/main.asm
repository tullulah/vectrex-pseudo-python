; main.asm - Main module that calls library function
; This will be compiled to main.vo and linked with lib.vo

; Section markers for linker
        ORG $C880

; .SECTION .text.main
; .EXTERN helper_draw_square

; Main program
main_program:
        LDA #127           ; Intensity
        JSR $F2AB          ; Intensity_a (BIOS)
        
        ; Draw a cross first
        LDA #0             ; y1
        LDB #-30           ; x1
        JSR $F312          ; Moveto_d
        
        LDA #0             ; dy
        LDB #60            ; dx (horizontal line)
        JSR $F3DF          ; Draw_Line_d
        
        LDA #0             ; y1
        LDB #0             ; x1  
        JSR $F312          ; Moveto_d
        
        LDA #-30           ; dy (vertical line)
        LDB #0             ; dx
        JSR $F3DF          ; Draw_Line_d
        
        LDA #60            ; dy
        LDB #0             ; dx
        JSR $F3DF          ; Draw_Line_d
        
        ; Call library function (will be external reference)
        JSR helper_draw_square  ; This creates a relocation
        
        RTS

; .END_SECTION

        END
