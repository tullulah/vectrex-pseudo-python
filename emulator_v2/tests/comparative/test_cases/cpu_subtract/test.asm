; Test CPU Subtraction - SUBA/SUBB/SUBD
; Verify SUB operations with different flags

        ORG $C800

start:
        ; Test SUBA
        LDA  #$50        ; A = 0x50
        SUBA #$20        ; A = 0x30 (C=0, V=0, Z=0, N=0)
        
        ; Test SUBB
        LDB  #$80        ; B = 0x80
        SUBB #$30        ; B = 0x50 (C=0, V=0, Z=0, N=0)
        
        ; Test SUBD (16-bit)
        LDD  #$1000      ; D = 0x1000
        SUBD #$0500      ; D = 0x0B00 (C=0, V=0, Z=0, N=0)

loop:   BRA  loop
