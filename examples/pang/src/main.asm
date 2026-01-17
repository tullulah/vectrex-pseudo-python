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
    FCC "PANG"
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
; MAIN PROGRAM
;***************************************************************************

MAIN:
    ; Initialize global variables
    LDD #30
    STD VAR_title_intensity
    LDD #0
    STD VAR_title_state
    LDD #-1
    STD VAR_current_music
    LDD #0
    STD VAR_current_location
    LDD #60
    STD VAR_location_glow_intensity
    LDD #0
    STD VAR_location_glow_direction
    LDD #0
    STD VAR_joy_x
    LDD #0
    STD VAR_joy_y
    LDD #0
    STD VAR_prev_joy_x
    LDD #0
    STD VAR_prev_joy_y
    LDD #0
    STD VAR_countdown_timer
    LDD #0
    STD VAR_countdown_active
    LDD #0
    STD VAR_joystick_poll_counter
    LDD #0
    STD VAR_hook_active
    LDD #0
    STD VAR_hook_x
    LDD #-70
    STD VAR_hook_y
    LDD #0
    STD VAR_hook_gun_x
    LDD #0
    STD VAR_hook_gun_y
    LDD #0
    STD VAR_hook_init_y
    LDD #0
    STD VAR_player_x
    LDD #0
    STD VAR_move_speed
    LDD #0
    STD VAR_abs_joy
    LDD #1
    STD VAR_player_anim_frame
    LDD #0
    STD VAR_player_anim_counter
    LDD #1
    STD VAR_player_facing
    ; Call main() for initialization
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_current_location
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_prev_joy_x
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_prev_joy_y
    LDD #80
    STD RESULT
    LDD RESULT
    STD VAR_location_glow_intensity
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_location_glow_direction
    LDD VAR_STATE_TITLE
    STD RESULT
    LDD RESULT
    STD VAR_screen
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_countdown_timer
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_countdown_active
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_hook_active
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_hook_x
    LDD #-70
    STD RESULT
    LDD RESULT
    STD VAR_hook_y

.MAIN_LOOP:
    JSR LOOP_BODY
    BRA .MAIN_LOOP

LOOP_BODY:
    JSR Wait_Recal   ; Synchronize with screen refresh (mandatory)
    JSR Reset0Ref    ; Reset beam to center (0,0)
    JSR read_joystick1_state
    LDD VAR_screen
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_STATE_TITLE
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_current_music
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    ; PLAY_MUSIC: Play music from asset
    JSR PLAY_MUSIC_RUNTIME
    LDD #0
    STD RESULT
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_current_music
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    JSR draw_title_screen
    ; TODO: Expr Logic { op: Or, left: Logic { op: Or, left: Logic { op: Or, left: Compare { op: Eq, left: Index { target: Ident(IdentInfo { name: "joystick1_state", source_line: 108, col: 4 }), index: Number(2) }, right: Number(1) }, right: Compare { op: Eq, left: Index { target: Ident(IdentInfo { name: "joystick1_state", source_line: 108, col: 31 }), index: Number(3) }, right: Number(1) } }, right: Compare { op: Eq, left: Index { target: Ident(IdentInfo { name: "joystick1_state", source_line: 108, col: 58 }), index: Number(4) }, right: Number(1) } }, right: Compare { op: Eq, left: Index { target: Ident(IdentInfo { name: "joystick1_state", source_line: 108, col: 85 }), index: Number(5) }, right: Number(1) } }
    LDD #0
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_STATE_MAP
    STD RESULT
    LDD RESULT
    STD VAR_screen
    LDD #-1
    STD RESULT
    LDD RESULT
    STD VAR_current_music
    ; PLAY_SFX: Play sound effect
    JSR PLAY_SFX_RUNTIME
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    JSR AUDIO_UPDATE  ; Auto-injected: update music + SFX (after all game logic)
    RTS

; Function: draw_map_screen
draw_map_screen:
    ; SET_INTENSITY: Set drawing intensity
    LDD #80
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; DRAW_VECTOR_EX: Draw vector asset with transformations
    ; Asset: map (with mirror + intensity)
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD #20
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD #0
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE .DSVEX_CHK_Y
    LDA #1
    STA MIRROR_X
.DSVEX_CHK_Y:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE .DSVEX_CHK_XY
    LDA #1
    STA MIRROR_Y
.DSVEX_CHK_XY:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE .DSVEX_CALL
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
.DSVEX_CALL:
    ; Set intensity override for drawing
    LDD #50
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_MAP_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LDD VAR_location_glow_direction
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_location_glow_intensity
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_location_glow_intensity
    LDD VAR_location_glow_intensity
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #127
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGE .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_location_glow_direction
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LBRA .IF_END
.IF_ELSE:
    LDD VAR_location_glow_intensity
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #3
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_location_glow_intensity
    LDD VAR_location_glow_intensity
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #80
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLE .CMP_TRUE
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
    STD VAR_location_glow_direction
    LBRA .IF_END
.IF_ELSE:
.IF_END:
.IF_END:
    ; PRINT_TEXT: Print text at position
    LDD #-120
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-80
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #VAR_LOCATION_NAMES_DATA  ; Array data address
    PSHS X
    LDD VAR_current_location
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LDX #VAR_LOCATION_X_COORDS_DATA  ; Array data address
    PSHS X
    LDD VAR_current_location
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_loc_x
    LDX #VAR_LOCATION_Y_COORDS_DATA  ; Array data address
    PSHS X
    LDD VAR_current_location
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_loc_y
    ; DRAW_VECTOR_EX: Draw vector asset with transformations
    ; Asset: location_marker (with mirror + intensity)
    LDD VAR_loc_y
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_loc_x
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD #0
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE .DSVEX_CHK_Y
    LDA #1
    STA MIRROR_X
