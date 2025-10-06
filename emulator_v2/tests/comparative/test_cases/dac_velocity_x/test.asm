; Port A DAC Test - Delayed Propagation to Integrator X
; Tests VELOCITY_X_DELAY (6 cycles) from Port A write to integrator_x update
;
; Test Scenario:
; 1. Write value to Port A (DAC input)
; 2. Wait exactly 6 cycles (VELOCITY_X_DELAY)
; 3. Verify integrator_x has been updated
;
; Expected Behavior:
; - Port A write triggers delayed update to integrator_x
; - Delay is exactly 6 cycles per Vectrexy spec
; - Value propagates: Port A → DAC → Integrator X
;
; Memory Map:
; 0xD001: Port A (ORA - Output Register A)
; 0xD000: Port B (ORB - for MUX control)

        ORG $C800

START:
        ; Set MUX to select Integrator X (MUX = 0 for X velocity)
        LDA #$00
        STA $D000       ; Port B = 0 (MUX select X)
        
        ; Write value to Port A (will go to DAC → Integrator X after delay)
        LDA #$7F        ; Mid-range positive value (127)
        STA $D001       ; Port A = 0x7F
        
        ; Wait EXACTLY 6 cycles (VELOCITY_X_DELAY)
        NOP             ; 2 cycles
        NOP             ; 2 cycles
        NOP             ; 2 cycles
        
        ; At this point, integrator_x should be updated
        ; Load Port A to verify value is still there
        LDB $D001       ; Read Port A
        
DONE:
        BRA DONE
