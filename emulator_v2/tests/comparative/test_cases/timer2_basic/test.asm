; Timer2 Basic Operation Test
; Tests Timer2 countdown (no interrupt, just countdown)
;
; Test Scenario:
; 1. Set Timer2 to value 30
; 2. Wait for it to countdown
; 3. Verify Timer2 reaches 0
; 4. Verify IFR bit 5 is set when timer expires
;
; Expected Behavior:
; - Timer2 counts down each cycle
; - When reaches 0, IFR bit 5 (0x20) is set
; - Timer2 stays at 0
;
; Memory Map:
; 0xD008: Timer2 Counter Low (read/write)
; 0xD00D: IFR (Interrupt Flag Register)

        ORG $C800

START:
        ; Set Timer2 to 30
        LDA #$1E        ; 30 decimal
        STA $D008       ; Timer2 Low = 30
        
        ; Wait ~40 cycles (timer should expire)
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        NOP
        
        ; Load IFR to verify bit 5 is set
        LDA $D00D       ; Read IFR
        
        ; Load Timer2 to verify it's 0
        LDB $D008       ; Read Timer2 Low
        
DONE:
        BRA DONE