.DSVEX_CHK_Y:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE .DSVEX_CHK_XY
    LDA #1
    STA MIRROR_Y
.DSVEX_CHK_XY:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE .DSVEX_CALL
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
.DSVEX_CALL:
    ; Set intensity override for drawing
    LDD VAR_location_glow_intensity
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_LOCATION_MARKER_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    RTS

; Function: draw_title_screen
draw_title_screen:
    ; SET_INTENSITY: Set drawing intensity
    LDD #80
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: logo
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #70
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
    LDX #_LOGO_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    ; SET_INTENSITY: Set drawing intensity
    LDD VAR_title_intensity
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-90
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_9120385685437879118      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    ; PRINT_TEXT: Print text at position
    LDD #-50
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD #-20
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDX #PRINT_TEXT_STR_2382167728733      ; Pointer to string in helpers bank
    STX VAR_ARG2
    JSR VECTREX_PRINT_TEXT
    LDD #0
    STD RESULT
    LDD VAR_title_state
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_title_intensity
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD VAR_title_intensity
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_title_state
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_title_intensity
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD VAR_title_intensity
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_title_intensity
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #80
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_title_state
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_title_intensity
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #30
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
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
    STD VAR_title_state
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    RTS

; Function: draw_level_background
draw_level_background:
    ; SET_INTENSITY: Set drawing intensity
    LDD #60
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    LDD VAR_current_location
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: fuji_bg
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #50
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
    LDX #_FUJI_BG_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: easter_bg
    LDD #0
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDD #50
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
    LDX #_EASTER_BG_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
.IF_END:
    RTS

; Function: draw_game_level
draw_game_level:
    JSR draw_level_background
    LDX #VAR_JOYSTICK1_STATE_DATA  ; Array data address
    PSHS X
    LDD #0
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_joy_x
    ; TODO: Expr Logic { op: Or, left: Compare { op: Lt, left: Ident(IdentInfo { name: "joy_x", source_line: 304, col: 3 }), right: Number(-20) }, right: Compare { op: Gt, left: Ident(IdentInfo { name: "joy_x", source_line: 304, col: 18 }), right: Number(20) } }
    LDD #0
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_joy_x
    STD RESULT
    LDD RESULT
    STD VAR_abs_joy
    LDD VAR_abs_joy
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #-1
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_abs_joy
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    STD VAR_abs_joy
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_abs_joy
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #40
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_move_speed
    LBRA .IF_END
.IF_ELSE:
    LDD #4
    STD RESULT
    LDD RESULT
    STD VAR_move_speed
.IF_END:
    LDD VAR_joy_x
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #-1
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_move_speed
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    STD VAR_move_speed
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_player_x
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_move_speed
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_player_x
    LDD VAR_player_x
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-110
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #-110
    STD RESULT
    LDD RESULT
    STD VAR_player_x
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_player_x
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #110
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
    LDD #110
    STD RESULT
    LDD RESULT
    STD VAR_player_x
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_joy_x
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #0
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #-1
    STD RESULT
    LDD RESULT
    STD VAR_player_facing
    LBRA .IF_END
.IF_ELSE:
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_player_facing
.IF_END:
    LDD VAR_player_anim_counter
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_player_anim_counter
    LDD VAR_player_anim_speed
    STD RESULT
    LDD RESULT
    STD VAR_anim_threshold
    ; TODO: Expr Logic { op: Or, left: Compare { op: Lt, left: Ident(IdentInfo { name: "joy_x", source_line: 345, col: 3 }), right: Number(-80) }, right: Compare { op: Gt, left: Ident(IdentInfo { name: "joy_x", source_line: 345, col: 18 }), right: Number(80) } }
    LDD #0
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_player_anim_speed
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR DIV16   ; D = X / D
    STD RESULT
    LDD RESULT
    STD VAR_anim_threshold
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_player_anim_counter
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_anim_threshold
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGE .CMP_TRUE
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
    STD VAR_player_anim_counter
    LDD VAR_player_anim_frame
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_player_anim_frame
    LDD VAR_player_anim_frame
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #5
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
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_player_anim_frame
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LBRA .IF_END
.IF_ELSE:
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_player_anim_frame
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_player_anim_counter
.IF_END:
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_mirror_mode
    LDD VAR_player_facing
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD #1
    STD RESULT
    LDD RESULT
    STD VAR_mirror_mode
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_player_anim_frame
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    ; DRAW_VECTOR_EX: Draw vector asset with transformations
    ; Asset: player_walk_1 (with mirror + intensity)
    LDD VAR_player_x
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_player_y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD VAR_mirror_mode
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE .DSVEX_CHK_Y
    LDA #1
    STA MIRROR_X
.DSVEX_CHK_Y:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE .DSVEX_CHK_XY
    LDA #1
    STA MIRROR_Y
.DSVEX_CHK_XY:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE .DSVEX_CALL
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
.DSVEX_CALL:
    ; Set intensity override for drawing
    LDD #80
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_PLAYER_WALK_1_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
    ; DRAW_VECTOR_EX: Draw vector asset with transformations
    ; Asset: player_walk_5 (with mirror + intensity)
    LDD VAR_player_x
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_player_y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD VAR_mirror_mode
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE .DSVEX_CHK_Y
    LDA #1
    STA MIRROR_X
