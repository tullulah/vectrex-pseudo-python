#!/usr/bin/env python3
"""Replace SFX code in emission.rs with Richard Chadd AYFX system"""

# Read the file
with open('core/src/backend/m6809/emission.rs', 'r') as f:
    lines = f.readlines()

# Find start and end of SFX block
start_idx = None
end_idx = None

for i, line in enumerate(lines):
    if 'if w.contains("PLAY_SFX_RUNTIME")' in line:
        start_idx = i
    if start_idx is not None and i > start_idx and line.strip() == '}' and lines[i-1].strip().endswith(';'):
        end_idx = i
        break

if start_idx is None or end_idx is None:
    print(f"ERROR: Could not find SFX block. start={start_idx}, end={end_idx}")
    exit(1)

print(f"Found SFX block: lines {start_idx+1} to {end_idx+1}")

# New AYFX code (Richard Chadd system - 1 channel, simple)
new_code = '''    // PLAY_SFX_RUNTIME: AYFX player (Richard Chadd system - 1 channel, channel C)
    // Only emit if PLAY_SFX() builtin is actually used in code
    if w.contains("PLAY_SFX_RUNTIME") {
        out.push_str(
            "; ============================================================================\\n\\
            ; AYFX SOUND EFFECTS PLAYER (Richard Chadd original system)\\n\\
            ; ============================================================================\\n\\
            ; Uses channel C (registers 4/5=tone, 6=noise, 10=volume, 7=mixer bit2/bit5)\\n\\
            ; RAM variables: sfx_pointer (16-bit), sfx_status (8-bit)\\n\\
            ; AYFX format: flag byte + optional data per frame, end marker $D0 $20\\n\\
            ; Flag bits: 0-3=volume, 4=disable tone, 5=tone data present,\\n\\
            ;            6=noise data present, 7=disable noise\\n\\
            ; ============================================================================\\n\\
            \\n\\
            ; RAM variables for SFX\\n\\
            sfx_pointer EQU RESULT+32    ; 2 bytes - Current AYFX frame pointer\\n\\
            sfx_status  EQU RESULT+34    ; 1 byte  - Active flag (0=inactive, 1=active)\\n\\
            \\n\\
            ; PLAY_SFX_RUNTIME - Start SFX playback\\n\\
            ; Input: X = pointer to AYFX data\\n\\
            PLAY_SFX_RUNTIME:\\n\\
                STX sfx_pointer        ; Store pointer\\n\\
                LDA #$01\\n\\
                STA sfx_status         ; Mark as active\\n\\
                RTS\\n\\
            \\n\\
            ; SFX_UPDATE - Process one AYFX frame (call once per frame in loop)\\n\\
            SFX_UPDATE:\\n\\
                LDA sfx_status         ; Check if active\\n\\
                BEQ noay               ; Not active, skip\\n\\
                JSR sfx_doframe        ; Process one frame\\n\\
            noay:\\n\\
                RTS\\n\\
            \\n\\
            ; sfx_doframe - AYFX frame parser (Richard Chadd original)\\n\\
            sfx_doframe:\\n\\
                LDU sfx_pointer        ; Get current frame pointer\\n\\
                LDB ,U+                ; Read flag byte\\n\\
                CMPB #$D0              ; Check end marker (first byte)\\n\\
                BNE sfx_checktonefreq  ; Not end, continue\\n\\
                LDA ,U                 ; Check second byte\\n\\
                CMPA #$20              ; End marker $D0 $20?\\n\\
                BEQ sfx_endofeffect    ; Yes, stop\\n\\
            \\n\\
            sfx_checktonefreq:\\n\\
                LEAY 1,U               ; Y = pointer to next data/flag\\n\\
                BITB #$20              ; Bit 5: tone data present?\\n\\
                BEQ sfx_checknoisefreq ; No, skip tone\\n\\
                ; Set tone frequency (channel C = reg 4/5)\\n\\
                LDA 1,U                ; High byte\\n\\
                PSHS A\\n\\
                LDA #$04               ; Register 4 (tone C low)\\n\\
                JSR Sound_Byte\\n\\
                PULS A\\n\\
                LDB 2,U                ; Low byte\\n\\
                PSHS B\\n\\
                LDA #$05               ; Register 5 (tone C high)\\n\\
                LDB 2,U\\n\\
                JSR Sound_Byte\\n\\
                PULS B\\n\\
                LEAY 2,Y               ; Skip 2 bytes\\n\\
            \\n\\
            sfx_checknoisefreq:\\n\\
                BITB #$40              ; Bit 6: noise data present?\\n\\
                BEQ sfx_checkvolume    ; No, skip noise\\n\\
                LDA ,Y                 ; Get noise period\\n\\
                PSHS A\\n\\
                LDA #$06               ; Register 6 (noise period)\\n\\
                LDB ,Y\\n\\
                JSR Sound_Byte\\n\\
                PULS A\\n\\
                LEAY 1,Y               ; Skip 1 byte\\n\\
            \\n\\
            sfx_checkvolume:\\n\\
                TFR B,A                ; Get flag byte in A\\n\\
                ANDA #$0F              ; Extract volume (bits 0-3)\\n\\
                PSHS A\\n\\
                LDA #$0A               ; Register 10 (volume C)\\n\\
                LDB ,S+                ; Get volume\\n\\
                JSR Sound_Byte\\n\\
            \\n\\
            sfx_checktonedisable:\\n\\
                LDA $C807              ; Read mixer (reg 7 shadow)\\n\\
                BITB #$10              ; Bit 4: disable tone?\\n\\
                BEQ sfx_enabletone\\n\\
            sfx_disabletone:\\n\\
                ORA #$04               ; Set bit 2 (disable tone C)\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\
                BEQ sfx_enablenoise\\n\\
                ORA #$20               ; Set bit 5 (disable noise C)\\n\\
                PSHS A\\n\\
                LDA #$07               ; Register 7 (mixer)\\n\\
                LDB ,S+\\n\\
                JSR Sound_Byte\\n\\
                STY sfx_pointer        ; Update pointer\\n\\
                RTS\\n\\
            \\n\\
            sfx_enabletone:\\n\\
                ANDA #$FB              ; Clear bit 2 (enable tone C)\\n\\
                BITB #$80              ; Bit 7: disable noise?\\n\\
                BEQ sfx_enablenoise\\n\\
                ORA #$20               ; Set bit 5 (disable noise C)\\n\\
                PSHS A\\n\\
                LDA #$07\\n\\
                LDB ,S+\\n\\
                JSR Sound_Byte\\n\\
                STY sfx_pointer\\n\\
                RTS\\n\\
            \\n\\
            sfx_enablenoise:\\n\\
                ANDA #$DF              ; Clear bit 5 (enable noise C)\\n\\
                PSHS A\\n\\
                LDA #$07\\n\\
                LDB ,S+\\n\\
                JSR Sound_Byte\\n\\
                STY sfx_pointer\\n\\
                RTS\\n\\
            \\n\\
            sfx_endofeffect:\\n\\
                ; Stop SFX - set volume to 0\\n\\
                CLR sfx_status         ; Mark as inactive\\n\\
                LDA #$0A               ; Register 10 (volume C)\\n\\
                LDB #$00               ; Volume = 0\\n\\
                JSR Sound_Byte\\n\\
                LDD #$0000\\n\\
                STD sfx_pointer        ; Clear pointer\\n\\
                RTS\\n\\
            \\n"
        );
    }
'''

# Replace the block
new_lines = lines[:start_idx] + [new_code] + lines[end_idx+1:]

# Write back
with open('core/src/backend/m6809/emission.rs', 'w') as f:
    f.writelines(new_lines)

print(f"✓ Replaced SFX code (removed {end_idx - start_idx + 1} lines, added {len(new_code.split(chr(10)))} lines)")
print("✓ New AYFX system: Richard Chadd 1-channel (channel C)")
