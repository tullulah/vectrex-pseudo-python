; Test CPU ABX - Add B to X
; Special instruction that adds unsigned B to X

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test 1: Simple ABX
        LDX  #$C800      ; X = 0xC800
        LDB  #$10        ; B = 16
        ABX              ; X = X + B = 0xC810
        
        ; Test 2: ABX with larger offset
        LDX  #$C900      ; X = 0xC900
        LDB  #$FF        ; B = 255
        ABX              ; X = X + B = 0xC9FF
        
        ; Test 3: ABX with zero
        LDX  #$D000      ; X = 0xD000
        LDB  #$00        ; B = 0
        ABX              ; X = 0xD000 (unchanged)
        
        ; Test 4: ABX doesn't affect flags
        LDX  #$FFFF      ; X = 0xFFFF
        LDB  #$01        ; B = 1
        ABX              ; X = 0x0000 (wraps around, no carry flag)

loop:   BRA  loop