.DSVEX_CHK_Y:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE .DSVEX_CHK_XY
    LDA #1
    STA MIRROR_Y
.DSVEX_CHK_XY:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE .DSVEX_CALL
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
.DSVEX_CALL:
    ; Set intensity override for drawing
    LDD #80
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_PLAYER_WALK_5_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
.IF_END:
    JSR update_enemies
    JSR draw_enemies
    LDD VAR_hook_active
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_hook_gun_x
    STD RESULT
    LDD RESULT
    STD VAR_ARG0
    LDD VAR_hook_init_y
    STD RESULT
    LDD RESULT
    STD VAR_ARG1
    LDD VAR_hook_x
    STD RESULT
    LDD RESULT
    STD VAR_ARG2
    LDD VAR_hook_y
    STD RESULT
    LDD RESULT
    STD VAR_ARG3
    JSR draw_hook_rope
    ; SET_INTENSITY: Set drawing intensity
    LDD #100
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    ; DRAW_VECTOR_EX: Draw vector asset with transformations
    ; Asset: hook (with mirror + intensity)
    LDD VAR_hook_x
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA DRAW_VEC_X
    LDD VAR_hook_y
    STD RESULT
    LDA RESULT+1  ; Y position (low byte)
    STA DRAW_VEC_Y
    LDD #0
    STD RESULT
    LDB RESULT+1  ; Mirror mode (0=normal, 1=X, 2=Y, 3=both)
    ; Decode mirror mode into separate flags:
    CLR MIRROR_X  ; Clear X flag
    CLR MIRROR_Y  ; Clear Y flag
    CMPB #1       ; Check if X-mirror (mode 1)
    BNE .DSVEX_CHK_Y
    LDA #1
    STA MIRROR_X
.DSVEX_CHK_Y:
    CMPB #2       ; Check if Y-mirror (mode 2)
    BNE .DSVEX_CHK_XY
    LDA #1
    STA MIRROR_Y
.DSVEX_CHK_XY:
    CMPB #3       ; Check if both-mirror (mode 3)
    BNE .DSVEX_CALL
    LDA #1
    STA MIRROR_X
    STA MIRROR_Y
.DSVEX_CALL:
    ; Set intensity override for drawing
    LDD #100
    STD RESULT
    LDA RESULT+1  ; Intensity (0-127)
    STA DRAW_VEC_INTENSITY  ; Store intensity override
    JSR $F1AA        ; DP_to_D0 (set DP=$D0 for VIA access)
    LDX #_HOOK_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    CLR DRAW_VEC_INTENSITY  ; Clear intensity override for next draw
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_active_count
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_i
.WHILE_START:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_MAX_ENEMIES
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .WHILE_END
    LDX #VAR_ENEMY_ACTIVE_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_active_count
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_active_count
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_i
    LBRA .WHILE_START
.WHILE_END:
    RTS

; Function: spawn_enemies
spawn_enemies:
    LDX #VAR_LEVEL_ENEMY_COUNT_DATA  ; Array data address
    PSHS X
    LDD VAR_current_location
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_count
    LDX #VAR_LEVEL_ENEMY_SPEED_DATA  ; Array data address
    PSHS X
    LDD VAR_current_location
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    STD VAR_speed
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_i
.WHILE_START:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_count
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .WHILE_END
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_active_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #1
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_size_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #4
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_x_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #-80
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #50
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_y_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #60
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vx_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD VAR_speed
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #2
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MOD16   ; D = X % D
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vx_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #-1
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_speed
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vy_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #0
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_i
    LBRA .WHILE_START
.WHILE_END:
    RTS

; Function: update_enemies
update_enemies:
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_i
.WHILE_START:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_MAX_ENEMIES
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .WHILE_END
    LDX #VAR_ENEMY_ACTIVE_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vy_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDX #VAR_ENEMY_VY_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_GRAVITY
    STD RESULT
    LDD RESULT
    SUBD ,S++
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_x_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDX #VAR_ENEMY_X_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_ENEMY_VX_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_y_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDX #VAR_ENEMY_Y_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_ENEMY_VY_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDX #VAR_ENEMY_Y_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_GROUND_Y
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLE .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_y_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD VAR_GROUND_Y
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vy_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #-1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_ENEMY_VY_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vy_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDX #VAR_ENEMY_VY_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_BOUNCE_DAMPING
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #20
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR DIV16   ; D = X / D
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDX #VAR_ENEMY_VY_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_MIN_BOUNCE_VY
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vy_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD VAR_MIN_BOUNCE_VY
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDX #VAR_ENEMY_X_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #-85
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLE .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_x_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #-85
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vx_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #-1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_ENEMY_VX_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDX #VAR_ENEMY_X_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #85
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBGE .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_x_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #85
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD VAR_i
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_enemy_vx_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDD #-1
    STD RESULT
    LDD RESULT
    PSHS D
    LDX #VAR_ENEMY_VX_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PULS X      ; Get left into X
    JSR MUL16   ; D = X * D
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_i
    LBRA .WHILE_START
.WHILE_END:
    RTS

; Function: draw_enemies
draw_enemies:
    LDD #0
    STD RESULT
    LDD RESULT
    STD VAR_i
