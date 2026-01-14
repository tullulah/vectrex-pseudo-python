; AUTO-GENERATED FLATTENED MULTIBANK ASM
; Banks: 32 | Bank size: 16384 bytes | Total: 524288 bytes

; ===== BANK #00 (physical offset $00000) =====
ORG $0000  ; Switchable bank window
    FCC "g GCE 1982"
    FCB $80
    FDB music1
    FCB $F8
    FCB $50
    FCB $20
    FCB $BB
    FCC "CALL GRAPH TEST"
    FCB $80
    FCB 0
; CODE SECTION
;***************************************************************************


    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)



;**** CONST DECLARATIONS (NUMBER-ONLY) ****

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ PROGRAM CODE SECTION - User VPy Code                            │
; │ This section contains the compiled user program logic.          │
; └─────────────────────────────────────────────────────────────────┘
;

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS (CRITICAL - do once at startup)
    CLR $C80E        ; Initialize Vec_Prev_Btns to 0 for Read_Btns debounce
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF       ; Initialize stack at top of RAM (safer than Vec_Default_Stk)
    LDA #0
    STA >CURRENT_ROM_BANK ; Initialize to bank 0 (RAM tracker for debugging)
    ; Note: NOT writing to hardware bank register - already in Bank #0

    ; *** DEBUG *** main() function code inline (initialization)
    ; VPy_LINE:19
    ; VPy_LINE:10
    LDD #-50
    STD VAR_ENEMY1_X
    ; VPy_LINE:11
    LDD #60
    STD VAR_ENEMY1_Y
    ; VPy_LINE:12
    LDD #0
    STD VAR_ENEMY2_X
    ; VPy_LINE:13
    LDD #0
    STD VAR_ENEMY2_Y
    ; VPy_LINE:14
    LDD #50
    STD VAR_ENEMY3_X
    ; VPy_LINE:15
    LDD #-60
    STD VAR_ENEMY3_Y
    ; VPy_LINE:16
    LDD #0
    STD VAR_FRAME_COUNT
    ; VPy_LINE:20
    LDD #127
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 20
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT

MAIN:
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
    ; === Initialize Joystick (one-time setup) ===
    CLR $C823    ; CRITICAL: Clear analog mode flag (Joy_Analog does DEC on this)
    LDA #$01     ; CRITICAL: Resolution threshold (power of 2: $40=fast, $01=accurate)
    STA $C81A    ; Vec_Joy_Resltn (loop terminates when B=this value after LSRBs)
    LDA #$01
    STA $C81F    ; Vec_Joy_Mux_1_X (enable X axis reading)
    LDA #$03
    STA $C820    ; Vec_Joy_Mux_1_Y (enable Y axis reading)
    LDA #$00
    STA $C821    ; Vec_Joy_Mux_2_X (disable joystick 2 - CRITICAL!)
    STA $C822    ; Vec_Joy_Mux_2_Y (disable joystick 2 - saves cycles)
    ; Mux configured - J1_X()/J1_Y() can now be called

    ; JSR Wait_Recal is now called at start of LOOP_BODY (see auto-inject)
    LDA #$80
    STA VIA_t1_cnt_lo
    ; *** Call loop() as subroutine (executed every frame)
    JSR LOOP_BODY
    BRA MAIN


; ================================================
; ================================================
    ; VPy_LINE:24
LOOP_BODY:
    LEAS -2,S ; allocate locals
    JSR Wait_Recal  ; CRITICAL: Sync with CRT refresh (50Hz frame timing)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; VPy_LINE:25
    LDD #12
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:26
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:28
    LDD #100
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
; NATIVE_CALL: VECTREX_SET_INTENSITY at line 28
    JSR VECTREX_SET_INTENSITY
    CLRA
    CLRB
    STD RESULT
    ; VPy_LINE:30
    LDD 0 ,S
    STD RESULT
; NATIVE_CALL: DEBUG_PRINT(a) at line 30
    LDD RESULT
    STA $C002
    STB $C000
    LDA #$FE
    STA $C001
    LDX #DEBUG_LABEL_A
    STX $C004
    BRA DEBUG_SKIP_DATA_0
DEBUG_LABEL_A:
    FCC "a"
    FCB $00
DEBUG_SKIP_DATA_0:
    LDD #0
    STD RESULT
    ; VPy_LINE:32
    JSR UPDATE_PLAYER
    ; VPy_LINE:33
    JSR UPDATE_ENEMIES
    ; VPy_LINE:34
    JSR DRAW_ALL
    ; VPy_LINE:35
    LDD 0 ,S
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #15
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    STX 0 ,S
    ; VPy_LINE:36
    LDD VAR_FRAME_COUNT
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_FRAME_COUNT
    STU TMPPTR
    STX ,U
    LEAS 2,S ; free locals
    RTS

    ; VPy_LINE:40
UPDATE_PLAYER: ; function
; --- function update_player ---
    ; VPy_LINE:42
    JSR CHECK_INPUT
    ; VPy_LINE:43
    JSR MOVE_PLAYER
    RTS

    ; VPy_LINE:45
CHECK_INPUT: ; function
; --- function check_input ---
    ; VPy_LINE:47
    ; pass (no-op)
    RTS

    ; VPy_LINE:49
MOVE_PLAYER: ; function
; --- function move_player ---
    ; VPy_LINE:51
    ; pass (no-op)
    RTS

    ; VPy_LINE:53
UPDATE_ENEMIES: ; function
; --- function update_enemies ---
    ; VPy_LINE:55
    LDD VAR_ENEMY1_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY1_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:56
    LDD VAR_ENEMY1_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_3
    LDD #0
    STD RESULT
    BRA CE_4
CT_3:
    LDD #1
    STD RESULT
CE_4:
    LDD RESULT
    LBEQ IF_NEXT_2
    ; VPy_LINE:57
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY1_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_1
IF_NEXT_2:
IF_END_1:
    ; VPy_LINE:60
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    ADDD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY2_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:61
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BGT CT_7
    LDD #0
    STD RESULT
    BRA CE_8
CT_7:
    LDD #1
    STD RESULT
CE_8:
    LDD RESULT
    LBEQ IF_NEXT_6
    ; VPy_LINE:62
    LDD #-100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY2_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_5
IF_NEXT_6:
IF_END_5:
    ; VPy_LINE:65
    LDD VAR_ENEMY3_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_X
    STU TMPPTR
    STX ,U
    ; VPy_LINE:66
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    PULS D
    STD TMPLEFT
    LDD TMPLEFT
    SUBD TMPRIGHT
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_Y
    STU TMPPTR
    STX ,U
    ; VPy_LINE:67
    LDD VAR_ENEMY3_X
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_11
    LDD #0
    STD RESULT
    BRA CE_12
CT_11:
    LDD #1
    STD RESULT
CE_12:
    LDD RESULT
    LBEQ IF_NEXT_10
    ; VPy_LINE:68
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_X
    STU TMPPTR
    STX ,U
    LBRA IF_END_9
IF_NEXT_10:
IF_END_9:
    ; VPy_LINE:69
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDD RESULT
    STD TMPLEFT
    LDD #-100
    STD RESULT
    LDD RESULT
    STD TMPRIGHT
    LDD TMPLEFT
    SUBD TMPRIGHT
    BLT CT_15
    LDD #0
    STD RESULT
    BRA CE_16
CT_15:
    LDD #1
    STD RESULT
