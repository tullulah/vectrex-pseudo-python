; Test CPU Multiply - MUL (8x8=16 bit)
; Verify MUL operation

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test 1: Simple multiply
        LDA  #$05        ; A = 5
        LDB  #$06        ; B = 6
        MUL              ; D = A * B = 30 (0x001E)
        
        ; Test 2: Larger numbers
        LDA  #$10        ; A = 16
        LDB  #$10        ; B = 16
        MUL              ; D = 256 (0x0100)
        
        ; Test 3: Max values
        LDA  #$FF        ; A = 255
        LDB  #$FF        ; B = 255
        MUL              ; D = 65025 (0xFE01)

loop:   BRA  loop