.WHILE_START:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD VAR_MAX_ENEMIES
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBLT .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .WHILE_END
    LDX #VAR_ENEMY_ACTIVE_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    ; SET_INTENSITY: Set drawing intensity
    LDD #80
    STD RESULT
    LDA RESULT+1    ; Load intensity (8-bit)
    JSR Intensity_a
    LDD #0
    STD RESULT
    LDX #VAR_ENEMY_SIZE_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #4
    STD RESULT
    LDD RESULT
    CMPD ,S++
    LBEQ .CMP_TRUE
    LDD #0
    LBRA .CMP_END
.CMP_TRUE:
    LDD #1
.CMP_END:
    STD RESULT
    LDD RESULT
    LBEQ .IF_ELSE
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: bubble_huge
    LDX #VAR_ENEMY_X_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDX #VAR_ENEMY_Y_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
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
    LDX #_BUBBLE_HUGE_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
    LBRA .IF_END
.IF_ELSE:
    ; DRAW_VECTOR: Draw vector asset at position
    ; Asset: bubble_small
    LDX #VAR_ENEMY_X_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
    STD RESULT
    LDA RESULT+1  ; X position (low byte)
    STA TMPPTR    ; Save X to temporary storage
    LDX #VAR_ENEMY_Y_DATA  ; Array data address
    PSHS X
    LDD VAR_i
    STD RESULT
    LDD RESULT  ; Index
    ASLB        ; Multiply by 2 (16-bit elements)
    ROLA
    PULS X      ; Array base
    LEAX D,X    ; X = base + (index * 2)
    LDD ,X      ; Load value
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
    LDX #_BUBBLE_SMALL_PATH0  ; Load first path
    JSR Draw_Sync_List_At_With_Mirrors
    JSR $F1AF        ; DP_to_C8 (restore DP for RAM access)
    LDD #0
    STD RESULT
.IF_END:
    LBRA .IF_END
.IF_ELSE:
.IF_END:
    LDD VAR_i
    STD RESULT
    LDD RESULT
    PSHS D
    LDD #1
    STD RESULT
    LDD RESULT
    ADDD ,S++
    STD RESULT
    LDD RESULT
    STD VAR_i
    LBRA .WHILE_START
.WHILE_END:
    RTS

; Function: draw_hook_rope
draw_hook_rope:
    ; DRAW_LINE: Draw line from (x0,y0) to (x1,y1)
    LDD VAR_start_x
    STD RESULT
    LDD RESULT
    STD TMPPTR+0    ; x0
    LDD VAR_start_y
    STD RESULT
    LDD RESULT
    STD TMPPTR+2    ; y0
    LDD VAR_end_x
    STD RESULT
    LDD RESULT
    STD TMPPTR+4    ; x1
    LDD VAR_end_y
    STD RESULT
    LDD RESULT
    STD TMPPTR+6    ; y1
    LDD #127
    STD RESULT
    LDD RESULT
    STD TMPPTR+8    ; intensity
    JSR DRAW_LINE_WRAPPER
    LDD #0
    STD RESULT
    RTS

; Function: read_joystick1_state
read_joystick1_state:
    LDD #0
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_joystick1_state_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    JSR J1X_BUILTIN
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #1
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_joystick1_state_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    JSR J1Y_BUILTIN
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #2
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_joystick1_state_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$01      ; Test bit 0
    BEQ .J1B1_OFF
    LDD #1
    BRA .J1B1_END
.J1B1_OFF:
    LDD #0
.J1B1_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #3
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_joystick1_state_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$02      ; Test bit 1
    BEQ .J1B2_OFF
    LDD #1
    BRA .J1B2_END
.J1B2_OFF:
    LDD #0
.J1B2_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #4
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_joystick1_state_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$04      ; Test bit 2
    BEQ .J1B3_OFF
    LDD #1
    BRA .J1B3_END
.J1B3_OFF:
    LDD #0
.J1B3_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    LDD #5
    STD RESULT
    LDD RESULT
    ASLB            ; Multiply index by 2 (16-bit elements)
    ROLA
    STD TMPPTR      ; Save offset temporarily
    LDD #VAR_joystick1_state_DATA  ; Load array data address
    TFR D,X         ; X = array base pointer
    LDD TMPPTR      ; D = offset
    LEAX D,X        ; X = base + offset
    STX TMPPTR2     ; Save computed address
    LDA $C811      ; Vec_Button_1_1 (transition bits)
    ANDA #$08      ; Test bit 3
    BEQ .J1B4_OFF
    LDD #1
    BRA .J1B4_END
.J1B4_OFF:
    LDD #0
.J1B4_END:
    STD RESULT
    LDX TMPPTR2     ; Load computed address
    LDD RESULT      ; Load value
    STD ,X          ; Store value
    RTS

