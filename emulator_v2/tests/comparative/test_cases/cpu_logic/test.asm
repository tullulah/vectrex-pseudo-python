; Test CPU Logic Operations - ANDA/ANDB/ORA/ORB/EORA/EORB
; Verify bitwise operations

        ORG $C800

start:
        ; Test ANDA (AND accumulator A)
        LDA  #$FF        ; A = 0xFF
        ANDA #$0F        ; A = 0x0F (mask low nibble)
        
        ; Test ORA (OR accumulator A)
        LDA  #$F0        ; A = 0xF0
        ORA  #$0F        ; A = 0xFF (all bits set)
        
        ; Test EORA (XOR accumulator A)
        LDA  #$AA        ; A = 0xAA (10101010)
        EORA #$55        ; A = 0xFF (11111111, XOR with 01010101)
        
        ; Test ANDB (AND accumulator B)
        LDB  #$F0        ; B = 0xF0
        ANDB #$CC        ; B = 0xC0 (11000000)

loop:   BRA  loop
