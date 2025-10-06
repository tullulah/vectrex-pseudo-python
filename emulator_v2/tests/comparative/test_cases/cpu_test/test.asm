; Test CPU Test - TST/TSTA/TSTB
; Verify test operations (compare with zero without modifying register)

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test TSTA - Test A (compare with 0)
        LDA  #$00        ; A = 0
        TSTA             ; Z=1, N=0 (zero)
        
        LDA  #$80        ; A = 128 (negative in signed)
        TSTA             ; Z=0, N=1 (negative)
        
        LDA  #$7F        ; A = 127 (positive)
        TSTA             ; Z=0, N=0 (positive)
        
        ; Test TSTB - Test B
        LDB  #$00        ; B = 0
        TSTB             ; Z=1, N=0
        
        LDB  #$FF        ; B = 255 (negative in signed)
        TSTB             ; Z=0, N=1
        
        ; Verify registers unchanged
        ; A should still be 0x7F
        ; B should still be 0xFF

loop:   BRA  loop
