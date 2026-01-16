    ORG $0000
CLR $C823    ; CRITICAL EQU $005E
LDA #$01     ; CRITICAL EQU $0060
JSR $F1AA  ; DP_to_D0 EQU $007E
JSR $F1AF  ; DP_to_C8 EQU $0082
JSR Wait_Recal  ; CRITICAL EQU $007C
JSR $F1BA  ; Read_Btns EQU $0080
; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 48 bytes
; ================================================
; ================================================
