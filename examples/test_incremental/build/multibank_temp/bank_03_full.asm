
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
PSG_MUSIC_BANK       EQU $C880+$23   ; PSG music bank ID (for multibank) (1 bytes)
SFX_PTR              EQU $C880+$24   ; SFX data pointer (2 bytes)
SFX_ACTIVE           EQU $C880+$26   ; SFX active flag (1 bytes)
VAR_PLAYING          EQU $C880+$27   ; User variable: PLAYING (2 bytes)
VAR_TITLE_INTENSITY  EQU $C880+$29   ; User variable: TITLE_INTENSITY (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)
CURRENT_ROM_BANK     EQU $CFEA   ; Current ROM bank ID (multibank tracking) (1 bytes)


; ================================================
    ORG $0000  ; Sequential bank model
    ; Reserved for future code overflow


; ================================================
