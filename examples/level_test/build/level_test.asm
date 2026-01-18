; VPy M6809 Assembly (Vectrex)
; ROM: 32768 bytes


    ORG $0000

;***************************************************************************
; DEFINE SECTION
;***************************************************************************
    INCLUDE "VECTREX.I"

;***************************************************************************
; CARTRIDGE HEADER
;***************************************************************************
    FCC "g GCE 2025"
    FCB $80                 ; String terminator
    FDB $0000              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "Level Debug"
    FCB $80                 ; String terminator
    FCB 0                   ; End of header

;***************************************************************************
; CODE SECTION
;***************************************************************************

START:
    LDA #$D0
    TFR A,DP        ; Set Direct Page for BIOS
    CLR $C80E        ; Initialize Vec_Prev_Btns
    LDA #$80
    STA VIA_t1_cnt_lo
    LDS #$CBFF       ; Initialize stack
    JMP MAIN

;***************************************************************************
; === RAM VARIABLE DEFINITIONS ===
;***************************************************************************
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Temporary pointer (2 bytes)
TMPPTR2              EQU $C880+$04   ; Temporary pointer 2 (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary Y/X coordinate storage (2 bytes)
VLINE_DX_16          EQU $C880+$08   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$0A   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$0C   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$0D   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$0E   ; DRAW_LINE remaining dy for segment 2 (1 bytes)
VAR_RAM_TEST_COUNT   EQU $C880+$0F   ; User variable: RAM_TEST_COUNT (2 bytes)
VAR_RAM_TEST_X       EQU $C880+$11   ; User variable: RAM_TEST_X (2 bytes)
VAR_RAM_TEST_Y       EQU $C880+$13   ; User variable: RAM_TEST_Y (2 bytes)
VAR_X                EQU $C880+$15   ; User variable: X (2 bytes)
VAR_Y                EQU $C880+$17   ; User variable: Y (2 bytes)
VAR_COUNT            EQU $C880+$19   ; User variable: COUNT (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)

; Internal builtin variables (aliases to RESULT slots)
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+2
MIRROR_X EQU RESULT+4
MIRROR_Y EQU RESULT+6
DRAW_VEC_INTENSITY EQU RESULT+8


;***************************************************************************
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    ; Call main() for initialization
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; ===== LOAD_LEVEL builtin =====
    ; Load level: 'test_level'
    LDX #LEVEL_TEST_LEVEL
    STX LEVEL_PTR          ; Store level data pointer
    LDA ,X+                ; Load width (byte)
    STA LEVEL_WIDTH
    LDA ,X+                ; Load height (byte)
    STA LEVEL_HEIGHT
    LDD #1                 ; Return success
    STD RESULT

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; ===== SHOW_LEVEL builtin =====
    JSR SHOW_LEVEL_RUNTIME
    LDD #0
    STD RESULT
    ; ===== UPDATE_LEVEL builtin =====
    ; Placeholder - extend for animated/destructible tiles
    LDD #0
    STD RESULT
    RTS

; Function: RAM_TEST_MAIN
RAM_TEST_MAIN:
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    RTS

; Function: RAM_TEST_LOOP
RAM_TEST_LOOP:
    ; WAIT_RECAL: Wait for screen refresh
    JSR Wait_Recal
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_68994724591312392      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #60
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_68729639722158      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_RAM_TEST_X
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_RAM_TEST_Y
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_RAM_TEST_COUNT
    LDD VAR_X
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_RAM_TEST_X
    LDD VAR_Y
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_RAM_TEST_Y
    LDD VAR_COUNT
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_RAM_TEST_COUNT
    LDD VAR_COUNT
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #100
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_COUNT
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    RTS

; Function: TEST_SIMPLE_MAIN
TEST_SIMPLE_MAIN:
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    RTS

; Function: TEST_SIMPLE_LOOP
TEST_SIMPLE_LOOP:
    ; WAIT_RECAL: Wait for screen refresh
    JSR Wait_Recal
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-60
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_2315958665076      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    RTS

;***************************************************************************
; RUNTIME HELPERS
;***************************************************************************

VECTREX_PRINT_TEXT:
    ; VPy signature: PRINT_TEXT(x, y, string)
    ; BIOS signature: Print_Str_d(A=Y, B=X, U=string)
    JSR $F1AA      ; DP_to_D0 - set Direct Page for BIOS/VIA access
    LDU VAR_ARG2   ; string pointer (third parameter)
    LDA VAR_ARG1+1 ; Y coordinate (second parameter, low byte)
    LDB VAR_ARG0+1 ; X coordinate (first parameter, low byte)
    JSR Print_Str_d ; Print string from U register
    JSR $F1AF      ; DP_to_C8 - restore DP before return
    RTS

MOD16:
    ; Modulo 16-bit X % D -> D
    PSHS X,D
.MOD16_LOOP:
    PSHS D         ; Save D
    LDD 4,S        ; Load dividend (after PSHS D)
    CMPD 2,S       ; Compare with divisor (after PSHS D)
    PULS D         ; Restore D
    BLT .MOD16_END
    LDX 2,S
    LDD ,S
    LEAX D,X
    STX 2,S
    BRA .MOD16_LOOP
.MOD16_END:
    LDD 2,S        ; Remainder
    LEAS 4,S
    RTS

; === SHOW_LEVEL_RUNTIME - Draw entire level ===
SHOW_LEVEL_RUNTIME:
    ; Input: LEVEL_PTR (pointer to level data)
    ;        LEVEL_WIDTH, LEVEL_HEIGHT (dimensions)
    ; Renders 8x8 tiles as rectangles
    
    LDA #$D0
    TFR A,DP
    JSR Reset0Ref
    
    ; Outer loop: Y (rows)
    CLR LEVEL_Y_IDX
.SL_Y_LOOP:
    LDA LEVEL_Y_IDX
    CMPA LEVEL_HEIGHT
    BHS .SL_DONE         ; If Y >= height, done
    
    ; Inner loop: X (columns)
    CLR LEVEL_X_IDX
.SL_X_LOOP:
    LDA LEVEL_X_IDX
    CMPA LEVEL_WIDTH
    BHS .SL_NEXT_Y       ; If X >= width, next row
    
    ; Calculate tile offset: (Y * width) + X
    LDA LEVEL_Y_IDX
    LDB LEVEL_WIDTH
    MUL                  ; D = Y * width
    ADDB LEVEL_X_IDX     ; D += X
    ADCA #0
    
    ; Add to level pointer (skip 2-byte header)
    ADDD #2              ; Skip width, height bytes
    ADDD LEVEL_PTR
    TFR D,X              ; X = address of tile
    LDA ,X               ; Load tile value
    
    ; If tile is 0 (empty), skip drawing
    CMPA #0
    BEQ .SL_SKIP_TILE
    
    ; Draw tile as 8x8 rectangle
    ; Calculate screen position
    LDA LEVEL_X_IDX
    LDB #8
    MUL                  ; B = X * 8 (pixel X)
    SUBB #128            ; Center horizontally
    STB LEVEL_TEMP       ; Save pixel X
    
    LDA LEVEL_Y_IDX
    LDB #8
    MUL                  ; B = Y * 8 (pixel Y)
    SUBB #128            ; Center vertically
    NEGB                 ; Flip Y (screen coords)
    TFR B,A              ; Y to A
    LDB LEVEL_TEMP       ; X to B
    
    ; Move to tile position
    JSR Moveto_d_7F
    
    ; Draw 8x8 rectangle
    LDA #$7F
    JSR Intensity_a
    
    CLR Vec_Misc_Count
    LDA #0
    LDB #8
    JSR Draw_Line_d      ; Right
    
    CLR Vec_Misc_Count
    LDA #-8
    LDB #0
    JSR Draw_Line_d      ; Down
    
    CLR Vec_Misc_Count
    LDA #0
    LDB #-8
    JSR Draw_Line_d      ; Left
    
    CLR Vec_Misc_Count
    LDA #8
    LDB #0
    JSR Draw_Line_d      ; Up
    
.SL_SKIP_TILE:
    ; Next column
    INC LEVEL_X_IDX
    BRA .SL_X_LOOP
    
.SL_NEXT_Y:
    ; Next row
    INC LEVEL_Y_IDX
    BRA .SL_Y_LOOP
    
.SL_DONE:
    RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_2315958665076:
    FCC "RAM TEST"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_68729639722158:
    FCC "NO LEVELS"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_68994724591312392:
    FCC "RAM TEST OK"
    FCB $80          ; Vectrex string terminator

;***************************************************************************
; EMBEDDED ASSETS (vectors, music, levels, SFX)
;***************************************************************************

; Generated from coin.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-8, max=8, width=16
; Center: (0, 0)

_COIN_WIDTH EQU 16
_COIN_CENTER_X EQU 0
_COIN_CENTER_Y EQU 0

_COIN_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _COIN_PATH0        ; pointer to path 0

_COIN_PATH0:    ; Path 0
    FCB 111              ; path0: intensity
    FCB $08,$00,0,0        ; path0: header (y=8, x=0, relative to center)
    FCB $FF,$FE,$06          ; line 0: flag=-1, dy=-2, dx=6
    FCB $FF,$FA,$02          ; line 1: flag=-1, dy=-6, dx=2
    FCB $FF,$FA,$FE          ; line 2: flag=-1, dy=-6, dx=-2
    FCB $FF,$FE,$FA          ; line 3: flag=-1, dy=-2, dx=-6
    FCB $FF,$02,$FA          ; line 4: flag=-1, dy=2, dx=-6
    FCB $FF,$06,$FE          ; line 5: flag=-1, dy=6, dx=-2
    FCB $FF,$06,$02          ; line 6: flag=-1, dy=6, dx=2
    FCB $FF,$02,$06          ; closing line: flag=-1, dy=2, dx=6
    FCB 2                ; End marker (path complete)
; Generated from square.vec (Malban Draw_Sync_List format)
; Total paths: 2, points: 6
; X bounds: min=-50, max=50, width=100
; Center: (0, 0)

_SQUARE_WIDTH EQU 100
_SQUARE_CENTER_X EQU 0
_SQUARE_CENTER_Y EQU 0

_SQUARE_VECTORS:  ; Main entry (header + 2 path(s))
    FCB 2               ; path_count (runtime metadata)
    FDB _SQUARE_PATH0        ; pointer to path 0
    FDB _SQUARE_PATH1        ; pointer to path 1

_SQUARE_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $32,$CE,0,0        ; path0: header (y=50, x=-50, relative to center)
    FCB $FF,$00,$64          ; line 0: flag=-1, dy=0, dx=100
    FCB $FF,$9C,$00          ; line 1: flag=-1, dy=-100, dx=0
    FCB $FF,$00,$9C          ; line 2: flag=-1, dy=0, dx=-100
    FCB 2                ; End marker (path complete)

_SQUARE_PATH1:    ; Path 1
    FCB 127              ; path1: intensity
    FCB $32,$CE,0,0        ; path1: header (y=50, x=-50, relative to center)
    FCB $FF,$9C,$00          ; line 0: flag=-1, dy=-100, dx=0
    FCB 2                ; End marker (path complete)
; Generated from bubble_huge.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-25, max=27, width=52
; Center: (1, 0)

_BUBBLE_HUGE_WIDTH EQU 52
_BUBBLE_HUGE_CENTER_X EQU 1
_BUBBLE_HUGE_CENTER_Y EQU 0

_BUBBLE_HUGE_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _BUBBLE_HUGE_PATH0        ; pointer to path 0

_BUBBLE_HUGE_PATH0:    ; Path 0
    FCB 111              ; path0: intensity
    FCB $00,$1A,0,0        ; path0: header (y=0, x=26, relative to center)
    FCB $FF,$12,$F8          ; line 0: flag=-1, dy=18, dx=-8
    FCB $FF,$08,$EE          ; line 1: flag=-1, dy=8, dx=-18
    FCB $FF,$F8,$EE          ; line 2: flag=-1, dy=-8, dx=-18
    FCB $FF,$EE,$F8          ; line 3: flag=-1, dy=-18, dx=-8
    FCB $FF,$EE,$08          ; line 4: flag=-1, dy=-18, dx=8
    FCB $FF,$F8,$12          ; line 5: flag=-1, dy=-8, dx=18
    FCB $FF,$08,$12          ; line 6: flag=-1, dy=8, dx=18
    FCB $FF,$12,$08          ; closing line: flag=-1, dy=18, dx=8
    FCB 2                ; End marker (path complete)
; Generated from bubble_large.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 8
; X bounds: min=-15, max=15, width=30
; Center: (0, 0)

_BUBBLE_LARGE_WIDTH EQU 30
_BUBBLE_LARGE_CENTER_X EQU 0
_BUBBLE_LARGE_CENTER_Y EQU 0

_BUBBLE_LARGE_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _BUBBLE_LARGE_PATH0        ; pointer to path 0

_BUBBLE_LARGE_PATH0:    ; Path 0
    FCB 111              ; path0: intensity
    FCB $0F,$00,0,0        ; path0: header (y=15, x=0, relative to center)
    FCB $FF,$FB,$0A          ; line 0: flag=-1, dy=-5, dx=10
    FCB $FF,$F6,$05          ; line 1: flag=-1, dy=-10, dx=5
    FCB $FF,$F6,$FB          ; line 2: flag=-1, dy=-10, dx=-5
    FCB $FF,$FB,$F6          ; line 3: flag=-1, dy=-5, dx=-10
    FCB $FF,$05,$F6          ; line 4: flag=-1, dy=5, dx=-10
    FCB $FF,$0A,$FB          ; line 5: flag=-1, dy=10, dx=-5
    FCB $FF,$0A,$05          ; line 6: flag=-1, dy=10, dx=5
    FCB $FF,$05,$0A          ; closing line: flag=-1, dy=5, dx=10
    FCB 2                ; End marker (path complete)
; Generated from mountain.vec (Malban Draw_Sync_List format)
; Total paths: 1, points: 4
; X bounds: min=-38, max=38, width=76
; Center: (0, 13)

_MOUNTAIN_WIDTH EQU 76
_MOUNTAIN_CENTER_X EQU 0
_MOUNTAIN_CENTER_Y EQU 13

_MOUNTAIN_VECTORS:  ; Main entry (header + 1 path(s))
    FCB 1               ; path_count (runtime metadata)
    FDB _MOUNTAIN_PATH0        ; pointer to path 0

_MOUNTAIN_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $F3,$DA,0,0        ; path0: header (y=-13, x=-38, relative to center)
    FCB $FF,$1A,$0D          ; line 0: flag=-1, dy=26, dx=13
    FCB $FF,$01,$33          ; line 1: flag=-1, dy=1, dx=51
    FCB $FF,$E4,$0C          ; line 2: flag=-1, dy=-28, dx=12
    FCB 2                ; End marker (path complete)
; Generated from fuji_bg.vec (Malban Draw_Sync_List format)
; Total paths: 5, points: 64
; X bounds: min=-124, max=125, width=249
; Center: (0, 12)

_FUJI_BG_WIDTH EQU 249
_FUJI_BG_CENTER_X EQU 0
_FUJI_BG_CENTER_Y EQU 12

_FUJI_BG_VECTORS:  ; Main entry (header + 5 path(s))
    FCB 5               ; path_count (runtime metadata)
    FDB _FUJI_BG_PATH0        ; pointer to path 0
    FDB _FUJI_BG_PATH1        ; pointer to path 1
    FDB _FUJI_BG_PATH2        ; pointer to path 2
    FDB _FUJI_BG_PATH3        ; pointer to path 3
    FDB _FUJI_BG_PATH4        ; pointer to path 4

_FUJI_BG_PATH0:    ; Path 0
    FCB 80              ; path0: intensity
    FCB $DC,$84,0,0        ; path0: header (y=-36, x=-124, relative to center)
    FCB $FF,$0A,$1E          ; line 0: flag=-1, dy=10, dx=30
    FCB $FF,$0E,$1E          ; line 1: flag=-1, dy=14, dx=30
    FCB $FF,$0F,$15          ; line 2: flag=-1, dy=15, dx=21
    FCB $FF,$11,$17          ; line 3: flag=-1, dy=17, dx=23
    FCB $FF,$0E,$0E          ; line 4: flag=-1, dy=14, dx=14
    FCB $FF,$FE,$03          ; line 5: flag=-1, dy=-2, dx=3
    FCB $FF,$03,$04          ; line 6: flag=-1, dy=3, dx=4
    FCB $FF,$FE,$04          ; line 7: flag=-1, dy=-2, dx=4
    FCB $FF,$01,$07          ; line 8: flag=-1, dy=1, dx=7
    FCB $FF,$02,$04          ; line 9: flag=-1, dy=2, dx=4
    FCB $FF,$FD,$06          ; line 10: flag=-1, dy=-3, dx=6
    FCB $FF,$03,$03          ; line 11: flag=-1, dy=3, dx=3
    FCB $FF,$EB,$11          ; line 12: flag=-1, dy=-21, dx=17
    FCB $FF,$F4,$11          ; line 13: flag=-1, dy=-12, dx=17
    FCB $FF,$F0,$16          ; line 14: flag=-1, dy=-16, dx=22
    FCB $FF,$F6,$14          ; line 15: flag=-1, dy=-10, dx=20
    FCB $FF,$F6,$18          ; line 16: flag=-1, dy=-10, dx=24
    FCB $FF,$00,$00          ; line 17: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH1:    ; Path 1
    FCB 95              ; path1: intensity
    FCB $0E,$F1,0,0        ; path1: header (y=14, x=-15, relative to center)
    FCB $FF,$06,$03          ; line 0: flag=-1, dy=6, dx=3
    FCB $FF,$04,$03          ; line 1: flag=-1, dy=4, dx=3
    FCB $FF,$FD,$04          ; line 2: flag=-1, dy=-3, dx=4
    FCB $FF,$FC,$FC          ; line 3: flag=-1, dy=-4, dx=-4
    FCB $FF,$FD,$FA          ; line 4: flag=-1, dy=-3, dx=-6
    FCB $FF,$00,$00          ; line 5: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH2:    ; Path 2
    FCB 95              ; path2: intensity
    FCB $13,$07,0,0        ; path2: header (y=19, x=7, relative to center)
    FCB $FF,$F9,$FD          ; line 0: flag=-1, dy=-7, dx=-3
    FCB $FF,$FA,$02          ; line 1: flag=-1, dy=-6, dx=2
    FCB $FF,$F9,$FD          ; line 2: flag=-1, dy=-7, dx=-3
    FCB $FF,$FD,$04          ; line 3: flag=-1, dy=-3, dx=4
    FCB $FF,$08,$03          ; line 4: flag=-1, dy=8, dx=3
    FCB $FF,$07,$FE          ; line 5: flag=-1, dy=7, dx=-2
    FCB $FF,$06,$01          ; line 6: flag=-1, dy=6, dx=1
    FCB $FF,$02,$FE          ; line 7: flag=-1, dy=2, dx=-2
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH3:    ; Path 3
    FCB 95              ; path3: intensity
    FCB $15,$18,0,0        ; path3: header (y=21, x=24, relative to center)
    FCB $FF,$F7,$05          ; line 0: flag=-1, dy=-9, dx=5
    FCB $FF,$F7,$0C          ; line 1: flag=-1, dy=-9, dx=12
    FCB $FF,$0B,$FA          ; line 2: flag=-1, dy=11, dx=-6
    FCB $FF,$07,$F5          ; line 3: flag=-1, dy=7, dx=-11
    FCB 2                ; End marker (path complete)

_FUJI_BG_PATH4:    ; Path 4
    FCB 100              ; path4: intensity
    FCB $F9,$C7,0,0        ; path4: header (y=-7, x=-57, relative to center)
    FCB $FF,$09,$1A          ; line 0: flag=-1, dy=9, dx=26
    FCB $FF,$EF,$F2          ; line 1: flag=-1, dy=-17, dx=-14
    FCB $FF,$1B,$22          ; line 2: flag=-1, dy=27, dx=34
    FCB $FF,$F2,$FB          ; line 3: flag=-1, dy=-14, dx=-5
    FCB $FF,$00,$03          ; line 4: flag=-1, dy=0, dx=3
    FCB $FF,$F7,$FB          ; line 5: flag=-1, dy=-9, dx=-5
    FCB $FF,$FA,$01          ; line 6: flag=-1, dy=-6, dx=1
    FCB $FF,$0E,$0E          ; line 7: flag=-1, dy=14, dx=14
    FCB $FF,$F1,$00          ; line 8: flag=-1, dy=-15, dx=0
    FCB $FF,$0A,$05          ; line 9: flag=-1, dy=10, dx=5
    FCB $FF,$EA,$06          ; line 10: flag=-1, dy=-22, dx=6
    FCB $FF,$1C,$05          ; line 11: flag=-1, dy=28, dx=5
    FCB $FF,$EF,$06          ; line 12: flag=-1, dy=-17, dx=6
    FCB $FF,$03,$01          ; line 13: flag=-1, dy=3, dx=1
    FCB $FF,$FD,$04          ; line 14: flag=-1, dy=-3, dx=4
    FCB $FF,$0B,$03          ; line 15: flag=-1, dy=11, dx=3
    FCB $FF,$F5,$05          ; line 16: flag=-1, dy=-11, dx=5
    FCB $FF,$10,$FF          ; line 17: flag=-1, dy=16, dx=-1
    FCB $FF,$EE,$13          ; line 18: flag=-1, dy=-18, dx=19
    FCB $FF,$12,$F7          ; line 19: flag=-1, dy=18, dx=-9
    FCB $FF,$F9,$0E          ; line 20: flag=-1, dy=-7, dx=14
    FCB $FF,$04,$02          ; line 21: flag=-1, dy=4, dx=2
    FCB $FF,$FC,$14          ; line 22: flag=-1, dy=-4, dx=20
    FCB 2                ; End marker (path complete)
; ==== Level: TEST_LEVEL ====
; Author: 
; Difficulty: medium

_TEST_LEVEL_LEVEL:
    FDB -96  ; World bounds: xMin (16-bit signed)
    FDB 95  ; xMax (16-bit signed)
    FDB -70  ; yMin (16-bit signed)
    FDB 127  ; yMax (16-bit signed)
    FDB 0  ; Time limit (seconds)
    FDB 0  ; Target score
    FCB 1  ; Background object count
    FCB 4  ; Gameplay object count
    FCB 0  ; Foreground object count
    FDB _TEST_LEVEL_BG_OBJECTS
    FDB _TEST_LEVEL_GAMEPLAY_OBJECTS
    FDB _TEST_LEVEL_FG_OBJECTS

_TEST_LEVEL_BG_OBJECTS:
; Object: obj_1767991638010 (enemy)
    FCB 1  ; type
    FDB 0  ; x
    FDB -2  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 0  ; velocity_x
    FCB 0  ; velocity_y
    FCB 0  ; physics_flags
    FCB 0  ; collision_flags
    FCB 10  ; collision_size
    FDB 0  ; spawn_delay
    FDB _FUJI_BG_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)


