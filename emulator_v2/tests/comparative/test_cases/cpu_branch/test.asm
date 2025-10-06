; Test CPU Branches - BEQ/BNE
; Expected: A=0x00, B=0xFF (branches taken/not taken correctly)

        ORG $C800

start:
        LDA  #$00        ; A = 0 (Z flag set)
        BEQ  taken1      ; Should branch (Z=1)
        LDA  #$FF        ; Should NOT execute
taken1:
        LDB  #$10        ; B = 0x10
        BNE  taken2      ; Should branch (Z=0)
        LDB  #$00        ; Should NOT execute
taken2:
        LDA  #$00        ; Final A = 0x00
        LDB  #$FF        ; Final B = 0xFF

loop:   BRA  loop        ; Infinite loop
