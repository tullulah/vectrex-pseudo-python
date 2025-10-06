; Test CPU LEA - Load Effective Address (LEAX, LEAY, LEAS, LEAU)
; Verify LEA operations calculate addresses without memory access

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test LEAX (Load Effective Address into X)
        LDX  #$C900      ; X = 0xC900
        LEAX 10,X        ; X = X + 10 = 0xC90A
        LEAX -5,X        ; X = X - 5 = 0xC905
        
        ; Test LEAY (Load Effective Address into Y)
        LDY  #$D000      ; Y = 0xD000
        LEAY 20,Y        ; Y = Y + 20 = 0xD014
        
        ; Test LEAS (Load Effective Address into S)
        LDS  #$CFFF      ; S = 0xCFFF
        LEAS -10,S       ; S = S - 10 = 0xCFF5
        
        ; Test LEAU (Load Effective Address into U)
        LDU  #$CA00      ; U = 0xCA00
        LEAU 15,U        ; U = U + 15 = 0xCA0F

loop:   BRA  loop