_TEST_LEVEL_GAMEPLAY_OBJECTS:
; Object: obj_1767970445633 (enemy)
    FCB 1  ; type
    FDB -37  ; x
    FDB -47  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 255  ; velocity_x
    FCB 0  ; velocity_y
    FCB 1  ; physics_flags
    FCB 1  ; collision_flags
    FCB 10  ; collision_size
    FDB 0  ; spawn_delay
    FDB _COIN_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)

; Object: obj_1767862794353 (enemy)
    FCB 1  ; type
    FDB 45  ; x
    FDB 71  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 255  ; velocity_x
    FCB 255  ; velocity_y
    FCB 1  ; physics_flags
    FCB 3  ; collision_flags
    FCB 27  ; collision_size
    FDB 0  ; spawn_delay
    FDB _BUBBLE_HUGE_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)

; Object: obj_1767883264744 (enemy)
    FCB 1  ; type
    FDB -27  ; x
    FDB 36  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 3  ; velocity_x
    FCB 1  ; velocity_y
    FCB 1  ; physics_flags
    FCB 3  ; collision_flags
    FCB 8  ; collision_size
    FDB 0  ; spawn_delay
    FDB _COIN_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)

; Object: obj_1767873800421 (enemy)
    FCB 1  ; type
    FDB -58  ; x
    FDB 68  ; y
    FDB 256  ; scale (8.8 fixed)
    FCB 0  ; rotation
    FCB 0  ; intensity (0=use vec, >0=override)
    FCB 0  ; velocity_x
    FCB 255  ; velocity_y
    FCB 1  ; physics_flags
    FCB 3  ; collision_flags
    FCB 15  ; collision_size
    FDB 0  ; spawn_delay
    FDB _BUBBLE_LARGE_VECTORS  ; vector_ptr
    FDB 0  ; properties_ptr (reserved)


_TEST_LEVEL_FG_OBJECTS:

