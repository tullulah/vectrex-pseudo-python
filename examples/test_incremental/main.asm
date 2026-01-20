; VPy M6809 Assembly (Vectrex)
; ROM: 524288 bytes
; Multibank cartridge: 32 banks (16KB each)
; Helpers bank: 31 (fixed bank at $4000-$7FFF)

; ================================================
; BANK #0 - Entry point and main code
; ================================================

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
    FDB music1              ; Music pointer
    FCB $F8,$50,$20,$BB     ; Height, Width, Rel Y, Rel X
    FCC "TEST INCREMENTAL"
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
    ; Initialize SFX variables to prevent random noise on startup
    CLR >SFX_ACTIVE         ; Mark SFX as inactive (0=off)
    LDD #$0000
    STD >SFX_PTR            ; Clear SFX pointer
; Bank 0 ($0000) is active; fixed bank 31 ($4000-$7FFF) always visible
    JMP MAIN

;***************************************************************************
; === RAM VARIABLE DEFINITIONS ===
;***************************************************************************
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Temporary pointer (2 bytes)
TMPPTR2              EQU $C880+$04   ; Temporary pointer 2 (2 bytes)
TEMP_YX              EQU $C880+$06   ; Temporary Y/X coordinate storage (2 bytes)
DRAW_LINE_ARGS       EQU $C880+$08   ; DRAW_LINE argument buffer (x0,y0,x1,y1,intensity) (10 bytes)
VLINE_DX_16          EQU $C880+$12   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$14   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$16   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$17   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$18   ; DRAW_LINE remaining dy for segment 2 (16-bit) (2 bytes)
VLINE_DX_REMAINING   EQU $C880+$1A   ; DRAW_LINE remaining dx for segment 2 (16-bit) (2 bytes)
PSG_MUSIC_PTR        EQU $C880+$1C   ; PSG music data pointer (2 bytes)
PSG_MUSIC_START      EQU $C880+$1E   ; PSG music start pointer (for loops) (2 bytes)
PSG_MUSIC_ACTIVE     EQU $C880+$20   ; PSG music active flag (1 bytes)
PSG_IS_PLAYING       EQU $C880+$21   ; PSG playing flag (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$22   ; PSG frame delay counter (1 bytes)
SFX_PTR              EQU $C880+$23   ; SFX data pointer (2 bytes)
SFX_ACTIVE           EQU $C880+$25   ; SFX active flag (1 bytes)
VAR_TITLE_INTENSITY  EQU $C880+$26   ; User variable: title_intensity (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)
CURRENT_ROM_BANK     EQU $CFEA   ; Current ROM bank ID (multibank tracking) (1 bytes)


;***************************************************************************
; MAIN PROGRAM (Bank #0)
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #127
    STD VAR_TITLE_INTENSITY
    ; === Initialize Joystick (one-time setup) ===
    JSR $F1AF    ; DP_to_C8 (required for RAM access)
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

    ; Call main() for initialization
    ; SET_INTENSITY: Set drawing intensity
    LDD #127
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; PLAY_MUSIC("music1") - play music asset (index=0)
    LDX #0        ; Music asset index for lookup
    JSR PLAY_MUSIC_BANKED  ; Play with automatic bank switching
    LDD #0
    STD RESULT

.MAIN_LOOP:
    JSR LOOP_BODY
    LBRA .MAIN_LOOP   ; Use long branch for multibank support

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR $F1AA  ; DP_to_D0: set direct page to $D0 for PSG access
    JSR $F1BA  ; Read_Btns: read PSG register 14, update $C80F (Vec_Btn_State)
    JSR $F1AF  ; DP_to_C8: restore direct page to $C8 for normal RAM access
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_TITLE_INTENSITY
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)
    RTS


; ================================================
; BANK #1 - 0 function(s), 2 asset(s)
; ================================================
    ORG $0000  ; Sequential bank model

;***************************************************************************
; ASSETS IN BANK #1 (11 assets)
;***************************************************************************

_STAR_VRELEASE_SFX:
    ; SFX: star_vrelease (powerup)
    ; Duration: 1720ms (86fr), Freq: 1Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $0F, $FF  ; Tone period = 4095 (big-endian)
    FCB $8F         ; Frame 1 - flags (vol=15, tone=N, noise=N)
    FCB $8D         ; Frame 2 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 3 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 4 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 5 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 6 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 7 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 8 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 9 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 10 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 11 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 12 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 13 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 14 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 15 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 16 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 17 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 18 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 19 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 20 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 21 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 22 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 23 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 24 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 25 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 26 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 27 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 28 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 29 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 30 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 31 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 32 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 33 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 34 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 35 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 36 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 37 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 38 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 39 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 40 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 41 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 42 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 43 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 44 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 45 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 46 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 47 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 48 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 49 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 50 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 51 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 52 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 53 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 54 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 55 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 56 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 57 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 58 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 59 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 60 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 61 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 62 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 63 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 64 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 65 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 66 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 67 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 68 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 69 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 70 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 71 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 72 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 73 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 74 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 75 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 76 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 77 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 78 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 79 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 80 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 81 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 82 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 83 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 84 - flags (vol=13, tone=N, noise=N)
    FCB $8D         ; Frame 85 - flags (vol=13, tone=N, noise=N)
    FCB $D0, $20    ; End of effect marker


; Generated from logo.vec (Malban Draw_Sync_List format)
; Total paths: 7, points: 65
; X bounds: min=-82, max=81, width=163
; Center: (0, 0)

_LOGO_WIDTH EQU 163
_LOGO_CENTER_X EQU 0
_LOGO_CENTER_Y EQU 0

_LOGO_VECTORS:  ; Main entry (header + 7 path(s))
    FCB 7               ; path_count (runtime metadata)
    FDB _LOGO_PATH0        ; pointer to path 0
    FDB _LOGO_PATH1        ; pointer to path 1
    FDB _LOGO_PATH2        ; pointer to path 2
    FDB _LOGO_PATH3        ; pointer to path 3
    FDB _LOGO_PATH4        ; pointer to path 4
    FDB _LOGO_PATH5        ; pointer to path 5
    FDB _LOGO_PATH6        ; pointer to path 6

_LOGO_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $13,$AE,0,0        ; path0: header (y=19, x=-82, relative to center)
    FCB $FF,$EF,$06          ; line 0: flag=-1, dy=-17, dx=6
    FCB $FF,$02,$07          ; line 1: flag=-1, dy=2, dx=7
    FCB $FF,$D6,$09          ; line 2: flag=-1, dy=-42, dx=9
    FCB $FF,$0B,$11          ; line 3: flag=-1, dy=11, dx=17
    FCB $FF,$0C,$FC          ; line 4: flag=-1, dy=12, dx=-4
    FCB $FF,$0D,$10          ; line 5: flag=-1, dy=13, dx=16
    FCB $FF,$0B,$09          ; line 6: flag=-1, dy=11, dx=9
    FCB $FF,$0C,$01          ; line 7: flag=-1, dy=12, dx=1
    FCB $FF,$08,$F8          ; line 8: flag=-1, dy=8, dx=-8
    FCB $FF,$02,$F0          ; line 9: flag=-1, dy=2, dx=-16
    FCB $FF,$FC,$F1          ; line 10: flag=-1, dy=-4, dx=-15
    FCB $FF,$F8,$EA          ; line 11: flag=-1, dy=-8, dx=-22
    FCB $FF,$00,$00          ; line 12: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH1:    ; Path 1
    FCB 127              ; path1: intensity
    FCB $FB,$E3,0,0        ; path1: header (y=-5, x=-29, relative to center)
    FCB $FF,$E7,$F8          ; line 0: flag=-1, dy=-25, dx=-8
    FCB $FF,$04,$10          ; line 1: flag=-1, dy=4, dx=16
    FCB $FF,$0C,$02          ; line 2: flag=-1, dy=12, dx=2
    FCB $FF,$03,$0B          ; line 3: flag=-1, dy=3, dx=11
    FCB $FF,$FA,$00          ; line 4: flag=-1, dy=-6, dx=0
    FCB $FF,$03,$0D          ; line 5: flag=-1, dy=3, dx=13
    FCB $FF,$22,$F7          ; line 6: flag=-1, dy=34, dx=-9
    FCB $FF,$FD,$F1          ; line 7: flag=-1, dy=-3, dx=-15
    FCB $FF,$F5,$FF          ; line 8: flag=-1, dy=-11, dx=-1
    FCB $FF,$F5,$F7          ; line 9: flag=-1, dy=-11, dx=-9
    FCB $FF,$00,$00          ; line 10: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH2:    ; Path 2
    FCB 127              ; path2: intensity
    FCB $07,$CE,0,0        ; path2: header (y=7, x=-50, relative to center)
    FCB $FF,$F8,$02          ; line 0: flag=-1, dy=-8, dx=2
    FCB $FF,$07,$08          ; line 1: flag=-1, dy=7, dx=8
    FCB $FF,$01,$F6          ; line 2: flag=-1, dy=1, dx=-10
    FCB $FF,$00,$00          ; line 3: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH3:    ; Path 3
    FCB 127              ; path3: intensity
    FCB $06,$F4,0,0        ; path3: header (y=6, x=-12, relative to center)
    FCB $FF,$F6,$FD          ; line 0: flag=-1, dy=-10, dx=-3
    FCB $FF,$02,$07          ; line 1: flag=-1, dy=2, dx=7
    FCB $FF,$08,$FC          ; line 2: flag=-1, dy=8, dx=-4
    FCB $FF,$FE,$01          ; line 3: flag=-1, dy=-2, dx=1
    FCB 2                ; End marker (path complete)

_LOGO_PATH4:    ; Path 4
    FCB 127              ; path4: intensity
    FCB $F3,$0A,0,0        ; path4: header (y=-13, x=10, relative to center)
    FCB $FF,$29,$02          ; line 0: flag=-1, dy=41, dx=2
    FCB $FF,$02,$0D          ; line 1: flag=-1, dy=2, dx=13
    FCB $FF,$EB,$0A          ; line 2: flag=-1, dy=-21, dx=10
    FCB $FF,$1A,$07          ; line 3: flag=-1, dy=26, dx=7
    FCB $FF,$03,$14          ; line 4: flag=-1, dy=3, dx=20
    FCB $FF,$D8,$EF          ; line 5: flag=-1, dy=-40, dx=-17
    FCB $FF,$FE,$F3          ; line 6: flag=-1, dy=-2, dx=-13
    FCB $FF,$0D,$F8          ; line 7: flag=-1, dy=13, dx=-8
    FCB $FF,$EE,$FC          ; line 8: flag=-1, dy=-18, dx=-4
    FCB $FF,$FC,$F6          ; line 9: flag=-1, dy=-4, dx=-10
    FCB $FF,$00,$00          ; line 10: flag=-1, dy=0, dx=0
    FCB 2                ; End marker (path complete)

_LOGO_PATH5:    ; Path 5
    FCB 127              ; path5: intensity
    FCB $06,$45,0,0        ; path5: header (y=6, x=69, relative to center)
    FCB $FF,$08,$F5          ; line 0: flag=-1, dy=8, dx=-11
    FCB $FF,$F4,$F7          ; line 1: flag=-1, dy=-12, dx=-9
    FCB $FF,$F7,$01          ; line 2: flag=-1, dy=-9, dx=1
    FCB $FF,$FE,$0C          ; line 3: flag=-1, dy=-2, dx=12
    FCB $FF,$03,$FA          ; line 4: flag=-1, dy=3, dx=-6
    FCB $FF,$05,$01          ; line 5: flag=-1, dy=5, dx=1
    FCB $FF,$02,$17          ; line 6: flag=-1, dy=2, dx=23
    FCB $FF,$F3,$FD          ; line 7: flag=-1, dy=-13, dx=-3
    FCB $FF,$F9,$EE          ; line 8: flag=-1, dy=-7, dx=-18
    FCB $FF,$04,$F0          ; line 9: flag=-1, dy=4, dx=-16
    FCB $FF,$0B,$F8          ; line 10: flag=-1, dy=11, dx=-8
    FCB 2                ; End marker (path complete)

_LOGO_PATH6:    ; Path 6
    FCB 127              ; path6: intensity
    FCB $06,$45,0,0        ; path6: header (y=6, x=69, relative to center)
    FCB $FF,$00,$0C          ; line 0: flag=-1, dy=0, dx=12
    FCB $FF,$0C,$F8          ; line 1: flag=-1, dy=12, dx=-8
    FCB $FF,$03,$F0          ; line 2: flag=-1, dy=3, dx=-16
    FCB $FF,$FB,$FC          ; line 3: flag=-1, dy=-5, dx=-4
    FCB 2                ; End marker (path complete)

; Generated from pang_logo.vec (Malban Draw_Sync_List format)
; Total paths: 14, points: 48
; X bounds: min=-110, max=110, width=220
; Center: (0, -10)

_PANG_LOGO_WIDTH EQU 220
_PANG_LOGO_CENTER_X EQU 0
_PANG_LOGO_CENTER_Y EQU -10

_PANG_LOGO_VECTORS:  ; Main entry (header + 14 path(s))
    FCB 14               ; path_count (runtime metadata)
    FDB _PANG_LOGO_PATH0        ; pointer to path 0
    FDB _PANG_LOGO_PATH1        ; pointer to path 1
    FDB _PANG_LOGO_PATH2        ; pointer to path 2
    FDB _PANG_LOGO_PATH3        ; pointer to path 3
    FDB _PANG_LOGO_PATH4        ; pointer to path 4
    FDB _PANG_LOGO_PATH5        ; pointer to path 5
    FDB _PANG_LOGO_PATH6        ; pointer to path 6
    FDB _PANG_LOGO_PATH7        ; pointer to path 7
    FDB _PANG_LOGO_PATH8        ; pointer to path 8
    FDB _PANG_LOGO_PATH9        ; pointer to path 9
    FDB _PANG_LOGO_PATH10        ; pointer to path 10
    FDB _PANG_LOGO_PATH11        ; pointer to path 11
    FDB _PANG_LOGO_PATH12        ; pointer to path 12
    FDB _PANG_LOGO_PATH13        ; pointer to path 13

_PANG_LOGO_PATH0:    ; Path 0
    FCB 127              ; path0: intensity
    FCB $D8,$A1,0,0        ; path0: header (y=-40, x=-95, relative to center)
    FCB $FF,$64,$00          ; line 0: flag=-1, dy=100, dx=0
    FCB $FF,$00,$1E          ; line 1: flag=-1, dy=0, dx=30
    FCB $FF,$F6,$14          ; line 2: flag=-1, dy=-10, dx=20
    FCB $FF,$D3,$00          ; line 3: flag=-1, dy=-45, dx=0
    FCB $FF,$FB,$EC          ; line 4: flag=-1, dy=-5, dx=-20
    FCB $FF,$0A,$E2          ; line 5: flag=-1, dy=10, dx=-30
    FCB $FF,$CE,$00          ; closing line: flag=-1, dy=-50, dx=0
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH1:    ; Path 1
    FCB 127              ; path1: intensity
    FCB $3C,$E2,0,0        ; path1: header (y=60, x=-30, relative to center)
    FCB $FF,$9C,$0F          ; line 0: flag=-1, dy=-100, dx=15
    FCB $FF,$64,$0F          ; line 1: flag=-1, dy=100, dx=15
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH2:    ; Path 2
    FCB 127              ; path2: intensity
    FCB $0F,$E7,0,0        ; path2: header (y=15, x=-25, relative to center)
    FCB $FF,$00,$1E          ; line 0: flag=-1, dy=0, dx=30
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH3:    ; Path 3
    FCB 127              ; path3: intensity
    FCB $D8,$0F,0,0        ; path3: header (y=-40, x=15, relative to center)
    FCB $FF,$64,$00          ; line 0: flag=-1, dy=100, dx=0
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH4:    ; Path 4
    FCB 127              ; path4: intensity
    FCB $3C,$0F,0,0        ; path4: header (y=60, x=15, relative to center)
    FCB $FF,$9C,$23          ; line 0: flag=-1, dy=-100, dx=35
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH5:    ; Path 5
    FCB 127              ; path5: intensity
    FCB $D8,$32,0,0        ; path5: header (y=-40, x=50, relative to center)
    FCB $FF,$64,$00          ; line 0: flag=-1, dy=100, dx=0
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH6:    ; Path 6
    FCB 127              ; path6: intensity
    FCB $DD,$5F,0,0        ; path6: header (y=-35, x=95, relative to center)
    FCB $FF,$FB,$EC          ; line 0: flag=-1, dy=-5, dx=-20
    FCB $FF,$0A,$F1          ; line 1: flag=-1, dy=10, dx=-15
    FCB $FF,$50,$00          ; line 2: flag=-1, dy=80, dx=0
    FCB $FF,$0A,$0F          ; line 3: flag=-1, dy=10, dx=15
    FCB $FF,$FB,$14          ; line 4: flag=-1, dy=-5, dx=20
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH7:    ; Path 7
    FCB 127              ; path7: intensity
    FCB $19,$5F,0,0        ; path7: header (y=25, x=95, relative to center)
    FCB $FF,$FB,$EC          ; line 0: flag=-1, dy=-5, dx=-20
    FCB $FF,$EC,$00          ; line 1: flag=-1, dy=-20, dx=0
    FCB $FF,$FB,$14          ; line 2: flag=-1, dy=-5, dx=20
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH8:    ; Path 8
    FCB 127              ; path8: intensity
    FCB $BA,$00,0,0        ; path8: header (y=-70, x=0, relative to center)
    FCB $FF,$14,$04          ; line 0: flag=-1, dy=20, dx=4
    FCB $FF,$FB,$15          ; line 1: flag=-1, dy=-5, dx=21
    FCB $FF,$0F,$F1          ; line 2: flag=-1, dy=15, dx=-15
    FCB $FF,$14,$05          ; line 3: flag=-1, dy=20, dx=5
    FCB $FF,$F4,$F1          ; line 4: flag=-1, dy=-12, dx=-15
    FCB $FF,$0C,$F1          ; line 5: flag=-1, dy=12, dx=-15
    FCB $FF,$EC,$05          ; line 6: flag=-1, dy=-20, dx=5
    FCB $FF,$F1,$F1          ; line 7: flag=-1, dy=-15, dx=-15
    FCB $FF,$05,$15          ; line 8: flag=-1, dy=5, dx=21
    FCB $FF,$EC,$04          ; closing line: flag=-1, dy=-20, dx=4
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH9:    ; Path 9
    FCB 100              ; path9: intensity
    FCB $0A,$00,0,0        ; path9: header (y=10, x=0, relative to center)
    FCB $FF,$DD,$23          ; line 0: flag=-1, dy=-35, dx=35
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH10:    ; Path 10
    FCB 100              ; path10: intensity
    FCB $0A,$00,0,0        ; path10: header (y=10, x=0, relative to center)
    FCB $FF,$00,$28          ; line 0: flag=-1, dy=0, dx=40
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH11:    ; Path 11
    FCB 100              ; path11: intensity
    FCB $0A,$00,0,0        ; path11: header (y=10, x=0, relative to center)
    FCB $FF,$23,$1E          ; line 0: flag=-1, dy=35, dx=30
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH12:    ; Path 12
    FCB 100              ; path12: intensity
    FCB $0A,$00,0,0        ; path12: header (y=10, x=0, relative to center)
    FCB $FF,$0A,$D8          ; line 0: flag=-1, dy=10, dx=-40
    FCB 2                ; End marker (path complete)

_PANG_LOGO_PATH13:    ; Path 13
    FCB 127              ; path13: intensity
    FCB $46,$92,0,0        ; path13: header (y=70, x=-110, relative to center)
    FCB $FF,$00,$7F          ; line 0: flag=-1, dy=0, dx=127
    FCB 2                ; End marker (path complete)

_COIN_SFX:
    ; SFX: coin (custom)
    ; Duration: 590ms (29fr), Freq: 855Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A7         ; Frame 1 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $AF         ; Frame 2 - flags (vol=15, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $AD         ; Frame 3 - flags (vol=13, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $AB         ; Frame 4 - flags (vol=11, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A9         ; Frame 5 - flags (vol=9, tone=Y, noise=N)
    FCB $00, $55  ; Tone period = 85 (big-endian)
    FCB $A7         ; Frame 6 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $55  ; Tone period = 85 (big-endian)
    FCB $A7         ; Frame 7 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $55  ; Tone period = 85 (big-endian)
    FCB $A7         ; Frame 8 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $55  ; Tone period = 85 (big-endian)
    FCB $A7         ; Frame 9 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A7         ; Frame 10 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A7         ; Frame 11 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A7         ; Frame 12 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A7         ; Frame 13 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $65  ; Tone period = 101 (big-endian)
    FCB $A7         ; Frame 14 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $65  ; Tone period = 101 (big-endian)
    FCB $A7         ; Frame 15 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $65  ; Tone period = 101 (big-endian)
    FCB $A7         ; Frame 16 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $65  ; Tone period = 101 (big-endian)
    FCB $A7         ; Frame 17 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $65  ; Tone period = 101 (big-endian)
    FCB $A7         ; Frame 18 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $A7         ; Frame 19 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $A7         ; Frame 20 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $A7         ; Frame 21 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $47  ; Tone period = 71 (big-endian)
    FCB $A6         ; Frame 22 - flags (vol=6, tone=Y, noise=N)
    FCB $00, $4B  ; Tone period = 75 (big-endian)
    FCB $A5         ; Frame 23 - flags (vol=5, tone=Y, noise=N)
    FCB $00, $4B  ; Tone period = 75 (big-endian)
    FCB $A4         ; Frame 24 - flags (vol=4, tone=Y, noise=N)
    FCB $00, $4B  ; Tone period = 75 (big-endian)
    FCB $A3         ; Frame 25 - flags (vol=3, tone=Y, noise=N)
    FCB $00, $4B  ; Tone period = 75 (big-endian)
    FCB $A2         ; Frame 26 - flags (vol=2, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A1         ; Frame 27 - flags (vol=1, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $A0         ; Frame 28 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $5F  ; Tone period = 95 (big-endian)
    FCB $D0, $20    ; End of effect marker


_LASER_SFX:
    ; SFX: laser (laser)
    ; Duration: 500ms (25fr), Freq: 880Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $34  ; Tone period = 52 (big-endian)
    FCB $AF         ; Frame 1 - flags (vol=15, tone=Y, noise=N)
    FCB $00, $3A  ; Tone period = 58 (big-endian)
    FCB $AC         ; Frame 2 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $42  ; Tone period = 66 (big-endian)
    FCB $AC         ; Frame 3 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $48  ; Tone period = 72 (big-endian)
    FCB $AC         ; Frame 4 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $4E  ; Tone period = 78 (big-endian)
    FCB $AC         ; Frame 5 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $56  ; Tone period = 86 (big-endian)
    FCB $AC         ; Frame 6 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $5C  ; Tone period = 92 (big-endian)
    FCB $AC         ; Frame 7 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $62  ; Tone period = 98 (big-endian)
    FCB $AC         ; Frame 8 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $6A  ; Tone period = 106 (big-endian)
    FCB $AC         ; Frame 9 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $70  ; Tone period = 112 (big-endian)
    FCB $AC         ; Frame 10 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $76  ; Tone period = 118 (big-endian)
    FCB $AC         ; Frame 11 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $7C  ; Tone period = 124 (big-endian)
    FCB $AC         ; Frame 12 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $84  ; Tone period = 132 (big-endian)
    FCB $AC         ; Frame 13 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $8A  ; Tone period = 138 (big-endian)
    FCB $AC         ; Frame 14 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $90  ; Tone period = 144 (big-endian)
    FCB $AC         ; Frame 15 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $98  ; Tone period = 152 (big-endian)
    FCB $AC         ; Frame 16 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $9E  ; Tone period = 158 (big-endian)
    FCB $AC         ; Frame 17 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $A4  ; Tone period = 164 (big-endian)
    FCB $AC         ; Frame 18 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $AC  ; Tone period = 172 (big-endian)
    FCB $AC         ; Frame 19 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $B2  ; Tone period = 178 (big-endian)
    FCB $AC         ; Frame 20 - flags (vol=12, tone=Y, noise=N)
    FCB $00, $B8  ; Tone period = 184 (big-endian)
    FCB $A9         ; Frame 21 - flags (vol=9, tone=Y, noise=N)
    FCB $00, $C0  ; Tone period = 192 (big-endian)
    FCB $A7         ; Frame 22 - flags (vol=7, tone=Y, noise=N)
    FCB $00, $C6  ; Tone period = 198 (big-endian)
    FCB $A4         ; Frame 23 - flags (vol=4, tone=Y, noise=N)
    FCB $00, $CC  ; Tone period = 204 (big-endian)
    FCB $A2         ; Frame 24 - flags (vol=2, tone=Y, noise=N)
    FCB $00, $D4  ; Tone period = 212 (big-endian)
    FCB $D0, $20    ; End of effect marker


_EXPLOSION1_SFX:
    ; SFX: explosion1 (custom)
    ; Duration: 740ms (37fr), Freq: 19531Hz, Channel: 0
    FCB $60         ; Frame 0 - flags (vol=0, tone=Y, noise=Y)
    FCB $00, $02  ; Tone period = 2 (big-endian)
    FCB $1A         ; Noise period
    FCB $07         ; Frame 1 - flags (vol=7, tone=N, noise=N)
    FCB $0E         ; Frame 2 - flags (vol=14, tone=N, noise=N)
    FCB $0E         ; Frame 3 - flags (vol=14, tone=N, noise=N)
    FCB $0E         ; Frame 4 - flags (vol=14, tone=N, noise=N)
    FCB $0E         ; Frame 5 - flags (vol=14, tone=N, noise=N)
    FCB $0D         ; Frame 6 - flags (vol=13, tone=N, noise=N)
    FCB $0D         ; Frame 7 - flags (vol=13, tone=N, noise=N)
    FCB $0D         ; Frame 8 - flags (vol=13, tone=N, noise=N)
    FCB $0D         ; Frame 9 - flags (vol=13, tone=N, noise=N)
    FCB $0C         ; Frame 10 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 11 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 12 - flags (vol=12, tone=N, noise=N)
    FCB $0B         ; Frame 13 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 14 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 15 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 16 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 17 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 18 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 19 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 20 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 21 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 22 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 23 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 24 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 25 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 26 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 27 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 28 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 29 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 30 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 31 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 32 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 33 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 34 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 35 - flags (vol=11, tone=N, noise=N)
    FCB $0B         ; Frame 36 - flags (vol=11, tone=N, noise=N)
    FCB $D0, $20    ; End of effect marker


_BOMBER_SHOT_SFX:
    ; SFX: bomber_shot (custom)
    ; Duration: 460ms (23fr), Freq: 1Hz, Channel: 0
    FCB $60         ; Frame 0 - flags (vol=0, tone=Y, noise=Y)
    FCB $00, $01  ; Tone period = 1 (big-endian)
    FCB $1E         ; Noise period
    FCB $6F         ; Frame 1 - flags (vol=15, tone=Y, noise=Y)
    FCB $01, $74  ; Tone period = 372 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 2 - flags (vol=10, tone=Y, noise=Y)
    FCB $02, $E8  ; Tone period = 744 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 3 - flags (vol=10, tone=Y, noise=Y)
    FCB $04, $5C  ; Tone period = 1116 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 4 - flags (vol=10, tone=Y, noise=Y)
    FCB $05, $D0  ; Tone period = 1488 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 5 - flags (vol=10, tone=Y, noise=Y)
    FCB $07, $44  ; Tone period = 1860 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 6 - flags (vol=10, tone=Y, noise=Y)
    FCB $08, $B8  ; Tone period = 2232 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 7 - flags (vol=10, tone=Y, noise=Y)
    FCB $0A, $2C  ; Tone period = 2604 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 8 - flags (vol=10, tone=Y, noise=Y)
    FCB $0B, $A2  ; Tone period = 2978 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 9 - flags (vol=10, tone=Y, noise=Y)
    FCB $0D, $16  ; Tone period = 3350 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 10 - flags (vol=10, tone=Y, noise=Y)
    FCB $0E, $8A  ; Tone period = 3722 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 11 - flags (vol=10, tone=Y, noise=Y)
    FCB $0F, $FE  ; Tone period = 4094 (big-endian)
    FCB $1E         ; Noise period
    FCB $6A         ; Frame 12 - flags (vol=10, tone=Y, noise=Y)
    FCB $0F, $FF  ; Tone period = 4095 (big-endian)
    FCB $1E         ; Noise period
    FCB $0A         ; Frame 13 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 14 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 15 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 16 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 17 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 18 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 19 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 20 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 21 - flags (vol=10, tone=N, noise=N)
    FCB $0A         ; Frame 22 - flags (vol=10, tone=N, noise=N)
    FCB $D0, $20    ; End of effect marker


; Generated from music1.vmus (internal name: Test Song)
; Tempo: 120 BPM, Total events: 7 (PSG Direct format)
; Format: FCB count, FCB reg, val, ... (per frame), FCB 0 (end)

_MUSIC1_MUSIC:
    ; Frame-based PSG register writes
    FCB     0              ; Delay 0 frames (maintain previous state)
    FCB     6              ; Frame 0 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $66             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $01             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 25 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $1C             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $01             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 50 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $EF             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 75 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $B3             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     25              ; Delay 25 frames (maintain previous state)
    FCB     6              ; Frame 100 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $EF             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $00             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     24              ; Delay 24 frames (maintain previous state)
    FCB     6              ; Frame 124 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $1C             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $01             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     26              ; Delay 26 frames (maintain previous state)
    FCB     6              ; Frame 150 - 6 register writes
    FCB     0               ; Reg 0 number
    FCB     $66             ; Reg 0 value
    FCB     1               ; Reg 1 number
    FCB     $01             ; Reg 1 value
    FCB     8               ; Reg 8 number
    FCB     $0F             ; Reg 8 value
    FCB     9               ; Reg 9 number
    FCB     $00             ; Reg 9 value
    FCB     10               ; Reg 10 number
    FCB     $00             ; Reg 10 value
    FCB     7               ; Reg 7 number
    FCB     $FE             ; Reg 7 value
    FCB     50              ; Delay 50 frames before loop
    FCB     $FF             ; Loop command ($FF never valid as count)
    FDB     _MUSIC1_MUSIC       ; Jump to start (absolute address)


_HIT_SFX:
    ; SFX: hit (hit)
    ; Duration: 300ms (15fr), Freq: 200Hz, Channel: 0
    FCB $60         ; Frame 0 - flags (vol=0, tone=Y, noise=Y)
    FCB $00, $8C  ; Tone period = 140 (big-endian)
    FCB $08         ; Noise period
    FCB $6F         ; Frame 1 - flags (vol=15, tone=Y, noise=Y)
    FCB $00, $AA  ; Tone period = 170 (big-endian)
    FCB $08         ; Noise period
    FCB $6F         ; Frame 2 - flags (vol=15, tone=Y, noise=Y)
    FCB $00, $C8  ; Tone period = 200 (big-endian)
    FCB $08         ; Noise period
    FCB $6E         ; Frame 3 - flags (vol=14, tone=Y, noise=Y)
    FCB $00, $E6  ; Tone period = 230 (big-endian)
    FCB $08         ; Noise period
    FCB $6D         ; Frame 4 - flags (vol=13, tone=Y, noise=Y)
    FCB $01, $04  ; Tone period = 260 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 5 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $22  ; Tone period = 290 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 6 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $40  ; Tone period = 320 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 7 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $5E  ; Tone period = 350 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 8 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $7C  ; Tone period = 380 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 9 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $9A  ; Tone period = 410 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 10 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $B8  ; Tone period = 440 (big-endian)
    FCB $08         ; Noise period
    FCB $6C         ; Frame 11 - flags (vol=12, tone=Y, noise=Y)
    FCB $01, $D6  ; Tone period = 470 (big-endian)
    FCB $08         ; Noise period
    FCB $69         ; Frame 12 - flags (vol=9, tone=Y, noise=Y)
    FCB $01, $F4  ; Tone period = 500 (big-endian)
    FCB $08         ; Noise period
    FCB $66         ; Frame 13 - flags (vol=6, tone=Y, noise=Y)
    FCB $02, $12  ; Tone period = 530 (big-endian)
    FCB $08         ; Noise period
    FCB $63         ; Frame 14 - flags (vol=3, tone=Y, noise=Y)
    FCB $02, $30  ; Tone period = 560 (big-endian)
    FCB $08         ; Noise period
    FCB $D0, $20    ; End of effect marker


_BONUS_COLLECTED_SFX:
    ; SFX: bonus_collected (custom)
    ; Duration: 460ms (23fr), Freq: 5Hz, Channel: 0
    FCB $60         ; Frame 0 - flags (vol=0, tone=Y, noise=Y)
    FCB $0F, $FF  ; Tone period = 4095 (big-endian)
    FCB $00         ; Noise period
    FCB $0E         ; Frame 1 - flags (vol=14, tone=N, noise=N)
    FCB $0E         ; Frame 2 - flags (vol=14, tone=N, noise=N)
    FCB $0E         ; Frame 3 - flags (vol=14, tone=N, noise=N)
    FCB $0D         ; Frame 4 - flags (vol=13, tone=N, noise=N)
    FCB $0D         ; Frame 5 - flags (vol=13, tone=N, noise=N)
    FCB $0D         ; Frame 6 - flags (vol=13, tone=N, noise=N)
    FCB $0C         ; Frame 7 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 8 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 9 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 10 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 11 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 12 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 13 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 14 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 15 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 16 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 17 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 18 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 19 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 20 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 21 - flags (vol=12, tone=N, noise=N)
    FCB $0C         ; Frame 22 - flags (vol=12, tone=N, noise=N)
    FCB $D0, $20    ; End of effect marker


_JUMP_SFX:
    ; SFX: jump (jump)
    ; Duration: 180ms (9fr), Freq: 330Hz, Channel: 0
    FCB $A0         ; Frame 0 - flags (vol=0, tone=Y, noise=N)
    FCB $00, $AA  ; Tone period = 170 (big-endian)
    FCB $AE         ; Frame 1 - flags (vol=14, tone=Y, noise=N)
    FCB $00, $CA  ; Tone period = 202 (big-endian)
    FCB $AD         ; Frame 2 - flags (vol=13, tone=Y, noise=N)
    FCB $00, $EA  ; Tone period = 234 (big-endian)
    FCB $AC         ; Frame 3 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $0A  ; Tone period = 266 (big-endian)
    FCB $AC         ; Frame 4 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $2A  ; Tone period = 298 (big-endian)
    FCB $AC         ; Frame 5 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $4A  ; Tone period = 330 (big-endian)
    FCB $AC         ; Frame 6 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $6A  ; Tone period = 362 (big-endian)
    FCB $AC         ; Frame 7 - flags (vol=12, tone=Y, noise=N)
    FCB $01, $8A  ; Tone period = 394 (big-endian)
    FCB $A6         ; Frame 8 - flags (vol=6, tone=Y, noise=N)
    FCB $01, $AA  ; Tone period = 426 (big-endian)
    FCB $D0, $20    ; End of effect marker



; ================================================
; BANK #2 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #3 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #4 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #5 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #6 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #7 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #8 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #9 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #10 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #11 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #12 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #13 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #14 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #15 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #16 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #17 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #18 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #19 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #20 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #21 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #22 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #23 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #24 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #25 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #26 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #27 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #28 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #29 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #30 - 0 function(s) [EMPTY]
; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
; BANK #31 - 0 function(s) [HELPERS ONLY]
; ================================================
    ORG $4000  ; Fixed bank (always visible at $4000-$7FFF)
    ; Runtime helpers (accessible from all banks)

;***************************************************************************
; ASSET LOOKUP TABLES (for banked asset access)
; Total: 2 vectors, 1 music, 8 sfx, 0 levels
;***************************************************************************

; Vector Asset Index Mapping:
;   0 = logo (Bank #1)
;   1 = pang_logo (Bank #1)

VECTOR_BANK_TABLE:
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID

VECTOR_ADDR_TABLE:
    FDB _LOGO_VECTORS    ; logo
    FDB _PANG_LOGO_VECTORS    ; pang_logo

; Music Asset Index Mapping:
;   0 = music1 (Bank #1)

MUSIC_BANK_TABLE:
    FCB 1              ; Bank ID

MUSIC_ADDR_TABLE:
    FDB _MUSIC1_MUSIC    ; music1

; SFX Asset Index Mapping:
;   0 = bomber_shot (Bank #1)
;   1 = bonus_collected (Bank #1)
;   2 = coin (Bank #1)
;   3 = explosion1 (Bank #1)
;   4 = hit (Bank #1)
;   5 = jump (Bank #1)
;   6 = laser (Bank #1)
;   7 = star_vrelease (Bank #1)

SFX_BANK_TABLE:
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID

SFX_ADDR_TABLE:
    FDB _BOMBER_SHOT_SFX    ; bomber_shot
    FDB _BONUS_COLLECTED_SFX    ; bonus_collected
    FDB _COIN_SFX    ; coin
    FDB _EXPLOSION1_SFX    ; explosion1
    FDB _HIT_SFX    ; hit
    FDB _JUMP_SFX    ; jump
    FDB _LASER_SFX    ; laser
    FDB _STAR_VRELEASE_SFX    ; star_vrelease

; Legacy unified tables (all assets)
ASSET_BANK_TABLE:
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID
    FCB 1              ; Bank ID

ASSET_ADDR_TABLE:
    FDB _STAR_VRELEASE_SFX    ; star_vrelease
    FDB _LOGO_VECTORS    ; logo
    FDB _PANG_LOGO_VECTORS    ; pang_logo
    FDB _COIN_SFX    ; coin
    FDB _LASER_SFX    ; laser
    FDB _EXPLOSION1_SFX    ; explosion1
    FDB _BOMBER_SHOT_SFX    ; bomber_shot
    FDB _MUSIC1_MUSIC    ; music1
    FDB _HIT_SFX    ; hit
    FDB _BONUS_COLLECTED_SFX    ; bonus_collected
    FDB _JUMP_SFX    ; jump

;***************************************************************************
; DRAW_VECTOR_BANKED - Draw vector asset with automatic bank switching
; Input: X = asset index (0-based), DRAW_VEC_X/Y set for position
; Uses: A, B, X, Y
; Preserves: CURRENT_ROM_BANK (restored after drawing)
;***************************************************************************
DRAW_VECTOR_BANKED:
    ; Save context: original bank + asset index on stack
    LDA CURRENT_ROM_BANK
    PSHS A,X             ; Stack: [original_bank, asset_index(2 bytes)]

    ; Get asset's bank from lookup table
    TFR X,D              ; D = asset index
    LDX #VECTOR_BANK_TABLE
    LDA D,X              ; A = bank ID for this asset
    STA CURRENT_ROM_BANK ; Update RAM tracker
    STA $DF00            ; Switch bank hardware register

    ; Get asset's address from lookup table (2 bytes per entry)
    LDD 1,S              ; Reload asset index from stack (offset 1, skip saved bank)
    ASLB                 ; *2 for FDB entries
    ROLA
    LDX #VECTOR_ADDR_TABLE
    LEAX D,X             ; X points to address entry
    LDX ,X               ; X = actual vector address in banked ROM

    ; Set up for drawing
    CLR MIRROR_X
    CLR MIRROR_Y
    CLR DRAW_VEC_INTENSITY
    JSR $F1AA            ; DP_to_D0

    ; Draw the vector (X already has address)
    JSR Draw_Sync_List_At_With_Mirrors

    JSR $F1AF            ; DP_to_C8

    ; Restore original bank from stack
    PULS A,X             ; A = original bank, X = asset index (discard)
    STA CURRENT_ROM_BANK
    STA $DF00            ; Restore bank

    RTS

;***************************************************************************
; PLAY_MUSIC_BANKED - Play music asset with automatic bank switching
; Input: X = music asset index (0-based)
; Uses: A, B, X
; Note: Music data is COPIED to RAM, so bank switch is temporary
;***************************************************************************
PLAY_MUSIC_BANKED:
    ; Save context: original bank + music index on stack
    LDA CURRENT_ROM_BANK
    PSHS A,X             ; Stack: [original_bank, music_index(2 bytes)]

    ; Get music's bank from lookup table
    TFR X,D              ; D = music index
    LDX #MUSIC_BANK_TABLE
    LDA D,X              ; A = bank ID for this music
    STA CURRENT_ROM_BANK ; Update RAM tracker
    STA $DF00            ; Switch bank hardware register

    ; Get music's address from lookup table (2 bytes per entry)
    LDD 1,S              ; Reload music index from stack (offset 1)
    ASLB                 ; *2 for FDB entries
    ROLA
    LDX #MUSIC_ADDR_TABLE
    LEAX D,X             ; X points to address entry
    LDX ,X               ; X = actual music address in banked ROM

    ; Call PLAY_MUSIC_RUNTIME with X pointing to music data
    JSR PLAY_MUSIC_RUNTIME

    ; Restore original bank from stack
    PULS A,X             ; A = original bank, X = music index (discard)
    STA CURRENT_ROM_BANK
    STA $DF00            ; Restore bank

    RTS

;***************************************************************************
; PLAY_SFX_BANKED - Play SFX asset with automatic bank switching
; Input: X = SFX asset index (0-based)
; Uses: A, B, X
;***************************************************************************
PLAY_SFX_BANKED:
    ; Save context: original bank + SFX index on stack
    LDA CURRENT_ROM_BANK
    PSHS A,X             ; Stack: [original_bank, sfx_index(2 bytes)]

    ; Get SFX's bank from lookup table
    TFR X,D              ; D = SFX index
    LDX #SFX_BANK_TABLE
    LDA D,X              ; A = bank ID for this SFX
    STA CURRENT_ROM_BANK ; Update RAM tracker
    STA $DF00            ; Switch bank hardware register

    ; Get SFX's address from lookup table (2 bytes per entry)
    LDD 1,S              ; Reload SFX index from stack (offset 1)
    ASLB                 ; *2 for FDB entries
    ROLA
    LDX #SFX_ADDR_TABLE
    LEAX D,X             ; X points to address entry
    LDX ,X               ; X = actual SFX address in banked ROM

    ; Call PLAY_SFX_RUNTIME with X pointing to SFX data
    JSR PLAY_SFX_RUNTIME

    ; Restore original bank from stack
    PULS A,X             ; A = original bank, X = SFX index (discard)
    STA CURRENT_ROM_BANK
    STA $DF00            ; Restore bank

    RTS

;***************************************************************************
; RUNTIME HELPERS
;***************************************************************************

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

; ============================================================================
; PSG DIRECT MUSIC PLAYER (inspired by Christman2024/malbanGit)
; ============================================================================
; Writes directly to PSG chip using WRITE_PSG sequence
;
; Music data format (frame-based):
;   FCB count           ; Number of register writes this frame
;   FCB reg, val        ; PSG register/value pairs
;   ...                 ; Repeat for each register
;   FCB $FF             ; End marker
;
; PSG Registers:
;   0-1: Channel A frequency (12-bit)
;   2-3: Channel B frequency
;   4-5: Channel C frequency
;   6:   Noise period
;   7:   Mixer control (enable/disable channels)
;   8-10: Channel A/B/C volume
;   11-12: Envelope period
;   13:  Envelope shape
; ============================================================================

; RAM variables (defined in SYSTEM RAM VARIABLES section):
; PSG_MUSIC_PTR, PSG_MUSIC_START, PSG_IS_PLAYING,
; PSG_MUSIC_ACTIVE, PSG_DELAY_FRAMES

; PLAY_MUSIC_RUNTIME - Start PSG music playback
; Input: X = pointer to PSG music data
PLAY_MUSIC_RUNTIME:
CMPX >PSG_MUSIC_START   ; Check if already playing this music
BNE PMr_start_new       ; If different, start fresh
LDA >PSG_IS_PLAYING     ; Check if currently playing
BNE PMr_done            ; If playing same song, ignore
PMr_start_new:
STX >PSG_MUSIC_PTR      ; Store current music pointer (force extended)
STX >PSG_MUSIC_START    ; Store start pointer for loops (force extended)
CLR >PSG_DELAY_FRAMES   ; Clear delay counter
LDA #$01
STA >PSG_IS_PLAYING     ; Mark as playing (extended - var at 0xC8A0)
PMr_done:
RTS

; ============================================================================
; UPDATE_MUSIC_PSG - Update PSG (call every frame)
; ============================================================================
UPDATE_MUSIC_PSG:
; CRITICAL: Set VIA to PSG mode BEFORE accessing PSG (don't assume state)
; DISABLED: Conflicts with SFX which uses Sound_Byte (HANDSHAKE mode)
; LDA #$00       ; VIA_cntl = $00 (PSG mode)
; STA >$D00C     ; VIA_cntl
LDA #$01
STA >PSG_MUSIC_ACTIVE   ; Mark music system active (for PSG logging)
LDA >PSG_IS_PLAYING     ; Check if playing (extended - var at 0xC8A0)
BEQ PSG_update_done     ; Not playing, exit

LDX >PSG_MUSIC_PTR      ; Load pointer (force extended - LDX has no DP mode)
BEQ PSG_update_done     ; No music loaded

; Read frame count byte (number of register writes)
LDB ,X+
BEQ PSG_music_ended     ; Count=0 means end (no loop)
CMPB #$FF               ; Check for loop command
BEQ PSG_music_loop      ; $FF means loop (never valid as count)

; Process frame - push counter to stack
PSHS B                  ; Save count on stack

; Write register/value pairs to PSG
PSG_write_loop:
LDA ,X+                 ; Load register number
LDB ,X+                 ; Load register value
PSHS X                  ; Save pointer (after reads)

; WRITE_PSG sequence
STA VIA_port_a          ; Store register number
LDA #$19                ; BDIR=1, BC1=1 (LATCH)
STA VIA_port_b
LDA #$01                ; BDIR=0, BC1=0 (INACTIVE)
STA VIA_port_b
LDA VIA_port_a          ; Read status
STB VIA_port_a          ; Store data
LDB #$11                ; BDIR=1, BC1=0 (WRITE)
STB VIA_port_b
LDB #$01                ; BDIR=0, BC1=0 (INACTIVE)
STB VIA_port_b

PULS X                  ; Restore pointer
PULS B                  ; Get counter
DECB                    ; Decrement
BEQ PSG_frame_done      ; Done with this frame
PSHS B                  ; Save counter back
BRA PSG_write_loop

PSG_frame_done:

; Frame complete - update pointer and done
STX >PSG_MUSIC_PTR      ; Update pointer (force extended)
BRA PSG_update_done

PSG_music_ended:
CLR >PSG_IS_PLAYING     ; Stop playback (extended - var at 0xC8A0)
; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing
; Music will fade naturally as frame data stops updating
BRA PSG_update_done

PSG_music_loop:
; Loop command: $FF followed by 2-byte address (FDB)
; X points past $FF, read the target address
LDD ,X                  ; Load 2-byte loop target address
STD >PSG_MUSIC_PTR      ; Update pointer to loop start
; Exit - next frame will start from loop target
BRA PSG_update_done

PSG_update_done:
CLR >PSG_MUSIC_ACTIVE   ; Clear flag (music system done)
RTS

; ============================================================================
; STOP_MUSIC_RUNTIME - Stop music playback
; ============================================================================
STOP_MUSIC_RUNTIME:
CLR >PSG_IS_PLAYING     ; Clear playing flag (extended - var at 0xC8A0)
CLR >PSG_MUSIC_PTR      ; Clear pointer high byte (force extended)
CLR >PSG_MUSIC_PTR+1    ; Clear pointer low byte (force extended)
; NOTE: Do NOT write PSG registers here - corrupts VIA for vector drawing
RTS

; ============================================================================
; AUDIO_UPDATE - Unified music + SFX update (auto-injected after WAIT_RECAL)
; ============================================================================
; Processes both music (channel B) and SFX (channel C) in one pass
; Uses Sound_Byte (BIOS) for PSG writes - compatible with both systems
; Sets DP=$D0 once at entry, restores at exit
; RAM variables: PSG_MUSIC_PTR, PSG_IS_PLAYING, PSG_DELAY_FRAMES
;                SFX_PTR, SFX_ACTIVE (defined in SYSTEM RAM VARIABLES)

AUDIO_UPDATE:
PSHS DP                 ; Save current DP
LDA #$D0                ; Set DP=$D0 (Sound_Byte requirement)
TFR A,DP

; UPDATE MUSIC (channel B: registers 9, 11-14)
LDA >PSG_IS_PLAYING     ; Check if music is playing
BEQ AU_SKIP_MUSIC       ; Skip if not

; Check delay counter first
LDA >PSG_DELAY_FRAMES   ; Load delay counter
BEQ AU_MUSIC_READ       ; If zero, read next frame data
DECA                    ; Decrement delay
STA >PSG_DELAY_FRAMES   ; Store back
CMPA #0                 ; Check if it just reached zero
BNE AU_UPDATE_SFX       ; If not zero yet, skip this frame

; Delay just reached zero, X points to count byte already
LDX >PSG_MUSIC_PTR      ; Load music pointer (points to count)
BEQ AU_SKIP_MUSIC       ; Skip if null
BRA AU_MUSIC_READ_COUNT ; Skip delay read, go straight to count

AU_MUSIC_READ:
LDX >PSG_MUSIC_PTR      ; Load music pointer
BEQ AU_SKIP_MUSIC       ; Skip if null

; Check if we need to read delay or we're ready for count
; PSG_DELAY_FRAMES just reached 0, so we read delay byte first
LDB ,X+                 ; Read delay counter (X now points to count byte)
CMPB #$FF               ; Check for loop marker
BEQ AU_MUSIC_LOOP       ; Handle loop
CMPB #0                 ; Check if delay is 0
BNE AU_MUSIC_HAS_DELAY  ; If not 0, process delay

; Delay is 0, read count immediately
AU_MUSIC_NO_DELAY:
AU_MUSIC_READ_COUNT:
LDB ,X+                 ; Read count (number of register writes)
BEQ AU_MUSIC_ENDED      ; If 0, end of music
CMPB #$FF               ; Check for loop marker (can appear after delay)
BEQ AU_MUSIC_LOOP       ; Handle loop
BRA AU_MUSIC_PROCESS_WRITES

AU_MUSIC_HAS_DELAY:
; B has delay > 0, store it and skip to next frame
DECB                    ; Delay-1 (we consume this frame)
STB >PSG_DELAY_FRAMES   ; Save delay counter
STX >PSG_MUSIC_PTR      ; Save pointer (X points to count byte)
BRA AU_UPDATE_SFX       ; Skip reading data this frame

AU_MUSIC_PROCESS_WRITES:
PSHS B                  ; Save count

; Mark that next time we should read delay, not count
; (This is implicit - after processing, X points to next delay byte)

AU_MUSIC_WRITE_LOOP:
LDA ,X+                 ; Load register number
LDB ,X+                 ; Load register value
PSHS X                  ; Save pointer
JSR Sound_Byte          ; Write to PSG using BIOS (DP=$D0)
PULS X                  ; Restore pointer
PULS B                  ; Get counter
DECB                    ; Decrement
BEQ AU_MUSIC_DONE       ; Done if count=0
PSHS B                  ; Save counter
BRA AU_MUSIC_WRITE_LOOP ; Continue

AU_MUSIC_DONE:
STX >PSG_MUSIC_PTR      ; Update music pointer
BRA AU_UPDATE_SFX       ; Now update SFX

AU_MUSIC_ENDED:
CLR >PSG_IS_PLAYING     ; Stop music
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_MUSIC_LOOP:
LDD ,X                  ; Load loop target
STD >PSG_MUSIC_PTR      ; Set music pointer to loop
CLR >PSG_DELAY_FRAMES   ; Clear delay on loop
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_SKIP_MUSIC:
BRA AU_UPDATE_SFX       ; Skip music, go to SFX

; UPDATE SFX (channel C: registers 4/5=tone, 6=noise, 10=volume, 7=mixer)
AU_UPDATE_SFX:
LDA >SFX_ACTIVE         ; Check if SFX is active
BEQ AU_DONE             ; Skip if not active

JSR sfx_doframe         ; Process one SFX frame (uses Sound_Byte internally)

AU_DONE:
PULS DP                 ; Restore original DP
RTS

; ============================================================================
; AYFX SOUND EFFECTS PLAYER (Richard Chadd original system)
; ============================================================================
; Uses channel C (registers 4/5=tone, 6=noise, 10=volume, 7=mixer bit2/bit5)
; RAM variables: SFX_PTR (16-bit), SFX_ACTIVE (8-bit)
; AYFX format: flag byte + optional data per frame, end marker $D0 $20
; Flag bits: 0-3=volume, 4=disable tone, 5=tone data present,
;            6=noise data present, 7=disable noise
; ============================================================================

; PLAY_SFX_RUNTIME - Start SFX playback
; Input: X = pointer to AYFX data
PLAY_SFX_RUNTIME:
STX >SFX_PTR           ; Store pointer (force extended addressing)
LDA #$01
STA >SFX_ACTIVE        ; Mark as active
RTS

; SFX_UPDATE - Process one AYFX frame (call once per frame in loop)
SFX_UPDATE:
LDA >SFX_ACTIVE        ; Check if active
BEQ noay               ; Not active, skip
JSR sfx_doframe        ; Process one frame
noay:
RTS

; sfx_doframe - AYFX frame parser (Richard Chadd original)
sfx_doframe:
LDU >SFX_PTR           ; Get current frame pointer
LDB ,U                 ; Read flag byte (NO auto-increment)
CMPB #$D0              ; Check end marker (first byte)
BNE sfx_checktonefreq  ; Not end, continue
LDB 1,U                ; Check second byte at offset 1
CMPB #$20              ; End marker $D0 $20?
BEQ sfx_endofeffect    ; Yes, stop

sfx_checktonefreq:
LEAY 1,U               ; Y = pointer to tone/noise data
LDB ,U                 ; Reload flag byte (Sound_Byte corrupts B)
BITB #$20              ; Bit 5: tone data present?
BEQ sfx_checknoisefreq ; No, skip tone
; Set tone frequency (channel C = reg 4/5)
LDB 2,U                ; Get LOW byte (fine tune)
LDA #$04               ; Register 4
JSR Sound_Byte         ; Write to PSG
LDB 1,U                ; Get HIGH byte (coarse tune)
LDA #$05               ; Register 5
JSR Sound_Byte         ; Write to PSG
LEAY 2,Y               ; Skip 2 tone bytes

sfx_checknoisefreq:
LDB ,U                 ; Reload flag byte
BITB #$40              ; Bit 6: noise data present?
BEQ sfx_checkvolume    ; No, skip noise
LDB ,Y                 ; Get noise period
LDA #$06               ; Register 6
JSR Sound_Byte         ; Write to PSG
LEAY 1,Y               ; Skip 1 noise byte

sfx_checkvolume:
LDB ,U                 ; Reload flag byte
ANDB #$0F              ; Get volume from bits 0-3
LDA #$0A               ; Register 10 (volume C)
JSR Sound_Byte         ; Write to PSG

sfx_checktonedisable:
LDB ,U                 ; Reload flag byte
BITB #$10              ; Bit 4: disable tone?
BEQ sfx_enabletone
sfx_disabletone:
LDB $C807              ; Read mixer shadow (MUST be B register)
ORB #$04               ; Set bit 2 (disable tone C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG
BRA sfx_checknoisedisable  ; Continue to noise check

sfx_enabletone:
LDB $C807              ; Read mixer shadow (MUST be B register)
ANDB #$FB              ; Clear bit 2 (enable tone C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG

sfx_checknoisedisable:
LDB ,U                 ; Reload flag byte
BITB #$80              ; Bit 7: disable noise?
BEQ sfx_enablenoise
sfx_disablenoise:
LDB $C807              ; Read mixer shadow (MUST be B register)
ORB #$20               ; Set bit 5 (disable noise C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG
BRA sfx_nextframe      ; Done, update pointer

sfx_enablenoise:
LDB $C807              ; Read mixer shadow (MUST be B register)
ANDB #$DF              ; Clear bit 5 (enable noise C)
LDA #$07               ; Register 7 (mixer)
JSR Sound_Byte         ; Write to PSG

sfx_nextframe:
STY >SFX_PTR            ; Update pointer for next frame
RTS

sfx_endofeffect:
; Stop SFX - set volume to 0
CLR >SFX_ACTIVE         ; Mark as inactive
LDA #$0A                ; Register 10 (volume C)
LDB #$00                ; Volume = 0
JSR Sound_Byte
LDD #$0000
STD >SFX_PTR            ; Clear pointer
RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_3232159404:
    FCC "music1"
    FCB $80          ; Vectrex string terminator

