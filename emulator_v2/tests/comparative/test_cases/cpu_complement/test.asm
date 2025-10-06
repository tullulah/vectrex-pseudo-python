; Test CPU Complement/Negate/Clear - COM/NEG/CLR
; Verify bitwise complement, two's complement negate, and clear operations

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test COMA (Complement A - one's complement)
        LDA  #$AA        ; A = 10101010
        COMA             ; A = 01010101 = 0x55
        
        ; Test COMB (Complement B)
        LDB  #$0F        ; B = 00001111
        COMB             ; B = 11110000 = 0xF0
        
        ; Test NEGA (Negate A - two's complement)
        LDA  #$05        ; A = 5
        NEGA             ; A = -5 = 0xFB (251 unsigned)
        
        ; Test NEGB (Negate B)
        LDB  #$10        ; B = 16
        NEGB             ; B = -16 = 0xF0 (240 unsigned)
        
        ; Test CLRA (Clear A)
        LDA  #$FF        ; A = 255
        CLRA             ; A = 0
        
        ; Test CLRB (Clear B)  
        LDB  #$88        ; B = 136
        CLRB             ; B = 0

loop:   BRA  loop
