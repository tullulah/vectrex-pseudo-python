    ORG $0000
LDA #$01     ; CRITICAL EQU $0047
JSR $F1BA  ; Read_Btns EQU $0065
JSR Wait_Recal  ; CRITICAL EQU $0061
JSR $F1AA  ; DP_to_D0 EQU $0063
JSR $F1AF  ; DP_to_C8 EQU $0067
CLR $C823    ; CRITICAL EQU $0045
; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 21 bytes
; ================================================
; ================================================
