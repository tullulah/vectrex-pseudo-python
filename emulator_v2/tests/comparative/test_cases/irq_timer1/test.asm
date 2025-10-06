; IRQ Timer1 Test Case
; Tests interrupt handling when Timer1 expires
;
; Test Scenario:
; 1. Enable Timer1 interrupt (IER bit 6)
; 2. Set Timer1 to small value (will expire quickly)
; 3. Execute NOPs while waiting for IRQ
; 4. IRQ vector should be called, PC jumps to BIOS handler
;
; Expected Behavior:
; - After ~100 cycles, Timer1 expires
; - IFR bit 6 set (Timer1 interrupt flag)
; - CPU takes IRQ (if I flag not set)
; - PC should jump to address at vector $FFF8
;
; Memory Map:
; 0xC800-0xCFFF: RAM (our code lives here)
; 0xD000-0xD00F: VIA registers
; 0xE000-0xFFFF: BIOS ROM

        ORG $C800

START:
        ; Disable interrupts initially (set I flag)
        ORCC #$10       ; Set I bit in CC register
        
        ; Setup Timer1 for quick expiration
        LDA #$C0        ; Bit 7=1 (SET), Bit 6=1 (Timer1 interrupt enable)
        STA $D00E       ; Write to IER (0xD00E)
        
        ; Set Timer1 to low value (will expire in ~100 cycles)
        LDA #$00
        STA $D005       ; Timer1 Counter High = 0
        LDA #$64        ; 100 decimal = 0x64
        STA $D004       ; Timer1 Counter Low = 100 (triggers load)
        
        ; Clear I flag to enable interrupts
        ANDCC #$EF      ; Clear I bit in CC register
        
        ; Wait loop - execute NOPs until interrupt
WAIT:
        NOP
        NOP
        NOP
        NOP
        BRA WAIT        ; Infinite loop (IRQ should break us out)
        
        ; Should never reach here
FAIL:
        LDA #$FF
        STA $D001       ; Write FF to Port A (debug signal)
        BRA FAIL

        END START
