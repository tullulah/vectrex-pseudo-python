; Test CPU Stack - PSHS/PULS/PSHU/PULU
; Verify stack push/pull operations

        ORG $C800

start:
        ; Initialize stack pointer to valid RAM area
        LDS  #$CFFF      ; S = 0xCFFF (top of RAM)
        
        ; Initialize registers
        LDA  #$AA        ; A = 0xAA
        LDB  #$BB        ; B = 0xBB
        LDX  #$1234      ; X = 0x1234
        LDY  #$5678      ; Y = 0x5678
        
        ; Push to S stack
        PSHS A,B,X,Y     ; Push all registers
        
        ; Modify registers
        LDA  #$00
        LDB  #$00
        LDX  #$0000
        LDY  #$0000
        
        ; Pull from S stack (should restore)
        PULS A,B,X,Y     ; A=0xAA, B=0xBB, X=0x1234, Y=0x5678

loop:   BRA  loop
