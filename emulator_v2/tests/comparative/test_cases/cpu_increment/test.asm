; Test CPU Increment/Decrement - INC/DEC (A/B/memory)
; Verify register and memory increment/decrement

        ORG $C800

start:
        ; Test INCA
        LDA  #$10        ; A = 0x10
        INCA             ; A = 0x11
        INCA             ; A = 0x12
        
        ; Test INCB
        LDB  #$FE        ; B = 0xFE
        INCB             ; B = 0xFF
        INCB             ; B = 0x00 (overflow, Z=1)
        
        ; Test DECA
        LDA  #$05        ; A = 0x05
        DECA             ; A = 0x04
        DECA             ; A = 0x03
        
        ; Test DECB
        LDB  #$01        ; B = 0x01
        DECB             ; B = 0x00 (Z=1)

loop:   BRA  loop
