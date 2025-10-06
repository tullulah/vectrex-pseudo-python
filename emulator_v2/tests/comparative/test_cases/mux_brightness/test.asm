; MUX Brightness Control Test
; Tests MUX routing: Port B selects function, Port A provides value
;
; Test Scenario:
; 1. Set Port B to brightness mode (MUX = 0)
; 2. Write brightness value to Port A
; 3. Verify signal is routed to screen brightness
;
; Expected Behavior:
; - Port B controls MUX selector
; - MUX = 0 routes Port A to Z (brightness)
; - Screen brightness updated with Port A value
;
; Memory Map:
; 0xD000: Port B (MUX control)
; 0xD001: Port A (value source)
;
; MUX Values:
; 0 = Z-axis (brightness)
; 1 = Sound output
; 2 = X integrator
; 3 = Y integrator

        ORG $C800

START:
        ; Set MUX to brightness mode
        LDA #$00
        STA $D000       ; Port B = 0 (select brightness)
        
        ; Set brightness to max
        LDA #$FF
        STA $D001       ; Port A = 0xFF (max brightness)
        
        ; Wait a few cycles for propagation
        NOP
        NOP
        
        ; Read back values to verify
        LDA $D000       ; Read Port B (MUX selector)
        LDB $D001       ; Read Port A (brightness value)
        
DONE:
        BRA DONE
