; Complete Vector Drawing Sequence Test
; Tests full drawing pipeline: Reset0Ref → Move → Draw → Blank
;
; Test Scenario:
; 1. Reset beam position (Reset0Ref via shift register)
; 2. Move to start point (set velocity, wait for integration)
; 3. Enable brightness (Z-axis)
; 4. Draw line (change velocity while bright)
; 5. Blank (disable brightness)
;
; Expected Behavior:
; - Reset0Ref zeros integrator positions
; - Move phase: beam moves with brightness off
; - Draw phase: beam moves with brightness on → vector line created
; - Blank phase: brightness off, ready for next vector
;
; Memory Map:
; 0xD000: Port B (MUX control)
; 0xD001: Port A (value)
; 0xD00A: Shift Register (for Reset0Ref)

        ORG $C800

START:
        ; === STEP 1: Reset beam to center (Reset0Ref) ===
        LDA #$00
        STA $D00A       ; Shift Register = 0 (trigger Reset0Ref)
        NOP
        NOP
        
        ; === STEP 2: Move to start position (brightness OFF) ===
        LDA #$00
        STA $D000       ; MUX = 0 (brightness)
        LDA #$00        
        STA $D001       ; Brightness = 0 (OFF)
        
        ; Set X velocity
        LDA #$02
        STA $D000       ; MUX = 2 (X integrator)
        LDA #$40        ; X velocity = 64 (move right)
        STA $D001
        
        ; Wait for movement (ramp + integration)
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        
        ; === STEP 3: Enable brightness and draw ===
        LDA #$00
        STA $D000       ; MUX = 0 (brightness)
        LDA #$FF
        STA $D001       ; Brightness = MAX (ON)
        
        ; Change Y velocity to draw diagonal
        LDA #$03
        STA $D000       ; MUX = 3 (Y integrator)
        LDA #$40        ; Y velocity = 64 (move up)
        STA $D001
        
        ; Wait for line to draw
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        
        ; === STEP 4: Blank (turn off brightness) ===
        LDA #$00
        STA $D000       ; MUX = 0 (brightness)
        LDA #$00
        STA $D001       ; Brightness = 0 (OFF)
        
        ; Read final state
        LDA $D000       ; MUX
        LDB $D001       ; Brightness
        
DONE:
        BRA DONE
