; Shift Register Reset0Ref Test
; Tests shift register zero-crossing detection for beam positioning reset
;
; Test Scenario:
; 1. Write specific pattern to shift register
; 2. Verify ZERO crossing detection (CB2 flag)
; 3. Test Reset0Ref signal generation
;
; Expected Behavior:
; - Shift register detects zero crossings
; - CB2 flag set on zero cross
; - Reset0Ref pulse generated for integrator reset
;
; Memory Map:
; 0xD00A: Shift Register
; 0xD00D: IFR (CB2 flag is bit 3)

        ORG $C800

START:
        ; Write to shift register to trigger zero detection
        LDA #$CB        ; Pattern: 11001011
        STA $D00A       ; Shift Register = 0xCB
        
        ; Wait for shift register to process
        NOP
        NOP
        NOP
        NOP
        
        ; Write zero pattern to trigger zero crossing
        LDA #$00
        STA $D00A       ; Shift Register = 0x00 (zero crossing)
        
        ; Wait for CB2 flag
        NOP
        NOP
        
        ; Read IFR to check CB2 flag (bit 3 = 0x08)
        LDA $D00D       ; Read IFR
        
        ; Read shift register value
        LDB $D00A       ; Read Shift Register
        
DONE:
        BRA DONE
