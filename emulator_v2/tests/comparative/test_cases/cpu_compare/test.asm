; Test CPU Compare - CMPA/CMPB/CMPD/CMPX/CMPY
; Compare operations set flags without modifying registers

        ORG $C800

start:
        ; Test CMPA - equal
        LDA  #$50        ; A = 0x50
        CMPA #$50        ; Compare (Z=1, N=0, V=0, C=0)
        
        ; Test CMPB - less than
        LDB  #$30        ; B = 0x30
        CMPB #$40        ; Compare (Z=0, N=1, C=1)
        
        ; Test CMPD - greater than
        LDD  #$2000      ; D = 0x2000
        CMPD #$1000      ; Compare (Z=0, N=0, C=0)
        
        ; Test CMPX
        LDX  #$C900      ; X = 0xC900
        CMPX #$C900      ; Compare (Z=1, N=0, C=0)
        
        ; Registers should remain unchanged
        ; A=0x50, B=0x30, D=0x2000 (since LDD loads both A and B)
        ; Actually D=0x2000 means A=0x20, B=0x00

loop:   BRA  loop
