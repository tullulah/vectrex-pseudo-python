; Integrator Ramp Up Test
; Tests integrator ramp-up delay (5 cycles) before integration starts
;
; Test Scenario:
; 1. Write velocity value to integrator
; 2. Wait for RAMP_UP_DELAY (5 cycles)
; 3. Verify integration begins after delay
;
; Expected Behavior:
; - First 5 cycles: ramp-up delay, no integration
; - After cycle 5: integration begins
; - Position accumulates based on velocity
;
; Memory Map:
; 0xD000: Port B (MUX selector)
; 0xD001: Port A (velocity value)

        ORG $C800

START:
        ; Set MUX to X integrator
        LDA #$02
        STA $D000       ; Port B = 2 (select X integrator)
        
        ; Set X velocity to +10
        LDA #$0A        ; Velocity = 10
        STA $D001       ; Port A = 10
        
        ; Wait for ramp-up delay (5 cycles) + some integration
        NOP             ; 2
        NOP             ; 2
        NOP             ; 2
        NOP             ; 2
        NOP             ; 2
        NOP             ; 2 (total 12 cycles: 5 ramp + 7 integration)
        
        ; Read back MUX and velocity
        LDA $D000       ; MUX selector
        LDB $D001       ; Velocity value
        
DONE:
        BRA DONE
