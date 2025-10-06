; Test CPU Subroutine - JSR/RTS
; Verify subroutine call and return

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack pointer
        LDA  #$00        ; A = 0
        JSR  increment   ; Call subroutine
        JSR  increment   ; Call again
        JSR  increment   ; Call again
        ; A should be 3 now
loop:   
        NOP              ; Marker - stay here
        BRA  loop        ; Loop forever

increment:
        INCA             ; A++
        RTS              ; Return

