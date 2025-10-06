; Test CPU Transfer/Exchange - TFR/EXG
; Verify register transfer and exchange operations

        ORG $C800

start:
        ; Setup initial values
        LDA  #$AA        ; A = 0xAA
        LDB  #$BB        ; B = 0xBB
        LDX  #$1234      ; X = 0x1234
        LDY  #$5678      ; Y = 0x5678
        
        ; Test TFR (Transfer)
        TFR  A,B         ; B = A = 0xAA (A unchanged)
        TFR  X,Y         ; Y = X = 0x1234 (X unchanged)
        
        ; Test EXG (Exchange)
        LDA  #$11        ; A = 0x11
        LDB  #$22        ; B = 0x22
        EXG  A,B         ; A = 0x22, B = 0x11
        
        LDX  #$AAAA      ; X = 0xAAAA
        LDY  #$BBBB      ; Y = 0xBBBB
        EXG  X,Y         ; X = 0xBBBB, Y = 0xAAAA

loop:   BRA  loop