;***************************************************************************
; === RAM VARIABLE DEFINITIONS ===
;***************************************************************************
RESULT               EQU $C880+$00   ; Main result temporary (2 bytes)
TMPPTR               EQU $C880+$02   ; Temporary pointer (2 bytes)
TMPPTR2              EQU $C880+$04   ; Temporary pointer 2 (2 bytes)
VLINE_DX_16          EQU $C880+$06   ; DRAW_LINE dx (16-bit) (2 bytes)
VLINE_DY_16          EQU $C880+$08   ; DRAW_LINE dy (16-bit) (2 bytes)
VLINE_DX             EQU $C880+$0A   ; DRAW_LINE dx clamped (8-bit) (1 bytes)
VLINE_DY             EQU $C880+$0B   ; DRAW_LINE dy clamped (8-bit) (1 bytes)
VLINE_DY_REMAINING   EQU $C880+$0C   ; DRAW_LINE remaining dy for segment 2 (1 bytes)
PSG_MUSIC_PTR        EQU $C880+$0D   ; PSG music data pointer (2 bytes)
PSG_IS_PLAYING       EQU $C880+$0F   ; PSG playing flag (1 bytes)
PSG_DELAY_FRAMES     EQU $C880+$10   ; PSG frame delay counter (1 bytes)
SFX_PTR              EQU $C880+$11   ; SFX data pointer (2 bytes)
SFX_ACTIVE           EQU $C880+$13   ; SFX active flag (1 bytes)
VAR_STATE_TITLE      EQU $C880+$14   ; User variable: STATE_TITLE (2 bytes)
VAR_level_enemy_speed EQU $C880+$16   ; User variable: level_enemy_speed (2 bytes)
VAR_MAX_ENEMIES      EQU $C880+$18   ; User variable: MAX_ENEMIES (2 bytes)
VAR_active_count     EQU $C880+$1A   ; User variable: active_count (2 bytes)
VAR_level_enemy_count EQU $C880+$1C   ; User variable: level_enemy_count (2 bytes)
VAR_countdown_active EQU $C880+$1E   ; User variable: countdown_active (2 bytes)
VAR_move_speed       EQU $C880+$20   ; User variable: move_speed (2 bytes)
VAR_STATE_GAME       EQU $C880+$22   ; User variable: STATE_GAME (2 bytes)
VAR_prev_joy_y       EQU $C880+$24   ; User variable: prev_joy_y (2 bytes)
VAR_start_x          EQU $C880+$26   ; User variable: start_x (2 bytes)
VAR_countdown_timer  EQU $C880+$28   ; User variable: countdown_timer (2 bytes)
VAR_hook_gun_y       EQU $C880+$2A   ; User variable: hook_gun_y (2 bytes)
VAR_hook_gun_x       EQU $C880+$2C   ; User variable: hook_gun_x (2 bytes)
VAR_enemy_active     EQU $C880+$2E   ; User variable: enemy_active (2 bytes)
VAR_location_glow_intensity EQU $C880+$30   ; User variable: location_glow_intensity (2 bytes)
VAR_end_y            EQU $C880+$32   ; User variable: end_y (2 bytes)
VAR_player_anim_speed EQU $C880+$34   ; User variable: player_anim_speed (2 bytes)
VAR_STATE_MAP        EQU $C880+$36   ; User variable: STATE_MAP (2 bytes)
VAR_loc_x            EQU $C880+$38   ; User variable: loc_x (2 bytes)
VAR_enemy_x          EQU $C880+$3A   ; User variable: enemy_x (2 bytes)
VAR_hook_active      EQU $C880+$3C   ; User variable: hook_active (2 bytes)
VAR_location_x_coords EQU $C880+$3E   ; User variable: location_x_coords (2 bytes)
VAR_player_anim_frame EQU $C880+$40   ; User variable: player_anim_frame (2 bytes)
VAR_num_locations    EQU $C880+$42   ; User variable: num_locations (2 bytes)
VAR_abs_joy          EQU $C880+$44   ; User variable: abs_joy (2 bytes)
VAR_BOUNCE_DAMPING   EQU $C880+$46   ; User variable: BOUNCE_DAMPING (2 bytes)
VAR_end_x            EQU $C880+$48   ; User variable: end_x (2 bytes)
VAR_current_location EQU $C880+$4A   ; User variable: current_location (2 bytes)
VAR_enemy_size       EQU $C880+$4C   ; User variable: enemy_size (2 bytes)
VAR_enemy_y          EQU $C880+$4E   ; User variable: enemy_y (2 bytes)
VAR_title_intensity  EQU $C880+$50   ; User variable: title_intensity (2 bytes)
VAR_start_y          EQU $C880+$52   ; User variable: start_y (2 bytes)
VAR_speed            EQU $C880+$54   ; User variable: speed (2 bytes)
VAR_hook_max_y       EQU $C880+$56   ; User variable: hook_max_y (2 bytes)
VAR_joystick1_state  EQU $C880+$58   ; User variable: joystick1_state (2 bytes)
VAR_prev_joy_x       EQU $C880+$5A   ; User variable: prev_joy_x (2 bytes)
VAR_title_state      EQU $C880+$5C   ; User variable: title_state (2 bytes)
VAR_enemy_vy         EQU $C880+$5E   ; User variable: enemy_vy (2 bytes)
VAR_i                EQU $C880+$60   ; User variable: i (2 bytes)
VAR_hook_y           EQU $C880+$62   ; User variable: hook_y (2 bytes)
VAR_joy_y            EQU $C880+$64   ; User variable: joy_y (2 bytes)
VAR_count            EQU $C880+$66   ; User variable: count (2 bytes)
VAR_GRAVITY          EQU $C880+$68   ; User variable: GRAVITY (2 bytes)
VAR_mirror_mode      EQU $C880+$6A   ; User variable: mirror_mode (2 bytes)
VAR_player_y         EQU $C880+$6C   ; User variable: player_y (2 bytes)
VAR_player_facing    EQU $C880+$6E   ; User variable: player_facing (2 bytes)
VAR_location_glow_direction EQU $C880+$70   ; User variable: location_glow_direction (2 bytes)
VAR_location_names   EQU $C880+$72   ; User variable: location_names (2 bytes)
VAR_MIN_BOUNCE_VY    EQU $C880+$74   ; User variable: MIN_BOUNCE_VY (2 bytes)
VAR_enemy_vx         EQU $C880+$76   ; User variable: enemy_vx (2 bytes)
VAR_hook_x           EQU $C880+$78   ; User variable: hook_x (2 bytes)
VAR_player_anim_counter EQU $C880+$7A   ; User variable: player_anim_counter (2 bytes)
VAR_anim_threshold   EQU $C880+$7C   ; User variable: anim_threshold (2 bytes)
VAR_current_music    EQU $C880+$7E   ; User variable: current_music (2 bytes)
VAR_hook_init_y      EQU $C880+$80   ; User variable: hook_init_y (2 bytes)
VAR_location_y_coords EQU $C880+$82   ; User variable: location_y_coords (2 bytes)
VAR_joystick_poll_counter EQU $C880+$84   ; User variable: joystick_poll_counter (2 bytes)
VAR_loc_y            EQU $C880+$86   ; User variable: loc_y (2 bytes)
VAR_joy_x            EQU $C880+$88   ; User variable: joy_x (2 bytes)
VAR_screen           EQU $C880+$8A   ; User variable: screen (2 bytes)
VAR_player_x         EQU $C880+$8C   ; User variable: player_x (2 bytes)
VAR_GROUND_Y         EQU $C880+$8E   ; User variable: GROUND_Y (2 bytes)
VAR_ARG0             EQU $CFE0   ; Function argument 0 (16-bit) (2 bytes)
VAR_ARG1             EQU $CFE2   ; Function argument 1 (16-bit) (2 bytes)
VAR_ARG2             EQU $CFE4   ; Function argument 2 (16-bit) (2 bytes)
VAR_ARG3             EQU $CFE6   ; Function argument 3 (16-bit) (2 bytes)
VAR_ARG4             EQU $CFE8   ; Function argument 4 (16-bit) (2 bytes)
VAR_joystick1_state_DATA EQU $C910   ; Array data: joystick1_state (12 bytes)
VAR_enemy_active_DATA EQU $C91C   ; Array data: enemy_active (16 bytes)
VAR_enemy_x_DATA     EQU $C92C   ; Array data: enemy_x (16 bytes)
VAR_enemy_y_DATA     EQU $C93C   ; Array data: enemy_y (16 bytes)
VAR_enemy_vx_DATA    EQU $C94C   ; Array data: enemy_vx (16 bytes)
VAR_enemy_vy_DATA    EQU $C95C   ; Array data: enemy_vy (16 bytes)
VAR_enemy_size_DATA  EQU $C96C   ; Array data: enemy_size (16 bytes)

