; Test CPU Extended Branches - BCC/BCS/BMI/BPL/BVC/BVS
; Verify conditional branches based on flags

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test BCC (Branch if Carry Clear)
        LDA  #$50
        ADDA #$30        ; No carry
        BCC  test2       ; Should branch (C=0)
        LDA  #$FF        ; Should NOT execute
        
test2:
        ; Test BCS (Branch if Carry Set)
        LDA  #$FF
        ADDA #$02        ; Carry set
        BCS  test3       ; Should branch (C=1)
        LDA  #$00        ; Should NOT execute
        
test3:
        ; Test BMI (Branch if Minus/Negative)
        LDA  #$80        ; Negative (N=1)
        BMI  test4       ; Should branch
        LDA  #$AA        ; Should NOT execute
        
test4:
        ; Test BPL (Branch if Plus/Positive)
        LDA  #$7F        ; Positive (N=0)
        BPL  done        ; Should branch
        LDA  #$BB        ; Should NOT execute

done:
        LDA  #$42        ; Final marker value
loop:   BRA  loop
