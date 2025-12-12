; Malban draw_synced_list - Complete Vectrex ROM
; Compiled from C with gcc6809 -O3
; Manual adaptation for Vectrex cartridge format

    ORG $0000

; Vectrex Cartridge Header (MANDATORY)
    FCC "g GCE 1982"        ; Copyright string
    FCB $80                 ; Delimiter
    FDB MUSIC_DATA          ; Music data pointer (dummy)
    FCB $FD                 ; Height
    FCB $0D                 ; Width
    FCB $F8                 ; Rel Y
    FCB $50                 ; Rel X
    FCC " MALBAN C CODE"    ; Title (20 chars max)
    FCB $80                 ; Delimiter
    FCB $00                 ; End of title
    FDB START               ; Code entry point

; Dummy music data
MUSIC_DATA:
    FCB $00
    FCB $00

; BIOS addresses
Wait_Recal      EQU $F192
Intensity_a     EQU $F2AB

; VIA registers (negative addresses in gcc6809)
VIA_port_a      EQU $D000
VIA_port_b      EQU $D002
VIA_t1_cnt_lo   EQU $D004
VIA_t1_cnt_hi   EQU $D005
VIA_cntl        EQU $D00B
VIA_int_flags   EQU $D00D
VIA_shift_reg   EQU $D05A

; Entry point
START:
    ; Setup intensity
    LDA #$7F
    JSR Intensity_a
    
MAIN_LOOP:
    ; Wait for retrace
    JSR Wait_Recal
    
    ; Draw square using Malban's algorithm
    LDX #SQUARE_DATA    ; u = square_data
    LDD #0              ; y=0, x=0 (B=y, A=x in stack convention)
    PSHS D              ; Push y, x
    LDD #$7F7F          ; scaleMove=$7F, scaleList=$7F
    PSHS D              ; Push scales
    
    JSR DRAW_SYNCED_LIST
    
    LEAS 4,S            ; Clean stack
    
    BRA MAIN_LOOP

; draw_synced_list_c implementation (from malban_gcc_o3.s lines 8-157)
DRAW_SYNCED_LIST:
    PSHS U
    LEAS -9,S
    LEAU ,X             ; u = list pointer
    LDB 18,S            ; scaleMove
    STB 2,S
    LDB 14,S            ; y
    STB 4,S
    LDB 16,S            ; x
    STB 5,S
    LDB 20,S            ; scaleList
    STB 6,S

DSL_FRAME:
    ; Frame initialization (lines 13-43)
    CLR VIA_shift_reg   ; Blank beam
    LDB #$CC
    STB VIA_cntl        ; Zero integrators
    CLR VIA_port_a      ; Reset offset
    LDB #$82
    STB VIA_port_b      ; Configure port_b
    LDB 2,S             ; scaleMove
    STB VIA_t1_cnt_lo
    
    ; DELAY LOOP (critical!)
    LDD #5
    STD 7,S
    LDX 7,S
    BLE DSL_AFTER_DELAY
DSL_DELAY:
    LDX 7,S
    TFR X,D
    ADDD #$FFFF         ; -1
    STD 7,S
    LDX 7,S
    BGT DSL_DELAY
    
DSL_AFTER_DELAY:
    LDB #$83
    STB VIA_port_b      ; Enable
    LDB 4,S             ; y position
    STB VIA_port_a
    LDB #$CE
    STB VIA_cntl        ; Integrator mode
    CLR VIA_port_b      ; Mux enable
    LDB #1
    STB VIA_port_b      ; Mux disable
    LDB 5,S             ; x position
    STB VIA_port_a
    CLR VIA_t1_cnt_hi   ; Start timer
    LDB 6,S             ; scaleList
    STB VIA_t1_cnt_lo
    
    LEAU 3,U            ; u += 3
    LDB -2,U
    STB 1,S
    LBNE DSL_CHECK_MOVE
    TST -1,U
    LBNE DSL_CHECK_MOVE

DSL_WAIT_MOVE:
    LDB VIA_int_flags
    CLRA
    CLRA
    ANDB #$40
    CMPD #0
    BEQ DSL_WAIT_MOVE
    
DSL_DRAW_LOOP:
    LDB ,U              ; c = *u
    BGE DSL_POSITIVE
    
    ; Draw line with beam ON (c < 0)
    LDB 1,U
    STB VIA_port_a      ; dy
    CLR VIA_port_b      ; Mux enable
    LDB #1
    STB VIA_port_b      ; Mux disable
    LDB 2,U
    STB VIA_port_a      ; dx
    CLR VIA_t1_cnt_hi   ; Start timer
    LDB #$FF
    STB VIA_shift_reg   ; Beam ON
DSL_WAIT_DRAW:
    LDB VIA_int_flags
    CLRA
    CLRA
    ANDB #$40
    CMPD #0
    BEQ DSL_WAIT_DRAW
    CLR VIA_shift_reg   ; Beam OFF
    LEAU 3,U
    LBRA DSL_CONTINUE

DSL_POSITIVE:
    TSTB
    BNE DSL_CHECK_END
    LDB 1,U
    STB 3,S
    BEQ DSL_END_CHECK
    LDB 2,U
    STB ,S
DSL_MOVE:
    LDB 3,S
    STB VIA_port_a
    LDB #$CE
    STB VIA_cntl
    CLR VIA_port_b
    LDB #1
    STB VIA_port_b
    LDB ,S
    STB VIA_port_a
    CLR VIA_t1_cnt_hi
DSL_WAIT_MOVE2:
    LDB VIA_int_flags
    CLRA
    CLRA
    ANDB #$40
    CMPD #0
    BEQ DSL_WAIT_MOVE2
    LEAU 3,U
    BRA DSL_CONTINUE

DSL_END_CHECK:
    LDB 2,U
    STB ,S
    BNE DSL_MOVE
    LEAU 3,U
    BRA DSL_CONTINUE

DSL_CHECK_END:
    CMPB #2
    LBNE DSL_FRAME
    LEAS 9,S
    PULS U,PC
    
DSL_CHECK_MOVE:
    LDB VIA_int_flags
    CLRA
    CLRA
    ANDB #$40
    CMPD #0
    BEQ DSL_CHECK_MOVE
    LDB 1,S
    STB VIA_port_a
    LDB #$CE
    STB VIA_cntl
    CLR VIA_port_b
    LDB #1
    STB VIA_port_b
    LDB -1,U
    STB VIA_port_a
    CLR VIA_t1_cnt_hi
DSL_WAIT_INIT:
    LDB VIA_int_flags
    CLRA
    CLRA
    ANDB #$40
    CMPD #0
    BEQ DSL_WAIT_INIT
    LBRA DSL_DRAW_LOOP
    
DSL_CONTINUE:
    LDB ,U
    LBGE DSL_POSITIVE

; Data: Square coordinates
SQUARE_DATA:
    FCB $00
    FCB $00
    FCB $00
    FCB $80
    FCB $50
    FCB $00
    FCB $80
    FCB $00
    FCB $50
    FCB $80
    FCB $B0
    FCB $00
    FCB $80
    FCB $00
    FCB $B0
    FCB $02
    FCB $00
    FCB $00

    ; Pad to 8K
    ORG $1FFE
    FDB START               ; Reset vector