;***************************************************************************
; ARRAY DATA
;***************************************************************************

; Array data storage
    ORG $C910  ; Start of array data section
; Array: VAR_joystick1_state_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
; Array: VAR_enemy_active_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
    FDB 0    ; Element 6
    FDB 0    ; Element 7
; Array: VAR_enemy_x_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
    FDB 0    ; Element 6
    FDB 0    ; Element 7
; Array: VAR_enemy_y_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
    FDB 0    ; Element 6
    FDB 0    ; Element 7
; Array: VAR_enemy_vx_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
    FDB 0    ; Element 6
    FDB 0    ; Element 7
; Array: VAR_enemy_vy_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
    FDB 0    ; Element 6
    FDB 0    ; Element 7
; Array: VAR_enemy_size_DATA
    FDB 0    ; Element 0
    FDB 0    ; Element 1
    FDB 0    ; Element 2
    FDB 0    ; Element 3
    FDB 0    ; Element 4
    FDB 0    ; Element 5
    FDB 0    ; Element 6
    FDB 0    ; Element 7

; Internal builtin variables (aliases to RESULT slots)
DRAW_VEC_X EQU RESULT+0
DRAW_VEC_Y EQU RESULT+2
MIRROR_X EQU RESULT+4
MIRROR_Y EQU RESULT+6
DRAW_VEC_INTENSITY EQU RESULT+8

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

MUL16:
    ; Multiply 16-bit X * D -> D
    ; Simple implementation (can be optimized)
    PSHS X,B,A
    LDD #0         ; Result accumulator
    LDX 2,S        ; Multiplier
.MUL16_LOOP:
    BEQ .MUL16_END
    ADDD ,S        ; Add multiplicand
    LEAX -1,X
    BRA .MUL16_LOOP
.MUL16_END:
    LEAS 4,S
    RTS

DIV16:
    ; Divide 16-bit X / D -> D
    ; Simple implementation
    PSHS X,D
    LDD #0         ; Quotient
.DIV16_LOOP:
    PSHS D         ; Save quotient
    LDD 4,S        ; Load dividend (after PSHS D)
    CMPD 2,S       ; Compare with divisor (after PSHS D)
    PULS D         ; Restore quotient
    BLT .DIV16_END
    ADDD #1        ; Increment quotient
    LDX 2,S
    PSHS D
    LDD 2,S        ; Divisor
    LEAX D,X       ; Subtract divisor
    STX 4,S
    PULS D
    BRA .DIV16_LOOP
.DIV16_END:
    LEAS 4,S
    RTS

