; ========================================
; BANK 0 - Boot Stub
; ========================================
; Minimal boot stub that switches to fixed bank and jumps to START

    ORG $0000
    
    ; Vectrex header (for BIOS detection)
    FCC "g GCE 1982"
    FCB $80
    FDB $0000           ; no music
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "ASM BANK TEST   "
    FCB $80
    FCB 0

    ; Boot stub: switch to bank 31 and jump
    LDA #31             ; Fixed bank ID
    STA $D000           ; Bank switch register
    JMP $4000+$0003     ; Jump to START in bank 31 (0x4003)