CE_16:
    LDD RESULT
    LBEQ IF_NEXT_14
    ; VPy_LINE:70
    LDD #100
    STD RESULT
    LDX RESULT
    LDU #VAR_ENEMY3_Y
    STU TMPPTR
    STX ,U
    LBRA IF_END_13
IF_NEXT_14:
IF_END_13:
    RTS

    ; VPy_LINE:74
DRAW_ALL: ; function
; --- function draw_all ---
    ; VPy_LINE:76
    JSR DRAW_PLAYER
    ; VPy_LINE:77
    JSR DRAW_ENEMIES
    RTS

    ; VPy_LINE:79
DRAW_PLAYER: ; function
; --- function draw_player ---
    ; VPy_LINE:81
; DRAW_VECTOR("player", x, y) - 1 path(s) at position
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #0
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_PLAYER_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS

    ; VPy_LINE:83
DRAW_ENEMIES: ; function
; --- function draw_enemies ---
    ; VPy_LINE:85
; DRAW_VECTOR("enemy", x, y) - 1 path(s) at position
    LDD VAR_ENEMY1_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY1_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_ENEMY_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; VPy_LINE:86
; DRAW_VECTOR("enemy", x, y) - 1 path(s) at position
    LDD VAR_ENEMY2_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY2_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_ENEMY_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; VPy_LINE:87
; DRAW_VECTOR("enemy", x, y) - 1 path(s) at position
    LDD VAR_ENEMY3_X
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD VAR_ENEMY3_Y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA TMPPTR+1  ; Save Y to temporary storage
    LDA TMPPTR    ; X position
    STA DRAW_VEC_X
    LDA TMPPTR+1  ; Y position
    STA DRAW_VEC_Y
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY  ; Use intensity from vector data
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_ENEMY_PATH0  ; Path 0
    JSR Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER  ; Cross-bank call to helper in bank #31
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    RTS


; ================================================


; ===== BANK #01 (physical offset $04000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #02 (physical offset $08000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #03 (physical offset $0C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #04 (physical offset $10000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #05 (physical offset $14000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #06 (physical offset $18000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #07 (physical offset $1C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #08 (physical offset $20000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #09 (physical offset $24000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #10 (physical offset $28000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #11 (physical offset $2C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #12 (physical offset $30000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #13 (physical offset $34000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #14 (physical offset $38000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #15 (physical offset $3C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #16 (physical offset $40000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #17 (physical offset $44000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #18 (physical offset $48000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #19 (physical offset $4C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #20 (physical offset $50000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #21 (physical offset $54000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #22 (physical offset $58000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #23 (physical offset $5C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #24 (physical offset $60000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #25 (physical offset $64000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #26 (physical offset $68000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #27 (physical offset $6C000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #28 (physical offset $70000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #29 (physical offset $74000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000


; ================================================


; ===== BANK #30 (physical offset $78000) =====
    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ================================================
    ORG $0000



; ===== BANK #31 (physical offset $7C000) =====
    ORG $4000  ; Fixed bank window (runtime helpers + interrupt vectors)
CUSTOM_RESET:
    LDA #0
    STA $DF00           ; Switch hardware bank to #0 (cart register)
    STA >CURRENT_ROM_BANK ; Keep RAM tracker in sync
    JMP START           ; Jump to program entry in Bank #0

;

    INCLUDE "VECTREX.I"