J1X_BUILTIN:
    ; Read J1_X from $CF00 and return -1/0/+1
    LDB $CF00      ; Joy_1_X (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J1X_LEFT  ; Branch if <108 (left)
    CMPB #148      ; Compare with upper threshold
    BHI .J1X_RIGHT ; Branch if >148 (right)
    ; Center (108-148)
    LDD #0
    RTS
.J1X_LEFT:
    LDD #-1
    RTS
.J1X_RIGHT:
    LDD #1
    RTS

J1Y_BUILTIN:
    ; Read J1_Y from $CF01 and return -1/0/+1
    LDB $CF01      ; Joy_1_Y (unsigned byte 0-255)
    CMPB #108      ; Compare with lower threshold
    BLO .J1Y_DOWN  ; Branch if <108 (down)
    CMPB #148      ; Compare with upper threshold
    BHI .J1Y_UP    ; Branch if >148 (up)
    ; Center (108-148)
    LDD #0
    RTS
.J1Y_DOWN:
    LDD #-1
    RTS
.J1Y_UP:
    LDD #1
    RTS

DRAW_LINE_WRAPPER:
    ; Line drawing wrapper with segmentation for lines > 127 pixels
    ; Args: TMPPTR+0=x0, TMPPTR+2=y0, TMPPTR+4=x1, TMPPTR+6=y1, TMPPTR+8=intensity
    ; Calculate deltas (16-bit signed)
    LDD TMPPTR+4    ; x1
    SUBD TMPPTR+0   ; x1 - x0
    STD VLINE_DX_16 ; Store 16-bit dx
    
    LDD TMPPTR+6    ; y1
    SUBD TMPPTR+2   ; y1 - y0
    STD VLINE_DY_16 ; Store 16-bit dy
    
    ; === SEGMENT 1: Clamp deltas to ±127 ===
    ; Check dy: if > 127, clamp to 127; if < -128, clamp to -128
    LDD VLINE_DY_16
    CMPD #127       ; Compare with max positive
    LBLE DLW_SEG1_DY_LO ; Branch if <= 127
    LDD #127        ; Clamp to 127
    STD VLINE_DY_16
DLW_SEG1_DY_LO:
    LDD VLINE_DY_16
    CMPD #-128      ; Compare with min negative
    LBGE DLW_SEG1_DY_READY ; Branch if >= -128
    LDD #-128       ; Clamp to -128
    STD VLINE_DY_16
DLW_SEG1_DY_READY:
    LDB VLINE_DY_16+1 ; Load low byte (8-bit clamped)
    STB VLINE_DY
    
    ; Check dx: if > 127, clamp to 127; if < -128, clamp to -128
    LDD VLINE_DX_16
    CMPD #127
    LBLE DLW_SEG1_DX_LO
    LDD #127
    STD VLINE_DX_16
DLW_SEG1_DX_LO:
    LDD VLINE_DX_16
    CMPD #-128
    LBGE DLW_SEG1_DX_READY
    LDD #-128
    STD VLINE_DX_16
DLW_SEG1_DX_READY:
    LDB VLINE_DX_16+1 ; Load low byte (8-bit clamped)
    STB VLINE_DX
    
    ; Set intensity
    LDA TMPPTR+8+1  ; Load intensity (low byte)
    JSR Intensity_a
    
    ; Move to start position (x0, y0)
    CLR Vec_Misc_Count
    LDA TMPPTR+2+1  ; y0 (low byte)
    LDB TMPPTR+0+1  ; x0 (low byte)
    JSR Moveto_d
    
    ; Draw first segment (clamped deltas)
    LDA VLINE_DY    ; 8-bit clamped dy
    LDB VLINE_DX    ; 8-bit clamped dx
    JSR Draw_Line_d
    
    ; === CHECK IF SEGMENT 2 NEEDED ===
    ; Original dy still in VLINE_DY_16, check if exceeds ±127
    LDD TMPPTR+6    ; Reload original y1
    SUBD TMPPTR+2   ; y1 - y0
    CMPD #127
    LBGT DLW_NEED_SEG2 ; dy > 127
    CMPD #-128
    LBLT DLW_NEED_SEG2 ; dy < -128
    LBRA DLW_DONE   ; No second segment needed
    
DLW_NEED_SEG2:
    ; Calculate remaining dy
    LDD TMPPTR+6    ; y1
    SUBD TMPPTR+2   ; y1 - y0
    ; Check sign: if positive, subtract 127; if negative, add 128
    CMPD #0
    LBGE DLW_SEG2_DY_POS
    ADDD #128       ; dy was negative, add 128
    LBRA DLW_SEG2_DY_DONE
DLW_SEG2_DY_POS:
    SUBD #127       ; dy was positive, subtract 127
DLW_SEG2_DY_DONE:
    STD VLINE_DY_REMAINING
    
    ; Draw second segment (remaining dy, dx=0)
    LDA VLINE_DY_REMAINING ; Low byte of remaining (it's already 8-bit)
    LDB #0          ; dx = 0 for vertical segment
    JSR Draw_Line_d
    
DLW_DONE:
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
LDA PSG_IS_PLAYING     ; Check if music is playing
BEQ AU_SKIP_MUSIC       ; Skip if not

; Check delay counter first
LDA PSG_DELAY_FRAMES   ; Load delay counter
BEQ AU_MUSIC_READ       ; If zero, read next frame data
DECA                    ; Decrement delay
STA PSG_DELAY_FRAMES   ; Store back
CMPA #0                 ; Check if it just reached zero
BNE AU_UPDATE_SFX       ; If not zero yet, skip this frame

; Delay just reached zero, X points to count byte already
LDX PSG_MUSIC_PTR      ; Load music pointer (points to count)
BEQ AU_SKIP_MUSIC       ; Skip if null
BRA AU_MUSIC_READ_COUNT ; Skip delay read, go straight to count

AU_MUSIC_READ:
LDX PSG_MUSIC_PTR      ; Load music pointer
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
STB PSG_DELAY_FRAMES   ; Save delay counter
STX PSG_MUSIC_PTR      ; Save pointer (X points to count byte)
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
STX PSG_MUSIC_PTR      ; Update music pointer
BRA AU_UPDATE_SFX       ; Now update SFX

AU_MUSIC_ENDED:
CLR PSG_IS_PLAYING     ; Stop music
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_MUSIC_LOOP:
LDD ,X                  ; Load loop target
STD PSG_MUSIC_PTR      ; Set music pointer to loop
CLR PSG_DELAY_FRAMES   ; Clear delay on loop
BRA AU_UPDATE_SFX       ; Continue to SFX

AU_SKIP_MUSIC:
BRA AU_UPDATE_SFX       ; Skip music, go to SFX

; UPDATE SFX (channel C: registers 4/5=tone, 6=noise, 10=volume, 7=mixer)
AU_UPDATE_SFX:
LDA SFX_ACTIVE         ; Check if SFX is active
BEQ AU_DONE             ; Skip if not active

JSR sfx_doframe         ; Process one SFX frame (uses Sound_Byte internally)

AU_DONE:
PULS DP                 ; Restore original DP
RTS

; ============================================================================
; sfx_doframe - AYFX frame parser (Richard Chadd original)
; ============================================================================
; Process one SFX frame - called by AUDIO_UPDATE
; Uses Sound_Byte BIOS call for PSG writes (DP=$D0 already set by caller)
; AYFX format: flag byte + optional data per frame, end marker $D0 $20
; Flag bits: 0-3=volume, 4=disable tone, 5=tone data present,
;            6=noise data present, 7=disable noise

sfx_doframe:
LDU SFX_PTR            ; Get current frame pointer
LDB ,U                  ; Read flag byte (NO auto-increment)
CMPB #$D0               ; Check end marker (first byte)
BNE sfx_checktonefreq   ; Not end, continue
LDB 1,U                 ; Check second byte at offset 1
CMPB #$20               ; End marker $D0 $20?
BEQ sfx_endofeffect     ; Yes, stop

sfx_checktonefreq:
LEAY 1,U                ; Y = pointer to tone/noise data
LDB ,U                  ; Reload flag byte (Sound_Byte corrupts B)
BITB #$20               ; Bit 5: tone data present?
BEQ sfx_checknoisefreq  ; No, skip tone
; Set tone frequency (channel C = reg 4/5)
LDB 2,U                 ; Get LOW byte (fine tune)
LDA #$04                ; Register 4
JSR Sound_Byte          ; Write to PSG
LDB 1,U                 ; Get HIGH byte (coarse tune)
LDA #$05                ; Register 5
JSR Sound_Byte          ; Write to PSG
LEAY 2,Y                ; Skip 2 tone bytes

sfx_checknoisefreq:
LDB ,U                  ; Reload flag byte
BITB #$40               ; Bit 6: noise data present?
BEQ sfx_checkvolume     ; No, skip noise
LDB ,Y                  ; Get noise period
LDA #$06                ; Register 6
JSR Sound_Byte          ; Write to PSG
LEAY 1,Y                ; Skip 1 noise byte

sfx_checkvolume:
LDB ,U                  ; Reload flag byte
ANDB #$0F               ; Get volume from bits 0-3
LDA #$0A                ; Register 10 (volume C)
JSR Sound_Byte          ; Write to PSG

sfx_checktonedisable:
LDB ,U                  ; Reload flag byte
BITB #$10               ; Bit 4: disable tone?
BEQ sfx_enabletone
sfx_disabletone:
LDB $C807               ; Read mixer shadow (MUST be B register)
ORB #$04                ; Set bit 2 (disable tone C)
LDA #$07                ; Register 7 (mixer)
JSR Sound_Byte          ; Write to PSG
BRA sfx_checknoisedisable  ; Continue to noise check

sfx_enabletone:
LDB $C807               ; Read mixer shadow (MUST be B register)
ANDB #$FB               ; Clear bit 2 (enable tone C)
LDA #$07                ; Register 7 (mixer)
JSR Sound_Byte          ; Write to PSG

sfx_checknoisedisable:
LDB ,U                  ; Reload flag byte
BITB #$80               ; Bit 7: disable noise?
BEQ sfx_enablenoise
sfx_disablenoise:
LDB $C807               ; Read mixer shadow (MUST be B register)
ORB #$20                ; Set bit 5 (disable noise C)
LDA #$07                ; Register 7 (mixer)
JSR Sound_Byte          ; Write to PSG
BRA sfx_nextframe       ; Done, update pointer

sfx_enablenoise:
LDB $C807               ; Read mixer shadow (MUST be B register)
ANDB #$DF               ; Clear bit 5 (enable noise C)
LDA #$07                ; Register 7 (mixer)
JSR Sound_Byte          ; Write to PSG

sfx_nextframe:
STY SFX_PTR            ; Update pointer for next frame
RTS

sfx_endofeffect:
; Stop SFX - set volume to 0
CLR SFX_ACTIVE         ; Mark as inactive
LDA #$0A                ; Register 10 (volume C)
LDB #$00                ; Volume = 0
JSR Sound_Byte
LDD #$0000
STD SFX_PTR            ; Clear pointer
RTS

;**** PRINT_TEXT String Data ****
PRINT_TEXT_STR_2382167728733:
    FCC "TO START"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_62529178322969:
    FCC "GET READY"
    FCB $80          ; Vectrex string terminator

PRINT_TEXT_STR_9120385685437879118:
    FCC "PRESS A BUTTON"
    FCB $80          ; Vectrex string terminator

