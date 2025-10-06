; Test CPU Shift/Rotate - ASL/ASR/LSL/LSR/ROL/ROR
; Verify shift and rotate operations

        ORG $C800

start:
        ; Test ASLA (Arithmetic Shift Left)
        LDA  #$40        ; A = 0x40 (01000000)
        ASLA             ; A = 0x80 (10000000, C=0, N=1)
        
        ; Test ASRA (Arithmetic Shift Right - sign extend)
        LDA  #$80        ; A = 0x80 (10000000)
        ASRA             ; A = 0xC0 (11000000, sign bit preserved)
        
        ; Test LSLA (Logical Shift Left)
        LDA  #$01        ; A = 0x01
        LSLA             ; A = 0x02
        LSLA             ; A = 0x04
        
        ; Test LSRA (Logical Shift Right)
        LDA  #$08        ; A = 0x08
        LSRA             ; A = 0x04
        LSRA             ; A = 0x02
        
        ; Test ROLA (Rotate Left through Carry)
        LDA  #$80        ; A = 0x80
        ROLA             ; A = 0x00, C=1 (bit 7 -> C)
        
        ; Test RORB (Rotate Right through Carry)
        LDB  #$01        ; B = 0x01
        RORB             ; B = 0x80 (C was 1, so C->bit7, bit0->C)

loop:   BRA  loop
