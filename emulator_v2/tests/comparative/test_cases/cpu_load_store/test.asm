; Test CPU Load/Store - Memory access
; Expected: A=0xAA (loaded back from RAM)

        ORG $C800

start:
        LDA  #$AA        ; A = 0xAA
        STA  $C850       ; Store to RAM address 0xC850
        LDA  #$00        ; Clear A
        LDA  $C850       ; Load back from RAM
        STA  $C851       ; Store to another RAM address
        LDA  #$00        ; Clear A again
        LDA  $C851       ; Load back should be 0xAA

loop:   BRA  loop        ; Infinite loop
