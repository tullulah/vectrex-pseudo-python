; Malban VIA Test - Código ASM directo
; Basado en draw_sync_list líneas 74-81

; VIA 6522 addresses
VIA_port_b      EQU $D002
VIA_port_a      EQU $D000
VIA_t1_cnt_lo   EQU $D004
VIA_t1_cnt_hi   EQU $D005
VIA_cntl        EQU $D00B
VIA_int_flags   EQU $D00D
VIA_shift_reg   EQU $D05A

; BIOS routines
Wait_Recal      EQU $F192

    ORG $0000

; Header Vectrex estándar
    FCB $67              ; "g"
    FCB $20              ; " "
    FDB music1           ; music address
    FCB $F8              ; height
    FCB $50              ; width
    FCB $20              ; rel y
    FCB $00              ; rel x
    FCC "MALBAN TEST"    ; title
    FCB $80              ; end of title
    FCB $00

START:
    JSR Wait_Recal       ; BIOS sync
    LDA #$80
    STA VIA_t1_cnt_lo    ; scale

MAIN:
    JSR Wait_Recal
    LDA #$80
    STA VIA_t1_cnt_lo
    JSR LOOP_BODY
    BRA MAIN

LOOP_BODY:
    ; Reset integrator (BIOS)
    JSR $F354            ; Reset0Ref
    
    ; Setup scale
    LDA #$7F
    STA VIA_t1_cnt_lo
    
    ; Primera línea: bottom (dy=0, dx=80)
    ; Malban líneas 74-81
    LDA #0               ; dy
    STA VIA_port_a       ; VIA_port_a = dy
    CLR VIA_port_b       ; VIA_port_b = 0 (mux enable)
    LDA #1
    STA VIA_port_b       ; VIA_port_b = 1 (mux disable)
    LDA #80              ; dx
    STA VIA_port_a       ; VIA_port_a = dx
    CLR VIA_t1_cnt_hi    ; start timer
    LDA #$FF
    STA VIA_shift_reg    ; beam ON
WAIT1:
    LDA VIA_int_flags
    ANDA #$40
    BEQ WAIT1            ; wait for timer
    CLR VIA_shift_reg    ; beam OFF
    
    ; Segunda línea: right (dy=80, dx=0)
    LDA #80              ; dy
    STA VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    CLR VIA_port_a       ; dx=0
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg
WAIT2:
    LDA VIA_int_flags
    ANDA #$40
    BEQ WAIT2
    CLR VIA_shift_reg
    
    ; Tercera línea: top (dy=0, dx=-80)
    CLR VIA_port_a       ; dy=0
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    LDA #-80             ; dx=-80
    STA VIA_port_a
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg
WAIT3:
    LDA VIA_int_flags
    ANDA #$40
    BEQ WAIT3
    CLR VIA_shift_reg
    
    ; Cuarta línea: left (dy=-80, dx=0)
    LDA #-80             ; dy=-80
    STA VIA_port_a
    CLR VIA_port_b
    LDA #1
    STA VIA_port_b
    CLR VIA_port_a       ; dx=0
    CLR VIA_t1_cnt_hi
    LDA #$FF
    STA VIA_shift_reg
WAIT4:
    LDA VIA_int_flags
    ANDA #$40
    BEQ WAIT4
    CLR VIA_shift_reg
    
    RTS

music1:
    FCB 1
    FCB $80

; Padding hasta el reset vector
    ORG $FFFE
    FDB START            ; Reset vector apunta al inicio

    END START