; === RAM VARIABLE DEFINITIONS (EQU) ===
; AUTO-GENERATED - All offsets calculated automatically
; Total RAM used: 73 bytes
RESULT               EQU $C880+$01   ; Main result temporary (2 bytes)
TMPLEFT              EQU $C880+$03   ; Left operand temp (2 bytes)
TMPLEFT2             EQU $C880+$05   ; Left operand temp 2 (for nested operations) (2 bytes)
TMPRIGHT             EQU $C880+$07   ; Right operand temp (2 bytes)
TMPRIGHT2            EQU $C880+$09   ; Right operand temp 2 (for nested operations) (2 bytes)
TMPPTR               EQU $C880+$0B   ; Pointer temp (used by DRAW_VECTOR, arrays, structs) (2 bytes)
TMPPTR2              EQU $C880+$0D   ; Pointer temp 2 (for nested array operations) (2 bytes)
TEMP_YX              EQU $C880+$0F   ; Temporary y,x storage (2 bytes)
TEMP_X               EQU $C880+$11   ; Temporary x storage (1 bytes)
TEMP_Y               EQU $C880+$12   ; Temporary y storage (1 bytes)
NUM_STR              EQU $C880+$13   ; String buffer for PRINT_NUMBER (2 bytes)
DRAW_VEC_X           EQU $C880+$15   ; X position offset for vector drawing (1 bytes)
DRAW_VEC_Y           EQU $C880+$16   ; Y position offset for vector drawing (1 bytes)
MIRROR_X             EQU $C880+$17   ; X-axis mirror flag (0=normal, 1=flip) (1 bytes)
MIRROR_Y             EQU $C880+$18   ; Y-axis mirror flag (0=normal, 1=flip) (1 bytes)
DRAW_VEC_INTENSITY   EQU $C880+$19   ; Intensity override (0=use vector's, >0=override) (1 bytes)
VLINE_DX_16          EQU $C880+$1A   ; x1-x0 (16-bit) for line drawing (2 bytes)
VLINE_DY_16          EQU $C880+$1C   ; y1-y0 (16-bit) for line drawing (2 bytes)
VLINE_DX             EQU $C880+$1E   ; Clamped dx (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$1F   ; Clamped dy (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$20   ; Remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$22   ; Remaining dx for segment 2 (16-bit) (2 bytes)
VLINE_STEPS          EQU $C880+$24   ; Line drawing step counter (1 bytes)
VLINE_LIST           EQU $C880+$25   ; 2-byte vector list (Y|endbit, X) (2 bytes)
DRAW_CIRCLE_XC       EQU $C880+$27   ; Circle center X (byte) (1 bytes)
DRAW_CIRCLE_YC       EQU $C880+$28   ; Circle center Y (byte) (1 bytes)
DRAW_CIRCLE_DIAM     EQU $C880+$29   ; Circle diameter (byte) (1 bytes)
DRAW_CIRCLE_INTENSITY EQU $C880+$2A   ; Circle intensity (byte) (1 bytes)
DRAW_CIRCLE_TEMP     EQU $C880+$2B   ; Circle drawing temporaries (radius=2, xc=2, yc=2, spare=2) (8 bytes)
VAR_ENEMY1_X         EQU $C880+$33   ; User variable (2 bytes)
VAR_ENEMY1_Y         EQU $C880+$35   ; User variable (2 bytes)
VAR_ENEMY2_X         EQU $C880+$37   ; User variable (2 bytes)
VAR_ENEMY2_Y         EQU $C880+$39   ; User variable (2 bytes)
VAR_ENEMY3_X         EQU $C880+$3B   ; User variable (2 bytes)
VAR_ENEMY3_Y         EQU $C880+$3D   ; User variable (2 bytes)
VAR_FRAME_COUNT      EQU $C880+$3F   ; User variable (2 bytes)
VAR_ARG0             EQU $C880+$41   ; Function argument 0 (2 bytes)
VAR_ARG1             EQU $C880+$43   ; Function argument 1 (2 bytes)
VAR_ARG2             EQU $C880+$45   ; Function argument 2 (2 bytes)
VAR_ARG3             EQU $C880+$47   ; Function argument 3 (2 bytes)
CURRENT_ROM_BANK     EQU $C880   ; Current ROM bank tracker (1 byte, FIXED at first RAM byte)


; ===== CROSS-BANK CALL WRAPPERS =====
; Auto-generated wrappers for bank switching


; Cross-bank wrapper for Draw_Sync_List_At_With_Mirrors (Bank #31)
Draw_Sync_List_At_With_Mirrors_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank from RAM
    PSHS A              ; Save current bank on stack
    LDA #31             ; Load target bank ID
    STA CURRENT_ROM_BANK ; Switch to target bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    JSR DRAW_SYNC_LIST_AT_WITH_MIRRORS              ; Call real function
    PULS A              ; Restore original bank from stack
    STA CURRENT_ROM_BANK ; Switch back to original bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for VECTREX_PRINT_TEXT (Bank #31)
VECTREX_PRINT_TEXT_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank from RAM
    PSHS A              ; Save current bank on stack
    LDA #31             ; Load target bank ID
    STA CURRENT_ROM_BANK ; Switch to target bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    JSR VECTREX_PRINT_TEXT              ; Call real function
    PULS A              ; Restore original bank from stack
    STA CURRENT_ROM_BANK ; Switch back to original bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for VECTREX_SET_INTENSITY (Bank #31)
VECTREX_SET_INTENSITY_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank from RAM
    PSHS A              ; Save current bank on stack
    LDA #31             ; Load target bank ID
    STA CURRENT_ROM_BANK ; Switch to target bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    JSR VECTREX_SET_INTENSITY              ; Call real function
    PULS A              ; Restore original bank from stack
    STA CURRENT_ROM_BANK ; Switch back to original bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for DRAW_LINE_WRAPPER (Bank #31)
DRAW_LINE_WRAPPER_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank from RAM
    PSHS A              ; Save current bank on stack
    LDA #31             ; Load target bank ID
    STA CURRENT_ROM_BANK ; Switch to target bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    JSR DRAW_LINE_WRAPPER              ; Call real function
    PULS A              ; Restore original bank from stack
    STA CURRENT_ROM_BANK ; Switch back to original bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    PULS A              ; Restore A register
    RTS

; Cross-bank wrapper for DRAW_CIRCLE_RUNTIME (Bank #31)
DRAW_CIRCLE_RUNTIME_BANK_WRAPPER:
    PSHS A              ; Save A register
    LDA CURRENT_ROM_BANK ; Read current bank from RAM
    PSHS A              ; Save current bank on stack
    LDA #31             ; Load target bank ID
    STA CURRENT_ROM_BANK ; Switch to target bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    JSR DRAW_CIRCLE_RUNTIME              ; Call real function
    PULS A              ; Restore original bank from stack
    STA CURRENT_ROM_BANK ; Switch back to original bank (RAM tracker)
    STA $DF00            ; Hardware bank switch register (cartucho intercepts)
    PULS A              ; Restore A register
    RTS
; ===== END CROSS-BANK WRAPPERS (helpers only) =====

;
; ┌─────────────────────────────────────────────────────────────────┐
; │ RUNTIME SECTION - VPy Builtin Helpers & System Functions       │
; │ This section contains reusable code shared across all VPy       │
; │ programs. These helpers are emitted once per compilation unit.  │
; └─────────────────────────────────────────────────────────────────┘
;

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; === BUTTON SYSTEM - BIOS TRANSITIONS ===
; J1_BUTTON_1-4() - Read transition bits from $C811
; Read_Btns (auto-injected) calculates: ~(new) OR Vec_Prev_Btns
; Result: bit=1 ONLY on rising edge (0→1 transition)
; Returns: D = 1 (just pressed), 0 (not pressed or still held)

J1B1_BUILTIN:
    LDA $C811      ; Read transition bits (Vec_Button_1_1)
    ANDA #$01      ; Test bit 0 (Button 1)
    LBEQ .J1B1_OFF ; Long branch (helpers may be >127 bytes away)
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    LBEQ .J1B2_OFF ; Long branch
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    LBEQ .J1B3_OFF ; Long branch
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    LBEQ .J1B4_OFF ; Long branch
    LDD #1
    RTS
.J1B4_OFF:
    LDD #0
    RTS

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG2   ; string pointer (ARG2 = third param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG0+1 ; X (ARG0 = first param)
    JSR Print_Str_d
    JSR $F1AF      ; DP_to_C8 (restore before return - CRITICAL for TMPPTR access)
    RTS
VECTREX_DEBUG_PRINT:
    ; Debug print to console - writes to gap area (C000-C7FF)
    ; Write both high and low bytes for proper 16-bit signed interpretation
    LDA VAR_ARG0     ; Load high byte (for signed interpretation)
    STA $C002        ; Debug output high byte in gap
    LDA VAR_ARG0+1   ; Load low byte
    STA $C000        ; Debug output low byte in unmapped gap
    LDA #$42         ; Debug marker
    STA $C001        ; Debug marker to indicate new output
    RTS
; DRAW_LINE unified wrapper - handles 16-bit signed coordinates
; Args: (x0,y0,x1,y1,intensity) as 16-bit words
; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).
DRAW_LINE_WRAPPER:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)
    STA >$D00C     ; VIA_cntl
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA RESULT+8+1  ; intensity (low byte of 16-bit value)
    JSR Intensity_a
    ; Move to start ONCE (y in A, x in B) - use low bytes (8-bit signed -127..+127)
    LDA RESULT+2+1  ; Y start (low byte of 16-bit value)
    LDB RESULT+0+1  ; X start (low byte of 16-bit value)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic
    ; dx = x1 - x0 (treating as signed 16-bit)
    LDD RESULT+4    ; x1 (RESULT+4, 16-bit)
    SUBD RESULT+0   ; subtract x0 (RESULT+0, 16-bit)
    STD VLINE_DX_16 ; Store full 16-bit dx
    ; dy = y1 - y0 (treating as signed 16-bit)
    LDD RESULT+6    ; y1 (RESULT+6, 16-bit)
    SUBD RESULT+2   ; subtract y0 (RESULT+2, 16-bit)
    STD VLINE_DY_16 ; Store full 16-bit dy
    ; SEGMENT 1: Clamp dy to ±127 and draw
    LDD VLINE_DY_16 ; Load full dy
    CMPD #127
    BLE DLW_SEG1_DY_LO
    LDA #127        ; dy > 127: use 127
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_LO:
    CMPD #-128
    BGE DLW_SEG1_DY_NO_CLAMP  ; -128 <= dy <= 127: use original (sign-extended)
    LDA #$80        ; dy < -128: use -128
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_NO_CLAMP:
    LDA VLINE_DY_16+1  ; Use original low byte (already in valid range)
DLW_SEG1_DY_READY:
    STA VLINE_DY    ; Save clamped dy for segment 1
    ; Clamp dx to ±127
    LDD VLINE_DX_16
    CMPD #127
    BLE DLW_SEG1_DX_LO
    LDB #127        ; dx > 127: use 127
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_LO:
    CMPD #-128
    BGE DLW_SEG1_DX_NO_CLAMP  ; -128 <= dx <= 127: use original (sign-extended)
    LDB #$80        ; dx < -128: use -128
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_NO_CLAMP:
    LDB VLINE_DX_16+1  ; Use original low byte (already in valid range)
DLW_SEG1_DX_READY:
    STB VLINE_DX    ; Save clamped dx for segment 1
    ; Draw segment 1
    CLR Vec_Misc_Count
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d ; Beam moves automatically
    ; Check if we need SEGMENT 2 (dy outside ±127 range)
    LDD VLINE_DY_16 ; Reload original dy
    CMPD #127
    BGT DLW_NEED_SEG2  ; dy > 127: needs segment 2
    CMPD #-128
    BLT DLW_NEED_SEG2  ; dy < -128: needs segment 2
    BRA DLW_DONE       ; dy in range ±127: no segment 2
DLW_NEED_SEG2:
    ; SEGMENT 2: Draw remaining dy and dx
    ; Calculate remaining dy
    LDD VLINE_DY_16 ; Load original full dy
    CMPD #127
    BGT DLW_SEG2_DY_POS  ; dy > 127
    ; dy < -128, so we drew -128 in segment 1
    ; remaining = dy - (-128) = dy + 128
    ADDD #128       ; Add back the -128 we already drew
    BRA DLW_SEG2_DY_DONE
DLW_SEG2_DY_POS:
    ; dy > 127, so we drew 127 in segment 1
    ; remaining = dy - 127
    SUBD #127       ; Subtract 127 we already drew
DLW_SEG2_DY_DONE:
    STD VLINE_DY_REMAINING  ; Store remaining dy (16-bit)
    ; Calculate remaining dx
    LDD VLINE_DX_16 ; Load original full dx
    CMPD #127
    BLE DLW_SEG2_DX_CHECK_NEG
    ; dx > 127, so we drew 127 in segment 1
    ; remaining = dx - 127
    SUBD #127
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_CHECK_NEG:
    CMPD #-128
    BGE DLW_SEG2_DX_NO_REMAIN  ; -128 <= dx <= 127: no remaining dx
    ; dx < -128, so we drew -128 in segment 1
    ; remaining = dx - (-128) = dx + 128
    ADDD #128
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_NO_REMAIN:
    LDD #0          ; No remaining dx
DLW_SEG2_DX_DONE:
    STD VLINE_DX_REMAINING  ; Store remaining dx (16-bit) in VLINE_DX_REMAINING
    ; Setup for Draw_Line_d: A=dy, B=dx (CRITICAL: order matters!)
    ; Load remaining dy from VLINE_DY_REMAINING (already saved)
    LDA VLINE_DY_REMAINING+1  ; Low byte of remaining dy
    LDB VLINE_DX_REMAINING+1  ; Low byte of remaining dx
    CLR Vec_Misc_Count
    JSR Draw_Line_d ; Beam continues from segment 1 endpoint
DLW_DONE:
    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code
    TFR A,DP
    RTS
VECTREX_SET_INTENSITY:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
    RTS
; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)
__Intensity_a:
TFR B,A         ; Move B to A (BIOS expects intensity in A)
JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller
__Reset0Ref:
JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller
__Moveto_d:
LDA 2,S         ; Get Y from stack (after return address)
JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller
__Draw_Line_d:
LDA 2,S         ; Get dy from stack (after return address)
JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller
; ============================================================================
; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c
; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
; ============================================================================
Draw_Sync_List:
; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
STD TEMP_YX             ; Guardar en variable temporal (evita stack)
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX             ; Recuperar y,x
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSL_W1:
LDA VIA_int_flags
ANDA #$40
LBEQ DSL_W1             ; Long branch (helpers may be far)
; Loop de dibujo
DSL_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSL_DONE           ; Exit if end (long branch)
CMPA #1                 ; Check next path marker
LBEQ DSL_NEXT_PATH      ; Process next path (long branch)
; Draw line
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
LDB ,X+                 ; dy
LDA ,X+                 ; dx
PSHS A                  ; Save dx
STB VIA_port_a          ; dy to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore dx
STA VIA_port_a          ; dx to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSL_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W2
CLR VIA_shift_reg
LBRA DSL_LOOP            ; Long branch back to loop start
; Next path: read new intensity and header, then continue drawing
DSL_NEXT_PATH:
; Save current X position before reading anything
TFR X,D                 ; D = X (current position)
PSHS D                  ; Save X address
LDA ,X+                 ; Read intensity (X now points to y_start)
PSHS A                  ; Save intensity
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start (X now points to next_y)
STD TEMP_YX             ; Save y,x
PULS A                  ; Get intensity back
PSHS A                  ; Save intensity again
LDA #$D0
TFR A,DP                ; Set DP=$D0 (BIOS requirement)
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)
; Restore X to point to next_y,next_x (after the 3 bytes we read)
PULS D                  ; Get original X
ADDD #3                 ; Skip intensity, y_start, x_start
TFR D,X                 ; X now points to next_y
; Reset to zero (same as Draw_Sync_List start)
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a          ; x to DAC
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move
DSL_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W3
CLR VIA_shift_reg       ; Clear before continuing
LBRA DSL_LOOP            ; Continue drawing - LONG BRANCH
DSL_DONE:
RTS
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
; NOTE: Caller must ensure DP=$D0 for VIA access
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec (already relative to center)
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_Y
NEGB                    ; ← Negate Y if flag set
DSWM_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec (already relative to center)
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_X
NEGA                    ; ← Negate X if flag set
DSWM_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX             ; Save adjusted position
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSWM_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W1
; Loop de dibujo (conditional mirrors)
DSWM_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSWM_DONE
CMPA #1                 ; Check next path marker
LBEQ DSWM_NEXT_PATH
; Draw line with conditional negations
LDB ,X+                 ; dy
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_DY
NEGB                    ; ← Negate dy if flag set
DSWM_NO_NEGATE_DY:
LDA ,X+                 ; dx
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_DX
NEGA                    ; ← Negate dx if flag set
DSWM_NO_NEGATE_DX:
PSHS A                  ; Save final dx
STB VIA_port_a          ; dy (possibly negated) to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore final dx
STA VIA_port_a          ; dx (possibly negated) to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSWM_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W2
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
; Next path: repeat mirror logic for new path header
DSWM_NEXT_PATH:
TFR X,D
PSHS D
; Check intensity override (same logic as start)
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_NEXT_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_NEXT_SET_INTENSITY
DSWM_NEXT_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_NEXT_SET_INTENSITY:
PSHS A
LDB ,X+                 ; y_start
TST MIRROR_Y
BEQ DSWM_NEXT_NO_NEGATE_Y
NEGB
DSWM_NEXT_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start
TST MIRROR_X
BEQ DSWM_NEXT_NO_NEGATE_X
NEGA
DSWM_NEXT_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX
PULS A                  ; Get intensity back
JSR $F2AB
PULS D
ADDD #3
TFR D,X
; Reset to zero
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X
; Wait for move
DSWM_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W3
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
DSWM_DONE:
RTS
; ============================================================================
; DRAW_CIRCLE_RUNTIME - Draw circle with runtime parameters
; ============================================================================
; Follows Draw_Sync_List_At pattern: read params BEFORE DP change
; Inputs: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY (bytes in RAM)
; Uses 8 segments (octagon) with lookup table for efficiency
DRAW_CIRCLE_RUNTIME:
; Read ALL parameters into registers/stack BEFORE changing DP (critical!)
; (These are byte variables, use LDB not LDD)
LDB DRAW_CIRCLE_INTENSITY
PSHS B                 ; Save intensity on stack

LDB DRAW_CIRCLE_DIAM
SEX                    ; Sign-extend to 16-bit (diameter is unsigned 0..255)
LSRA                   ; Divide by 2 to get radius
RORB
STD DRAW_CIRCLE_TEMP   ; DRAW_CIRCLE_TEMP = radius (16-bit)

LDB DRAW_CIRCLE_XC     ; xc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+2 ; Save xc

LDB DRAW_CIRCLE_YC     ; yc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+4 ; Save yc

; NOW safe to setup BIOS (all params are in DRAW_CIRCLE_TEMP+stack)
LDA #$D0
TFR A,DP
JSR Reset0Ref

; Set intensity (from stack)
PULS A                 ; Get intensity from stack
CMPA #$5F
BEQ DCR_intensity_5F
JSR Intensity_a
BRA DCR_after_intensity
DCR_intensity_5F:
JSR Intensity_5F
DCR_after_intensity:

; Move to start position: (xc + radius, yc)
; radius = DRAW_CIRCLE_TEMP, xc = DRAW_CIRCLE_TEMP+2, yc = DRAW_CIRCLE_TEMP+4
LDD DRAW_CIRCLE_TEMP   ; D = radius
ADDD DRAW_CIRCLE_TEMP+2 ; D = xc + radius
TFR B,B                ; Keep X in B (low byte)
PSHS B                 ; Save X on stack
LDD DRAW_CIRCLE_TEMP+4 ; Load yc
TFR B,A                ; Y to A
PULS B                 ; X to B
JSR Moveto_d

; Loop through 8 segments using lookup table
LDX #DCR_DELTA_TABLE   ; Point to delta table
LDB #8                 ; 8 segments
PSHS B                 ; Save counter on stack

DCR_LOOP:
CLR Vec_Misc_Count     ; Relative drawing

; Load delta multipliers from table
LDA ,X+                ; dx multiplier (-1, 0, 1, or 2 for half)
LDB ,X+                ; dy multiplier
PSHS A,B               ; Save multipliers

; Calculate dy = (dy_mult * radius) / 2 if needed
LDD DRAW_CIRCLE_TEMP   ; Load radius
PULS A,B               ; Get multipliers (A=dx_mult, B=dy_mult)
PSHS A                 ; Save dx_mult

; Process dy_mult
TSTB
BEQ DCR_dy_zero        ; dy = 0
CMPB #2
BEQ DCR_dy_half        ; dy = r/2
CMPB #$FE              ; -2 (half negative)
BEQ DCR_dy_neg_half
CMPB #1
BEQ DCR_dy_pos         ; dy = r
; dy = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dy_done
DCR_dy_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dy_done
DCR_dy_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dy_done:
TFR B,A                ; Move dy result to A (we only need 8-bit for Vectrex coordinates)
PSHS A                 ; Save dy on stack

; Process dx_mult (same logic)
LDB 1,S                ; Get dx_mult from stack
TSTB
BEQ DCR_dx_zero
CMPB #2
BEQ DCR_dx_half
CMPB #$FE
BEQ DCR_dx_neg_half
CMPB #1
BEQ DCR_dx_pos
; dx = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dx_done
DCR_dx_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dx_done
DCR_dx_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dx_done:
TFR B,B                ; dx in B
PULS A                 ; dy in A
LEAS 1,S               ; Drop dx_mult

; Draw line with calculated deltas (preserve X - it points to table)
PSHS X                 ; Save table pointer
JSR Draw_Line_d
PULS X                 ; Restore table pointer

; Loop control
DEC ,S                 ; Decrement counter
BNE DCR_LOOP

LEAS 1,S               ; Clean counter from stack

; DP is ALREADY $D0 from BIOS, no need to restore (Draw_Sync_List_At doesn't restore either)
RTS

RTS

; Delta multiplier table: 8 segments (dx_mult, dy_mult)
; 0=zero, 1=r, -1=$FF=-r, 2=r/2, -2=$FE=-r/2
DCR_DELTA_TABLE:
FCB 2,2      ; Seg 1: dx=r/2, dy=r/2 (right-up)
FCB 0,1      ; Seg 2: dx=0, dy=r (up)
FCB $FE,2    ; Seg 3: dx=-r/2, dy=r/2 (left-up)
FCB $FF,0    ; Seg 4: dx=-r, dy=0 (left)
FCB $FE,$FE  ; Seg 5: dx=-r/2, dy=-r/2 (left-down)
FCB 0,$FF    ; Seg 6: dx=0, dy=-r (down)
FCB 2,$FE    ; Seg 7: dx=r/2, dy=-r/2 (right-down)
FCB 1,0      ; Seg 8: dx=r, dy=0 (right)

;***************************************************************************
; DATA SECTION
;***************************************************************************

; ========================================
; ASSET DATA SECTION
; Embedded 2 of 2 assets (unused assets excluded)
; ========================================

; Vector asset: player
; Generated from player.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 3
; X bounds: min=-15, max=15, width=30
; Center: (0, 5)

_PLAYER_WIDTH EQU 30
_PLAYER_CENTER_X EQU 0
_PLAYER_CENTER_Y EQU 5

_PLAYER_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _PLAYER_PATH0        ; pointer to path 0

_PLAYER_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$E2,$F1          ; line 0: flag=-1, dy=-30, dx=-15
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$1E,$F1          ; closing line: flag=-1, dy=30, dx=-15
    FCB 2                ; End marker (path complete)

; Vector asset: enemy
; Generated from enemy.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4
; X bounds: min=-10, max=10, width=20
; Center: (0, 0)

_ENEMY_WIDTH EQU 20
_ENEMY_CENTER_X EQU 0
_ENEMY_CENTER_Y EQU 0

_ENEMY_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _ENEMY_PATH0        ; pointer to path 0

_ENEMY_PATH0:    ; Path 0
    FCB 100              ; path0: intensity
    FCB $0A,$F6,0,0        ; path0: header (y=10, x=-10, relative to center)
    FCB $FF,$00,$14          ; line 0: flag=-1, dy=0, dx=20
    FCB $FF,$EC,$00          ; line 1: flag=-1, dy=-20, dx=0
    FCB $FF,$00,$EC          ; line 2: flag=-1, dy=0, dx=-20
    FCB $FF,$14,$00          ; closing line: flag=-1, dy=20, dx=0
    FCB 2                ; End marker (path complete)

; === INLINE ARRAY LITERALS (from function bodies) ===

; === Multibank Mode: Interrupt Vectors in Bank #31 (Linker) ===
; All vectors handled by multi_bank_linker
; Bank #0-#30: Local 0xFFF0-0xFFFF addresses are unreachable
; Bank #31: Contains complete interrupt vector table (fixed at 0x4000-0x7FFF window)

; ┌─────────────────────────────────────────────────────────────────┐
; │ RUNTIME SECTION - VPy Builtin Helpers & System Functions       │
; │ This section contains reusable code shared across all VPy       │
; │ programs. These helpers are emitted once per compilation unit.  │
; └─────────────────────────────────────────────────────────────────┘
;

; === JOYSTICK BUILTIN SUBROUTINES ===
; J1_X() - Read Joystick 1 X axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81B after Joy_Analog call
J1X_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81B from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81B)
    LDB $C81B    ; Vec_Joy_1_X (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; J1_Y() - Read Joystick 1 Y axis (INCREMENTAL - with state preservation)
; Returns: D = raw value from $C81C after Joy_Analog call
J1Y_BUILTIN:
    PSHS X       ; Save X (Joy_Analog uses it)
    JSR $F1AA    ; DP_to_D0 (required for Joy_Analog BIOS call)
    JSR $F1F5    ; Joy_Analog (updates $C81C from hardware)
    JSR $F1AF    ; DP_to_C8 (required to read RAM $C81C)
    LDB $C81C    ; Vec_Joy_1_Y (BIOS writes ~$FE at center)
    SEX          ; Sign-extend B to D
    ADDD #2      ; Calibrate center offset
    PULS X       ; Restore X
    RTS

; === BUTTON SYSTEM - BIOS TRANSITIONS ===
; J1_BUTTON_1-4() - Read transition bits from $C811
; Read_Btns (auto-injected) calculates: ~(new) OR Vec_Prev_Btns
; Result: bit=1 ONLY on rising edge (0→1 transition)
; Returns: D = 1 (just pressed), 0 (not pressed or still held)

J1B1_BUILTIN:
    LDA $C811      ; Read transition bits (Vec_Button_1_1)
    ANDA #$01      ; Test bit 0 (Button 1)
    LBEQ .J1B1_OFF ; Long branch (helpers may be >127 bytes away)
    LDD #1         ; Return pressed (rising edge)
    RTS
.J1B1_OFF:
    LDD #0         ; Return not pressed
    RTS

J1B2_BUILTIN:
    LDA $C811
    ANDA #$02      ; Test bit 1 (Button 2)
    LBEQ .J1B2_OFF ; Long branch
    LDD #1
    RTS
.J1B2_OFF:
    LDD #0
    RTS

J1B3_BUILTIN:
    LDA $C811
    ANDA #$04      ; Test bit 2 (Button 3)
    LBEQ .J1B3_OFF ; Long branch
    LDD #1
    RTS
.J1B3_OFF:
    LDD #0
    RTS

J1B4_BUILTIN:
    LDA $C811
    ANDA #$08      ; Test bit 3 (Button 4)
    LBEQ .J1B4_OFF ; Long branch
    LDD #1
    RTS
.J1B4_OFF:
    LDD #0
    RTS

VECTREX_PRINT_TEXT:
    ; CRITICAL: Print_Str_d requires DP=$D0 and signature is (Y, X, string)
    ; VPy signature: PRINT_TEXT(x, y, string) -> args (ARG0=x, ARG1=y, ARG2=string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for text rendering)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDU VAR_ARG2   ; string pointer (ARG2 = third param)
    LDA VAR_ARG1+1 ; Y (ARG1 = second param)
    LDB VAR_ARG0+1 ; X (ARG0 = first param)
    JSR Print_Str_d
    JSR $F1AF      ; DP_to_C8 (restore before return - CRITICAL for TMPPTR access)
    RTS
VECTREX_DEBUG_PRINT:
    ; Debug print to console - writes to gap area (C000-C7FF)
    ; Write both high and low bytes for proper 16-bit signed interpretation
    LDA VAR_ARG0     ; Load high byte (for signed interpretation)
    STA $C002        ; Debug output high byte in gap
    LDA VAR_ARG0+1   ; Load low byte
    STA $C000        ; Debug output low byte in unmapped gap
    LDA #$42         ; Debug marker
    STA $C001        ; Debug marker to indicate new output
    RTS
; DRAW_LINE unified wrapper - handles 16-bit signed coordinates
; Args: (x0,y0,x1,y1,intensity) as 16-bit words
; ALWAYS sets intensity. Does NOT reset origin (allows connected lines).
DRAW_LINE_WRAPPER:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode for vector drawing)
    STA >$D00C     ; VIA_cntl
    ; Set DP to hardware registers
    LDA #$D0
    TFR A,DP
    ; ALWAYS set intensity (no optimization)
    LDA RESULT+8+1  ; intensity (low byte of 16-bit value)
    JSR Intensity_a
    ; Move to start ONCE (y in A, x in B) - use low bytes (8-bit signed -127..+127)
    LDA RESULT+2+1  ; Y start (low byte of 16-bit value)
    LDB RESULT+0+1  ; X start (low byte of 16-bit value)
    JSR Moveto_d
    ; Compute deltas using 16-bit arithmetic
    ; dx = x1 - x0 (treating as signed 16-bit)
    LDD RESULT+4    ; x1 (RESULT+4, 16-bit)
    SUBD RESULT+0   ; subtract x0 (RESULT+0, 16-bit)
    STD VLINE_DX_16 ; Store full 16-bit dx
    ; dy = y1 - y0 (treating as signed 16-bit)
    LDD RESULT+6    ; y1 (RESULT+6, 16-bit)
    SUBD RESULT+2   ; subtract y0 (RESULT+2, 16-bit)
    STD VLINE_DY_16 ; Store full 16-bit dy
    ; SEGMENT 1: Clamp dy to ±127 and draw
    LDD VLINE_DY_16 ; Load full dy
    CMPD #127
    BLE DLW_SEG1_DY_LO
    LDA #127        ; dy > 127: use 127
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_LO:
    CMPD #-128
    BGE DLW_SEG1_DY_NO_CLAMP  ; -128 <= dy <= 127: use original (sign-extended)
    LDA #$80        ; dy < -128: use -128
    BRA DLW_SEG1_DY_READY
DLW_SEG1_DY_NO_CLAMP:
    LDA VLINE_DY_16+1  ; Use original low byte (already in valid range)
DLW_SEG1_DY_READY:
    STA VLINE_DY    ; Save clamped dy for segment 1
    ; Clamp dx to ±127
    LDD VLINE_DX_16
    CMPD #127
    BLE DLW_SEG1_DX_LO
    LDB #127        ; dx > 127: use 127
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_LO:
    CMPD #-128
    BGE DLW_SEG1_DX_NO_CLAMP  ; -128 <= dx <= 127: use original (sign-extended)
    LDB #$80        ; dx < -128: use -128
    BRA DLW_SEG1_DX_READY
DLW_SEG1_DX_NO_CLAMP:
    LDB VLINE_DX_16+1  ; Use original low byte (already in valid range)
DLW_SEG1_DX_READY:
    STB VLINE_DX    ; Save clamped dx for segment 1
    ; Draw segment 1
    CLR Vec_Misc_Count
    LDA VLINE_DY
    LDB VLINE_DX
    JSR Draw_Line_d ; Beam moves automatically
    ; Check if we need SEGMENT 2 (dy outside ±127 range)
    LDD VLINE_DY_16 ; Reload original dy
    CMPD #127
    BGT DLW_NEED_SEG2  ; dy > 127: needs segment 2
    CMPD #-128
    BLT DLW_NEED_SEG2  ; dy < -128: needs segment 2
    BRA DLW_DONE       ; dy in range ±127: no segment 2
DLW_NEED_SEG2:
    ; SEGMENT 2: Draw remaining dy and dx
    ; Calculate remaining dy
    LDD VLINE_DY_16 ; Load original full dy
    CMPD #127
    BGT DLW_SEG2_DY_POS  ; dy > 127
    ; dy < -128, so we drew -128 in segment 1
    ; remaining = dy - (-128) = dy + 128
    ADDD #128       ; Add back the -128 we already drew
    BRA DLW_SEG2_DY_DONE
DLW_SEG2_DY_POS:
    ; dy > 127, so we drew 127 in segment 1
    ; remaining = dy - 127
    SUBD #127       ; Subtract 127 we already drew
DLW_SEG2_DY_DONE:
    STD VLINE_DY_REMAINING  ; Store remaining dy (16-bit)
    ; Calculate remaining dx
    LDD VLINE_DX_16 ; Load original full dx
    CMPD #127
    BLE DLW_SEG2_DX_CHECK_NEG
    ; dx > 127, so we drew 127 in segment 1
    ; remaining = dx - 127
    SUBD #127
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_CHECK_NEG:
    CMPD #-128
    BGE DLW_SEG2_DX_NO_REMAIN  ; -128 <= dx <= 127: no remaining dx
    ; dx < -128, so we drew -128 in segment 1
    ; remaining = dx - (-128) = dx + 128
    ADDD #128
    BRA DLW_SEG2_DX_DONE
DLW_SEG2_DX_NO_REMAIN:
    LDD #0          ; No remaining dx
DLW_SEG2_DX_DONE:
    STD VLINE_DX_REMAINING  ; Store remaining dx (16-bit) in VLINE_DX_REMAINING
    ; Setup for Draw_Line_d: A=dy, B=dx (CRITICAL: order matters!)
    ; Load remaining dy from VLINE_DY_REMAINING (already saved)
    LDA VLINE_DY_REMAINING+1  ; Low byte of remaining dy
    LDB VLINE_DX_REMAINING+1  ; Low byte of remaining dx
    CLR Vec_Misc_Count
    JSR Draw_Line_d ; Beam continues from segment 1 endpoint
DLW_DONE:
    LDA #$C8       ; CRITICAL: Restore DP to $C8 for our code
    TFR A,DP
    RTS
VECTREX_SET_INTENSITY:
    ; CRITICAL: Set VIA to DAC mode BEFORE calling BIOS (don't assume state)
    LDA #$98       ; VIA_cntl = $98 (DAC mode)
    STA >$D00C     ; VIA_cntl
    LDA #$D0
    TFR A,DP       ; Set Direct Page to $D0 for BIOS
    LDA VAR_ARG0+1
    JSR __Intensity_a
    RTS
; BIOS Wrappers - VIDE compatible (ensure DP=$D0 per call)
__Intensity_a:
TFR B,A         ; Move B to A (BIOS expects intensity in A)
JMP Intensity_a ; JMP (not JSR) - BIOS returns to original caller
__Reset0Ref:
JMP Reset0Ref   ; JMP (not JSR) - BIOS returns to original caller
__Moveto_d:
LDA 2,S         ; Get Y from stack (after return address)
JMP Moveto_d    ; JMP (not JSR) - BIOS returns to original caller
__Draw_Line_d:
LDA 2,S         ; Get dy from stack (after return address)
JMP Draw_Line_d ; JMP (not JSR) - BIOS returns to original caller
; ============================================================================
; Draw_Sync_List - EXACT port of Malban's draw_synced_list_c
; Data: FCB intensity, y_start, x_start, next_y, next_x, [flag, dy, dx]*, 2
; ============================================================================
Draw_Sync_List:
; ITERACIÓN 11: Loop completo dentro (bug assembler arreglado, datos embebidos OK)
LDA ,X+                 ; intensity
JSR $F2AB               ; BIOS Intensity_a (expects value in A)
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start
STD TEMP_YX             ; Guardar en variable temporal (evita stack)
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX             ; Recuperar y,x
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSL_W1:
LDA VIA_int_flags
ANDA #$40
LBEQ DSL_W1             ; Long branch (helpers may be far)
; Loop de dibujo
DSL_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSL_DONE           ; Exit if end (long branch)
CMPA #1                 ; Check next path marker
LBEQ DSL_NEXT_PATH      ; Process next path (long branch)
; Draw line
CLR Vec_Misc_Count      ; Clear for relative line drawing (CRITICAL for continuity)
LDB ,X+                 ; dy
LDA ,X+                 ; dx
PSHS A                  ; Save dx
STB VIA_port_a          ; dy to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore dx
STA VIA_port_a          ; dx to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSL_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W2
CLR VIA_shift_reg
LBRA DSL_LOOP            ; Long branch back to loop start
; Next path: read new intensity and header, then continue drawing
DSL_NEXT_PATH:
; Save current X position before reading anything
TFR X,D                 ; D = X (current position)
PSHS D                  ; Save X address
LDA ,X+                 ; Read intensity (X now points to y_start)
PSHS A                  ; Save intensity
LDB ,X+                 ; y_start
LDA ,X+                 ; x_start (X now points to next_y)
STD TEMP_YX             ; Save y,x
PULS A                  ; Get intensity back
PSHS A                  ; Save intensity again
LDA #$D0
TFR A,DP                ; Set DP=$D0 (BIOS requirement)
PULS A                  ; Restore intensity
JSR $F2AB               ; BIOS Intensity_a (may corrupt X!)
; Restore X to point to next_y,next_x (after the 3 bytes we read)
PULS D                  ; Get original X
ADDD #3                 ; Skip intensity, y_start, x_start
TFR D,X                 ; X now points to next_y
; Reset to zero (same as Draw_Sync_List start)
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a          ; x to DAC
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move
DSL_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSL_W3
CLR VIA_shift_reg       ; Clear before continuing
LBRA DSL_LOOP            ; Continue drawing - LONG BRANCH
DSL_DONE:
RTS
Draw_Sync_List_At_With_Mirrors:
; Unified mirror support using flags: MIRROR_X and MIRROR_Y
; Conditionally negates X and/or Y coordinates and deltas
; NOTE: Caller must ensure DP=$D0 for VIA access
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_SET_INTENSITY
DSWM_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_SET_INTENSITY:
JSR $F2AB               ; BIOS Intensity_a
LDB ,X+                 ; y_start from .vec (already relative to center)
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_Y
NEGB                    ; ← Negate Y if flag set
DSWM_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start from .vec (already relative to center)
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_X
NEGA                    ; ← Negate X if flag set
DSWM_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX             ; Save adjusted position
; Reset completo
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move sequence
LDD TEMP_YX
STB VIA_port_a          ; y to DAC
PSHS A                  ; Save x
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore x
STA VIA_port_a          ; x to DAC
; Timing setup
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X                ; Skip next_y, next_x
; Wait for move to complete
DSWM_W1:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W1
; Loop de dibujo (conditional mirrors)
DSWM_LOOP:
LDA ,X+                 ; Read flag
CMPA #2                 ; Check end marker
LBEQ DSWM_DONE
CMPA #1                 ; Check next path marker
LBEQ DSWM_NEXT_PATH
; Draw line with conditional negations
LDB ,X+                 ; dy
; Check if Y mirroring is enabled
TST MIRROR_Y
BEQ DSWM_NO_NEGATE_DY
NEGB                    ; ← Negate dy if flag set
DSWM_NO_NEGATE_DY:
LDA ,X+                 ; dx
; Check if X mirroring is enabled
TST MIRROR_X
BEQ DSWM_NO_NEGATE_DX
NEGA                    ; ← Negate dx if flag set
DSWM_NO_NEGATE_DX:
PSHS A                  ; Save final dx
STB VIA_port_a          ; dy (possibly negated) to DAC
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A                  ; Restore final dx
STA VIA_port_a          ; dx (possibly negated) to DAC
CLR VIA_t1_cnt_hi
LDA #$FF
STA VIA_shift_reg
; Wait for line draw
DSWM_W2:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W2
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
; Next path: repeat mirror logic for new path header
DSWM_NEXT_PATH:
TFR X,D
PSHS D
; Check intensity override (same logic as start)
LDA DRAW_VEC_INTENSITY  ; Check if intensity override is set
BNE DSWM_NEXT_USE_OVERRIDE   ; If non-zero, use override
LDA ,X+                 ; Otherwise, read intensity from vector data
BRA DSWM_NEXT_SET_INTENSITY
DSWM_NEXT_USE_OVERRIDE:
LEAX 1,X                ; Skip intensity byte in vector data
DSWM_NEXT_SET_INTENSITY:
PSHS A
LDB ,X+                 ; y_start
TST MIRROR_Y
BEQ DSWM_NEXT_NO_NEGATE_Y
NEGB
DSWM_NEXT_NO_NEGATE_Y:
ADDB DRAW_VEC_Y         ; Add Y offset
LDA ,X+                 ; x_start
TST MIRROR_X
BEQ DSWM_NEXT_NO_NEGATE_X
NEGA
DSWM_NEXT_NO_NEGATE_X:
ADDA DRAW_VEC_X         ; Add X offset
STD TEMP_YX
PULS A                  ; Get intensity back
JSR $F2AB
PULS D
ADDD #3
TFR D,X
; Reset to zero
CLR VIA_shift_reg
LDA #$CC
STA VIA_cntl
CLR VIA_port_a
LDA #$82
STA VIA_port_b
NOP
NOP
NOP
NOP
NOP
LDA #$83
STA VIA_port_b
; Move to new start position
LDD TEMP_YX
STB VIA_port_a
PSHS A
LDA #$CE
STA VIA_cntl
CLR VIA_port_b
LDA #1
STA VIA_port_b
PULS A
STA VIA_port_a
LDA #$7F
STA VIA_t1_cnt_lo
CLR VIA_t1_cnt_hi
LEAX 2,X
; Wait for move
DSWM_W3:
LDA VIA_int_flags
ANDA #$40
BEQ DSWM_W3
CLR VIA_shift_reg
LBRA DSWM_LOOP          ; Long branch
DSWM_DONE:
RTS
; ============================================================================
; DRAW_CIRCLE_RUNTIME - Draw circle with runtime parameters
; ============================================================================
; Follows Draw_Sync_List_At pattern: read params BEFORE DP change
; Inputs: DRAW_CIRCLE_XC, DRAW_CIRCLE_YC, DRAW_CIRCLE_DIAM, DRAW_CIRCLE_INTENSITY (bytes in RAM)
; Uses 8 segments (octagon) with lookup table for efficiency
DRAW_CIRCLE_RUNTIME:
; Read ALL parameters into registers/stack BEFORE changing DP (critical!)
; (These are byte variables, use LDB not LDD)
LDB DRAW_CIRCLE_INTENSITY
PSHS B                 ; Save intensity on stack

LDB DRAW_CIRCLE_DIAM
SEX                    ; Sign-extend to 16-bit (diameter is unsigned 0..255)
LSRA                   ; Divide by 2 to get radius
RORB
STD DRAW_CIRCLE_TEMP   ; DRAW_CIRCLE_TEMP = radius (16-bit)

LDB DRAW_CIRCLE_XC     ; xc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+2 ; Save xc

LDB DRAW_CIRCLE_YC     ; yc (signed -128..127)
SEX
STD DRAW_CIRCLE_TEMP+4 ; Save yc

; NOW safe to setup BIOS (all params are in DRAW_CIRCLE_TEMP+stack)
LDA #$D0
TFR A,DP
JSR Reset0Ref

; Set intensity (from stack)
PULS A                 ; Get intensity from stack
CMPA #$5F
BEQ DCR_intensity_5F
JSR Intensity_a
BRA DCR_after_intensity
DCR_intensity_5F:
JSR Intensity_5F
DCR_after_intensity:

; Move to start position: (xc + radius, yc)
; radius = DRAW_CIRCLE_TEMP, xc = DRAW_CIRCLE_TEMP+2, yc = DRAW_CIRCLE_TEMP+4
LDD DRAW_CIRCLE_TEMP   ; D = radius
ADDD DRAW_CIRCLE_TEMP+2 ; D = xc + radius
TFR B,B                ; Keep X in B (low byte)
PSHS B                 ; Save X on stack
LDD DRAW_CIRCLE_TEMP+4 ; Load yc
TFR B,A                ; Y to A
PULS B                 ; X to B
JSR Moveto_d

; Loop through 8 segments using lookup table
LDX #DCR_DELTA_TABLE   ; Point to delta table
LDB #8                 ; 8 segments
PSHS B                 ; Save counter on stack

DCR_LOOP:
CLR Vec_Misc_Count     ; Relative drawing

; Load delta multipliers from table
LDA ,X+                ; dx multiplier (-1, 0, 1, or 2 for half)
LDB ,X+                ; dy multiplier
PSHS A,B               ; Save multipliers

; Calculate dy = (dy_mult * radius) / 2 if needed
LDD DRAW_CIRCLE_TEMP   ; Load radius
PULS A,B               ; Get multipliers (A=dx_mult, B=dy_mult)
PSHS A                 ; Save dx_mult

; Process dy_mult
TSTB
BEQ DCR_dy_zero        ; dy = 0
CMPB #2
BEQ DCR_dy_half        ; dy = r/2
CMPB #$FE              ; -2 (half negative)
BEQ DCR_dy_neg_half
CMPB #1
BEQ DCR_dy_pos         ; dy = r
; dy = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dy_done
DCR_dy_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dy_done
DCR_dy_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dy_done
DCR_dy_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dy_done:
TFR B,A                ; Move dy result to A (we only need 8-bit for Vectrex coordinates)
PSHS A                 ; Save dy on stack

; Process dx_mult (same logic)
LDB 1,S                ; Get dx_mult from stack
TSTB
BEQ DCR_dx_zero
CMPB #2
BEQ DCR_dx_half
CMPB #$FE
BEQ DCR_dx_neg_half
CMPB #1
BEQ DCR_dx_pos
; dx = -r
LDD DRAW_CIRCLE_TEMP
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_zero:
LDD #0                 ; Clear both A and B
BRA DCR_dx_done
DCR_dx_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
BRA DCR_dx_done
DCR_dx_neg_half:
LDD DRAW_CIRCLE_TEMP
LSRA
RORB
NEGA
NEGB
SBCA #0
BRA DCR_dx_done
DCR_dx_pos:
LDD DRAW_CIRCLE_TEMP
DCR_dx_done:
TFR B,B                ; dx in B
PULS A                 ; dy in A
LEAS 1,S               ; Drop dx_mult

; Draw line with calculated deltas (preserve X - it points to table)
PSHS X                 ; Save table pointer
JSR Draw_Line_d
PULS X                 ; Restore table pointer

; Loop control
DEC ,S                 ; Decrement counter
BNE DCR_LOOP

LEAS 1,S               ; Clean counter from stack

; DP is ALREADY $D0 from BIOS, no need to restore (Draw_Sync_List_At doesn't restore either)
RTS

RTS

; Delta multiplier table: 8 segments (dx_mult, dy_mult)
; 0=zero, 1=r, -1=$FF=-r, 2=r/2, -2=$FE=-r/2
DCR_DELTA_TABLE:
FCB 2,2      ; Seg 1: dx=r/2, dy=r/2 (right-up)
FCB 0,1      ; Seg 2: dx=0, dy=r (up)
FCB $FE,2    ; Seg 3: dx=-r/2, dy=r/2 (left-up)
FCB $FF,0    ; Seg 4: dx=-r, dy=0 (left)
FCB $FE,$FE  ; Seg 5: dx=-r/2, dy=-r/2 (left-down)
FCB 0,$FF    ; Seg 6: dx=0, dy=-r (down)
FCB 2,$FE    ; Seg 7: dx=r/2, dy=-r/2 (right-down)
FCB 1,0      ; Seg 8: dx=r, dy=0 (right)




