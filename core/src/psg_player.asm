; PSG Direct Player (inspired by Christman2024 / malbanGit)
; Writes directly to PSG chip, bypassing BIOS music system
; Simpler format: Frame-based music data

; ============================================================================
; WRITE_PSG macro - Write to PSG register
; Input: A = register number (0-15), B = data value
; ============================================================================
WRITE_PSG           macro                                 
                    STA      VIA_port_a                  ; Store register number
                    LDA      #$19                        ; BDIR=1, BC1=1 (LATCH mode)
                    STA      VIA_port_b
                    LDA      #$01                        ; BDIR=0, BC1=0 (INACTIVE)
                    STA      VIA_port_b
                    LDA      VIA_port_a                  ; Read status (?)
                    STB      VIA_port_a                  ; Store data byte
                    LDB      #$11                        ; BDIR=1, BC1=0 (WRITE mode)
                    STB      VIA_port_b
                    LDB      #$01                        ; BDIR=0, BC1=0 (INACTIVE)
                    STB      VIA_port_b
                    endm

; ============================================================================
; PSG Music Data RAM Variables
; ============================================================================
PSG_MUSIC_PTR       EQU      RESULT+26                   ; Current position in music data (2 bytes)
PSG_FRAME_COUNTER   EQU      RESULT+28                   ; Current frame counter (1 byte)
PSG_IS_PLAYING      EQU      RESULT+29                   ; $00=stopped, $01=playing (1 byte)

; ============================================================================
; PLAY_MUSIC_PSG - Start PSG music playback
; Input: X = pointer to PSG music data
; Format: DW frame_count + frame_data[] + $FF end marker
; ============================================================================
PLAY_MUSIC_PSG:
    STX      PSG_MUSIC_PTR              ; Store music data pointer
    LDA      #$01
    STA      PSG_IS_PLAYING             ; Mark as playing
    CLR      PSG_FRAME_COUNTER          ; Reset frame counter
    RTS

; ============================================================================
; UPDATE_MUSIC_PSG - Update PSG (call every frame before Wait_Recal)
; Reads next frame of PSG register data and writes directly to chip
; ============================================================================
UPDATE_MUSIC_PSG:
    LDA      PSG_IS_PLAYING
    BEQ      PSG_update_done            ; Not playing, exit
    
    LDX      PSG_MUSIC_PTR
    BEQ      PSG_update_done            ; No music loaded
    
    ; Read frame count byte (number of register writes this frame)
    LDB      ,X+
    CMPB     #$FF                       ; Check for end marker
    BEQ      PSG_music_ended
    
    ; Write B register/value pairs to PSG
PSG_write_loop:
    LDA      ,X+                        ; Load register number
    PSHS     B                          ; Save counter
    LDB      ,X+                        ; Load register value
    PSHS     X                          ; Save pointer
    
    ; Write to PSG using macro pattern
    STA      VIA_port_a                  ; Store register number
    LDA      #$19                        ; BDIR=1, BC1=1 (LATCH)
    STA      VIA_port_b
    LDA      #$01                        ; BDIR=0, BC1=0 (INACTIVE)
    STA      VIA_port_b
    LDA      VIA_port_a                  ; Read status
    STB      VIA_port_a                  ; Store data
    LDB      #$11                        ; BDIR=1, BC1=0 (WRITE)
    STB      VIA_port_b
    LDB      #$01                        ; BDIR=0, BC1=0 (INACTIVE)
    STB      VIA_port_b
    
    PULS     X                          ; Restore pointer
    PULS     B                          ; Restore counter
    DECB
    BNE      PSG_write_loop
    
    ; Update pointer and continue
    STX      PSG_MUSIC_PTR
    BRA      PSG_update_done

PSG_music_ended:
    CLR      PSG_IS_PLAYING             ; Stop playback
    ; Silence all channels
    LDA      #7
    LDB      #$FF                        ; All channels off
    PSHS     X
    STA      VIA_port_a
    LDA      #$19
    STA      VIA_port_b
    LDA      #$01
    STA      VIA_port_b
    LDA      VIA_port_a
    STB      VIA_port_a
    LDB      #$11
    STB      VIA_port_b
    LDB      #$01
    STB      VIA_port_b
    PULS     X
    
PSG_update_done:
    RTS

; ============================================================================
; STOP_MUSIC_PSG - Stop PSG music playback
; ============================================================================
STOP_MUSIC_PSG:
    CLR      PSG_IS_PLAYING
    CLR      PSG_MUSIC_PTR
    CLR      PSG_MUSIC_PTR+1
    
    ; Silence PSG (write $00 to volume registers 8,9,10)
    LDA      #8
    LDB      #$00
    PSHS     X
    STA      VIA_port_a
    LDA      #$19
    STA      VIA_port_b
    LDA      #$01
    STA      VIA_port_b
    LDA      VIA_port_a
    STB      VIA_port_a
    LDB      #$11
    STB      VIA_port_b
    LDB      #$01
    STB      VIA_port_b
    
    LDA      #9
    LDB      #$00
    STA      VIA_port_a
    LDA      #$19
    STA      VIA_port_b
    LDA      #$01
    STA      VIA_port_b
    LDA      VIA_port_a
    STB      VIA_port_a
    LDB      #$11
    STB      VIA_port_b
    LDB      #$01
    STB      VIA_port_b
    
    LDA      #10
    LDB      #$00
    STA      VIA_port_a
    LDA      #$19
    STA      VIA_port_b
    LDA      #$01
    STA      VIA_port_b
    LDA      VIA_port_a
    STB      VIA_port_a
    LDB      #$11
    STB      VIA_port_b
    LDB      #$01
    STB      VIA_port_b
    PULS     X
    
    RTS
