; Test CPU SEX - Sign Extend B into A
; Extends sign bit of B into A (makes D a signed 16-bit value)

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test 1: Positive value (bit 7 = 0)
        LDB  #$7F        ; B = 127 (0111 1111)
        SEX              ; A = 0x00, D = 0x007F (positive)
        
        ; Test 2: Negative value (bit 7 = 1)
        LDB  #$80        ; B = 128 (1000 0000, -128 signed)
        SEX              ; A = 0xFF, D = 0xFF80 (negative extended)
        
        ; Test 3: Another negative
        LDB  #$FF        ; B = 255 (1111 1111, -1 signed)
        SEX              ; A = 0xFF, D = 0xFFFF (-1 in 16-bit)
        
        ; Test 4: Small positive
        LDB  #$01        ; B = 1
        SEX              ; A = 0x00, D = 0x0001
        
        ; Test 5: Zero
        LDB  #$00        ; B = 0
        SEX              ; A = 0x00, D = 0x0000

loop:   BRA  loop
