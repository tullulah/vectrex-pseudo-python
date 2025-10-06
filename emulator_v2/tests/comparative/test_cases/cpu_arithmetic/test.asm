; Test CPU Arithmetic - ADDA/ADDB
; Simple test - no BIOS calls, just verify ALU operations

        ORG $C800

start:
        LDA  #$10        ; A = 0x10
        ADDA #$20        ; A = 0x30 (C=0, V=0, Z=0, N=0)
        LDB  #$30        ; B = 0x30
        ADDB #$25        ; B = 0x55 (C=0, V=0, Z=0, N=0)

loop:   BRA  loop        ; Infinite loop
