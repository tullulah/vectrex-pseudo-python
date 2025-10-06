; Timer1 Basic Operation Test
; Tests Timer1 countdown and IFR flag setting
;
; Test Scenario:
; 1. Set Timer1 to value 50
; 2. Wait for it to countdown to 0
; 3. Verify IFR bit 6 is set when timer expires
; 4. Verify timer stops at 0
;
; Expected Behavior:
; - Timer1 counts down each cycle
; - When reaches 0, IFR bit 6 (0x40) is set
; - Timer1 stays at 0 (doesn't wrap)
;
; Memory Map:
; 0xD004: Timer1 Counter Low
; 0xD005: Timer1 Counter High
; 0xD00D: IFR (Interrupt Flag Register)

        ORG $C800

START:
        ; Set Timer1 to 50
        LDA #$00
        STA $D005       ; Timer1 High = 0
        LDA #$32        ; 50 decimal
        STA $D004       ; Timer1 Low = 50 (triggers load)
        
        ; Wait ~60 cycles (timer should expire)
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        
        ; Load IFR to verify bit 6 is set
        LDA $D00D       ; Read IFR
        
        ; Load Timer1 to verify it's 0
        LDB $D004       ; Read Timer1 Low
        
DONE:
        BRA DONE
